// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Maps loaded 3.0 data onto the pack format (decision 0007). Pure: no file access.

import { convertCdu, leakedMarkers, verifyCdu } from "./cdu.js";
import { FLIGHT_FIELDS, forEachStep } from "./load3x.js";
import { normalizeDeep, normalizeSpelling, SpellingLog } from "./spelling.js";

export const PACK_ID = "ifly-737-max8";
const WALKTHROUGH = "preflight-setup";
const BASIS_TAGS = ["prior-release", "manufacturer-docs", "ifly-docs", "sim-observed"];
const KINDS = { Procedure: "flow", Checklist: "checklist", "Non-normal": "non-normal" };
const PANEL_KINDS = {
  t: "toggle_switch",
  g: "toggle_switch",
  k: "selector",
  p: "push_button",
  l: "light",
  n: "light",
  b: "light",
  d: "gauge",
  s: "readout",
  u: "display_unit",
};

/** Drops undefined and empty optional values, keeping key order. */
function clean(object) {
  return Object.fromEntries(
    Object.entries(object).filter(
      ([, v]) =>
        !(
          v === undefined ||
          v === null ||
          v === "" ||
          (Array.isArray(v) && v.length === 0) ||
          (typeof v === "object" && !Array.isArray(v) && Object.keys(v).length === 0)
        ),
    ),
  );
}

/** A lower-case, dash-separated name for use in ids. */
export function slug(text, max = 48) {
  const full = text
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "-")
    .replace(/^-+|-+$/g, "");
  if (full.length <= max) return full;
  const cut = full.slice(0, max);
  const dash = cut.lastIndexOf("-");
  return (dash > 12 ? cut.slice(0, dash) : cut).replace(/-+$/g, "");
}

function uniqueIn(taken, base) {
  let name = base;
  for (let n = 2; taken.has(name); n++) name = `${base}-${n}`;
  taken.add(name);
  return name;
}

const POWER_GPU = { option: "power", equals: "gpu" };
const POWER_APU = { option: "power", equals: "apu" };
const START_COLD = { option: "start", equals: "cold" };
const START_TURN = { option: "start", equals: "turn" };
const USES_GPU = { any_of: [POWER_GPU, START_TURN] };
const USES_APU_NOW = { all_of: [POWER_APU, START_COLD] };

function showIf(options, id, gaps) {
  const rules = [];
  if (options.gpu) rules.push(USES_GPU);
  if (options.apuNow) rules.push(USES_APU_NOW);
  if (options.appr) {
    rules.push(
      options.appr.length === 1
        ? { option: "approach", equals: options.appr[0] }
        : { option: "approach", one_of: options.appr },
    );
  }
  if (rules.length > 1) gaps.push(`${id}: more than one display rule; kept only the first`);
  return rules[0];
}

function timer(tm, idMap, id, gaps) {
  if (!tm) return undefined;
  let autoStart;
  if (tm.a) {
    if (!tm.m) gaps.push(`${id}: timer auto-start has no earliest or latest setting; used latest`);
    autoStart = { from: tm.a.map((old) => idMap.get(old)), pick: tm.m === "min" ? "earliest" : "latest" };
  }
  return clean({ seconds: tm.s, label: tm.l, note: tm.n, auto_start: autoStart });
}

function buildIds(data) {
  const idMap = new Map();
  const perPhase = new Map();
  forEachStep(data.PHASES, (oldId, row, phase, list) => {
    if (!perPhase.has(phase.id)) perPhase.set(phase.id, new Set());
    const taken = perPhase.get(phase.id);
    const base = slug(normalizeSpelling(row[1])) || "step";
    let short = base;
    if (taken.has(short)) {
      const tag = list.k === "Checklist" ? "check" : slug(normalizeSpelling(list.t), 20);
      short = `${base}-${tag}`;
    }
    idMap.set(oldId, `${PACK_ID}.${phase.id}.${uniqueIn(taken, short)}`);
  });
  return idMap;
}

