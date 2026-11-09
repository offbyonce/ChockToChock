// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Reads a local copy of the 3.0 source. Data layers run in build order, as the 3.0 build does.
// Behavior that exists only in 3.0's page code is taken by running those functions.

import fs from "node:fs";
import path from "node:path";
import vm from "node:vm";

/** 3.0 saved-value keys, and the app's name for each. Values must be unique. */
export const FLIGHT_FIELDS = {
  from: ["ZZAA", "origin"],
  to: ["ZZBB", "destination"],
  no: ["XQ4821", "flight_number"],
  route: [null, "route"],
  rwy: ["99X", "runway"],
  sid: ["SIDXQ", "sid"],
  trans: ["TRNXQ", "transition"],
  alt: ["12000", "initial_altitude"],
  sq: ["4521", "squawk"],
  gnd: ["121.75", "ground_frequency"],
  dep: ["118.35", "departure_frequency"],
  qnh: ["1013.2", "qnh"],
  hdg: ["236", "heading"],
  toflaps: ["15", "takeoff_flaps"],
  ldgflaps: ["XLF", "landing_flaps"],
  v1: ["141", "v1"],
  vr: ["143", "vr"],
  v2: ["147", "v2"],
  vref: ["138", "vref"],
  cg: ["27.3", "cg"],
  trim: ["5.25", "takeoff_trim"],
  ab: ["XAB", "autobrake"],
  mins: ["XMIN", "minimums"],
  landalt: ["XLA", "landing_altitude"],
  mintof: ["XMT", "minimum_takeoff_fuel"],
  fuel: ["73210", "fuel"],
  zfw: ["61.7", "zfw"],
  tow: ["133.9", "takeoff_weight"],
  pax: ["158", "passengers"],
  cargo: ["XCG", "cargo"],
  resv: ["8.6", "reserves"],
  ci: ["64", "cost_index"],
  fl: ["387", "cruise_level"],
  cwind: ["123/45", "cruise_wind"],
  isa: ["7", "isa_deviation"],
  ta: ["17500", "transition_altitude"],
  tl: ["195", "transition_level"],
  olat: ["47.5", "origin_position"],
  olon: ["-122.25", "origin_position"],
};

const LEG_COUNT = 12;
const FIXED_TIME = Date.UTC(2026, 2, 15, 12, 34, 56);

function listFrom(buildPy, name) {
  const match = new RegExp(`${name}\\s*=\\s*\\[([\\s\\S]*?)\\]`).exec(buildPy);
  if (!match) throw new Error(`build.py has no ${name}`);
  return [...match[1].matchAll(/'([^']+)'/g)].map((m) => m[1]);
}

function between(text, start, end) {
  const from = text.indexOf(start);
  const to = text.indexOf(end, from + start.length);
  if (from < 0 || to < 0) throw new Error(`3.0 template: cannot find ${start}`);
  return text.slice(from, to);
}

function line(text, start) {
  return between(text, start, "\n");
}

function loadData(src) {
  const buildPy = fs.readFileSync(path.join(src, "build.py"), "utf8");
  const layers = listFrom(buildPy, "LAYERS");
  const names = /NAMES\s*=\s*'([^']+)'/.exec(buildPy)?.[1];
  if (!names) throw new Error("build.py has no NAMES");
  const code = layers.map((file) => `${fs.readFileSync(path.join(src, file), "utf8")}\n`).join("");
  const context = { console: { log() {}, error() {} } };
  vm.createContext(context);
  vm.runInContext(`${code}\nglobalThis.__out=JSON.stringify({${names}});`, context);
  return JSON.parse(context.__out);
}

function sampleSet() {
  const set = {};
  for (const [key, [value]] of Object.entries(FLIGHT_FIELDS)) if (value) set[key] = value;
  const route = [`WPA01`];
  for (let n = 2; n <= LEG_COUNT; n++) {
    route.push(`VIQ${String(n).padStart(2, "0")}`, `WPA${String(n).padStart(2, "0")}`);
  }
  set.route = route.join(" ");
  return set;
}

function fixedDateClass() {
  return class FixedDate extends Date {
    constructor(...args) {
      if (args.length > 0) super(...args);
      else super(FIXED_TIME);
    }

    static now() {
      return FIXED_TIME;
    }
  };
}

