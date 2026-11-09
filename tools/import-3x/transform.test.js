// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import assert from "node:assert/strict";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { test } from "node:test";

import { writePack } from "./convert.js";
import { load3x, parseIntro } from "./load3x.js";
import { slug, transform } from "./transform.js";

// Invented data with the shape of the 3.0 structures. Not real procedures.
function data() {
  return {
    ZONES: { z1: ["Area", "Zone one", "Z1"] },
    C: {
      c1: ["Control one", "z1", "On the left.", "Does a thing."],
      c2: ["Spoken", "__briefing", "Between the crew.", "A review."],
    },
    STAGES: [
      ["Start", ["cold-dark", "gpu-power", "apu-power"]],
      ["Go", ["apu-power"]],
    ],
    PHASES: [
      {
        id: "cold-dark",
        t: "Cold",
        lists: [
          {
            k: "Procedure",
            t: "First flow",
            items: [
              { h: "A section" },
              [
                1,
                "THE SWITCH",
                "ON",
                "c1",
                {
                  q: 1,
                  x: { y: "Why it matters", c: "Check it", cs: "More", f: "If not", fs: "Then" },
                  d: "A note",
                  how: "h1",
                  why: "w1",
                  say: "Say it",
                },
              ],
              [2, "THE SWITCH", "V ___ AND ___", "c2", { qt: 1, gpu: 1 }],
              [0, "OPTIONAL", "SET (NON-RECALL)", ["c1", "c2"], { a: 1, apuNow: 1, ns: 1 }],
              [3, "STOP", "CLEAR", "c1", { x: 40, y: 50, look: "Look here", where: "Outside" }],
              [
                4,
                "WAIT",
                "DONE",
                "c1",
                { tm: { s: 60, l: "Wait", n: "Typical", a: ["cold-dark-0-0"], m: "min" }, appr: ["ils"] },
              ],
              [5, "MANY", "DONE", "c1", { appr: ["ils", "rnav"], pcl: { c1: ["LIGHT"] } }],
            ],
          },
          { k: "Checklist", t: "A checklist", items: [[1, "THE SWITCH", "ON", "c1"]] },
        ],
      },
      {
        id: "gpu-power",
        t: "Ground",
        lists: [{ k: "Procedure", t: "Ground flow", items: [[1, "CENTRE", "COLOUR", "c1"]] }],
      },
      {
        id: "apu-power",
        t: "APU",
        lists: [{ k: "Procedure", t: "APU flow", items: [[1, "APU", "ON", "c1"]] }],
      },
      {
        id: "memory",
        t: "Memory",
        opt: 1,
        ref: 1,
        lists: [
          {
            k: "Non-normal",
            nn: 1,
            mem: 1,
            t: "Smoke",
            when: "When",
            why: "Why",
            items: [[1, "MASK", "ON", "c1"]],
          },
        ],
      },
    ],
    BRIEFS: [
      {
        t: "Depart",
        m: "AB",
        b: [
          ["A", "LPHA"],
          ["B", "RAVO", "A detail"],
        ],
      },
    ],
    SOURCES: [["Some site", "https://example.com/secret-source"]],
    HOW: { h1: ["Do this."] },
    GLOSS: { TERM: "A meaning." },
    WHY: { w1: { w: "What", y: "Why", s: "Good", b: "Bad" } },
    PANELS: {
      oh: {
        t: "Overhead",
        w: 100,
        h: 100,
        subs: [
          [
            "sec",
            "Section",
            0,
            0,
            50,
            50,
            [
              ["g", "GUARD", 1, 2, "c1", "OFF, ON", "Guarded."],
              ["l", "AMBER", 3, 4, "c1|c2", "", "A light."],
              ["n", "GREEN", 5.5, 6, null, "", "A light."],
              ["u", "SCREEN", 7, 8, null, "", "A screen.", 20, 30],
            ],
          ],
        ],
      },
    },
    LIGHTS21: {
      "oh.0.1": {
        m: "Means",
        w: "When",
        a: "Act",
        n: false,
        lab: "AMBER",
        sub: "Section",
        p: "oh/sec/AMBER/1",
        c: "confirm",
      },
      "oh.0.2": {
        m: "Means",
        w: "When",
        a: "Act",
        n: true,
        lab: "GREEN",
        sub: "Section",
        p: "oh/sec/GREEN/1",
      },
    },
    CAUSE21: {
      "cold-dark|THE SWITCH": { cz: ["A cause"], a: "An action" },
      "cold-dark|GONE": { cz: ["A cause"], a: "An action" },
    },
    PSTATES: [
      {
        k: "ready",
        t: "Ready",
        d: "Set",
        power: "apu",
        at: "cold-dark",
        done: ["cold-dark-0-0"],
        skip: ["cold-dark-1-0"],
      },
    ],
  };
}