function stepFor(oldId, row, section, ctx) {
  const options = row[4] || {};
  const recallMark = /\s*\(NON-RECALL\)\s*/;
  const nonRecall = recallMark.test(row[2]);
  const expect = row[2].replace(recallMark, " ").trim();
  if (!expect) ctx.gaps.push(`${oldId}: nothing left of the value after removing its marker`);
  const detail = typeof options.x === "object" && options.x !== null ? options.x : {};
  const walkaround = typeof options.x === "number" ? { x: options.x, y: options.y } : undefined;
  return clean({
    id: ctx.idMap.get(oldId),
    number: row[0] === 0 ? undefined : row[0],
    section,
    text: row[1],
    expect,
    controls: [].concat(row[3]).filter(Boolean),
    fill: ctx.input.fills[oldId],
    hint: ctx.input.hints[oldId],
    opens: ctx.input.opens.includes(oldId) ? WALKTHROUGH : undefined,
    non_recall: nonRecall || undefined,
    quick: options.q ? "always" : options.qt ? "turnaround" : undefined,
    show_if: showIf(options, oldId, ctx.gaps),
    note: options.d,
    why: detail.y,
    check: detail.c,
    check_detail: detail.cs,
    if_not: detail.f,
    if_not_detail: detail.fs,
    how: options.how,
    explain: options.why,
    walkaround,
    look_for: options.look,
    location: options.where,
    callout: options.say,
    timer: timer(options.tm, ctx.idMap, oldId, ctx.gaps),
    panel_lights: options.pcl,
    not_simulated: options.ns ? true : undefined,
    status: "draft",
  });
}

function buildLists(ctx) {
  const { data } = ctx.input;
  const lists = [];
  const taken = new Set();
  const phases = data.PHASES.map((phase) => {
    const listIds = [];
    phase.lists.forEach((list, listIndex) => {
      const id = uniqueIn(taken, slug(normalizeSpelling(`${phase.id}-${list.t}`), 64));
      listIds.push(id);
      const steps = [];
      let i = 0;
      let n = 0;
      let section;
      for (const row of list.items) {
        if (row.h) {
          section = row.h;
          continue;
        }
        const options = row[4] || {};
        const oldId =
          options.id || (options.a ? `${phase.id}-${listIndex}-a${n++}` : `${phase.id}-${listIndex}-${i++}`);
        steps.push(stepFor(oldId, row, section, ctx));
        section = undefined;
      }
      if (section) ctx.gaps.push(`${id}: section "${section}" has no steps after it`);
      ctx.listIds.set(`${phase.id}/${listIndex}`, id);
      if (list.mem) ctx.memoryLists.set(list.t, id);
      lists.push(
        clean({
          id,
          title: list.t,
          kind: KINDS[list.k],
          basis: ["prior-release"],
          memory: list.mem ? true : undefined,
          when: list.when,
          why: list.why,
          steps,
        }),
      );
    });
    return clean({
      id: phase.id,
      title: phase.t,
      reference: phase.ref ? true : undefined,
      intro: ctx.input.intros[phase.id],
      lists: listIds,
    });
  });
  return { lists, phases };
}

function stagePlace(phase, stageIndex) {
  if (phase === "cold-dark") return { phase, show_if: START_COLD };
  if (phase === "gpu-power") return { phase, show_if: { all_of: [POWER_GPU, START_COLD] } };
  if (phase === "apu-power") return { phase, show_if: stageIndex === 1 ? USES_GPU : USES_APU_NOW };
  return { phase };
}

function buildPhasesFile(ctx, phases) {
  const { data, defaults } = ctx.input;
  const stages = data.STAGES.map(([title, ids], stageIndex) => ({
    title,
    phases: ids.map((phase) => stagePlace(phase, stageIndex)),
  }));
  return {
    options: [
      { id: "power", title: "Power source", values: ["gpu", "apu"], default: defaults.power },
      { id: "start", title: "Start type", values: ["cold", "turn"], default: defaults.start },
      { id: "approach", title: "Approach type", values: ["ils", "rnav", "vis"], default: defaults.appr },
    ],
    stages,
    phases,
  };
}

function buildOverview({ width, height, captions, rows }) {
  return {
    width,
    height,
    captions: captions.map(([text, x, y]) => ({ text, x, y })),
    areas: rows.map(([zone, x, y, w, h, label]) => clean({ zone, x, y, width: w, height: h, label })),
  };
}

function buildControls(data, overview) {
  return {
    zones: Object.entries(data.ZONES).map(([id, [area, title, short]]) => ({
      id,
      area,
      title,
      short_title: short,
    })),
    controls: Object.entries(data.C).map(([id, [name, zone, location, fn]]) =>
      clean({ id, name, zone: data.ZONES[zone] ? zone : undefined, location, function: fn }),
    ),
    overview: buildOverview(overview),
  };
}

function buildNotes(data) {
  return {
    how: Object.entries(data.HOW).map(([id, steps]) => ({ id, steps })),
    explain: Object.entries(data.WHY).map(([id, w]) =>
      clean({
        id,
        what: w.w,
        why: w.y,
        success: w.s,
        failure: w.b,
        check: w.c,
        check_detail: w.cs,
        if_not: w.f,
        if_not_detail: w.fs,
      }),
    ),
  };
}

