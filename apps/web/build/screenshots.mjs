// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Screenshots of the built app for review, at the five review sizes (phone, tablet portrait,
// tablet landscape, 1440 and 2560 by 1440; decision 0021), in the dark and light themes: `just screenshots <out-dir>` after `just site`. The theme comes from a Firefox
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
  phone: { width: 390, height: 844 },
  tablet: { width: 820, height: 1180 },
  "tablet-landscape": { width: 1180, height: 820 },
  desktop: { width: 1440, height: 900 },
  wide: { width: 2560, height: 1440 },
};
// layout.css.prefers-color-scheme.content-override: 0 dark, 1 light.
const THEMES = { dark: 0, light: 1 };

const click = (selector, n = 0) => `document.querySelectorAll(${JSON.stringify(selector)})[${n}].click(); true`;
/** Clicks `part` (".row" or ".chev") of the list step whose text contains `title`. */
const stepNamed = (title, part) =>
  `[...document.querySelectorAll(".it")].find((it) => it.textContent.trim().toUpperCase().includes(${JSON.stringify(title)})).querySelector(${JSON.stringify(part)}).click(); true`;
/** Clicks `part` of the list step whose name is exactly `name`. */
const stepExact = (name, part) =>
  `[...document.querySelectorAll(".it")].find((it) => [...(it.querySelector(".name")?.childNodes ?? [])].filter((c) => c.nodeType === 3).map((c) => c.textContent).join("").trim().toUpperCase() === ${JSON.stringify(name)}).querySelector(${JSON.stringify(part)}).click(); true`;
/** Scrolls the side pane `pane` so its controls drawing starts at the top. */
const paneToControls = (pane) =>
  `(() => { const p = document.querySelector(${JSON.stringify(pane)}); const h = [...p.querySelectorAll("*")].find((e) => e.children.length === 0 && e.textContent.trim().toUpperCase() === "THE CONTROLS"); p.scrollTop += h.getBoundingClientRect().top - p.getBoundingClientRect().top; })(); true`;
/** Types `value` into the input `selector`, as the pilot would. */
const typeInto = (selector, value) =>
  `(() => { const el = document.querySelector(${JSON.stringify(selector)}); el.value = ${JSON.stringify(value)}; el.dispatchEvent(new Event("input", { bubbles: true })); el.dispatchEvent(new Event("change", { bubbles: true })); })(); true`;
