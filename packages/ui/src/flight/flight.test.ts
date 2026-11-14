// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import type {
  Decoded,
  FillField,
  Flight,
  SavedPlan,
  Settings,
  StartState,
  UserOption,
} from "@chocktochock/bindings";
import { cleanup, fireEvent, render, screen, within } from "@testing-library/svelte";
import { afterEach, describe, expect, it } from "vitest";
import { type Host, hostContext } from "../host";
import FlightPage from "./FlightPage.svelte";
import { blankLinks, home, SECTIONS, toEnter } from "./sections";

afterEach(cleanup);

const flight = (values: Partial<Record<FillField, string>> = {}, notes = ""): Flight =>
  ({
    format: 2,
    pack: "p",
    packVersion: "1",
    options: { power: "gpu", start: "cold" },
    startState: null,
    session: { done: {}, skipped: {} },
    values,
    blanks: {},
    notes,
    timers: {},
  }) as Flight;
const settings = { descriptions: true, simbriefUser: null } as unknown as Settings;
const METAR: Decoded = {
  station: "KSEA",
  time: [12, 17, 53],
  groups: [
    ["KSEA", 0],
    ["121753Z", 1],
    ["18008KT", 2],
    ["A3012", 3],
  ],
  rows: [
    { element: "station", raw: ["KSEA"], lines: ["KSEA"], source: "FM 15 · 15.2" },
    { element: "time", raw: ["121753Z"], lines: ["Day 12, 17:53 UTC"], source: "FM 15 · 15.3" },
    { element: "wind", raw: ["18008KT"], lines: ["180° at 8 kt"], source: "FM 15 · 15.5" },
    {
      element: "altimeter",
      raw: ["A3012"],
      lines: ["30.12 inHg"],
      source: "US · 3.1.5.11",
    },
  ],
};
const plan = {
  format: 1,
  loadedAt: 0,
  plan: {
    general: {},
    origin: {
      icao: "KSEA",
      metar: "KSEA 121753Z 18008KT A3012",
      taf: "TAF KSEA 121720Z 1218/1324 18008KT P6SM FM130200 20010KT P6SM",
    },
    destination: { icao: "KSFO" },
    alternates: [],
    fixes: [],
    alternateFixes: [],
    fuel: { trip: 8100, taxi: 400 },
    weights: {},
    times: { enrouteS: 6720 },
  },
} as unknown as SavedPlan;
const host = { decodeMetar: async () => METAR, decodeTaf: async () => METAR } as unknown as Host;

function page(props: Record<string, unknown>) {
  const sets: [string, string][] = [];
  const options: UserOption[] = [
    {
      id: "power",
      title: "Power source",
      values: ["gpu", "apu"],
      default: "gpu",
      choices: [
        { value: "gpu", label: "Ground power at the gate" },
        { value: "apu", label: "No ground power" },
      ],
    },
    { id: "exterior", title: "Exterior inspection", values: ["on", "off"], default: "on" },
    {
      id: "approach",
      title: "Approach type",
      values: ["ils", "rnav", "vis"],
      default: "ils",
      choices: [
        { value: "ils", label: "ILS" },
        { value: "rnav", label: "RNAV" },
        { value: "vis", label: "Visual" },
      ],
    },
  ];
  const states = [
    { id: "cd", title: "Cold & Dark", description: "", phase: "cold-dark", done: [] },
  ] as unknown as StartState[];
  const calls: unknown[] = [];
  render(FlightPage, {
    context: hostContext(host),
    props: {
      section: "overview",
      focus: null,
      flight: flight(),
      settings,
      plan: null,
      options,
      states,
      backHref: "#/phase/before-start",
      onSet: (f: string, v: string) => sets.push([f, v]),
      onNotes: (n: string) => calls.push(["notes", n]),
      onOption: (o: string, v: string) => calls.push([o, v]),
      onStartState: (s: string | null) => calls.push(["state", s]),
      onImport: async () => "Couldn't reach SimBrief.",
      onFile: async () => null,
      onClear: () => calls.push("clear"),
      onDescriptions: (on: boolean) => calls.push(["descriptions", on]),
      ...props,
    },
  });
  return { sets, calls };
}

