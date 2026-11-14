// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// An invented fixture for the decision step (Decision.dc.html), for the design gallery and tests
// only: never part of a shipped pack or the app's runtime. It carries what pack format 4 cannot
// yet say, the fuel step's value for each answer and a plan suggestion; batch 7 proposes the pack
// field and the suggestion rule (with a 0022 amendment). The location line and "0 lb" are
// the board's stand-ins.

import type { FlatPack, PhaseList, Step, StepRow, StepState, UserOption } from "@chocktochock/bindings";

export const OPTION: UserOption = {
  id: "center_tank",
  title: "Center tank",
  values: ["more", "low"],
  default: "low",
  description: "The center tank holds",
  choices: [
    { value: "more", label: "More than 1,000 lb (450 kg)" },
    { value: "low", label: "1,000 lb (450 kg) or less" },
  ],
};

/** The fuel step's value before an answer and for each answer (fixture only). */
export const FUEL_VALUE: Record<string, string> = {
  none: "MAIN PUMPS ON\nCENTER PUMPS ON or OFF",
  more: "MAIN PUMPS ON\nCENTER PUMPS ON",
  low: "MAIN PUMPS ON\nCENTER PUMPS OFF",
};

/** The plan's suggestion (the board's stand-in figure). */
export const SUGGESTION = { value: "low", amount: "0 lb" };

const ID = "fixture.before-start";
const step = (id: string, text: string, expect: string, more: Partial<Step> = {}): Step => ({
  id: `${ID}.${id}`,
  text,
  expect,
  ...more,
});

export const STEPS: Step[] = [
  step("landing-gear-pins", "LANDING GEAR PINS", "REMOVED"),
  step("n1-bugs", "N1 BUGS", "SET"),
  step("mcp", "MCP", "SET"),
  step("door-lights", "DOOR LIGHTS", "OUT"),
  step("terr", "TERR", "ON", { targets: [{ at: "gl/efis/TERR/1", set: "ON" }] }),
  step("center-tank-fuel", "CENTER TANK FUEL", "CHECK QUANTITY", {
    decide: OPTION.id,
    controls: ["fuelqty"],
  }),
  step("fuel-pump-switches", "FUEL PUMP SWITCHES", FUEL_VALUE.none ?? "", {
    controls: ["fuelpumps"],
    targets: [
      { at: "oh/fuel/CTR L/1", open: true },
      { at: "oh/fuel/CTR L/1", set: "ON", when: { option: OPTION.id, equals: "more" } },
      { at: "oh/fuel/CTR L/1", set: "OFF", when: { option: OPTION.id, equals: "low" } },
    ],
  }),
  step("electric-hydraulic-pump-switches", "ELECTRIC HYDRAULIC PUMP SWITCHES", "ON"),
];

export const PACK: FlatPack = {
  manifest: {
    id: "fixture",
    name: "Fixture",
    version: "0.0.0",
    schemaVersion: 4,
    kind: "aircraft",
    engine: "1",
    dependencies: {},
    license: "CC-BY-NC-SA-4.0",
    attribution: [],
    basisTags: [],
    checklists: ["before-start"],
  },
  lists: [{ id: "before-start", title: "Before start procedure", kind: "flow", basis: [], steps: STEPS }],
  content: {
    phases: {
      options: [OPTION],
      stages: [],
      phases: [{ id: "before-start", title: "Before start", lists: ["before-start"] }],
    },
    controls: {
      zones: [
        {
          id: "p2",
          area: "Main panel (P2)",
          title: "fuel quantity on the upper display",
          short_title: "Fuel quantity",
        },
        { id: "p5", area: "Forward overhead (P5)", title: "Fuel panel", short_title: "Fuel" },
      ],
      controls: [
        { id: "fuelqty", name: "Fuel quantity", zone: "p2", location: "", function: "" },
        { id: "fuelpumps", name: "Fuel pump switches", zone: "p5", location: "", function: "" },
      ],
    },
    notes: null,
    panels: [],
    lights: null,
    faults: null,
    glossary: null,
    briefings: null,
    start_states: null,
    cdu: null,
    aliases: null,
  },
};

/** The procedure card's list for an answer (null: unanswered): five done, then the decision. */
export function fixtureList(answered: boolean): PhaseList {
  const states: StepState[] = STEPS.map((s, n) => {
    if (n < 5) return "done";
    if (n === 5) return answered ? "done" : "current";
    if (n === 6) return answered ? "current" : "to_do";
    return "to_do";
  });
  const rows: StepRow[] = STEPS.map((s, n) => ({
    id: s.id,
    number: n + 1,
    state: states[n] ?? "to_do",
    counts: true,
    targets: [],
    essential: true,
    values: [],
    hint: "",
  }));
  const done = states.filter((s) => s === "done").length;
  return {
    id: "before-start",
    title: "Before start procedure",
    kind: "flow",
    steps: rows,
    // The board's counts: the card is an excerpt of a ten-step list with one skipped further on.
    progress: { done, skipped: 1, toDo: 10 - done - 1, total: 10 },
    complete: false,
    hidden: 0,
    current: rows.find((r) => r.state === "current")?.id ?? null,
  };
}