function page(rows) {
  return { pg: "P", t: "Page", a: "Act.", c: "Check.", s: { t: "P", p: "1/1", hl: [], r: rows } };
}

function input() {
  return {
    data: data(),
    overview: {
      width: 200,
      height: 100,
      captions: [["TOP", 1, 2]],
      rows: [
        ["z1", 1, 2, 3, 4, "Label"],
        ["z1", 5, 6, 7, 8],
      ],
    },
    intros: { memory: [{ heading: "When", text: "Soon." }, { text: "Then more." }] },
    defaults: { power: "gpu", start: "cold", appr: "ils" },
    briefingPhases: { "cold-dark": [0] },
    colors: { "oh.0.1": "amber", "oh.0.2": "green" },
    opens: ["cold-dark-0-0"],
    fills: { "cold-dark-0-1": ["v2", "manual"] },
    hints: { "cold-dark-0-0": [{ prefix: "SQUAWK ", field: "squawk" }] },
    problems: [],
    cdu: {
      blank: [page([["A", "b", "", ""]])],
      filled: [page([["A", "b", "", ""]])],
      table: {
        values: [{ value: "ZZAA", target: { field: "origin" } }],
        whole: [],
        leg1Via: { value: "DIRECT", target: { field: "route_leg", index: 1, part: "via" } },
        overflow: { value: "7", target: { route_legs: "overflow" } },
      },
    },
  };
}

const result = () => transform(input());
const flow = (r) => r.files.lists[0][1];

test("slugs are lower-case and cut at a word boundary", () => {
  assert.equal(slug("A/P & A/T Switch"), "a-p-a-t-switch");
  assert.equal(slug("word ".repeat(20), 22), "word-word-word-word");
});

test("lists get a kind and the prior-release basis, and steps start as draft", () => {
  const r = result();
  assert.deepEqual(
    r.files.lists.map(([, l]) => l.kind),
    ["flow", "checklist", "flow", "flow", "non-normal"],
  );
  for (const [, list] of r.files.lists) {
    assert.deepEqual(list.basis, ["prior-release"]);
    for (const step of list.steps) assert.equal(step.status, "draft");
  }
});

test("step ids follow the format, and clashes in a phase stay unique", () => {
  const r = result();
  assert.equal(flow(r).steps[0].id, "ifly-737-max8.cold-dark.the-switch");
  assert.equal(flow(r).steps[1].id, "ifly-737-max8.cold-dark.the-switch-first-flow");
  assert.equal(r.files.lists[1][1].steps[0].id, "ifly-737-max8.cold-dark.the-switch-check");
});