/** The value table: each marker's text on the CDU screen, and what it stands for. */
function cduTable(run) {
  const f = (name) => ({ field: name });
  const values = [
    ["ZZAA", f("origin")],
    ["ZZBB", f("destination")],
    ["XQ4821", f("flight_number")],
    ["99X", f("runway")],
    ["SIDXQ", f("sid")],
    ["TRNXQ", f("transition")],
    ["73.3", f("fuel_thousands")],
    ["135.0", f("gross_weight")],
    ["61.7", f("zfw")],
    ["133.9", f("takeoff_weight")],
    ["8.6", f("reserves")],
    ["64", f("cost_index")],
    ["387", f("cruise_level")],
    ["123°/ 45", f("cruise_wind")],
    ["17500", f("transition_altitude")],
    ["236", f("heading")],
    ["27.3", f("cg")],
    ["5.25", f("takeoff_trim")],
    ["15", f("takeoff_flaps")],
    ["141", f("v1")],
    ["143", f("vr")],
    ["147", f("v2")],
    ["XKG", f("weight_unit")],
    ["-56°F-49°C", f("top_of_climb_oat")],
    [run("cdwPos('47.5','-122.25')"), f("origin_position")],
  ].map(([value, target]) => ({ value, target }));
  for (let n = 1; n <= LEG_COUNT; n++) {
    const id = String(n).padStart(2, "0");
    values.push({ value: `WPA${id}`, target: { field: "route_leg", index: n, part: "to" } });
    if (n > 1) values.push({ value: `VIQ${id}`, target: { field: "route_leg", index: n, part: "via" } });
  }
  const always = [
    ["1234.9z", f("utc_time")],
    ["MAR15/26", f("utc_date")],
    [run("'AIRAC-'+String(cdwAirac().id).padStart(4,'0')"), f("nav_database")],
    [
      run(
        "(()=>{const a=cdwAirac();return cdwDay(a.t)+cdwDay(a.nx)+'/'+String(new Date(a.nx).getUTCFullYear()).slice(2)})()",
      ),
      f("nav_active_dates"),
    ],
  ].map(([value, target]) => ({ value, target, always: true }));
  return {
    values: [...values, ...always],
    whole: [{ value: "+13°F +7°C", target: f("isa_deviation") }],
    leg1Via: { value: "DIRECT", target: { field: "route_leg", index: 1, part: "via" } },
    overflow: { value: String(LEG_COUNT - 5), target: { route_legs: "overflow" } },
  };
}

function loadCdu(template, set) {
  const code = between(template, "let cdwI=0;", "function cdwScreen(");
  const context = { S: { set: {}, flight: {} }, esc: (s) => s, Date: fixedDateClass(), console };
  vm.createContext(context);
  vm.runInContext(code, context);
  const run = (expr) => vm.runInContext(expr, context);
  const steps = () => JSON.parse(JSON.stringify(run("cdwSteps()")));
  const blank = steps();
  context.S = { set, flight: {}, fu: "XKG" };
  const filled = steps();
  return { blank, filled, table: cduTable(run) };
}

/** Assigns 3.0's step ids exactly as its page code does. */
export function forEachStep(phases, visit) {
  for (const phase of phases) {
    phase.lists.forEach((list, listIndex) => {
      let i = 0;
      let n = 0;
      for (const row of list.items) {
        if (row.h) continue;
        const options = row[4] || {};
        const id =
          options.id || (options.a ? `${phase.id}-${listIndex}-a${n++}` : `${phase.id}-${listIndex}-${i++}`);
        visit(id, row, phase, list, listIndex);
      }
    });
  }
}

function blankLabels(value) {
  const parts = value.split("___");
  const labels = [];
  let label = parts[0].split(",").pop().trim();
  for (let i = 1; i < parts.length; i++) {
    labels.push(label);
    const rest = parts[i];
    label = rest.includes(",") ? rest.split(",").pop().trim() : rest.trim();
  }
  return labels;
}

function loadStepBehavior(template, data, set) {
  const context = { S: { set }, ITEM: null, LAB: "" };
  vm.createContext(context);
  vm.runInContext(
    `${line(template, "function autoVal(")}\n${between(template, "function hintOf(", "function valHTML(")}`,
    context,
  );
  const byValue = new Map(
    Object.entries(FLIGHT_FIELDS).map(([key, [value, name]]) => [set[key] ?? value, name]),
  );
  const fills = {};
  const hints = {};
  const problems = [];
  forEachStep(data.PHASES, (id, row, phase) => {
    context.ITEM = { id, t: row[1], v: row[2], p: { id: phase.id } };
    if (row[2].includes("___")) {
      fills[id] = blankLabels(row[2]).map((label) => {
        context.LAB = label;
        const value = vm.runInContext("autoVal(ITEM,LAB)", context);
        if (!value) return "manual";
        const name = byValue.get(value);
        if (!name) problems.push(`${id}: blank filled from unknown value ${value}`);
        return name ?? "manual";
      });
    }
    const hint = vm.runInContext("hintOf(ITEM)", context);
    if (hint) {
      hints[id] = hint.split(" · ").map((part) => {
        const known = [...byValue.entries()].find(([value]) => part.endsWith(value));
        if (!known) {
          problems.push(`${id}: hint part not understood`);
          return null;
        }
        return { prefix: part.slice(0, -known[0].length), field: known[1] };
      });
      hints[id] = hints[id].filter(Boolean);
    }
  });
  return { fills, hints, problems };
}

