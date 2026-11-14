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

/** The index just past the `</tag>` that closes the block opening at `from`, counting nested blocks. */
function closeOf(body, tag, from) {
  const re = new RegExp(`<${tag}[\\s>]|</${tag}>`, "g");
  re.lastIndex = from + 1;
  let depth = 1;
  for (let m = re.exec(body); m; m = re.exec(body)) {
    depth += m[0].startsWith("</") ? -1 : 1;
    if (depth === 0) return { inner: body.indexOf(">", from) + 1, end: m.index, after: m.index + m[0].length };
  }
  throw new Error(`unclosed <${tag}>`);
}

/**
 * Fills a template: <sc-for list="{{ … }}" as="x"> repeats its body for each item (with x in
 * scope), <sc-if value="{{ … }}"> keeps or drops its body, outermost first so loop variables
 * reach the blocks inside; then <dc-import>ed boards and the {{ … }} holes.
 */
function fill(body, vals, dir) {
  let out = "";
  for (;;) {
    const m = /<sc-(for|if)\s([^>]*)>/.exec(body);
    if (!m) break;
    const head = body.slice(0, m.index);
    out += plain(head, vals, dir);
    const tag = `sc-${m[1]}`;
    const { inner, end, after } = closeOf(body, tag, m.index);
    const block = body.slice(inner, end);
    const attr = (k) => new RegExp(`${k}="([^"]*)"`).exec(m[2])?.[1] ?? "";
    const expr = attr(m[1] === "for" ? "list" : "value").replace(/^\{\{\s*|\s*\}\}$/g, "");
    if (m[1] === "for") {
      const list = evaluate(expr, vals);
      const as = attr("as");
      for (const item of Array.isArray(list) ? list : []) out += fill(block, { ...vals, [as]: item }, dir);
    } else if (evaluate(expr, vals)) {
      out += fill(block, vals, dir);
    }
    body = body.slice(after);
  }
  return out + plain(body, vals, dir);
}

/** Imports and holes in a stretch with no blocks left in it. */
function plain(text, vals, dir) {
  text = text.replace(/<dc-import ([^>]*)><\/dc-import>/g, (_, attrs) => {
    const a = Object.fromEntries([...attrs.matchAll(/([\w-]+)="([^"]*)"/g)].map(([, k, v]) => [camel(k), v.replace(/\{\{\s*([\s\S]*?)\s*\}\}/g, (_, e) => String(evaluate(e, vals)))]));
    const { name: inner, hintSize: _hint, ...rest } = a;
    for (const k of Object.keys(rest)) if (rest[k] === "true" || rest[k] === "false") rest[k] = rest[k] === "true";
    return render(inner, rest, dir);
  });
  return text.replace(/\{\{\s*([\s\S]*?)\s*\}\}/g, (_, e) => String(evaluate(e, vals)));
}

/** One board as HTML, for these props. */
function render(name, given = {}, dir = boards) {
  const text = fs.readFileSync(path.join(dir, `${name}.dc.html`), "utf8");
  const props = propsOf(text, given);
  const vals = valuesOf(text, props);
  let body = text.slice(text.indexOf("<x-dc>") + 6, text.indexOf("</x-dc>"));
  body = body.replace(/<helmet>[\s\S]*?<\/helmet>/, "");
  return fill(body, vals, dir);
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
  ["R6TabLPane", { orient: "landscape", overlay: "none", r6: true }, 1180, 820, "Tablet", round6],
  ["R6Flow", { view: "flow", r6: true }, 1440, 900, "Views", round6],
  ["R6Settings", { view: "settings", r6: true }, 1440, 900, "Views", round6],
  // Round 6's Checklist at 1600 by 900, for the 2560 by 1440 side-by-side (scaled 1.6, as Wide).
  ["R6Wide", { r6: true, preview: true, w: 1600, h: 900 }, 1600, 900, "Checklist", round6],
  ["OpenSteps", {}, 1440, 900],
  // The decision step (round 6 update): each state of Decision.dc.html.
  ["DecUnanswered", { state: "unanswered" }, 1120, 560, "Decision", round6],
  ["DecSuggested", { state: "suggested" }, 1120, 560, "Decision", round6],
  ["DecAnswered", { state: "answered" }, 1120, 560, "Decision", round6],
  ["DecChanging", { state: "changing" }, 1120, 560, "Decision", round6],
  ["DecChanged", { state: "changed" }, 1120, 560, "Decision", round6],
  ["DecPhone", { state: "phone" }, 390, 844, "Decision", round6],
  // Batch 4: the strip panel and the Flight pages.
  ["StripOpen", {}, 1440, 900],
  ["SimBrief", {}, 1440, 900],
  ["Start", {}, 1440, 900],
  ["R6Weather", {}, 1440, 900, "Weather", round6],
  ["R6Winds", { tab: "winds" }, 1440, 900, "Weather", round6],
  ["R6Approach", { view: "descent" }, 1440, 900, "Views", round6],
  ["FSOverview", { section: "overview" }, 1440, 900, "FlightSection", round6],
  ["FSRoute", { section: "route" }, 1440, 900, "FlightSection", round6],
  ["FSFuel", { section: "fuel" }, 1440, 900, "FlightSection", round6],
  ["FSCruise", { section: "cruise" }, 1440, 900, "FlightSection", round6],
  ["FSClearance", { section: "clearance" }, 1440, 900, "FlightSection", round6],
  ["FSTakeoff", { section: "takeoff" }, 1440, 900, "FlightSection", round6],
  ["FSApproach", { section: "approach" }, 1440, 900, "FlightSection", round6],
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