test("a step's fields map onto the format", () => {
  const [a, b, c, d, e, f] = flow(result()).steps;
  assert.equal(a.section, "A section");
  assert.equal(a.number, 1);
  assert.equal(a.quick, "always");
  assert.equal(a.why, "Why it matters");
  assert.equal(a.check_detail, "More");
  assert.equal(a.if_not_detail, "Then");
  assert.equal(a.note, "A note");
  assert.equal(a.how, "h1");
  assert.equal(a.explain, "w1");
  assert.equal(a.callout, "Say it");
  assert.equal(a.opens, "preflight-setup");
  assert.deepEqual(a.hint, [{ prefix: "SQUAWK ", field: "squawk" }]);
  assert.equal(b.quick, "turnaround");
  assert.deepEqual(b.fill, ["v2", "manual"]);
  assert.equal(b.show_if.any_of.length, 2);
  assert.equal(c.number, undefined);
  assert.equal(c.expect, "SET");
  assert.equal(c.non_recall, true);
  assert.equal(c.not_simulated, true);
  assert.equal(c.show_if.all_of.length, 2);
  assert.deepEqual(c.controls, ["c1", "c2"]);
  assert.deepEqual(d.walkaround, { x: 40, y: 50 });
  assert.equal(d.look_for, "Look here");
  assert.equal(d.location, "Outside");
  assert.deepEqual(e.timer.auto_start, { from: ["ifly-737-max8.cold-dark.the-switch"], pick: "earliest" });
  assert.deepEqual(e.show_if, { option: "approach", equals: "ils" });
  assert.deepEqual(f.show_if, { option: "approach", one_of: ["ils", "rnav"] });
  assert.deepEqual(f.panel_lights, { c1: ["LIGHT"] });
});

test("reference phases, memory lists and when/why carry over", () => {
  const r = result();
  assert.equal(r.files.phases.phases[3].reference, true);
  const memory = r.files.lists[4][1];
  assert.equal(memory.memory, true);
  assert.equal(memory.when, "When");
});

test("stages show the start phases by option", () => {
  const { stages } = result().files.phases;
  assert.deepEqual(stages[0].phases[0].show_if, { option: "start", equals: "cold" });
  assert.equal(stages[0].phases[1].show_if.all_of.length, 2);
  assert.ok(stages[0].phases[2].show_if.all_of);
  assert.ok(stages[1].phases[0].show_if.any_of);
});

test("options take their defaults from 3.0", () => {
  const [power, start, approach] = result().files.phases.options;
  assert.equal(power.default, "gpu");
  assert.equal(start.default, "cold");
  assert.equal(approach.default, "ils");
});

test("controls, panels and lights map across", () => {
  const r = result();
  assert.equal(r.files.controls.controls[1].zone, undefined);
  const [guard, amber, green, screen] = r.files.panels[0][1].sections[0].items;
  assert.equal(guard.kind, "toggle_switch");
  assert.equal(guard.guarded, true);
  assert.equal(amber.color, "amber");
  assert.deepEqual(amber.controls, ["c1", "c2"]);
  assert.equal(green.kind, "light");
  assert.equal(green.color, "green");
  assert.equal(screen.kind, "display_unit");
  assert.equal(screen.width, 20);
  assert.equal(r.files.lights.lights[0].status, "draft");
  assert.equal(r.files.lights.lights[1].status, undefined);
  assert.equal(guard.controls.length, 1);
  assert.equal(green.controls, undefined);
});

test("fault causes match by title, and ones with no step are reported", () => {
  const r = result();
  assert.equal(r.files.faults.faults.length, 1);
  assert.equal(r.files.faults.faults[0].steps.length, 3);
  assert.equal(r.report.dropped.length, 1);
});

test("a briefing item's detail carries over", () => {
  const r = result();
  const { items, phase } = r.files.briefings.briefings[0];
  assert.equal(phase, "cold-dark");
  assert.equal(items[0].detail, undefined);
  assert.equal(items[1].detail, "A detail");
  assert.equal(r.report.counts.briefing_details.pack, 1);
});

test("the cockpit overview map carries over, keeping every rectangle", () => {
  const { overview } = result().files.controls;
  assert.equal(overview.width, 200);
  assert.deepEqual(overview.captions, [{ text: "TOP", x: 1, y: 2 }]);
  assert.deepEqual(overview.areas, [
    { zone: "z1", x: 1, y: 2, width: 3, height: 4, label: "Label" },
    { zone: "z1", x: 5, y: 6, width: 7, height: 8 },
  ]);
});

