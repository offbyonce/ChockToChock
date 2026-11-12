// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Screenshots of the built app for review, at desktop and tablet sizes, in the dark and light
// themes: `just screenshots <out-dir>` after `just site`. The theme comes from a Firefox
// preference, so the app needs no test hook. Writes PNG files to <out-dir>, which should be
// outside the repo. Needs Firefox on the PATH (or FIREFOX=/path/to/firefox).
//
// With CHOCKTOCHOCK_BASELINE set to a local copy of the 3.0 checklist's HTML file, it also shoots
// the same screens of 3.0 into <out-dir>/baseline and writes side-by-side images (3.0 left, new
// right, same size and theme) into <out-dir>/compare. The 3.0 copy is never part of the repo.
// SCREENSHOTS_ONLY=phase-desktop-dark,flow limits the run to keys that start with one of those.

import fs from "node:fs";
import path from "node:path";
import { serve, session, sleep, startFirefox } from "./browser.mjs";

const web = path.resolve(import.meta.dirname, "..");
const dist = path.join(web, "dist");
const work = path.resolve(web, "../../target/screenshots");
const BIDI = 9234;

const VIEWPORTS = {
  desktop: { width: 1440, height: 900 },
  tablet: { width: 820, height: 1180 },
};
// layout.css.prefers-color-scheme.content-override: 0 dark, 1 light.
const THEMES = { dark: 0, light: 1 };

const click = (selector, n = 0) => `document.querySelectorAll(${JSON.stringify(selector)})[${n}].click(); true`;
const pressText = (text) =>
  `[...document.querySelectorAll("button")].find((b) => b.textContent.trim() === ${JSON.stringify(text)}).click(); true`;
const go = (hash) => `location.hash = ${JSON.stringify(hash)}; true`;

/**
 * The screens to capture: a name, and for each app the address after `#` and an optional script
 * to run first. `base` is the matching 3.0 screen.
 */
const SHOTS = [
  { name: "phase", app: { hash: "" }, base: { hash: "#cold-dark" } },
  {
    // The same step selected and opened on both sides (Alternate flaps); the app also skips one.
    name: "states",
    app: {
      hash: "",
      before: [
        click(".it .tick", 0),
        click(".it .tick", 1),
        click(".it .chev", 3),
        `[...document.querySelectorAll(".dskip .btn")][0].click(); true`,
        click(".it .row", 2),
        click(".it .chev", 2),
      ],
    },
    base: { hash: "#cold-dark", before: [click(".it .tick", 0), click(".it .tick", 1)] },
  },
  {
    // N1 BUGS opened on both sides (the pack lists Landing gear pins first).
    name: "detail",
    app: { hash: "#/phase/before-start", before: [click(".it .row", 1), click(".it .chev", 1)] },
    base: { hash: "#before-start", before: [click(".it .chev", 0)] },
  },
  { name: "checklist", app: { hash: "#/phase/before-start" }, base: { hash: "#before-start" } },
  {
    // N1 BUGS in Flow on both sides: the app ticks Landing gear pins first.
    name: "flow",
    app: { hash: "#/phase/before-start", before: [click(".it .tick", 0), click("header.top .tool", 1)] },
    base: { hash: "#before-start", before: [click("#flowb")] },
  },
  {
    name: "essentials",
    app: { hash: "#/settings", before: [pressText("Essentials"), go("")] },
    base: { hash: "#settings", before: [pressText("Essentials"), go("#cold-dark")] },
  },
  {
    name: "open",
    app: { hash: "", before: [click(".it .tick", 0), go("#/phase/gpu-power"), click(".opn .line")] },
    base: { hash: "#cold-dark", before: [click(".it .tick", 0), go("#gpu-power"), click("#opbtn")] },
  },
  { name: "emergency", app: { hash: "#/reference" }, base: { hash: "#memory" } },
  {
    // The first memory item opened and its first step chosen, so the drawing shows.
    name: "emergency-step",
    app: { hash: "#/reference", before: [click("details summary", 0), click(".it .row", 0)] },
    base: { hash: "#memory", before: [click("details summary", 0), click(".it .chev", 0)] },
  },
  { name: "settings", app: { hash: "#/settings" }, base: { hash: "#settings" } },
  {
    // Before start: MCP (glareshield). The app lists Landing gear pins first, so its rows run one ahead.
    name: "panel-mcp",
    only: "desktop",
    app: { hash: "#/phase/before-start", before: [click(".it .row", 2)] },
    base: { hash: "#before-start", before: [click(".it .chev", 1)] },
  },
  {
    // Before start: door lights (overhead). The app lists Landing gear pins first, so its rows run one ahead.
    name: "panel-doors",
    only: "desktop",
    app: { hash: "#/phase/before-start", before: [click(".it .row", 3)] },
    base: { hash: "#before-start", before: [click(".it .chev", 2)] },
  },
  {
    // Before start: TERR (main panel). The app lists Landing gear pins first, so its rows run one ahead.
    name: "panel-terr",
    only: "desktop",
    app: { hash: "#/phase/before-start", before: [click(".it .row", 4)] },
    base: { hash: "#before-start", before: [click(".it .chev", 3)] },
  },
  {
    // Before start: fuel pumps (overhead). The app lists Landing gear pins first, so its rows run one ahead.
    name: "panel-fuel",
    only: "desktop",
    app: { hash: "#/phase/before-start", before: [click(".it .row", 5)] },
    base: { hash: "#before-start", before: [click(".it .chev", 4)] },
  },
  {
    name: "menu",
    only: "tablet",
    app: { hash: "", before: [click('[aria-controls="drawer"]')] },
    base: { hash: "#cold-dark", before: [click("#menu")] },
  },
];