describe("Flight sections data", () => {
  it("counts what the steps still need, per section and in all", () => {
    expect(toEnter({}, "takeoff").map((r) => r.label)).toEqual([
      "Runway heading",
      "Flaps",
      "V2",
      "Stabilizer trim",
    ]);
    expect(toEnter({ heading: "163", takeoff_flaps: "5" }, "takeoff").length).toBe(2);
    // Departure frequency shows as still needed in its box, but no step's blank uses it.
    expect(toEnter({}, "clearance").map((r) => r.field)).toEqual(["runway", "initial_altitude"]);
  });

  it("gives every editable field one home and links each step's blanks to it", () => {
    const fields = SECTIONS.flatMap((s) => s.groups.flatMap((g) => g.rows))
      .filter((r) => r.field && !r.link)
      .map((r) => r.field);
    expect(new Set(fields).size).toBe(fields.length);
    expect(home("takeoff_trim")?.section).toBe("takeoff");
    expect(blankLinks(["takeoff_trim", "manual", "qnh", "destination_qnh", "minimum_takeoff_fuel"])).toEqual([
      { href: "#/flight/takeoff/takeoff_trim", name: "takeoff trim" },
      null,
      { href: "#/flight/weather/qnh", name: "QNH" },
      { href: "#/flight/weather/destination_qnh", name: "QNH" },
      // Read-only on Fuel and weights: typed in the step.
      null,
    ]);
  });
});

