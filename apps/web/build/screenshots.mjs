// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Screenshots of the built app for review, at desktop and tablet sizes, in the dark and light
// themes: `just screenshots <out-dir>` after `just site`. The theme comes from a Firefox
// preference, so the app needs no test hook. Writes PNG files to <out-dir>, which should be
// outside the repo. Needs Firefox on the PATH (or FIREFOX=/path/to/firefox).

import fs from "node:fs";
import path from "node:path";
import { serve, session, sleep, startFirefox } from "./browser.mjs";

const web = path.resolve(import.meta.dirname, "..");
const dist = path.join(web, "dist");
const work = path.resolve(web, "../../target/screenshots");
const PORT = 8772;
const BIDI = 9234;

const VIEWPORTS = {
  desktop: { width: 1440, height: 900 },
  tablet: { width: 820, height: 1180 },
};
// layout.css.prefers-color-scheme.content-override: 0 dark, 1 light.
const THEMES = { dark: 0, light: 1 };

/** The screens to capture: a name, the address after `#`, and an optional step before. */
const SHOTS = [
  { name: "phase", hash: "" },
  { name: "emergency", hash: "#/reference" },
  { name: "settings", hash: "#/settings" },
  {
    name: "phases-menu",
    hash: "",
    only: "tablet",
    before: `document.querySelector('[aria-controls="phase-rail"]').click(); true`,
  },
];

async function main() {
  const out = path.resolve(process.argv[2] ?? "");
  if (!process.argv[2]) throw new Error("usage: screenshots.mjs <out-dir>");
  if (!fs.existsSync(path.join(dist, "app/index.html"))) throw new Error("build the site first: just site");
  fs.mkdirSync(out, { recursive: true });
  const server = await serve(dist, PORT);
  const written = [];
  try {
    for (const [theme, override] of Object.entries(THEMES)) {
      for (const [size, viewport] of Object.entries(VIEWPORTS)) {
        // One Firefox per theme and size. The window size includes some frame, so measure the
        // viewport and, if it is off, start again with the difference added.
        const prefs = { "layout.css.prefers-color-scheme.content-override": override };
        const profile = path.join(work, `profile-${theme}-${size}`);
        let firefox;
        try {
          firefox = await startFirefox(profile, BIDI, prefs, viewport);
          let browser = await session(BIDI);
          // Scripts cannot run in the browser's own start page, so open a blank page first.
          await browser.send("browsingContext.navigate", { context: browser.context, url: "about:blank", wait: "complete" });
          const measured = await browser.js("[innerWidth, innerHeight].join(',')");
          const [w, h] = measured.split(",").map(Number);
          if (w !== viewport.width || h !== viewport.height) {
            firefox.kill();
            await sleep(800);
            const window = { width: 2 * viewport.width - w, height: 2 * viewport.height - h };
            firefox = await startFirefox(profile, BIDI, prefs, window);
            browser = await session(BIDI);
          }
          const { send, js, until, context } = browser;
          for (const shot of SHOTS.filter((s) => !s.only || s.only === size)) {
            await send("browsingContext.navigate", {
              context,
              url: `http://localhost:${PORT}/app/${shot.hash}`,
              wait: "complete",
            });
            await until("!!document.querySelector('[data-app-ready]')", "the app to be ready");
            await js("document.fonts.ready.then(() => true)");
            if (shot.before) await js(shot.before);
            await sleep(400);
            const { data } = await send("browsingContext.captureScreenshot", { context });
            const file = path.join(out, `${shot.name}-${size}-${theme}.png`);
            fs.writeFileSync(file, Buffer.from(data, "base64"));
            written.push(path.basename(file));
          }
        } finally {
          firefox?.kill();
          await sleep(500);
        }
      }
    }
  } finally {
    server.close();
  }
  process.stdout.write(`screenshots: ${written.length} files in ${out}\n${written.join("\n")}\n`);
}

await main();