function loadColors(template, data) {
  const context = {};
  vm.createContext(context);
  vm.runInContext(line(template, "function m21Color("), context);
  context.m21 = vm.runInContext("m21Color", context);
  const names = { amb: "amber", red: "red", grn: "green", blu: "blue" };
  const colors = {};
  for (const [key, panel] of Object.entries(data.PANELS)) {
    panel.subs.forEach((section, s) => {
      section[6].forEach((item, i) => {
        if (["l", "n", "b"].includes(item[0])) colors[`${key}.${s}.${i}`] = names[context.m21(item, key)];
      });
    });
  }
  return colors;
}

function evaluate(template, start) {
  const context = {};
  vm.createContext(context);
  const text = line(template, start);
  return { context, text };
}

function loadDefaults(template) {
  const { context, text } = evaluate(template, "const blank=()=>");
  return JSON.parse(vm.runInContext(`${text}\nJSON.stringify(blank())`, context));
}

function loadBriefingMap(template) {
  const { context, text } = evaluate(template, "const BMAP=");
  return JSON.parse(vm.runInContext(`${text}\nJSON.stringify(BMAP)`, context));
}

function loadOverview(template) {
  const from = template.indexOf("const R=[");
  const to = template.indexOf("];", from);
  if (from < 0 || to < 0) throw new Error("3.0 template: cannot find the overview map");
  const context = {};
  vm.createContext(context);
  const rows = JSON.parse(vm.runInContext(`${template.slice(from, to + 2)}\nJSON.stringify(R)`, context));
  const map = template.indexOf("function mapSVG(");
  const captionCode = between(template.slice(map), "const caps=", ";");
  const captions = JSON.parse(
    vm.runInContext(`${captionCode.replace("const ", "var ")}\nJSON.stringify(caps)`, context),
  );
  const size = /viewBox="0 0 (\d+) (\d+)"/.exec(template.slice(map));
  if (!size) throw new Error("3.0 template: cannot find the overview map size");
  return { rows, captions, width: Number(size[1]), height: Number(size[2]) };
}

/** Plain paragraphs from 3.0's small amount of intro HTML. */
export function parseIntro(html, problems, name) {
  const paragraphs = [...html.matchAll(/<p(?: class="lead")?>(.*?)<\/p>/g)].map((match) => {
    const lead = /^<b>(.*?):<\/b>\s*(.*)$/.exec(match[1]);
    const paragraph = lead ? { heading: lead[1], text: lead[2] } : { text: match[1] };
    if (/[<>]/.test(paragraph.text)) problems.push(`intro for ${name}: unexpected markup kept out`);
    return paragraph;
  });
  if (paragraphs.length === 0) problems.push(`intro for ${name}: no paragraphs found`);
  return paragraphs;
}

function loadIntros(template, problems) {
  const code = between(template, "refView(PH[cur],{", "}[cur])").slice("refView(PH[cur],".length);
  const context = {};
  vm.createContext(context);
  const raw = JSON.parse(vm.runInContext(`JSON.stringify(${code}})`, context));
  return Object.fromEntries(Object.entries(raw).map(([id, html]) => [id, parseIntro(html, problems, id)]));
}

/** Everything the converter needs from a 3.0 `src` folder. */
export function load3x(src) {
  const data = loadData(src);
  const template = fs.readFileSync(path.join(src, "template.html"), "utf8");
  const set = sampleSet();
  const behavior = loadStepBehavior(template, data, set);
  return {
    data,
    overview: loadOverview(template),
    intros: loadIntros(template, behavior.problems),
    defaults: loadDefaults(template),
    briefingPhases: loadBriefingMap(template),
    colors: loadColors(template, data),
    opens: [...new Set([...template.matchAll(/it\.id==='([^']+)'\?cduHTML/g)].map((m) => m[1]))],
    cdu: loadCdu(template, set),
    ...behavior,
  };
}