test("phase intros carry over only where they exist", () => {
  const { phases } = result().files.phases;
  assert.equal(phases[0].intro, undefined);
  assert.deepEqual(phases[3].intro, [{ heading: "When", text: "Soon." }, { text: "Then more." }]);
});

test("intro HTML becomes plain paragraphs", () => {
  const problems = [];
  const html = '<div class="intro"><p><b>When:</b> Soon.</p><p class="lead">Plain.</p></div>';
  assert.deepEqual(parseIntro(html, problems, "x"), [{ heading: "When", text: "Soon." }, { text: "Plain." }]);
  assert.deepEqual(problems, []);
  parseIntro("<p>A <i>tag</i></p>", problems, "x");
  assert.equal(problems.length, 1);
});

test("aliases cover every kind of saved id", () => {
  const { aliases } = result().files.aliases;
  const kinds = new Set(aliases.map((a) => a.kind));
  assert.deepEqual(
    kinds,
    new Set(["step", "blank", "briefing_item", "start_state", "option_value", "fill_field", "list", "phase"]),
  );
  const find = (from) => aliases.find((a) => a.from === from)?.to;
  assert.equal(find("cold-dark-0-a0"), "ifly-737-max8.cold-dark.optional");
  assert.equal(find("b-0-1"), "depart.bravo");
  assert.equal(find("cold-dark-0-1-v1"), "ifly-737-max8.cold-dark.the-switch-first-flow#1");
  assert.equal(find("appr:vis"), "approach:vis");
  assert.equal(find("trim"), "takeoff_trim");
  assert.equal(find("m:Smoke"), "memory-smoke");
});

test("text and ids are American, and the changes are reported", () => {
  const r = result();
  const step = r.files.lists[2][1].steps[0];
  assert.equal(step.text, "CENTER");
  assert.equal(step.expect, "COLOR");
  assert.equal(step.id, "ifly-737-max8.gpu-power.center");
  assert.equal(r.report.spelling.changes["centre -> center"], 1);
});

test("rewrites replace text by id, keep the id, and are reported", () => {
  const id = "ifly-737-max8.cold-dark.the-switch";
  const r = transform(input(), {
    steps: { [id]: { text: "OUR WORDING" } },
    lists: { "memory-smoke": { when: "Our condition." } },
    controls: { c1: { function: "Our function." } },
    briefingItems: { "depart.bravo": { detail: "Our detail." } },
  });
  assert.equal(flow(r).steps[0].id, id);
  assert.equal(flow(r).steps[0].text, "OUR WORDING");
  assert.equal(r.files.lists[4][1].when, "Our condition.");
  assert.equal(r.files.controls.controls[0].function, "Our function.");
  assert.equal(r.files.briefings.briefings[0].items[1].detail, "Our detail.");
  assert.equal(r.report.rewrites.length, 4);
  assert.deepEqual(r.report.problems, []);
});

test("a rewrite aimed at something that does not exist is reported", () => {
  const r = transform(input(), { steps: { ghost: { text: "X" } } });
  assert.deepEqual(r.report.problems, ["rewrite target not found: step ghost"]);
});

test("3.0's sources and links do not carry over", () => {
  const text = JSON.stringify(result().files);
  assert.ok(!text.includes("example.com"));
  assert.ok(!text.includes("Some site"));
});

test("counts compare the source with the pack", () => {
  const { counts } = result().report;
  assert.equal(counts.steps.source, counts.steps.pack);
  assert.equal(counts.section_headers.pack, 1);
  assert.equal(counts.panel_items.pack, 4);
  assert.equal(counts.sources.pack, 0);
});