/** The same takeoff data typed into Flight setup, with each app's field ids. */
const TAKEOFF = [
  ["runway", "rwy", "16L"],
  ["takeoff_flaps", "toflaps", "5"],
  ["v2", "v2", "152"],
  ["takeoff_trim", "trim", "5.25"],
];
/** The same arrival data typed into the Arrival data box, with each app's field ids. */
const ARRIVAL = [
  ["landing_altitude", "landalt", "13"],
  ["vref", "vref", "142"],
  ["autobrake", "ab", "2"],
  ["minimums", "mins", "213"],
];
/** The same clearance typed into Flight setup, with each app's field ids. */
const CLEARANCE = [
  ["runway", "rwy", "16L"],
  ["sid", "sid", "SUMMA2"],
  ["transition", "trans", "SEA"],
  ["initial_altitude", "alt", "5000"],
  ["squawk", "sq", "4521"],
];
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
    // The same steps on both sides: two ticked, Wipers skipped, Alternate flaps selected and opened.
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
    base: {
      hash: "#cold-dark",
      before: [
        click(".it .tick", 0),
        click(".it .tick", 1),
        click(".it .chev", 3),
        `[...document.querySelectorAll(".dskip .btn")][0].click(); true`,
        // 3.0 opens a step when its row is clicked.
        click(".it .row", 2),
      ],
    },
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
    app: { hash: "#/reference", before: [click("details summary", 0), click(".it .row", 0), click(".it .chev", 0)] },
    base: { hash: "#memory", before: [click("details summary", 0), click(".it .chev", 0)] },
  },
  { name: "settings", app: { hash: "#/settings" }, base: { hash: "#settings" } },
  // Flight setup, empty on both sides.
  { name: "setup", app: { hash: "#/setup" }, base: { hash: "#setup" } },
  {
    // Before takeoff with takeoff data from Flight setup: the blanks filled in on both sides.
    name: "values",
    app: {
      hash: "#/setup",
      before: [...TAKEOFF.map(([ours, , v]) => typeInto(`#s-${ours}`, v)), go("#/phase/before-takeoff")],
    },
    base: {
      hash: "#setup",
      before: [...TAKEOFF.map(([, theirs, v]) => typeInto(`#s-${theirs}`, v)), go("#before-takeoff")],
    },
  },
  {
    // Descent with the same arrival data typed into the Arrival data box on both sides.
    name: "arrival",
    only: "desktop",
    app: { hash: "#/phase/descent", before: ARRIVAL.map(([ours, , v]) => typeInto(`#a-${ours}`, v)) },
    base: { hash: "#descent", before: ARRIVAL.map(([, theirs, v]) => typeInto(`#s-${theirs}`, v)) },
  },
  {
    // The flight strip with a clearance from Flight setup, its pop-out open on both sides.
    name: "strip",
    app: {
      hash: "#/setup",
      before: [...CLEARANCE.map(([ours, , v]) => typeInto(`#s-${ours}`, v)), go("#/phase/cold-dark"), click(".fsline")],
    },
    base: {
      hash: "#setup",
      before: [...CLEARANCE.map(([, theirs, v]) => typeInto(`#s-${theirs}`, v)), go("#cold-dark"), click("#fsbtn")],
    },
  },
  {
    // Before start: MCP (glareshield). The app lists Landing gear pins first, so its rows run one ahead.
    // Both sides show the step opened.
    name: "panel-mcp",
    only: ["desktop", "wide"],
    app: { hash: "#/phase/before-start", before: [click(".it .row", 2), click(".it .chev", 2)] },
    base: { hash: "#before-start", before: [click(".it .chev", 1)] },
  },
  {
    // Before start: door lights (overhead). The app lists Landing gear pins first, so its rows run one ahead.
    // Both sides show the step opened.
    name: "panel-doors",
    only: "desktop",
    app: { hash: "#/phase/before-start", before: [click(".it .row", 3), click(".it .chev", 3)] },
    base: { hash: "#before-start", before: [click(".it .chev", 2)] },
  },
  {
    // Before start: TERR (main panel). The app lists Landing gear pins first, so its rows run one ahead.
    // Both sides show the step opened.
    name: "panel-terr",
    only: "desktop",
    app: { hash: "#/phase/before-start", before: [click(".it .row", 4), click(".it .chev", 4)] },
    base: { hash: "#before-start", before: [click(".it .chev", 3)] },
  },
  {
    // Before start: fuel pumps (overhead). The app lists Landing gear pins first, so its rows run one ahead.
    // Both sides show the step opened.
    name: "panel-fuel",
    only: ["desktop", "wide"],
    app: { hash: "#/phase/before-start", before: [click(".it .row", 5), click(".it .chev", 5)] },
    base: { hash: "#before-start", before: [click(".it .chev", 4)] },
  },
  {
    // Preflight: the electrical warning lights, with the side pane scrolled to the meter dials.
    name: "panel-elec",
    only: "desktop",
    app: {
      hash: "#/phase/preflight",
      before: [
        stepNamed("BAT DISCHARGE, TR UNIT", ".row"),
        stepNamed("BAT DISCHARGE, TR UNIT", ".chev"),
        paneToControls("aside.pane"),
      ],
    },
    base: { hash: "#preflight", before: [stepNamed("BAT DISCHARGE, TR UNIT", ".chev"), paneToControls("#pane")] },
  },
  {
    // The widest sections in the pack: the captain's lights (main panel, 918 units across) and the
    // parking brake and start levers (control stand, 725).
    name: "panel-lights",
    only: ["desktop", "wide"],
    app: { hash: "#/phase/preflight", before: [stepNamed("STAB OUT OF TRIM LIGHT", ".row"), stepNamed("STAB OUT OF TRIM LIGHT", ".chev"), paneToControls("aside.pane")] },
    base: { hash: "#preflight", before: [stepNamed("STAB OUT OF TRIM LIGHT", ".chev"), paneToControls("#pane")] },
  },
  {
    // The aft overhead (IRS mode selectors) and the pilot station (oxygen mask), so every panel in
    // the pack has a drawing in the review set. No 3.0 counterpart is shot for these.
    name: "panel-aft",
    only: ["desktop", "wide"],
    app: { hash: "#/phase/preflight", before: [stepExact("IRS", ".row"), stepExact("IRS", ".chev"), paneToControls("aside.pane")] },
  },
  {
    name: "panel-seat",
    only: ["desktop", "wide"],
    app: { hash: "#/phase/preflight", before: [stepExact("OXYGEN", ".row"), stepExact("OXYGEN", ".chev"), paneToControls("aside.pane")] },
  },
  {
    name: "panel-brake",
    only: ["desktop", "wide"],
    app: { hash: "#/phase/gpu-power", before: [stepNamed("PARKING BRAKE", ".row"), stepNamed("PARKING BRAKE", ".chev"), paneToControls("aside.pane")] },
    base: { hash: "#gpu-power", before: [stepNamed("PARKING BRAKE", ".chev"), paneToControls("#pane")] },
  },
  {
    name: "menu",
    only: "tablet",
    app: { hash: "", before: [click('[aria-controls="drawer"]')] },
    base: { hash: "#cold-dark", before: [click("#menu")] },
  },
];

