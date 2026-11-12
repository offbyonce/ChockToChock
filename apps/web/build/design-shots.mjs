// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// The design system's screenshots (decision 0020), in headless Firefox:
//   node apps/web/build/design-shots.mjs <out-dir>            mockups at phone, tablet and desktop
//   node apps/web/build/design-shots.mjs --components          compare each component with its baseline
//   node apps/web/build/design-shots.mjs --components --update  save new component baselines
// Build the site first (just site). Component baselines live in build/design-baselines.

import fs from "node:fs";
import path from "node:path";
import { serve, session, sleep, startFirefox } from "./browser.mjs";

const web = path.resolve(import.meta.dirname, "..");
const dist = path.join(web, "dist");
const work = path.resolve(web, "../../target/design-shots");
const baselines = path.join(import.meta.dirname, "design-baselines");
const BIDI = 9244;
const PORT = 8774;

/** The layout classes the mockups show (docs/design/system.md). */
const SIZES = {
  phone: { width: 390, height: 844 },
  "tablet-portrait": { width: 820, height: 1180 },
  "tablet-landscape": { width: 1180, height: 820 },
  desktop: { width: 1440, height: 900 },
};
const VIEWS = ["phase", "step", "flow", "setup"];
const THEMES = { dark: 0, light: 1 };
const COMPONENTS = ["button", "chip", "segmented", "field", "card", "callout", "header"];

/**
 * Starts Firefox and sets the page's viewport to exactly `viewport` with WebDriver BiDi's
 * setViewport: a window cannot get narrower than 500 px, and phones are narrower. That command
 * needs `-remote-allow-system-access`, acceptable here: a local, headless browser with a throwaway
 * profile, opening only this site.
 */
async function openAt(profile, prefs, viewport) {
  const firefox = await startFirefox(profile, BIDI, prefs, { width: Math.max(viewport.width, 800), height: viewport.height + 100 }, [
    "-remote-allow-system-access",
  ]);
  try {
    const browser = await session(BIDI);
    // The start page is privileged: open one of the site's pages first, then size the viewport.
    await browser.send("browsingContext.navigate", { context: browser.context, url: `http://localhost:${PORT}/design/`, wait: "complete" });
    await browser.send("browsingContext.setViewport", { context: browser.context, viewport });
    const got = await browser.js("[innerWidth, innerHeight].join('x')");
    if (got !== `${viewport.width}x${viewport.height}`) throw new Error(`viewport ${got}, wanted ${viewport.width}x${viewport.height}`);
    return { firefox, browser };
  } catch (e) {
    firefox.kill();
    throw e;
  }
}

async function load(browser, query) {
  const { send, until, js, context } = browser;
  await send("browsingContext.navigate", { context, url: `http://localhost:${PORT}/design/?${query}`, wait: "complete" });
  await until("!!document.querySelector('[data-design-ready]')", "the design page");
  await js("document.fonts.ready.then(() => true)");
  await sleep(500);
}

async function capture(browser, file, clip) {
  const { data } = await browser.send("browsingContext.captureScreenshot", { context: browser.context, ...(clip ? { clip } : {}) });
  fs.writeFileSync(file, Buffer.from(data, "base64"));
  return Buffer.from(data, "base64");
}

async function mockups(out) {
  fs.mkdirSync(out, { recursive: true });
  for (const [theme, override] of Object.entries(THEMES)) {
    for (const [size, viewport] of Object.entries(SIZES)) {
      const profile = path.join(work, `profile-${theme}-${size}`);
      fs.rmSync(profile, { recursive: true, force: true });
      const { firefox, browser } = await openAt(profile, { "layout.css.prefers-color-scheme.content-override": override }, viewport);
      try {
        for (const view of VIEWS) {
          await load(browser, `view=${view}&theme=${theme}`);
          const file = path.join(out, `${view}-${size}-${theme}.png`);
          await capture(browser, file);
          console.log(path.relative(out, file));
        }
      } finally {
        firefox.kill();
        await sleep(500);
      }
    }
  }
}

async function components(update) {
  fs.mkdirSync(baselines, { recursive: true });
  const changed = [];
  for (const [theme, override] of Object.entries(THEMES)) {
    const profile = path.join(work, `profile-components-${theme}`);
    fs.rmSync(profile, { recursive: true, force: true });
    const { firefox, browser } = await openAt(profile, { "layout.css.prefers-color-scheme.content-override": override }, { width: 1024, height: 2000 });
    try {
      await load(browser, `view=gallery&theme=${theme}`);
      for (const name of COMPONENTS) {
        const box = JSON.parse(
          await browser.js(
            `JSON.stringify((({x, y, width, height}) => ({x, y, width, height}))(document.querySelector('[data-component="${name}"]').getBoundingClientRect()))`,
          ),
        );
        const file = path.join(baselines, `${name}-${theme}.png`);
        const fresh = path.join(work, `${name}-${theme}.png`);
        const shot = await capture(browser, fresh, { type: "box", ...box });
        if (update || !fs.existsSync(file)) {
          fs.copyFileSync(fresh, file);
        } else if (!shot.equals(fs.readFileSync(file))) {
          changed.push(`${name}-${theme} (new shot: ${fresh})`);
        }
      }
    } finally {
      firefox.kill();
      await sleep(500);
    }
  }
  if (changed.length) {
    console.error(`components changed from their baselines:\n  ${changed.join("\n  ")}\nIf the change is intended, run with --update.`);
    process.exit(1);
  }
  console.log(update ? "component baselines saved" : "every component matches its baseline");
}

async function main() {
  if (!fs.existsSync(path.join(dist, "design/index.html"))) throw new Error("build the site first: just site");
  fs.mkdirSync(work, { recursive: true });
  const server = await serve(dist, PORT);
  try {
    if (process.argv.includes("--components")) await components(process.argv.includes("--update"));
    else {
      const out = process.argv[2];
      if (!out) throw new Error("usage: design-shots.mjs <out-dir> | --components [--update]");
      await mockups(path.resolve(out));
    }
  } finally {
    server.close();
  }
}

await main();