describe("FlightPage", () => {
  it("lists the sections with what each still needs, and goes back to the checklist", () => {
    page({});
    const nav = screen.getByRole("navigation", { name: "Flight sections" });
    expect(within(nav).getByRole("link", { name: /^Takeoff/ }).textContent).toContain("4 to enter");
    expect(
      within(nav)
        .getByRole("link", { name: /^Overview/ })
        .getAttribute("aria-current"),
    ).toBe("page");
    expect(screen.getByRole("link", { name: "Checklist" }).getAttribute("href")).toBe("#/phase/before-start");
    expect(screen.getByText("How an OFP is laid out")).toBeTruthy();
  });

  it("shows weights in full with the unit and stores thousands where the CDU takes them", async () => {
    const { sets } = page({
      section: "fuel",
      flight: flight({ zfw: "126.5", fuel: "12400", weight_unit: "LB" }),
    });
    const zfw = document.querySelector<HTMLInputElement>('[data-field="zfw"]');
    expect(zfw?.value).toBe("126,500 lb");
    await fireEvent.change(zfw as HTMLInputElement, { target: { value: "127,300 lb" } });
    expect(sets.at(-1)).toEqual(["zfw", "127.3"]);
    await fireEvent.click(screen.getByRole("button", { name: "kg" }));
    expect(sets.at(-1)).toEqual(["weight_unit", "KG"]);
  });

  it("prints values as the boards do (FL350, +2, 18,000, 24.0%) and reads them back", async () => {
    const { sets } = page({
      section: "cruise",
      flight: flight({ cruise_level: "350", isa_deviation: "2", transition_altitude: "18000" }),
    });
    const val = (f: string) => document.querySelector<HTMLInputElement>(`[data-field="${f}"]`)?.value;
    expect([val("cruise_level"), val("isa_deviation"), val("transition_altitude")]).toEqual([
      "FL350",
      "+2",
      "18,000",
    ]);
    await fireEvent.change(document.querySelector('[data-field="transition_altitude"]') as HTMLElement, {
      target: { value: "10,000" },
    });
    expect(sets.at(-1)).toEqual(["transition_altitude", "10000"]);
  });

  it("names each of a row's two boxes, and shows a value outside 3.0's range under its box", async () => {
    page({ section: "takeoff", flight: flight({ thrust_rating: "26K", v2: "40" }) });
    expect(screen.getByLabelText("Thrust: rating")).toBeTruthy();
    expect(screen.getByLabelText("Thrust: assumed temperature")).toBeTruthy();
    expect(screen.getByRole("alert").textContent).toBe("Use 100 to 250 knots");
    expect(document.querySelector('[data-field="v2"]')?.getAttribute("aria-invalid")).toBe("true");
  });

  it("marks both parts of Reserves in the fuel plan close-up, so it agrees with the field", async () => {
    const withFuel = {
      ...plan,
      plan: {
        ...plan.plan,
        fuel: { trip: 8100, alternate: 2000, reserve: 1400, taxi: 400, planRamp: 12400 },
      },
    } as SavedPlan;
    page({ section: "fuel", plan: withFuel, flight: flight({ reserves: "3.4", fuel: "12400" }) });
    await fireEvent.click(document.querySelector('[data-row="reserves"]') as HTMLElement);
    const marked = [...document.querySelectorAll(".printed .mark")].map((m) => m.textContent?.trim());
    expect(marked).toEqual(["ALTN       2000", "RESERVE    1400"]);
  });

  it("draws a value still needed amber, a choice as a menu, and a cross-section row as a link", async () => {
    const { sets } = page({ section: "takeoff", flight: flight({ runway: "16L" }) });
    const v2 = document.querySelector<HTMLInputElement>('[data-field="v2"]');
    expect(v2?.classList.contains("need")).toBe(true);
    expect(v2?.placeholder).toBe("From the FMC");
    await fireEvent.click(document.querySelector('[data-field="takeoff_flaps"]') as HTMLElement);
    await fireEvent.click(screen.getByRole("button", { name: "5" }));
    expect(sets.at(-1)).toEqual(["takeoff_flaps", "5"]);
    expect(screen.getByRole("link", { name: "Clearance ›" }).getAttribute("href")).toBe("#/flight/clearance");
    expect(document.querySelector('[data-row="takeoff_runway"] .ro')?.textContent).toBe("16L");
  });

  it("Weather: the METAR with the selected group marked, the Altimeter as a field, and the TAF as printed", async () => {
    const { sets } = page({
      section: "weather",
      plan,
      flight: flight({ origin: "KSEA", destination: "KSFO", qnh: "30.12", qnh_unit: "inHg" }),
    });
    expect(await screen.findByText("Observed 17:53 UTC")).toBeTruthy();
    expect(document.querySelector(".groups .hi")?.textContent).toBe("18008KT");
    const alt = screen.getByLabelText("Altimeter") as HTMLInputElement;
    expect(alt.value).toBe("30.12");
    expect(screen.getByText("From METAR A3012")).toBeTruthy();
    await fireEvent.change(alt, { target: { value: "30.10" } });
    expect(sets.at(-1)).toEqual(["qnh", "30.10"]);
    await fireEvent.click(screen.getByRole("button", { name: "hPa" }));
    expect(sets).toContainEqual(["qnh_unit", "hPa"]);
    expect(sets).toContainEqual(["qnh", "1020"]);
    await fireEvent.click(screen.getByRole("button", { name: /^TAF: the forecast for KSEA/ }));
    expect(document.querySelector(".tafp")?.textContent).toBe(
      "TAF KSEA 121720Z 1218/1324 18008KT P6SM\nFM130200 20010KT P6SM",
    );
    // The OFP panel follows the selected row.
    expect(screen.getByText("Wind is in the weather section")).toBeTruthy();
  });

  it("Weather's ? says where the code is defined, in plain words", async () => {
    page({ section: "weather", plan, flight: flight({ origin: "KSEA", qnh: "30.12", qnh_unit: "inHg" }) });
    await screen.findByText("Observed 17:53 UTC");
    await fireEvent.click(screen.getByRole("button", { name: "More about wind" }));
    expect(screen.getByText(/Defined in the WMO's METAR code \(FM 15\), regulation 15\.5\./)).toBeTruthy();
    await fireEvent.click(screen.getByRole("button", { name: "More about the altimeter setting" }));
    expect(
      screen.getByText(/US practice, defined in the FAA's aviation weather guidance, section 3\.1\.5\.11\./),
    ).toBeTruthy();
  });

  it("Weather without a plan: the empty state, and the Altimeter still to enter", async () => {
    page({ section: "weather", focus: "qnh" });
    expect(
      screen.getByText(
        /Import your SimBrief plan to see the weather, or enter the departure and destination/,
      ),
    ).toBeTruthy();
    expect((screen.getByLabelText("Altimeter") as HTMLInputElement).classList.contains("need")).toBe(true);
  });

  it("Start options and SimBrief: toggles, the panel state, and an import problem shown", async () => {
    const { calls } = page({ section: "start" });
    await fireEvent.click(screen.getByRole("switch", { name: "Exterior inspection" }));
    expect(calls.at(-1)).toEqual(["exterior", "off"]);
    await fireEvent.change(screen.getByLabelText("Panel state"), { target: { value: "cd" } });
    expect(calls.at(-1)).toEqual(["state", "cd"]);
    cleanup();
    page({ section: "simbrief" });
    await fireEvent.input(screen.getByLabelText("SimBrief username or pilot ID"), {
      target: { value: "pilot" },
    });
    await fireEvent.click(screen.getByRole("button", { name: "Import latest flight" }));
    expect(await screen.findByRole("alert")).toBeTruthy();
  });
});
