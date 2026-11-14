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
  `[...document.querySelectorAll(".it")].find((it) => (it.querySelector(".nm")?.textContent ?? "").trim().toUpperCase() === ${JSON.stringify(name)}).querySelector(${JSON.stringify(part)}).click(); true`;
/** Scrolls the side pane `pane` so its controls drawing starts at the top. */
const paneToControls = (pane) =>
  `(() => { const p = document.querySelector(${JSON.stringify(pane)}); const h = [...p.querySelectorAll("*")].find((e) => e.children.length === 0 && e.textContent.trim().toUpperCase().startsWith("THE CONTROLS")); p.scrollTop += h.getBoundingClientRect().top - p.getBoundingClientRect().top; })(); true`;
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
  ["transition", "trans", "SUMMA"],
  ["initial_altitude", "alt", "5000"],
  ["squawk", "sq", "4521"],
];
/** Where each field lives in the Flight pages (batch 4), for typing the same data as 3.0. */
const SECTION_OF = {
  runway: "clearance",
  sid: "clearance",
  transition: "clearance",
  initial_altitude: "clearance",
  squawk: "clearance",
  takeoff_flaps: "takeoff",
  v2: "takeoff",
  takeoff_trim: "takeoff",
  landing_altitude: "approach",
  vref: "approach",
  autobrake: "approach",
  minimums: "approach",
};
/** Our side: the values typed in their Flight sections. */
const inFlight = (list) => typed(list.map(([ours, , v]) => [SECTION_OF[ours], ours, v]));
const pressText = (text) =>
  `[...document.querySelectorAll("button")].find((b) => b.textContent.trim() === ${JSON.stringify(text)}).click(); true`;
const go = (hash) => `location.hash = ${JSON.stringify(hash)}; true`;

/**
 * The screens to capture: a name, and for each app the address after `#` and an optional script
 * to run first. `base` is the matching 3.0 screen.
 */
/** Skips the list step `title`: opens it if it is closed, then presses Skip for now. */
const skipStep = (title) =>
  `(() => { const it = [...document.querySelectorAll(".it")].find((i) => i.textContent.toUpperCase().includes(${JSON.stringify(title)})); const skip = () => [...it.querySelectorAll("button")].find((b) => b.textContent.includes("Skip for now")); if (!skip()) it.querySelector(".chev").click(); return new Promise((go) => setTimeout(() => { skip().click(); go(true); }, 300)); })()`;
/**
 * The Flight pages in the boards' state (batch 4): the invented KSEA to KSFO OFP dropped on SimBrief
 * and OFP file, as a pilot would, then the values the boards show typed in. The import's Undo toast
 * is left to time out (10 s) so it is not in the shot.
 */
const OFP = fs.readFileSync(path.join(import.meta.dirname, "fixtures/board-ofp.json"), "utf8");
const dropOfp = `(() => { const dt = new DataTransfer(); dt.items.add(new File([${JSON.stringify(OFP)}], "ofp.json", { type: "application/json" })); document.querySelector(".drop").dispatchEvent(new DragEvent("drop", { dataTransfer: dt, bubbles: true, cancelable: true })); return new Promise((go) => setTimeout(() => go(true), 1200)); })()`;
const typed = (pairs) =>
  `(async () => { const wait = (ms) => new Promise((go) => setTimeout(go, ms)); for (const [section, field, value] of ${JSON.stringify(pairs)}) { if (!location.hash.startsWith("#/flight/" + section)) { location.hash = "#/flight/" + section; await wait(400); } const el = document.querySelector('[data-field="' + field + '"]'); if (!el) continue; if (el.tagName === "BUTTON") { el.click(); await wait(100); [...document.querySelectorAll('[role="option"] button')].find((b) => b.textContent.trim() === value)?.click(); } else { el.value = value; el.dispatchEvent(new Event("change", { bubbles: true })); } await wait(120); } return true; })()`;