function buildPanels(ctx) {
  const { data, colors } = ctx.input;
  return Object.entries(data.PANELS).map(([key, panel]) => ({
    id: key,
    title: panel.t,
    width: panel.w,
    height: panel.h,
    sections: panel.subs.map(([id, title, x, y, width, height, items], s) => ({
      id,
      title,
      x,
      y,
      width,
      height,
      items: items.map((item, i) => {
        const [code, label, ix, iy, controlField, positions, description, w, h] = item;
        const controls = controlField ? String(controlField).split("|") : [];
        return clean({
          kind: PANEL_KINDS[code],
          label,
          color: colors[`${key}.${s}.${i}`],
          guarded: code === "g" ? true : undefined,
          x: ix,
          y: iy,
          controls,
          positions: positions || undefined,
          description: description || label,
          width: w,
          height: h,
        });
      }),
    })),
  }));
}

function buildLights(data) {
  return {
    lights: Object.values(data.LIGHTS21).map((l) =>
      clean({
        locator: l.p,
        meaning: l.m,
        when: l.w,
        action: l.a,
        normal: l.n,
        status: l.c === "confirm" ? "draft" : undefined,
      }),
    ),
  };
}

function buildFaults(ctx) {
  const { data } = ctx.input;
  const faults = [];
  for (const [key, cause] of Object.entries(data.CAUSE21)) {
    const [phaseId, title] = key.split("|");
    const steps = [];
    forEachStep(data.PHASES, (oldId, row, phase) => {
      if (phase.id === phaseId && row[1] === title) steps.push(ctx.idMap.get(oldId));
    });
    if (steps.length === 0) {
      ctx.dropped.push(`fault causes for "${key}": no step has that title`);
      continue;
    }
    faults.push({ steps, causes: cause.cz, action: cause.a });
  }
  return { faults };
}

function buildBriefings(ctx) {
  const { data, briefingPhases } = ctx.input;
  const phaseOf = new Map();
  for (const [phase, indexes] of Object.entries(briefingPhases)) {
    for (const index of indexes) phaseOf.set(index, phase);
  }
  const taken = new Set();
  const briefings = data.BRIEFS.map((brief, b) => {
    const id = uniqueIn(taken, slug(normalizeSpelling(brief.t)));
    const inBrief = new Set();
    const items = brief.b.map(([lead, rest, detail], i) => {
      const itemId = `${id}.${uniqueIn(inBrief, slug(normalizeSpelling(`${lead}${rest}`), 30))}`;
      ctx.aliases.push({ kind: "briefing_item", from: `b-${b}-${i}`, to: itemId });
      return clean({ id: itemId, lead, rest, detail });
    });
    return { id, title: brief.t, mnemonic: brief.m, phase: phaseOf.get(b), items };
  });
  return { briefings };
}

function buildStartStates(ctx) {
  return {
    states: ctx.input.data.PSTATES.map((state) => {
      ctx.aliases.push({ kind: "start_state", from: state.k, to: state.k });
      return clean({
        id: state.k,
        title: state.t,
        description: state.d,
        set: state.power ? { power: state.power } : undefined,
        phase: state.at,
        done: state.done.map((id) => ctx.idMap.get(id)),
        skip: state.skip.map((id) => ctx.idMap.get(id)),
      });
    }),
  };
}

function buildAliases(ctx, phases) {
  const { input, aliases, idMap } = ctx;
  for (const [from, to] of idMap) aliases.push({ kind: "step", from, to });
  forEachStep(input.data.PHASES, (oldId) => {
    (input.fills[oldId] ?? []).forEach((_, n) => {
      aliases.push({ kind: "blank", from: `${oldId}-v${n}`, to: `${idMap.get(oldId)}#${n}` });
    });
  });
  for (const value of ["gpu", "apu"])
    aliases.push({ kind: "option_value", from: `power:${value}`, to: `power:${value}` });
  for (const value of ["cold", "turn"])
    aliases.push({ kind: "option_value", from: `start:${value}`, to: `start:${value}` });
  for (const value of ["ils", "rnav", "vis"])
    aliases.push({ kind: "option_value", from: `appr:${value}`, to: `approach:${value}` });
  for (const [from, [, to]] of Object.entries(FLIGHT_FIELDS)) aliases.push({ kind: "fill_field", from, to });
  for (const [title, id] of ctx.memoryLists) aliases.push({ kind: "list", from: `m:${title}`, to: id });
  for (const phase of phases) aliases.push({ kind: "phase", from: `f:${phase.id}`, to: phase.id });
  ctx.notConverted.push(
    "practice scores saved per cockpit area (c:...) have no alias: an area spans several zones, and practice is Phase 3",
  );
  return { aliases };
}