/** Starts Firefox with its viewport at exactly `viewport` (the window adds some frame). */
async function openAt(profile, prefs, viewport) {
  let firefox = await startFirefox(profile, BIDI, prefs, viewport);
  try {
    let browser = await session(BIDI);
    // Scripts cannot run in the browser's own start page, so open a blank page first.
    await browser.send("browsingContext.navigate", { context: browser.context, url: "about:blank", wait: "complete" });
    const [w, h] = (await browser.js("[innerWidth, innerHeight].join(',')")).split(",").map(Number);
    if (w !== viewport.width || h !== viewport.height) {
      firefox.kill();
      await sleep(800);
      firefox = await startFirefox(profile, BIDI, prefs, {
        width: 2 * viewport.width - w,
        height: 2 * viewport.height - h,
      });
      browser = await session(BIDI);
    }
    return { firefox, browser };
  } catch (e) {
    firefox.kill();
    throw e;
  }
}

async function visit(browser, url, ready) {
  await browser.send("browsingContext.navigate", { context: browser.context, url, wait: "complete" });
  await browser.until(ready, "the page to be ready");
}

async function shoot(browser, url, ready, steps, file) {
  const { send, js, until, context } = browser;
  await send("browsingContext.navigate", { context, url, wait: "complete" });
  await until(ready, "the page to be ready");
  await js("document.fonts.ready.then(() => true)");
  await sleep(300);
  for (const step of steps ?? []) {
    await js(step);
    await sleep(350);
  }
  await sleep(300);
  if (await js("!!document.getElementById('update-ready')")) process.stderr.write(`update offer showing: ${file}\n`);
  const { data } = await send("browsingContext.captureScreenshot", { context });
  fs.writeFileSync(file, Buffer.from(data, "base64"));
}

/** One side-by-side image per screen: 3.0 on the left, the new app on the right. */
async function compose(out, keys) {
  const dir = path.join(work, "compose");
  fs.rmSync(dir, { recursive: true, force: true });
  fs.mkdirSync(dir, { recursive: true });
  fs.mkdirSync(path.join(out, "compare"), { recursive: true });
  const server = await serve(dir, 8774);
  const written = [];
  try {
    for (const [size, { width, height }] of Object.entries(VIEWPORTS)) {
      const mine = keys.filter((k) => k.includes(`-${size}-`));
      if (!mine.length) continue;
      const canvas = { width: 2 * width + 16, height: height + 28 };
      const { firefox, browser } = await openAt(path.join(work, `profile-compose-${size}`), {}, canvas);
      try {
        for (const key of mine) {
          fs.copyFileSync(path.join(out, "baseline", `${key}.png`), path.join(dir, `${key}-3.0.png`));
          fs.copyFileSync(path.join(out, `${key}.png`), path.join(dir, `${key}-new.png`));
          const half = (label, src) =>
            `<div style="width:${width}px"><p>${label}</p><img src="${src}" width="${width}" height="${height}"></div>`;
          fs.writeFileSync(
            path.join(dir, `${key}.html`),
            `<!doctype html><meta charset="utf-8"><style>body{margin:0;display:flex;gap:16px;background:#7a7f87}` +
              `p{margin:0;padding-left:12px;font:600 14px/28px sans-serif;color:#fff}img{display:block}</style>` +
              half("3.0", `${key}-3.0.png`) +
              half("New", `${key}-new.png`),
          );
          const file = path.join(out, "compare", `${key}.png`);
          const ready = "[...document.images].every((i) => i.complete)";
          await shoot(browser, `http://localhost:8774/${key}.html`, ready, [], file);
          written.push(`compare/${key}.png`);
        }
      } finally {
        firefox.kill();
        await sleep(500);
      }
    }
  } finally {
    server.close();
  }
  return written;
}