const BOARD_FLIGHT = [
  go("#/flight/simbrief"),
  dropOfp,
  typed([
    ["clearance", "initial_altitude", "5000"],
    ["clearance", "squawk", "4521"],
    ["clearance", "ground_frequency", "121.700"],
    ["clearance", "tower_frequency", "119.900"],
    ["takeoff", "heading", "163"],
    ["takeoff", "takeoff_flaps", "5"],
    ["takeoff", "thrust_rating", "TO-1"],
    ["takeoff", "assumed_temperature", "45"],
    ["takeoff", "v1", "142"],
    ["takeoff", "vr", "144"],
    ["approach", "landing_flaps", "30"],
    ["approach", "autobrake", "2"],
    ["fuel", "cg", "24.0"],
  ]),
  `(() => { location.hash = "#/flight/clearance"; return new Promise((go) => setTimeout(() => { const el = document.querySelector('[data-row="notes"] input'); el.value = "Expect FL370 ten minutes after departure"; el.dispatchEvent(new Event("change", { bubbles: true })); go(true); }, 400)); })()`,
  "new Promise((go) => setTimeout(() => go(true), 10500))",
];

/** Ticks every step of phase `id`, its checklist too once it unfolds, then goes to phase `then`. */
const finishPhase = (id, then) =>
  `(async () => { const wait = (ms) => new Promise((go) => setTimeout(go, ms)); location.hash = "#/phase/${id}"; await wait(500); for (let i = 0; i < 200; i++) { const b = [...document.querySelectorAll(".it .tick")].find((t) => t.getAttribute("aria-pressed") !== "true"); if (!b) break; b.click(); await wait(80); } location.hash = "#/phase/${then}"; await wait(500); return true; })()`;
/** Opens the row menu ("···") of the list step whose name is exactly `name`. */
const rowMenu = (name) =>
  `[...document.querySelectorAll(".it")].find((it) => it.querySelector(".nm")?.textContent.trim() === ${JSON.stringify(name)}).querySelector("button.more").click(); true`;
/**
 * The approved boards' state for Before start: Landing gear pins to Door lights done, TERR skipped,
 * so the fuel pump switches are the current step (the boards' center tank step is batch 7's).
 */
const BOARD_STATE = [
  finishPhase("apu-power", "before-start"),
  stepNamed("LANDING GEAR PINS", ".tick"),
  stepNamed("N1 BUGS", ".tick"),
  stepNamed("MCP", ".tick"),
  stepNamed("DOOR LIGHTS", ".tick"),
  skipStep("TERR (EFIS"),
];
/** Opens the list step `title` and its narrow-screen details (its chevron first if it is closed). */
const openNarrow = (title) =>
  `(() => { const it = [...document.querySelectorAll(".it")].find((i) => i.textContent.toUpperCase().includes(${JSON.stringify(title)})); const chev = it.querySelector(".chev"); if (chev.getAttribute("aria-expanded") !== "true") chev.click(); return new Promise((go) => setTimeout(() => { [...it.querySelectorAll("button")].find((b) => b.textContent.includes("Show on map")).click(); go(true); }, 300)); })()`;
/** Show map on the pane's location line (where the drawing took the map figure's room). */
const showMap = `[...document.querySelectorAll("aside.pane .line button")].find((b) => b.textContent.trim() === "Show map")?.click(); true`;
/** Taps an element as a pointer does (detail 1): an SVG control's tap target, or a button whose focus
 * differs for the keyboard (element.click() reports detail 0, which the app reads as a keyboard press). */
const tap = (selector) =>
  `document.querySelector(${JSON.stringify(selector)}).dispatchEvent(new MouseEvent("click", { detail: 1, bubbles: true })); true`;