function counts(input, files) {
  const { data, cdu } = input;
  let steps = 0;
  let headers = 0;
  let lists = 0;
  const bySource = { Checklist: 0, Procedure: 0, "Non-normal": 0 };
  for (const phase of data.PHASES) {
    for (const list of phase.lists) {
      lists++;
      bySource[list.k]++;
      for (const row of list.items) {
        if (row.h) headers++;
        else steps++;
      }
    }
  }
  const panelItems = Object.values(data.PANELS).reduce(
    (a, panel) => a + panel.subs.reduce((b, section) => b + section[6].length, 0),
    0,
  );
  const packLists = files.lists.map(([, list]) => list);
  const packSteps = packLists.flatMap((list) => list.steps);
  const packPanelItems = files.panels
    .flatMap(([, panel]) => panel.sections)
    .reduce((a, section) => a + section.items.length, 0);
  const sourcePanelItems = Object.values(data.PANELS).flatMap((panel) => panel.subs.flatMap((sub) => sub[6]));
  const packPanelItemList = files.panels
    .flatMap(([, panel]) => panel.sections)
    .flatMap((section) => section.items);
  const packBriefItems = files.briefings.briefings.flatMap((b) => b.items);
  const kindCount = (kind) => packLists.filter((list) => list.kind === kind).length;
  const row = (source, pack) => ({ source, pack });
  return {
    phases: row(data.PHASES.length, files.phases.phases.length),
    stages: row(data.STAGES.length, files.phases.stages.length),
    lists: row(lists, packLists.length),
    checklists: row(bySource.Checklist, kindCount("checklist")),
    flows: row(bySource.Procedure, kindCount("flow")),
    non_normal: row(bySource["Non-normal"], kindCount("non-normal")),
    memory_lists: row(
      data.PHASES.flatMap((p) => p.lists).filter((l) => l.mem).length,
      packLists.filter((l) => l.memory).length,
    ),
    steps: row(steps, packSteps.length),
    section_headers: row(headers, packSteps.filter((s) => s.section).length),
    timers: row(4, packSteps.filter((s) => s.timer).length),
    blanks_filled_steps: row(Object.keys(input.fills).length, packSteps.filter((s) => s.fill).length),
    controls: row(Object.keys(data.C).length, files.controls.controls.length),
    zones: row(Object.keys(data.ZONES).length, files.controls.zones.length),
    how_notes: row(Object.keys(data.HOW).length, files.notes.how.length),
    explanations: row(Object.keys(data.WHY).length, files.notes.explain.length),
    panels: row(Object.keys(data.PANELS).length, files.panels.length),
    panel_items: row(panelItems, packPanelItems),
    light_notes: row(Object.keys(data.LIGHTS21).length, files.lights.lights.length),
    fault_causes: row(Object.keys(data.CAUSE21).length, files.faults.faults.length),
    overview_rows: row(input.overview.rows.length, files.controls.overview.areas.length),
    overview_captions: row(input.overview.captions.length, files.controls.overview.captions.length),
    phase_intros: row(Object.keys(input.intros).length, files.phases.phases.filter((p) => p.intro).length),
    briefing_items: row(data.BRIEFS.flatMap((b) => b.b).length, packBriefItems.length),
    briefing_details: row(
      data.BRIEFS.flatMap((b) => b.b).filter((item) => item[2]).length,
      packBriefItems.filter((item) => item.detail).length,
    ),
    panel_items_with_several_controls: row(
      sourcePanelItems.filter((item) => String(item[4] ?? "").includes("|")).length,
      packPanelItemList.filter((item) => (item.controls ?? []).length > 1).length,
    ),
    panel_control_links: row(
      sourcePanelItems.reduce((n, item) => n + (item[4] ? String(item[4]).split("|").length : 0), 0),
      packPanelItemList.reduce((n, item) => n + (item.controls ?? []).length, 0),
    ),
    glossary_terms: row(Object.keys(data.GLOSS).length, files.glossary.terms.length),
    briefings: row(data.BRIEFS.length, files.briefings.briefings.length),
    start_states: row(data.PSTATES.length, files.startStates.states.length),
    cdu_pages: row(cdu.blank.length, files.cdu.walkthroughs[0].pages.length),
    sources: row(data.SOURCES.length, 0),
  };
}

