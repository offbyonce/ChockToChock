// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// The drawing renderer's review images (decision 0021), in headless Firefox: `just drawing-shots
// <out-dir>` after `just site`. For each theme it saves the whole gallery page (design/), each
// control state on its own (cells/), and the approved board's own control drawings
// (docs/design/boards/Checklist.dc.html, its controls figure) with the board's panel colors
// filled in (board-*.png), for the side-by-sides in the reviewer brief. Writes PNG files to
// <out-dir>, which should be outside the repo. Needs Firefox.

import fs from "node:fs";
import path from "node:path";
import { serve, session, sleep, startFirefox } from "./browser.mjs";

const web = path.resolve(import.meta.dirname, "..");
const root = path.resolve(web, "../..");
const dist = path.join(web, "dist");
const work = path.join(root, "target/drawing-shots");
const BIDI = 9254;
const PORT = 8775;
const THEMES = { dark: 0, light: 1 };
const VIEWPORT = { width: 1440, height: 900 };

/** The board's controls figure (the fuel panel), with its {{p.x}} holes filled from tokens.json. */
function boardPage(theme) {
  const board = fs.readFileSync(path.join(root, "docs/design/boards/Checklist.dc.html"), "utf8");
  const tokens = JSON.parse(fs.readFileSync(path.join(root, "docs/design/tokens.json"), "utf8"));
  const start = board.indexOf('<svg viewBox="0 0 440 190"');
  const svg = board.slice(start, board.indexOf("</svg>", start) + 6);
  const p = tokens.panel[theme];
  const holes = { bg: p.bg, panel: p.fill, stroke: p.stroke, label: p.label, text: p.text, hiFill: p["changed-fill"], accent: p.changed, hiText: p["changed-text"], rest: p.resting };
  const filled = svg.replace(/\{\{p\.(\w+)\}\}/g, (_, k) => holes[k] ?? "#ff00ff").replace('width="100%"', 'width="880"');
  return `<!doctype html><meta charset="utf-8"><title>board</title><body style="margin:0;background:${p.bg};font-family:sans-serif"><div id="board" style="padding:16px;width:880px">${filled}</div></body>`;
}

async function openAt(profile, prefs, firstUrl) {
  const firefox = await startFirefox(profile, BIDI, prefs, { width: VIEWPORT.width, height: VIEWPORT.height + 100 }, [
    "-remote-allow-system-access",
  ]);
  try {
    const browser = await session(BIDI);
    await browser.send("browsingContext.navigate", { context: browser.context, url: firstUrl, wait: "complete" });
    await browser.send("browsingContext.setViewport", { context: browser.context, viewport: VIEWPORT });
    return { firefox, browser };
  } catch (e) {
    firefox.kill();
    throw e;
  }
}

async function shotOf(browser, file, clip) {
  const { data } = await browser.send("browsingContext.captureScreenshot", {
    context: browser.context,
    ...(clip ? { clip: { type: "box", ...clip } } : { origin: "document" }),
  });
  fs.writeFileSync(file, Buffer.from(data, "base64"));
}

async function main() {
  const out = process.argv[2] ? path.resolve(process.argv[2]) : null;
  if (!out) throw new Error("usage: drawing-shots.mjs <out-dir>");
  if (!fs.existsSync(path.join(dist, "design/index.html"))) throw new Error("build the site first: just site");
  fs.mkdirSync(path.join(out, "cells"), { recursive: true });
  fs.mkdirSync(work, { recursive: true });
  for (const theme of Object.keys(THEMES)) fs.writeFileSync(path.join(dist, `board-${theme}.html`), boardPage(theme));
  const server = await serve(dist, PORT);
  try {
    for (const [theme, override] of Object.entries(THEMES)) {
      const profile = path.join(work, `profile-${theme}`);
      const { firefox, browser } = await openAt(profile, { "layout.css.prefers-color-scheme.content-override": override }, `http://localhost:${PORT}/design/`);
      try {
        await browser.send("browsingContext.navigate", { context: browser.context, url: `http://localhost:${PORT}/design/?theme=${theme}`, wait: "complete" });
        await browser.until("!!document.querySelector('[data-gallery-ready]')", "the gallery");
        await browser.js("document.fonts.ready.then(() => true)");
        await sleep(600);
        await shotOf(browser, path.join(out, `gallery-${theme}.png`));
        const cells = JSON.parse(
          await browser.js(
            `JSON.stringify([...document.querySelectorAll('[data-cell], [data-region]')].map((e) => { e.scrollIntoView(); const r = e.getBoundingClientRect(); return { name: e.dataset.cell ?? 'Where is this: ' + e.dataset.region, x: r.x, y: r.y, width: r.width, height: r.height }; }))`,
          ),
        );
        for (const cell of cells) {
          await browser.js(`[...document.querySelectorAll('[data-cell], [data-region]')].find((e) => (e.dataset.cell ?? 'Where is this: ' + e.dataset.region) === ${JSON.stringify(cell.name)}).scrollIntoView({ block: 'center' }); true`);
          await sleep(150);
          const box = JSON.parse(
            await browser.js(
              `JSON.stringify((({ x, y, width, height }) => ({ x, y, width, height }))([...document.querySelectorAll('[data-cell], [data-region]')].find((e) => (e.dataset.cell ?? 'Where is this: ' + e.dataset.region) === ${JSON.stringify(cell.name)}).getBoundingClientRect()))`,
            ),
          );
          const name = cell.name.toLowerCase().replace(/[^a-z0-9]+/g, "-").replace(/^-|-$/g, "");
          await shotOf(browser, path.join(out, "cells", `${name}-${theme}.png`), box);
        }
        await browser.send("browsingContext.navigate", { context: browser.context, url: `http://localhost:${PORT}/board-${theme}.html`, wait: "complete" });
        await sleep(300);
        await shotOf(browser, path.join(out, `board-fuel-${theme}.png`), { x: 0, y: 0, width: 912, height: 420 });
        console.log(`${theme}: gallery, ${cells.length} cells, board`);
      } finally {
        firefox.kill();
        await sleep(500);
      }
    }
  } finally {
    server.close();
    for (const theme of Object.keys(THEMES)) fs.rmSync(path.join(dist, `board-${theme}.html`), { force: true });
  }
}

await main();