/** Opens the checklists still folded until their procedure is done. */
const unfold = `[...document.querySelectorAll("button.folded")].forEach((b) => b.click()); true`;
/** Presses a key on the page, as the pilot would. */
const press = (key) => `window.dispatchEvent(new KeyboardEvent("keydown", { key: ${JSON.stringify(key)}, bubbles: true })); true`;
/** The Flow view button in the top bar. */
const toFlow = `[...document.querySelectorAll("header.top .seg button")].find((b) => b.textContent.trim() === "Flow").click(); true`;

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
        // The two done steps fold into one line; Show keeps every row in place.
        `[...document.querySelectorAll(".done-group .show")][0]?.click(); true`,
        click(".it .more", 3),
        `[...document.querySelectorAll('[role="menuitem"]')].find((b) => b.textContent.trim() === "Skip for now").click(); true`,
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
    app: { hash: "#/phase/before-start", before: [click(".it .tick", 0), toFlow] },
    base: { hash: "#before-start", before: [click("#flowb")] },
  },
  {
    name: "essentials",
    app: { hash: "#/settings", before: [pressText("Essentials"), go("")] },
    base: { hash: "#settings", before: [pressText("Essentials"), go("#cold-dark")] },
  },
  {
    name: "open",
    app: { hash: "", before: [click(".it .tick", 0), go("#/phase/gpu-power"), click(".opener")] },
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
  // Flight setup's place: the Flight pages' Overview on our side.
  { name: "setup", app: { hash: "#/flight/overview" }, base: { hash: "#setup" } },
  {
    // Before takeoff with takeoff data from Flight setup: the blanks filled in on both sides.
    name: "values",
    app: {
      hash: "#/flight/overview",
      before: [inFlight(TAKEOFF), go("#/phase/before-takeoff")],
    },
    base: {
      hash: "#setup",
      before: [...TAKEOFF.map(([, theirs, v]) => typeInto(`#s-${theirs}`, v)), go("#before-takeoff")],
    },
  },
  {
    // Descent with the same arrival data: in Flight › Approach on our side (the Arrival data box
    // is a link there since batch 4), in 3.0's box on the other.
    name: "arrival",
    only: "desktop",
    app: { hash: "#/flight/approach", before: [inFlight(ARRIVAL), go("#/phase/descent")] },
    base: { hash: "#descent", before: ARRIVAL.map(([, theirs, v]) => typeInto(`#s-${theirs}`, v)) },
  },
  {
    // The flight strip with a clearance, its panel (3.0's pop-out) open on both sides.
    name: "strip",
    app: {
      hash: "#/flight/clearance",
      before: [inFlight(CLEARANCE), go("#/phase/cold-dark"), tap(".fsline")],
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
    app: { hash: "#/phase/before-start", before: [click(".it .row", 2), click(".it .chev", 2), press("p")] },
    base: { hash: "#before-start", before: [click(".it .chev", 1)] },
  },
  {
    // Before start: door lights (overhead). The app lists Landing gear pins first, so its rows run one ahead.
    // Both sides show the step opened.
    name: "panel-doors",
    only: "desktop",
    app: { hash: "#/phase/before-start", before: [click(".it .row", 3), click(".it .chev", 3), press("p")] },
    base: { hash: "#before-start", before: [click(".it .chev", 2)] },
  },
  {
    // Before start: TERR (main panel). The app lists Landing gear pins first, so its rows run one ahead.
    // Both sides show the step opened.
    name: "panel-terr",
    only: "desktop",
    app: { hash: "#/phase/before-start", before: [click(".it .row", 4), click(".it .chev", 4), press("p")] },
    base: { hash: "#before-start", before: [click(".it .chev", 3)] },
  },
  {
    // Before start: fuel pumps (overhead). The app lists Landing gear pins first, so its rows run one ahead.
    // Both sides show the step opened.
    name: "panel-fuel",
    only: ["desktop", "wide", "tablet-landscape"],
    app: { hash: "#/phase/before-start", before: [click(".it .row", 5), click(".it .chev", 5), press("p")] },
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
        press("p"),
      ],
    },
    base: { hash: "#preflight", before: [stepNamed("BAT DISCHARGE, TR UNIT", ".chev"), paneToControls("#pane")] },
  },
  {
    // The widest sections in the pack: the captain's lights (main panel, 918 units across) and the
    // parking brake and start levers (control stand, 725).
    name: "panel-lights",
    only: ["desktop", "wide"],
    app: { hash: "#/phase/preflight", before: [stepNamed("STAB OUT OF TRIM LIGHT", ".row"), stepNamed("STAB OUT OF TRIM LIGHT", ".chev"), press("p")] },
    base: { hash: "#preflight", before: [stepNamed("STAB OUT OF TRIM LIGHT", ".chev"), paneToControls("#pane")] },
  },
  {
    // The aft overhead (IRS mode selectors; the pane unscrolled, title and Check in frame) and the pilot station (oxygen mask), so every panel in
    // the pack has a drawing in the review set. No 3.0 counterpart is shot for these.
    name: "panel-aft",
    only: ["desktop", "wide"],
    app: { hash: "#/phase/preflight", before: [stepExact("IRS", ".row"), stepExact("IRS", ".chev"), press("p")] },
  },
  {
    name: "panel-seat",
    only: ["desktop", "wide"],
    app: { hash: "#/phase/preflight", before: [unfold, stepExact("OXYGEN", ".row"), stepExact("OXYGEN", ".chev"), press("p")] },
  },
  {
    // Tablet portrait and phone keep a dialog for Where is this? until batch 5.
    name: "where-narrow",
    only: ["phone", "tablet"],
    app: { hash: "#/phase/before-start", before: [openNarrow("FUEL PUMP SWITCHES"), click(".where-btn")] },
  },
  {
    name: "panel-brake",
    only: ["desktop", "wide"],
    app: { hash: "#/phase/gpu-power", before: [stepNamed("PARKING BRAKE", ".row"), stepNamed("PARKING BRAKE", ".chev"), press("p")] },
    base: { hash: "#gpu-power", before: [stepNamed("PARKING BRAKE", ".chev"), paneToControls("#pane")] },
  },
  // Board pairs: each screen in its board's step and state, shot at the board's size and whole
  // (the full page and the side pane top to bottom), for docs/design/boards side-by-sides.
  // The Checklist board also draws the row menu open on Anti-collision light.
  {
    name: "board-checklist",
    full: true,
    only: ["desktop", "wide"],
    app: { hash: "#/phase/before-start", before: [...BOARD_FLIGHT, go("#/phase/before-start"), ...BOARD_STATE, rowMenu("Anti-collision light")] },
  },
  // The flight strip with values and a note: its Notes chip against the Checklist board's (color.chip).
  {
    name: "board-strip",
    full: true,
    only: ["desktop"],
    app: {
      hash: "#/phase/before-start",
      before: [
        ...BOARD_FLIGHT,
        go("#/phase/before-start"),
        ...BOARD_STATE,
        tap(".fsline"),
        typeInto("#sp-notes", "Expect runway 16L"),
        tap(".fsline"),
        "document.activeElement?.blur(); true",
      ],
    },
  },
  // A step's blank linked to its Flight field: Before takeoff's stabilizer trim, still empty,
  // reads "— enter takeoff trim" (Checklist board), with the board flight imported.
  {
    name: "board-blanks",
    full: true,
    only: ["desktop"],
    app: { hash: "#/phase/before-takeoff", before: [...BOARD_FLIGHT, go("#/phase/before-takeoff"), unfold] },
  },
  // Batch 4: the Flight pages, each section in the boards' state.
  ...["overview", "route", "fuel", "cruise", "weather", "clearance", "takeoff", "approach", "start", "simbrief"].map((section) => ({
    name: `board-flight-${section}`,
    full: true,
    only: ["desktop", "wide", "tablet-landscape"],
    app: { hash: "#/phase/before-start", before: [...BOARD_FLIGHT, go(`#/flight/${section}`)] },
  })),
  // Every placeholder fits at the narrow sizes too (owner, batch 4 review).
  ...["takeoff", "approach"].map((section) => ({
    name: `flight-${section}`,
    full: true,
    only: ["phone", "tablet"],
    app: { hash: "#/phase/before-start", before: [...BOARD_FLIGHT, go(`#/flight/${section}`)] },
  })),
  // A value out of range (FSInvalid): Cruise level at FL450.
  {
    name: "board-flight-invalid",
    full: true,
    only: ["desktop", "tablet-landscape", "phone"],
    app: { hash: "#/phase/before-start", before: [...BOARD_FLIGHT, typed([["cruise", "cruise_level", "FL450"]]), click('[data-row="cruise_level"] .label')] },
  },
  {
    name: "board-flight-winds",
    full: true,
    only: ["desktop", "tablet-landscape"],
    app: { hash: "#/phase/before-start", before: [...BOARD_FLIGHT, go("#/flight/weather"), click('[role="tab"]', 3)] },
  },
  // The strip panel, open, with the plan imported (StripOpen) and with Frequencies open (StripFreq).
  {
    name: "board-strip-open",
    full: true,
    only: ["desktop", "wide", "tablet-landscape"],
    app: { hash: "#/phase/before-start", before: [...BOARD_FLIGHT, go("#/phase/before-start"), ...BOARD_STATE, tap(".fsline")] },
  },
  {
    name: "board-strip-freq",
    full: true,
    only: ["desktop"],
    app: { hash: "#/phase/before-start", before: [...BOARD_FLIGHT, go("#/phase/before-start"), ...BOARD_STATE, tap(".fsline"), click("button.freq")] },
  },
  // The tablet landscape board draws no menu.
  { name: "board-checklist", full: true, only: ["tablet-landscape"], app: { hash: "#/phase/before-start", before: BOARD_STATE } },
  // Round 6's slide-out: Show panel (P), Where is this? (W), and a control's card (tap CTR L).
  { name: "board-panel", full: true, only: ["desktop", "wide", "tablet-landscape"], app: { hash: "#/phase/before-start", before: [...BOARD_STATE, press("p")] } },
  { name: "board-where", full: true, only: ["desktop", "tablet-landscape"], app: { hash: "#/phase/before-start", before: [...BOARD_STATE, press("w")] } },
  {
    name: "board-control",
    full: true,
    only: ["desktop", "tablet-landscape"],
    app: { hash: "#/phase/before-start", before: [...BOARD_STATE, press("p"), tap('.slide-box rect.hit[aria-label="CTR L: what it is"]')] },
  },
  { name: "board-open", full: true, only: ["desktop"], app: { hash: "#/phase/before-start", before: [...BOARD_STATE, click(".opener")] } },
  { name: "board-flow", full: true, only: ["desktop"], app: { hash: "#/phase/before-start", before: [...BOARD_STATE, toFlow] } },
  { name: "board-settings", full: true, only: ["desktop"], app: { hash: "#/settings" } },
  {
    name: "board-phonewhere",
    full: true,
    only: ["phone"],
    app: { hash: "#/phase/before-start", before: [...BOARD_STATE, openNarrow("FUEL PUMP SWITCHES"), click(".where-btn")] },
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

async function shoot(browser, url, ready, steps, file, full = false) {
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
  if (full) await shootWhole(browser, file);
}

/**
 * The whole screen, not only what the viewport shows: the full page (`-page.png`) and the side
 * pane from top to bottom in viewport-high slices (`-pane-<n>.png`, with `-pane.json` giving each
 * slice's scroll offset), so a drawing or legend below the fold shows up in review.
 */
async function shootWhole(browser, file) {
  const { send, js, context } = browser;
  const base = file.replace(/\.png$/, "");
  const page = await send("browsingContext.captureScreenshot", { context, origin: "document" });
  fs.writeFileSync(`${base}-page.png`, Buffer.from(page.data, "base64"));
  // What scrolls on its own: the side pane, or on the Flight pages the section column (the whole
  // body on narrow screens), sliced top to bottom.
  const sel = await js(
    `["aside.pane", "main.fmain", ".flight .cols"].find((s) => { const e = document.querySelector(s); return e && e.clientHeight > 0 && (s === "aside.pane" || e.scrollHeight > e.clientHeight + 1); }) ?? null`,
  );
  if (!sel) return;
  const q = JSON.stringify(sel);
  const box = await js(
    `(() => { const p = document.querySelector(${q}); const r = p.getBoundingClientRect(); return JSON.stringify({ x: r.x, y: r.y, w: r.width, h: r.height, scroll: p.scrollHeight, view: p.clientHeight, top: p.scrollTop }); })()`,
  );
  const pane = JSON.parse(box);
  if (!pane.w || !pane.h) return;
  const offsets = [];
  for (let at = 0; ; at += pane.view) {
    const top = Math.min(at, pane.scroll - pane.view);
    await js(`document.querySelector(${q}).scrollTop = ${top}; true`);
    await sleep(250);
    const real = await js(`document.querySelector(${q}).scrollTop`);
    const shot = await send("browsingContext.captureScreenshot", {
      context,
      clip: { type: "box", x: pane.x, y: pane.y, width: pane.w, height: pane.h },
    });
    fs.writeFileSync(`${base}-pane-${offsets.length}.png`, Buffer.from(shot.data, "base64"));
    offsets.push(real);
    if (top >= pane.scroll - pane.view) break;
  }
  fs.writeFileSync(`${base}-pane.json`, JSON.stringify({ offsets, height: pane.scroll, view: pane.view, what: sel }));
  await js(`document.querySelector(${q}).scrollTop = ${pane.top}; true`);
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
              const full = target.side === "app" && !!shot.full;
              const key = `${shot.name}-${size}-${theme}`;
              // One retry, in case a page stalls once.
              for (let attempt = 1; ; attempt++) {
                try {
                  if (target.reset) {
                    await visit(browser, target.url(hash), target.ready);
                    await browser.js(target.reset);
                  }
                  await shoot(browser, target.url(hash), target.ready, before, path.join(target.dir, `${key}.png`), full);
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