function manifest(files) {
  const f = (name) => `${name}.yaml`;
  return {
    $schema: "../../schemas/pack.schema.json",
    id: PACK_ID,
    name: "iFly 737 MAX 8",
    version: "0.1.0",
    schemaVersion: 2,
    kind: "aircraft",
    engine: ">=0.1.0, <1.0.0",
    license: "CC-BY-NC-SA-4.0",
    attribution: ["A study aid for flight simulation. Not for real-world use."],
    basisTags: BASIS_TAGS,
    checklists: files.lists.map(([name]) => name),
    phases: f("phases"),
    controls: f("controls"),
    notes: f("notes"),
    panels: files.panels.map(([name]) => name),
    lights: f("lights"),
    faults: f("faults"),
    glossary: f("glossary"),
    briefings: f("briefings"),
    startStates: f("start-states"),
    cdu: f("cdu"),
    aliases: f("aliases"),
  };
}

/** Replaces flagged text with our own wording. Returns what changed and any unknown target. */
function applyRewrites(files, rewrites) {
  const applied = [];
  const missing = [];
  const apply = (group, label, targets) => {
    const byId = new Map(targets.map((target) => [target.id, target]));
    for (const [id, fields] of Object.entries(rewrites[group] ?? {})) {
      const target = byId.get(id);
      if (!target) {
        missing.push(`rewrite target not found: ${label} ${id}`);
        continue;
      }
      for (const [field, text] of Object.entries(fields)) {
        target[field] = text;
        applied.push(`${label} ${id}: ${field}`);
      }
    }
  };
  const lists = files.lists.map(([, list]) => list);
  apply(
    "steps",
    "step",
    lists.flatMap((list) => list.steps),
  );
  apply("lists", "list", lists);
  apply("controls", "control", files.controls.controls);
  apply(
    "briefingItems",
    "briefing item",
    files.briefings.briefings.flatMap((briefing) => briefing.items),
  );
  return { applied, missing };
}

/** The pack's files as data, and a report of what happened. */
export function transform(input, rewrites = {}) {
  const { data } = input;
  const log = new SpellingLog();
  const ctx = {
    input,
    gaps: [...input.problems],
    dropped: [],
    notConverted: [],
    aliases: [],
    listIds: new Map(),
    memoryLists: new Map(),
    idMap: buildIds(data),
  };
  const { lists, phases } = buildLists(ctx);
  const cduGaps = [];
  const cduPages = convertCdu(input.cdu.blank, input.cdu.filled, input.cdu.table, cduGaps);
  const legPages = /(\d+)$/.exec(
    input.cdu.filled
      .map((page) => page.s.p)
      .find((_, i) => input.cdu.filled[i].s.p !== input.cdu.blank[i].s.p) ?? "",
  );
  const cduDifferences = verifyCdu(cduPages, input.cdu.blank, input.cdu.filled, input.cdu.table, {
    pages: Number(legPages?.[1] ?? 1),
  });
  const leaks = leakedMarkers(cduPages, input.cdu.table);
  const files = {
    lists: lists.map((list) => [`lists/${list.id}.yaml`, list]),
    phases: buildPhasesFile(ctx, phases),
    controls: buildControls(data, input.overview),
    notes: buildNotes(data),
    panels: buildPanels(ctx).map((panel) => [`panels/${panel.id}.yaml`, panel]),
    lights: buildLights(data),
    faults: buildFaults(ctx),
    glossary: { terms: Object.entries(data.GLOSS).map(([term, meaning]) => ({ term, meaning })) },
    briefings: buildBriefings(ctx),
    startStates: buildStartStates(ctx),
    cdu: {
      walkthroughs: [
        {
          id: WALKTHROUGH,
          title: "CDU walkthrough",
          pages: cduPages,
        },
      ],
    },
  };
  files.aliases = buildAliases(ctx, phases);
  const rewritten = applyRewrites(files, rewrites);
  const normalized = {
    ...normalizeDeep(files, log),
    lists: files.lists.map(([name, list]) => [name, normalizeDeep(list, log)]),
    panels: files.panels.map(([name, panel]) => [name, normalizeDeep(panel, log)]),
  };
  return {
    manifest: manifest(normalized),
    files: normalized,
    report: {
      counts: counts(input, normalized),
      spelling: log.summary(),
      problems: [
        ...ctx.gaps,
        ...cduGaps,
        ...cduDifferences,
        ...leaks.map((m) => `CDU marker leaked: ${m}`),
        ...rewritten.missing,
      ],
      rewrites: rewritten.applied,
      dropped: ctx.dropped,
      notConverted: ctx.notConverted,
    },
  };
}