async function main() {
  const out = path.resolve(process.argv[2] ?? "");
  if (!process.argv[2]) throw new Error("usage: screenshots.mjs <out-dir>");
  if (!fs.existsSync(path.join(dist, "app/index.html"))) throw new Error("build the site first: just site");
  const baseline = process.env.CHOCKTOCHOCK_BASELINE ? path.resolve(process.env.CHOCKTOCHOCK_BASELINE) : null;
  if (baseline && !fs.existsSync(baseline)) throw new Error(`CHOCKTOCHOCK_BASELINE: no such file ${baseline}`);
  fs.mkdirSync(out, { recursive: true });

  const targets = [
    {
      side: "app",
      root: dist,
      port: 8772,
      // A fresh query each time, so each screen is a new page load and starts from a new flight.
      url: (hash) => `http://localhost:8772/app/?shot=${Date.now()}${hash}`,
      ready: "!!document.querySelector('[data-app-ready]')",
      dir: out,
      // The app saves progress on the device: each screen starts from nothing saved.
      reset: "localStorage.clear(); true",
    },
  ];
  if (baseline) {
    fs.mkdirSync(path.join(out, "baseline"), { recursive: true });
    targets.push({
      side: "base",
      root: path.dirname(baseline),
      port: 8773,
      url: (hash) => `http://localhost:8773/${encodeURIComponent(path.basename(baseline))}?${Date.now()}${hash}`,
      ready: "document.readyState === 'complete'",
      dir: path.join(out, "baseline"),
      reset: "localStorage.clear(); true",
    });
  }

  const servers = await Promise.all(targets.map((t) => serve(t.root, t.port)));
  const written = [];
  const pairs = [];
  try {
    for (const [theme, override] of Object.entries(THEMES)) {
      for (const [size, viewport] of Object.entries(VIEWPORTS)) {
        for (const target of targets) {
          const prefs = { "layout.css.prefers-color-scheme.content-override": override };
          const only = (process.env.SCREENSHOTS_ONLY ?? "").split(",").filter(Boolean);
          const wanted = (key) => !only.length || only.some((o) => key.startsWith(o) || `${theme}-${size}`.endsWith(o));
          const shots = SHOTS.filter((s) => (!s.only || s.only === size) && wanted(`${s.name}-${size}-${theme}`));
          if (!shots.length) continue;
          const profile = path.join(work, `profile-${target.side}-${theme}-${size}`);
          const { firefox, browser } = await openAt(profile, prefs, viewport);
          try {
            for (const shot of shots) {
              const { hash, before } = shot[target.side];
              const key = `${shot.name}-${size}-${theme}`;
              // One retry, in case a page stalls once.
              for (let attempt = 1; ; attempt++) {
                try {
                  if (target.reset) {
                    await visit(browser, target.url(hash), target.ready);
                    await browser.js(target.reset);
                  }
                  await shoot(browser, target.url(hash), target.ready, before, path.join(target.dir, `${key}.png`));
                  break;
                } catch (e) {
                  if (attempt === 2) throw e;
                  process.stderr.write(`retrying ${target.side} ${key}: ${e.message}\n`);
                }
              }
              if (target.side === "app") written.push(`${key}.png`);
              else pairs.push(key);
            }
          } finally {
            firefox.kill();
            await sleep(500);
          }
        }
      }
    }
  } finally {
    for (const s of servers) s.close();
  }
  if (baseline) written.push(...pairs.map((k) => `baseline/${k}.png`), ...(await compose(out, pairs)));
  process.stdout.write(`screenshots: ${written.length} files in ${out}\n${written.join("\n")}\n`);
}

await main();