/**
 * Starts Firefox and sets the page's viewport to exactly `viewport` with WebDriver BiDi's
 * setViewport, since a window cannot get narrower than 500 px and phones are narrower. That command
 * needs `-remote-allow-system-access`, acceptable here: a local, headless browser with a throwaway
 * profile, opening only the pages served for these screenshots. The start page is privileged, so
 * `firstUrl` (one of those pages) is opened before the viewport is set.
 */
async function openAt(profile, prefs, viewport, firstUrl) {
  const window = { width: Math.max(viewport.width, 800), height: viewport.height + 100 };
  const firefox = await startFirefox(profile, BIDI, prefs, window, ["-remote-allow-system-access"]);
  try {
    const browser = await session(BIDI);
    await browser.send("browsingContext.navigate", { context: browser.context, url: firstUrl, wait: "complete" });
    await browser.send("browsingContext.setViewport", { context: browser.context, viewport });
    const got = await browser.js("[innerWidth, innerHeight].join('x')");
    if (got !== `${viewport.width}x${viewport.height}`) {
      throw new Error(`viewport ${got}, wanted ${viewport.width}x${viewport.height}`);
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
  fs.writeFileSync(path.join(dir, "index.html"), "<!doctype html><title>compose</title>");
  const server = await serve(dir, 8774);
  const written = [];
  try {
    for (const [size, { width, height }] of Object.entries(VIEWPORTS)) {
      const mine = keys.filter((k) => k.includes(`-${size}-`));
      if (!mine.length) continue;
      const canvas = { width: 2 * width + 16, height: height + 28 };
      const { firefox, browser } = await openAt(path.join(work, `profile-compose-${size}`), {}, canvas, "http://localhost:8774/");
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
          const fits = (s) => !s.only || [s.only].flat().includes(size);
          const shots = SHOTS.filter((s) => fits(s) && wanted(`${s.name}-${size}-${theme}`));
          if (!shots.length) continue;
          const profile = path.join(work, `profile-${target.side}-${theme}-${size}`);
          const { firefox, browser } = await openAt(profile, prefs, viewport, target.url(""));
          try {
            for (const shot of shots) {
              if (!shot[target.side]) continue;
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
