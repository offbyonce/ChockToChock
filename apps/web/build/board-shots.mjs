// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Screenshots of the approved boards (docs/design/boards), for the board-against-app side-by-sides
// every reviewer brief needs (AGENTS.md, interface design). The boards load a canvas runtime that
// is not part of the handoff, so this renders them itself: it runs each board's renderVals() for
// its props, fills the {{ … }} holes, keeps or drops <sc-if> blocks and inlines <dc-import>ed
// boards, with the bundled fonts instead of the remote ones. Writes <board>-<theme>.png into
// <out-dir> (outside the repo):
//   node apps/web/build/board-shots.mjs <out-dir> [Board ...]
// Needs Firefox.

import fs from "node:fs";
import path from "node:path";
import { serve, session, sleep, startFirefox } from "./browser.mjs";

const web = path.resolve(import.meta.dirname, "..");
const root = path.resolve(web, "../..");
const boards = path.join(root, "docs/design/boards");
/** Round 6's boards (docs/design/round6), which import each other from their own folder. */
const round6 = path.join(root, "docs/design/round6/boards");
const work = path.join(root, "target/board-shots");
const BIDI = 9264;
const PORT = 8776;

const camel = (name) => name.replace(/-([a-z])/g, (_, c) => c.toUpperCase());

/** A board's props: its declared defaults, then those given. */
function propsOf(text, given) {
  const m = /data-props='([^']*)'/.exec(text);
  const declared = m ? JSON.parse(m[1]) : {};
  const props = {};
  for (const [k, v] of Object.entries(declared)) if (!k.startsWith("$") && v && "default" in v) props[k] = v.default;
  return { ...props, ...given };
}

/** The values a board's template sees: renderVals() for these props, plus the props. */
function valuesOf(text, props) {
  const m = /<script type="text\/x-dc"[^>]*>([\s\S]*?)<\/script>/.exec(text);
  if (!m) return { ...props };
  const code = `class DCLogic { constructor(props) { this.props = props; } }\n${m[1]}\nreturn new Component(props).renderVals();`;
  return { ...props, ...new Function("props", code)(props) };
}

const evaluate = (expr, vals) => {
  try {
    return new Function(...Object.keys(vals), `return (${expr});`)(...Object.values(vals));
  } catch {
    return "";
  }
};

/** One board as HTML, for these props. */
function render(name, given = {}, dir = boards) {
  const text = fs.readFileSync(path.join(dir, `${name}.dc.html`), "utf8");
  const props = propsOf(text, given);
  const vals = valuesOf(text, props);
  let body = text.slice(text.indexOf("<x-dc>") + 6, text.indexOf("</x-dc>"));
  body = body.replace(/<helmet>[\s\S]*?<\/helmet>/, "");
  // Conditional blocks, innermost first, before imports, so a board imported inside a
  // false block is never rendered.
  const sc = /<sc-if value="\{\{\s*([^"]*?)\s*\}\}"[^>]*>((?:(?!<sc-if)[\s\S])*?)<\/sc-if>/;
  while (sc.test(body)) body = body.replace(sc, (_, e, inner) => (evaluate(e, vals) ? inner : ""));
  // Then imports: each renders its board with its attributes as props.
  body = body.replace(/<dc-import ([^>]*)><\/dc-import>/g, (_, attrs) => {
    const a = Object.fromEntries([...attrs.matchAll(/([\w-]+)="([^"]*)"/g)].map(([, k, v]) => [camel(k), v.replace(/\{\{\s*([\s\S]*?)\s*\}\}/g, (_, e) => String(evaluate(e, vals)))]));
    const { name: inner, hintSize: _hint, ...rest } = a;
    for (const k of Object.keys(rest)) if (rest[k] === "true" || rest[k] === "false") rest[k] = rest[k] === "true";
    return render(inner, rest, dir);
  });
  return body.replace(/\{\{\s*([\s\S]*?)\s*\}\}/g, (_, e) => String(evaluate(e, vals)));
}

function page(name, given, dir) {
  return `<!doctype html><html lang="en"><head><meta charset="utf-8"><link rel="stylesheet" href="fonts/fonts.css"><style>body{margin:0}</style></head><body>${render(name, given, dir)}</body></html>`;
}

/** Each board shot: file name, board, props, viewport. */
const SHOTS = [
  ["Checklist", {}, 1440, 900],
  ["MapHidden", { mapHidden: true }, 1440, 900, "Checklist"],
  ["WherePop", {}, 1440, 900],
  ["Settings", { view: "settings" }, 1440, 900, "Views"],
  ["Flow", { view: "flow" }, 1440, 900, "Views"],
  ["TabL", { orient: "landscape", overlay: "none" }, 1180, 820, "Tablet"],
  ["TabWhere", { orient: "landscape", overlay: "where" }, 1180, 820, "Tablet"],
  ["PhoneWhere", { screen: "where" }, 390, 844, "Mobile"],
  ["Wide", {}, 2560, 1440],
  // Round 6 (docs/design/round6/boards).
  ["R6PanePre", { r6: true, preview: true }, 1440, 900, "Checklist", round6],
  ["R6Panel", { layout: "desktop", state: "panel" }, 1440, 900, "Locator", round6],
  ["R6Where", { layout: "desktop", state: "where" }, 1440, 900, "Locator", round6],
  ["R6Control", { layout: "desktop", state: "control" }, 1440, 900, "Locator", round6],
  ["R6TabL", { layout: "tabletL", state: "panel" }, 1180, 820, "Locator", round6],
];

async function main() {
  const out = process.argv[2] ? path.resolve(process.argv[2]) : null;
  if (!out) throw new Error("usage: board-shots.mjs <out-dir> [Board ...]");
  const only = process.argv.slice(3);
  fs.rmSync(work, { recursive: true, force: true });
  fs.mkdirSync(path.join(work, "fonts"), { recursive: true });
  fs.mkdirSync(out, { recursive: true });
  for (const f of fs.readdirSync(path.join(web, "src/fonts"))) fs.copyFileSync(path.join(web, "src/fonts", f), path.join(work, "fonts", f));
  const shots = SHOTS.filter(([n]) => !only.length || only.includes(n));
  for (const theme of ["dark", "light"]) {
    for (const [file, given, , , board, dir] of shots) fs.writeFileSync(path.join(work, `${file}-${theme}.html`), page(board ?? file, { ...given, theme }, dir));
  }
  const server = await serve(work, PORT);
  try {
    for (const [file, , width, height] of shots) {
      const firefox = await startFirefox(path.join(work, `profile-${file}`), BIDI, {}, { width: Math.max(width, 800), height: height + 100 }, [
        "-remote-allow-system-access",
      ]);
      try {
        const browser = await session(BIDI);
        await browser.send("browsingContext.navigate", { context: browser.context, url: `http://localhost:${PORT}/${file}-dark.html`, wait: "complete" });
        await browser.send("browsingContext.setViewport", { context: browser.context, viewport: { width, height } });
        for (const theme of ["dark", "light"]) {
          await browser.send("browsingContext.navigate", { context: browser.context, url: `http://localhost:${PORT}/${file}-${theme}.html`, wait: "complete" });
          await browser.js("document.fonts.ready.then(() => true)");
          await sleep(300);
          const { data } = await browser.send("browsingContext.captureScreenshot", { context: browser.context });
          fs.writeFileSync(path.join(out, `${file}-${theme}.png`), Buffer.from(data, "base64"));
          console.log(`${file}-${theme}.png`);
        }
      } finally {
        firefox.kill();
        await sleep(500);
      }
    }
  } finally {
    server.close();
  }
}

await main();