function fakeSource() {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), "import-3x-"));
  const d = data();
  fs.writeFileSync(
    path.join(dir, "build.py"),
    "LAYERS = ['a.js',\n  'b.js']  # layers\nNAMES = 'ZONES,C,STAGES,PHASES,BRIEFS,SOURCES,HOW,GLOSS,WHY,PANELS,LIGHTS21,CAUSE21,PSTATES'\n",
  );
  fs.writeFileSync(
    path.join(dir, "a.js"),
    `const ZONES=${JSON.stringify(d.ZONES)};const C=${JSON.stringify(d.C)};const STAGES=${JSON.stringify(d.STAGES)};const PHASES=${JSON.stringify(d.PHASES)};const BRIEFS=${JSON.stringify(d.BRIEFS)};const SOURCES=${JSON.stringify(d.SOURCES)};const HOW=${JSON.stringify(d.HOW)};const GLOSS=${JSON.stringify(d.GLOSS)};const WHY=${JSON.stringify(d.WHY)};const PANELS=${JSON.stringify(d.PANELS)};const LIGHTS21=${JSON.stringify(d.LIGHTS21)};const CAUSE21=${JSON.stringify(d.CAUSE21)};const PSTATES=${JSON.stringify(d.PSTATES)};`,
  );
  fs.writeFileSync(path.join(dir, "b.js"), "PHASES[0].t='Cold start';");
  fs.writeFileSync(
    path.join(dir, "template.html"),
    [
      "const R=[\n['z1',1,2,3,4,'Label'],['z1',5,6,7,8]\n];",
      "function mapSVG(on){const caps=[['TOP',1,2]];return `<svg viewBox=\"0 0 200 100\">`}",
      "m.innerHTML=refView(PH[cur],{memory:'<p class=\"lead\">Intro.</p>'}[cur]);",
      "const blank=()=>({power:'gpu',appr:'ils',start:'cold'});",
      "const BMAP={'cold-dark':[0]};",
      "function autoVal(it,lab){return lab==='V'?S.set.v2:''}",
      "function hintOf(it){return it.id==='cold-dark-0-0'?'SQUAWK '+S.set.sq:''}",
      "function valHTML(){}",
      "function m21Color(i,A){return i[0]==='n'?'grn':'amb'}",
      "it.id==='cold-dark-0-0'?cduHTML()",
      "let cdwI=0;",
      "function cdwPos(a,b){return a+' '+b}",
      "function cdwAirac(){return{id:2601,t:0,nx:0}}",
      "function cdwDay(){return 'JAN01'}",
      "function cdwSteps(){const z=S.set;return[{pg:'P',t:'Page',a:'Act.',c:'Check.',s:{t:'P',p:'1/1',hl:[],r:[['RWY',z.rwy?{u:z.rwy}:'---','',''],[{sep:1},'<A','','B>']]}}]}",
      "function cdwScreen(){}",
    ].join("\n"),
  );
  return dir;
}

test("a source folder loads in build order and writes a pack", () => {
  const src = fakeSource();
  const out = path.join(src, "out");
  try {
    const result3x = load3x(src);
    assert.equal(result3x.data.PHASES[0].t, "Cold start");
    assert.deepEqual(result3x.fills["cold-dark-0-1"], ["v2", "manual"]);
    assert.deepEqual(result3x.opens, ["cold-dark-0-0"]);
    const converted = transform(result3x);
    assert.deepEqual(converted.report.problems, []);
    assert.equal(converted.files.controls.overview.areas.length, 2);
    assert.deepEqual(converted.files.phases.phases[3].intro, [{ text: "Intro." }]);
    assert.deepEqual(converted.files.cdu.walkthroughs[0].pages[0].screen.rows[0].left, [
      { field: "runway", style: "user", empty: "---" },
    ]);
    writePack(out, converted);
    const manifest = JSON.parse(fs.readFileSync(path.join(out, "pack.json"), "utf8"));
    assert.equal(manifest.schemaVersion, 2);
    for (const file of [
      ...manifest.checklists,
      ...manifest.panels,
      manifest.phases,
      manifest.cdu,
      manifest.aliases,
    ]) {
      assert.ok(fs.existsSync(path.join(out, file)), file);
    }
    assert.ok(fs.readFileSync(path.join(out, "phases.yaml"), "utf8").startsWith("# SPDX-FileCopyrightText"));
  } finally {
    fs.rmSync(src, { recursive: true, force: true });
  }
});
