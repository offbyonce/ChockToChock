// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// The Flight sections (FlightSection boards, batch 4): each row's label, the flight field it
// edits (or a read-only value, or a link to the section that holds it), its description,
// destination chip and where the OFP prints it. Labels, descriptions, chips and OFP close-ups
// are the boards' stand-ins, on batch 7's validation list.

import type { FillField, Flight, SavedPlan } from "@chocktochock/bindings";
import type { FlightSection } from "../route.svelte";

/** What a row's OFP panel reads. */
export type Ctx = {
  values: Partial<Record<FillField, string>>;
  plan: SavedPlan | null;
  notes: string;
  /** "lb" or "kg". */
  unit: string;
};

/** A stretch of a close-up, marked when it is the value. */
export type Part = { text: string; hi?: boolean; br?: boolean };

/** The OFP's sections, in SimBrief's default order. */
export const OFP_SECTIONS = [
  "Header and times",
  "Fuel plan",
  "Weights",
  "Navigation log, with the wind at each fix",
  "Alternate",
  "Weather: METAR and TAF",
  "NOTAMs",
] as const;

export type Ofp = {
  /** Index into OFP_SECTIONS, or null for a panel without the page outline. */
  section: number | null;
  title: string;
  body: string;
  /** Not set from the OFP (Clearance, Takeoff, Approach's chart values). */
  notin?: boolean;
  cap: string;
  close: (c: Ctx) => Part[];
  note: string;
};

export type Row = {
  id: string;
  label: string;
  /** The field this row edits. */
  field?: FillField;
  /** A read-only value (plain mono text, no box). */
  read?: (c: Ctx) => string;
  /** A row that belongs to another section: its value is read-only and the chip is a link. */
  link?: FlightSection;
  /** A read-only row whose chip links to the phase that uses it ("Before start ›"). */
  phase?: string;
  /** A second field in the same row (Passengers and cargo). */
  second?: {
    field: FillField;
    placeholder: string;
    /** Its own accessible name ("assumed temperature"). */
    name: string;
    max: number;
    mode?: "numeric" | "decimal";
    weight?: "thousands";
  };
  /** The first box's own name when a row has two ("rating", "passengers"). */
  firstName?: string;
  /** The flight's notes from ATC (Clearance), not a fill field. */
  notes?: boolean;
  /** How a value reads: "FL180", "163°". */
  show?: (value: string) => string;
  /** Choice values: a field-styled button that opens a menu. */
  choices?: string[];
  /** The empty box's placeholder; amber while empty. */
  placeholder?: string;
  /** A weight in the flight's unit: shown in full with the unit ("12,400 lb"); `thousands`
   * when the flight stores thousands, as the CDU takes them. */
  weight?: "full" | "thousands";
  max?: number;
  mode?: "numeric" | "decimal" | "text";
  desc: string;
  /** Where the value goes in the cockpit, a plain muted line under the description ("Enter on
   * the CDU › PERF INIT"). */
  loc?: string;
  /** The link text of a row whose value belongs to a phase ("Before start ›"). */
  chip?: string;
  /** A two-way control under the description (Minimums: BARO / RADIO). */
  seg?: { field: FillField; values: [string, string]; label: string };
  ofp: Ofp;
};

export type Group = { title: string; note?: (c: Ctx) => string; units?: boolean; rows: Row[] };
export type Section = { id: FlightSection; title: string; sub: string; groups: Group[]; selected: string };

const v = (c: Ctx, f: FillField) => c.values[f] ?? "";
const dash = (s: string | undefined | null) => (s ? s : "—");
const grouped = (n: number) => Math.round(n).toLocaleString("en-US");
/** "+2" for a positive deviation, as the boards print it. */
const signed = (x: string) => (x && Number(x) > 0 && !x.startsWith("+") ? `+${x}` : x);

/** A plan figure in the flight's unit, or a dash without a plan. */
const planWeight = (c: Ctx, n: number | undefined) => (c.plan && n != null ? `${grouped(n)} ${c.unit}` : "—");

function minutes(s: number | undefined): string {
  if (s == null) return "—";
  const m = Math.round(s / 60);
  return `${Math.floor(m / 60)} h ${m % 60} min`;
}

// Close-ups, built from the flight's own values (the boards' stand-ins in shape).
const routeLine = (c: Ctx, hi: "route" | "sid" | "arrival"): Part[] => {
  const dep = `${dash(v(c, "origin"))}/${dash(v(c, "runway"))} `;
  const route = v(c, "route") || "—";
  const arr = ` ${dash(v(c, "destination"))}/${dash(v(c, "arrival_runway"))}`;
  if (hi === "sid") {
    const [first, ...rest] = route.split(" ");
    return [{ text: dep }, { text: first ?? "—", hi: true }, { text: ` ${rest.slice(0, 3).join(" ")} …` }];
  }
  if (hi === "arrival") {
    const words = route.split(" ");
    return [
      { text: `… ${words.at(-2) ?? ""} ` },
      { text: `${words.at(-1) ?? "—"}${arr.replace(" ", " ")}`, hi: true },
    ];
  }
  return [{ text: dep }, { text: route, hi: true }, { text: arr }];
};
const header = (
  c: Ctx,
  hi: "level" | "ci" | "wind" | "isa" | "ta" | "tl" | "fn" | "airports" | "alt" | "dist" | "time",
): Part[] => {
  // The close-up shows what the OFP prints: the plan's figures, or the flight's without a plan.
  const g = c.plan?.plan.general;
  const three = (n: number) => String(Math.round(n)).padStart(3, "0");
  const level =
    g?.cruiseAltitudeFt != null ? String(Math.round(g.cruiseAltitudeFt / 100)) : v(c, "cruise_level");
  const ci = g?.costIndex != null ? String(g.costIndex) : v(c, "cost_index");
  const wind =
    g?.avgWindDir != null && g.avgWindSpeed != null
      ? `${three(g.avgWindDir)}/${three(g.avgWindSpeed)}`
      : v(c, "cruise_wind");
  const isa = g?.avgIsaDeviation != null ? String(g.avgIsaDeviation) : v(c, "isa_deviation");
  const parts: [string, string][] = [
    ["level", `FL${dash(level)}`],
    ["ci", `CI ${dash(ci)}`],
    ["wind", `AVG WIND ${dash(wind)}`],
    ["isa", `ISA ${dash(signed(isa))}`],
  ];
  if (["level", "ci", "wind", "isa"].includes(hi))
    return parts.flatMap(([k, text], n): Part[] => [...(n ? [{ text: "  " }] : []), { text, hi: k === hi }]);
  if (hi === "ta" || hi === "tl")
    return [
      { text: `TRANS ALT ${dash(v(c, "transition_altitude"))}`, hi: hi === "ta" },
      { text: "   " },
      { text: `TRANS LVL FL${dash(v(c, "transition_level"))}`, hi: hi === "tl" },
    ];
  if (hi === "fn")
    return [
      { text: dash(v(c, "flight_number")), hi: true },
      { text: `  ${dash(v(c, "origin"))}-${dash(v(c, "destination"))}` },
    ];
  if (hi === "alt") return [{ text: "ALTN " }, { text: dash(v(c, "alternate")), hi: true }];
  if (hi === "dist")
    return [
      { text: "AIR DIST " },
      {
        text: c.plan?.plan.general.airDistanceNm != null ? `${c.plan.plan.general.airDistanceNm} NM` : "—",
        hi: true,
      },
    ];
  if (hi === "time")
    return [{ text: "EST TIME ENR " }, { text: minutes(c.plan?.plan.times.enrouteS), hi: true }];
  return [
    { text: dash(v(c, "origin")), hi: true },
    { text: " → " },
    { text: dash(v(c, "destination")), hi: true },
  ];
};
const fuelPlan = (c: Ctx, hi: "trip" | "reserve" | "taxi" | "block" | "min"): Part[] => {
  const f = c.plan?.plan.fuel;
  const line = (k: string, label: string, n: number | string | undefined, last = false): Part => ({
    text: `${label.padEnd(9)}${String(n ?? "—").padStart(6)}`,
    hi: k === hi || (hi === "reserve" && k === "altn"),
    br: !last,
  });
  return [
    line("trip", "TRIP", f?.trip),
    line("altn", "ALTN", f?.alternate),
    line("reserve", "RESERVE", f?.reserve),
    line("taxi", "TAXI", f?.taxi),
    line("min", "MIN T/O", f?.minTakeoff),
    line("block", "BLOCK", f?.planRamp ?? v(c, "fuel"), true),
  ];
};
const weights = (c: Ctx, hi: "zfw" | "tow" | "law" | "pax"): Part[] => {
  const w = c.plan?.plan.weights;
  const k = (n: number | undefined) => (n != null ? (n / 1000).toFixed(1) : "—");
  if (hi === "pax")
    return [
      { text: "PAX " },
      { text: `${w?.passengers ?? "—"}`, hi: true },
      { text: `   CARGO ${w?.cargo ?? "—"}` },
    ];
  return [
    { text: `ZFW ${k(w?.zfw)}`, hi: hi === "zfw" },
    { text: "   " },
    { text: `TOW ${k(w?.takeoff)}`, hi: hi === "tow" },
    { text: "   " },
    { text: `LAW ${k(w?.landing)}`, hi: hi === "law" },
  ];
};

const OFP_NOTE = "Section order depends on the OFP layout chosen in SimBrief.";
const fromHeader = (title: string, close: Ofp["close"], note: string): Ofp => ({
  section: 0,
  title,
  body: "The header at the top of page 1 has the flight number, the airports, the route as filed, the distance and the times.",
  cap: "Close-up, the header",
  close,
  note,
});
const fromFuel = (title: string, hi: Parameters<typeof fuelPlan>[1], note: string): Ofp => ({
  section: 1,
  title,
  body: "The fuel plan lists each part of the fuel, then the total you board.",
  cap: "Close-up, the fuel plan",
  close: (c) => fuelPlan(c, hi),
  note,
});
const fromWeights = (title: string, hi: Parameters<typeof weights>[1], note: string): Ofp => ({
  section: 2,
  title,
  body: "The weights section gives the planned zero fuel, takeoff and landing weights, in thousands.",
  cap: "Close-up, the weights section",
  close: (c) => weights(c, hi),
  note,
});
const clearanceOfp = (title = "Your clearance comes from ATC"): Ofp => ({
  section: 0,
  title,
  body: "The OFP shows what was planned and filed. ATC's clearance can differ, so enter what ATC gives you here.",
  notin: true,
  cap: "Close-up, the filed route",
  close: (c) => routeLine(c, "sid"),
  note: "The filed SID is first in the route. Check it against your clearance.",
});
const takeoffOfp: Ofp = {
  section: 2,
  title: "Takeoff speeds come from the FMC",
  body: "The OFP plans the flight, not the takeoff. Speeds and trim come from the CDU's TAKEOFF REF page once weights and flaps are set. Some SimBrief layouts add takeoff performance at the end.",
  notin: true,
  cap: "Close-up, the weights section",
  close: (c) => weights(c, "tow"),
  note: "The takeoff weight (in thousands of lb) is what the FMC uses for these speeds.",
};
const approachRoute: Ofp = {
  section: 0,
  title: "The arrival runway and STAR end the route",
  body: "The header's route line ends with the arrival: the STAR, then the destination and its runway. The destination's elevation is in the header too. Minimums and landing speed come from the approach chart and the FMC, not the OFP.",
  cap: "Close-up, the end of the route",
  close: (c) => routeLine(c, "arrival"),
  note: "Check the planned runway against the ATIS before you brief; it often changes.",
};
const chartOfp = (title: string): Ofp => ({
  section: null,
  title,
  body: "Minimums come from the approach chart, and the landing speed and flaps from the CDU's APPROACH REF page. The OFP doesn't print them.",
  notin: true,
  cap: "Close-up, the end of the route",
  close: (c) => routeLine(c, "arrival"),
  note: "The arrival in the plan is where these values apply.",
});

export const FLAPS = ["1", "5", "10", "15", "25"];
export const LANDING_FLAPS = ["15", "30", "40"];
/** The takeoff thrust ratings on the CDU's N1 LIMIT page (FSTakeoff board). */
export const THRUST = ["TO", "TO-1", "TO-2"];
export const AUTOBRAKE = ["OFF", "1", "2", "3", "MAX"];

/** Fields the steps' blanks use, and the STAR: what "N to enter" and Still to enter count. */
export const NEEDED: FillField[] = [
  "runway",
  "initial_altitude",
  "heading",
  "takeoff_flaps",
  "v2",
  "takeoff_trim",
  "fuel",
  "minimum_takeoff_fuel",
  "qnh",
  "destination_qnh",
  "star",
  "minimums",
  "vref",
  "landing_flaps",
  "autobrake",
  "landing_altitude",
];

export const SECTIONS: Section[] = [
  {
    id: "route",
    title: "Route",
    sub: "Where you're going and the way there. Used by the route and CDU steps.",
    selected: "route",
    groups: [
      {
        title: "Airports",
        rows: [
          {
            id: "origin",
            label: "Departure",
            field: "origin",
            max: 4,
            desc: "The airport you depart from. Its weather is on the Weather page.",
            loc: "Enter on the CDU › RTE",
            ofp: fromHeader(
              "The airports are in the header",
              (c) => header(c, "airports"),
              "The header names both airports first, in ICAO codes.",
            ),
          },
          {
            id: "destination",
            label: "Destination",
            field: "destination",
            max: 4,
            desc: "Where you plan to land.",
            loc: "Enter on the CDU › RTE",
            ofp: fromHeader(
              "The airports are in the header",
              (c) => header(c, "airports"),
              "The header names both airports first, in ICAO codes.",
            ),
          },
          {
            id: "alternate",
            label: "Alternate",
            field: "alternate",
            max: 4,
            desc: "Where you'd divert if you couldn't land at the destination.",
            loc: "Enter on the CDU › RTE 2",
            ofp: {
              section: 4,
              title: "The alternate has its own section",
              body: "The alternate section gives the alternate airport, the route there and the fuel to get there.",
              cap: "Close-up, the alternate section",
              close: (c) => header(c, "alt"),
              note: "The header names the alternate too.",
            },
          },
          {
            id: "flight_number",
            label: "Flight number",
            field: "flight_number",
            max: 7,
            desc: "Your flight's number, as filed.",
            loc: "Enter on the CDU › RTE",
            ofp: fromHeader(
              "The flight number opens the header",
              (c) => header(c, "fn"),
              "It comes first, before the airports.",
            ),
          },
        ],
      },
      {
        title: "The route",
        rows: [
          {
            id: "route",
            label: "Route",
            field: "route",
            max: 400,
            mode: "text",
            desc: "The filed route from the SID to the arrival, as you enter it in the CDU.",
            loc: "Enter on the CDU › RTE",
            ofp: {
              ...fromHeader(
                "The route is in the header",
                (c) => routeLine(c, "route"),
                "The route sits between the departure and the destination, each with its runway.",
              ),
              cap: "Close-up, the header's route line",
            },
          },
          {
            id: "distance",
            label: "Distance",
            read: (c) =>
              c.plan?.plan.general.airDistanceNm != null ? `${c.plan.plan.general.airDistanceNm} nm` : "—",
            desc: "Planned air distance.",
            ofp: fromHeader(
              "The distance is in the header",
              (c) => header(c, "dist"),
              "Air distance allows for the winds along the route.",
            ),
          },
          {
            id: "time",
            label: "Planned time",
            read: (c) => minutes(c.plan?.plan.times.enrouteS),
            desc: "Takeoff to landing.",
            ofp: fromHeader(
              "The planned time is in the header",
              (c) => header(c, "time"),
              "Estimated time en route, from takeoff to landing.",
            ),
          },
        ],
      },
    ],
  },
  {
    id: "fuel",
    title: "Fuel and weights",
    sub: "Numbers for the fuel and weight steps and the CDU's PERF INIT page.",
    selected: "fuel",
    groups: [
      {
        title: "Fuel",
        units: true,
        rows: [
          {
            id: "fuel",
            label: "Block fuel",
            field: "fuel",
            weight: "full",
            mode: "numeric",
            desc: "All the fuel on board before engine start.",
            loc: "Check on the fuel gauges",
            ofp: fromFuel(
              "Block fuel is in the fuel plan",
              "block",
              "Block is the last line: everything on board before engine start.",
            ),
          },
          // Center tank: hidden until batch 7 (owner); then block fuel less the wing capacity, exact.
          {
            id: "trip",
            label: "Trip fuel",
            read: (c) => planWeight(c, c.plan?.plan.fuel.trip),
            desc: "Planned burn from takeoff to landing.",
            ofp: fromFuel(
              "Trip fuel is in the fuel plan",
              "trip",
              "Trip is the first line: the burn from takeoff to landing.",
            ),
          },
          {
            id: "reserves",
            label: "Reserves",
            field: "reserves",
            weight: "thousands",
            mode: "decimal",
            desc: "Alternate plus final reserve: what must be left at landing.",
            loc: "Enter on the CDU › PERF INIT",
            ofp: fromFuel(
              "Reserves are in the fuel plan",
              "reserve",
              "Reserve is the final reserve; the alternate fuel is listed beside it.",
            ),
          },
          {
            id: "taxi",
            label: "Taxi fuel",
            read: (c) => planWeight(c, c.plan?.plan.fuel.taxi),
            desc: "Planned burn before takeoff.",
            ofp: fromFuel(
              "Taxi fuel is in the fuel plan",
              "taxi",
              "Taxi is the planned burn before takeoff.",
            ),
          },
          {
            id: "min_fuel",
            label: "Minimum takeoff fuel",
            read: (c) =>
              v(c, "minimum_takeoff_fuel")
                ? `${grouped(Number(v(c, "minimum_takeoff_fuel")))} ${c.unit}`
                : "—",
            phase: "before-start",
            desc: "Block fuel less taxi fuel. Check it before takeoff.",
            chip: "Before start ›",
            ofp: fromFuel(
              "Minimum takeoff fuel is in the fuel plan",
              "min",
              "The least fuel to start the takeoff with.",
            ),
          },
        ],
      },
      {
        title: "Weights",
        note: () => "Same unit as fuel",
        rows: [
          {
            id: "zfw",
            label: "Zero fuel weight",
            field: "zfw",
            weight: "thousands",
            mode: "decimal",
            desc: "The airplane's weight without fuel.",
            loc: "Enter on the CDU › PERF INIT",
            ofp: fromWeights(
              "Zero fuel weight is in the weights section",
              "zfw",
              "In thousands, as the CDU takes it.",
            ),
          },
          {
            id: "cg",
            label: "ZFW CG",
            field: "cg",
            show: (x) => `${x}%`,
            mode: "decimal",
            desc: "Center of gravity without fuel, used for takeoff trim.",
            loc: "Enter on the CDU › TAKEOFF REF",
            ofp: fromWeights(
              "The CG is in the weights section",
              "zfw",
              "Some layouts print the CG beside the zero fuel weight.",
            ),
          },
          {
            id: "payload",
            label: "Passengers and cargo",
            field: "passengers",
            max: 3,
            mode: "numeric",
            firstName: "passengers",
            second: {
              field: "cargo",
              placeholder: "Cargo",
              name: "cargo",
              max: 7,
              mode: "decimal",
              weight: "thousands",
            },
            desc: "Payload in the load sheet.",
            ofp: fromWeights("The payload is in the weights section", "pax", "Passengers, then cargo."),
          },
          {
            id: "tow",
            label: "Planned takeoff weight",
            field: "takeoff_weight",
            weight: "thousands",
            mode: "decimal",
            desc: "Zero fuel weight plus fuel at takeoff, from the plan.",
            loc: "Enter on the CDU › PERF INIT",
            ofp: fromWeights(
              "The takeoff weight is in the weights section",
              "tow",
              "Cross-check it with the FMC's gross weight.",
            ),
          },
        ],
      },
    ],
  },
  {
    id: "cruise",
    title: "Cruise and levels",
    sub: "Used by the climb, cruise and descent steps.",
    selected: "cost_index",
    groups: [
      {
        title: "Cruise",
        rows: [
          {
            id: "cruise_level",
            label: "Cruise level",
            field: "cruise_level",
            show: (x) => `FL${x}`,
            max: 3,
            mode: "numeric",
            desc: "The level you plan to cruise at.",
            loc: "Enter on the CDU › PERF INIT",
            // FSCruise and FSInvalid: the same heading, body and note as Cost index.
            ofp: {
              ...fromHeader(
                "Cost index and cruise level are in the header",
                (c) => header(c, "level"),
                "The navigation log repeats the level and wind at each fix.",
              ),
              body: "The header gives the cruise level, the cost index, and the average wind and temperature for the flight.",
            },
          },
          {
            id: "cost_index",
            label: "Cost index",
            field: "cost_index",
            max: 4,
            mode: "numeric",
            desc: "Trades fuel against time. Lower saves fuel.",
            loc: "Enter on the CDU › PERF INIT",
            ofp: {
              ...fromHeader(
                "Cost index and cruise level are in the header",
                (c) => header(c, "ci"),
                "The navigation log repeats the level and wind at each fix.",
              ),
              body: "The header gives the cruise level, the cost index, and the average wind and temperature for the flight.",
            },
          },
          {
            id: "cruise_wind",
            label: "Cruise wind",
            field: "cruise_wind",
            max: 7,
            desc: "Average wind at cruise level, from the plan.",
            loc: "Enter on the CDU › PERF INIT",
            ofp: fromHeader(
              "The average wind is in the header",
              (c) => header(c, "wind"),
              "Each fix's own wind is in the navigation log.",
            ),
          },
          {
            id: "isa_deviation",
            label: "ISA deviation",
            field: "isa_deviation",
            show: signed,
            max: 3,
            mode: "numeric",
            desc: "How much warmer than standard the air is at cruise.",
            loc: "Enter on the CDU › PERF INIT",
            ofp: fromHeader(
              "The ISA deviation is in the header",
              (c) => header(c, "isa"),
              "Plus is warmer than standard.",
            ),
          },
        ],
      },
      {
        title: "Levels",
        rows: [
          {
            id: "transition_altitude",
            label: "Transition altitude",
            field: "transition_altitude",
            show: (x) => (Number.isFinite(Number(x)) ? grouped(Number(x)) : x),
            max: 5,
            mode: "numeric",
            desc: "Where you set standard pressure on the way up.",
            loc: "Enter on the CDU › PERF INIT",
            ofp: fromHeader(
              "The transition altitude is in the header",
              (c) => header(c, "ta"),
              "The departure airport's transition altitude.",
            ),
          },
          {
            id: "transition_level_link",
            label: "Transition level",
            field: "transition_level",
            link: "approach",
            show: (x) => `FL${x}`,
            desc: "Where you switch back to the local setting on the way down. Also on Approach.",
            chip: "Approach ›",
            ofp: fromHeader(
              "The transition level is in the header",
              (c) => header(c, "tl"),
              "The destination's transition level.",
            ),
          },
        ],
      },
    ],
  },
  {
    id: "clearance",
    title: "Clearance",
    sub: "What ATC cleared you to fly. The same fields as the flight strip.",
    selected: "sid",
    groups: [
      {
        title: "Departure clearance",
        rows: [
          {
            id: "runway",
            label: "Runway",
            field: "runway",
            max: 4,
            desc: "The runway you depart from.",
            loc: "Enter on the CDU › DEP/ARR",
            ofp: clearanceOfp(),
          },
          {
            id: "sid",
            label: "SID",
            field: "sid",
            max: 8,
            desc: "The departure procedure.",
            loc: "Enter on the CDU › DEP/ARR",
            ofp: clearanceOfp(),
          },
          {
            id: "transition",
            label: "Transition",
            field: "transition",
            max: 8,
            desc: "Where the SID joins your route.",
            loc: "Enter on the CDU › DEP/ARR",
            ofp: clearanceOfp(),
          },
          {
            id: "initial_altitude",
            label: "Initial altitude",
            field: "initial_altitude",
            max: 5,
            mode: "numeric",
            desc: "The altitude you're cleared to climb to first.",
            loc: "Set on the MCP › ALTITUDE",
            ofp: clearanceOfp(),
          },
          {
            id: "squawk",
            label: "Squawk",
            field: "squawk",
            max: 4,
            mode: "numeric",
            desc: "Your transponder code.",
            loc: "Set on the transponder",
            ofp: clearanceOfp("Your squawk comes from ATC"),
          },
        ],
      },
      {
        title: "Frequencies",
        rows: [
          {
            id: "ground",
            label: "Ground",
            field: "ground_frequency",
            max: 7,
            mode: "decimal",
            desc: "For pushback and taxi.",
            loc: "Set on the radio panel",
            ofp: clearanceOfp("Frequencies come from the chart and ATIS"),
          },
          {
            id: "tower",
            label: "Tower",
            field: "tower_frequency",
            max: 7,
            mode: "decimal",
            desc: "For takeoff.",
            loc: "Set on the radio panel",
            ofp: clearanceOfp("Frequencies come from the chart and ATIS"),
          },
          {
            id: "departure",
            label: "Departure frequency",
            field: "departure_frequency",
            max: 7,
            mode: "decimal",
            placeholder: "Enter",
            desc: "The frequency to call after takeoff.",
            loc: "Set on the radio panel",
            ofp: clearanceOfp("Frequencies come from the chart and ATIS"),
          },
        ],
      },
      {
        title: "Notes from ATC",
        note: (c) => {
          const n = c.notes.split("\n").filter((l) => l.trim()).length;
          return n ? `${n} ${n === 1 ? "note" : "notes"}` : "";
        },
        rows: [
          {
            id: "notes",
            label: "Note",
            notes: true,
            desc: "Anything else ATC told you.",
            ofp: clearanceOfp(),
          },
        ],
      },
    ],
  },
  {
    id: "takeoff",
    title: "Takeoff",
    sub: "Numbers for the takeoff steps and the CDU's TAKEOFF REF page.",
    selected: "v2",
    groups: [
      {
        title: "Runway",
        rows: [
          {
            id: "takeoff_runway",
            label: "Takeoff runway",
            field: "runway",
            link: "clearance",
            desc: "Set in Clearance.",
            chip: "Clearance ›",
            ofp: takeoffOfp,
          },
          {
            id: "heading",
            label: "Runway heading",
            field: "heading",
            max: 3,
            mode: "numeric",
            show: (x) => `${x}°`,
            desc: "Set on the MCP heading before takeoff.",
            loc: "Set on the MCP › HEADING",
            ofp: takeoffOfp,
          },
        ],
      },
      {
        title: "Settings and speeds",
        rows: [
          {
            id: "takeoff_flaps",
            label: "Flaps",
            field: "takeoff_flaps",
            choices: FLAPS,
            desc: "Takeoff flaps for your weight and runway.",
            loc: "Enter on the CDU › TAKEOFF REF",
            ofp: takeoffOfp,
          },
          {
            id: "thrust_rating",
            label: "Takeoff thrust",
            field: "thrust_rating",
            choices: THRUST,
            desc: "The rating from your takeoff performance in the EFB. TO is full thrust; TO-1 and TO-2 are fixed derates that give less.",
            loc: "Enter on the CDU › N1 LIMIT",
            ofp: takeoffOfp,
          },
          {
            id: "assumed_temperature",
            label: "Assumed temperature (SEL)",
            field: "assumed_temperature",
            max: 3,
            mode: "numeric",
            show: (x) => `${x} °C`,
            desc: "From your takeoff performance in the EFB. A temperature hotter than the real one, so the FMC sets less thrust and saves the engines.",
            loc: "Enter on the CDU › N1 LIMIT",
            ofp: takeoffOfp,
          },
          {
            id: "v1",
            label: "V1",
            field: "v1",
            max: 3,
            mode: "numeric",
            desc: "Decision speed. Before it you can stop; after it you go.",
            loc: "Enter on the CDU › TAKEOFF REF",
            ofp: takeoffOfp,
          },
          {
            id: "vr",
            label: "VR",
            field: "vr",
            max: 3,
            mode: "numeric",
            desc: "Rotation speed.",
            loc: "Enter on the CDU › TAKEOFF REF",
            ofp: takeoffOfp,
          },
          {
            id: "v2",
            label: "V2",
            field: "v2",
            max: 3,
            mode: "numeric",
            placeholder: "From the FMC",
            desc: "Takeoff safety speed for your takeoff flaps.",
            loc: "Enter on the CDU › TAKEOFF REF",
            ofp: takeoffOfp,
          },
          {
            id: "takeoff_trim",
            label: "Stabilizer trim",
            field: "takeoff_trim",
            max: 5,
            mode: "decimal",
            placeholder: "From the FMC",
            desc: "Takeoff trim for your weight and balance.",
            loc: "Enter on the CDU › TAKEOFF REF",
            ofp: takeoffOfp,
          },
        ],
      },
    ],
  },
  {
    id: "approach",
    title: "Approach",
    sub: "Used by the Descent and Approach steps and the approach briefing. Arrival values live only here.",
    selected: "arrival_runway",
    groups: [
      {
        title: "Arrival",
        rows: [
          {
            id: "arrival_runway",
            label: "Arrival runway",
            field: "arrival_runway",
            max: 4,
            desc: "The runway you expect to land on. Check it against the ATIS before you brief.",
            loc: "Enter on the CDU › DEP/ARR",
            ofp: approachRoute,
          },
          {
            id: "star",
            label: "Arrival (STAR)",
            field: "star",
            max: 8,
            placeholder: "Enter",
            desc: "The published route from the airway to the approach.",
            loc: "Enter on the CDU › DEP/ARR",
            ofp: approachRoute,
          },
          {
            id: "transition_level",
            label: "Transition level",
            field: "transition_level",
            max: 3,
            mode: "numeric",
            show: (x) => `FL${x}`,
            desc: "Where you switch the altimeters to the local setting on the way down.",
            loc: "Enter on the CDU › DESCENT",
            ofp: fromHeader(
              "The transition level is in the header",
              (c) => header(c, "tl"),
              "The destination's transition level.",
            ),
          },
        ],
      },
      {
        title: "Landing",
        rows: [
          {
            id: "minimums",
            label: "Minimums",
            field: "minimums",
            max: 5,
            mode: "numeric",
            placeholder: "Enter",
            desc: "The height where you must see the runway to continue, from the approach chart. BARO for most approaches; RADIO when the chart gives a radio altimeter height.",
            loc: "Set on the EFIS › MINS",
            seg: { field: "minimums_type", values: ["BARO", "RADIO"], label: "Minimums type" },
            ofp: chartOfp("Minimums come from the approach chart"),
          },
          {
            id: "vref",
            label: "Landing speed",
            field: "vref",
            max: 3,
            mode: "numeric",
            placeholder: "From the FMC",
            desc: "VREF for your landing flaps, plus any wind additive.",
            loc: "Enter on the CDU › APPROACH REF",
            ofp: chartOfp("Landing speed comes from the FMC"),
          },
          {
            id: "landing_flaps",
            label: "Landing flaps",
            field: "landing_flaps",
            choices: LANDING_FLAPS,
            desc: "The flaps you land with. Landing speed is VREF for this setting.",
            loc: "Enter on the CDU › APPROACH REF",
            ofp: chartOfp("Landing flaps come from the FMC"),
          },
          {
            id: "autobrake",
            label: "Autobrake",
            field: "autobrake",
            choices: AUTOBRAKE,
            desc: "How hard the brakes apply on touchdown.",
            loc: "Set on the autobrake selector",
            ofp: chartOfp("Autobrake is your choice for the runway"),
          },
          {
            id: "landing_altitude",
            label: "Landing altitude",
            field: "landing_altitude",
            max: 5,
            mode: "numeric",
            desc: "The destination field elevation, from the plan. Set on the pressurization panel.",
            loc: "Set on the pressurization panel › LAND ALT",
            ofp: {
              ...approachRoute,
              title: "The destination elevation is in the header",
              note: "The header gives each airport's elevation.",
            },
          },
        ],
      },
    ],
  },
];

/** The section that holds a field, and its row, for "— enter …" links and Still to enter. */
export function home(field: FillField): { section: FlightSection; row: Row } | null {
  for (const s of SECTIONS)
    for (const g of s.groups)
      for (const r of g.rows) if (r.field === field && !r.link) return { section: s.id, row: r };
  return null;
}

/** Empty fields the steps need, per section ("2 to enter"). */
export function toEnter(values: Partial<Record<FillField, string>>, section?: FlightSection): Row[] {
  return SECTIONS.filter((s) => !section || s.id === section)
    .flatMap((s) => s.groups.flatMap((g) => g.rows))
    .filter((r) => r.field && !r.link && NEEDED.includes(r.field) && !values[r.field]);
}

/** The flight's weight unit, lower case, for labels. */
export function unitOf(flight: Flight): string {
  return (flight.values?.weight_unit ?? "LB").toLowerCase();
}

/** What a step's "— enter …" link names: "takeoff trim", "V2", "QNH". */
const SPOKEN: Partial<Record<FillField, string>> = {
  v1: "V1",
  vr: "VR",
  v2: "V2",
  vref: "VREF",
  qnh: "QNH",
  destination_qnh: "QNH",
  heading: "runway heading",
};

/**
 * For each of a step's blanks, the Flight field it links to (Checklist board: "— enter takeoff
 * trim"), or null for a blank typed in place: the pilot's own, or a value the Flight pages only
 * show. The altimeter settings live on Weather's Departure and Destination tabs.
 */
export function blankLinks(fill: FillField[]): ({ href: string; name: string } | null)[] {
  return fill.map((field) => {
    if (field === "manual") return null;
    const name = SPOKEN[field] ?? field.replace(/_/g, " ");
    if (field === "qnh" || field === "destination_qnh") return { href: `#/flight/weather/${field}`, name };
    const at = home(field);
    return at ? { href: `#/flight/${at.section}/${field}`, name } : null;
  });
}

/** A number from a field's text, or NaN. */
const number = (v: string) => (v.trim() === "" ? Number.NaN : Number(v));
const within = (lo: number, hi: number, message: string) => (v: string) => {
  const n = number(v);
  return Number.isFinite(n) && n >= lo && n <= hi ? null : message;
};
const QNH = (v: string, c: Ctx) =>
  c.values.qnh_unit === "hPa"
    ? within(948, 1050, "Enter 948 to 1050 hPa")(v)
    : within(28, 31, "Enter 28.00 to 31.00 inHg")(v);

/**
 * The ranges 3.0's Flight setup gave its number fields (21 of them), as checks (FSInvalid board): a value
 * outside one shows "Enter X to Y" under its box while it stays out of range. Fields 3.0 gave no
 * range have no check.
 */
export const RANGES: Partial<Record<FillField, (value: string, c: Ctx) => string | null>> = {
  cruise_level: within(100, 410, "Enter FL100 to FL410"),
  initial_altitude: within(0, 41000, "Enter 0 to 41,000 ft"),
  heading: within(1, 360, "Enter 001 to 360"),
  qnh: QNH,
  destination_qnh: QNH,
  v1: within(80, 250, "Enter 80 to 250 knots"),
  vr: within(80, 250, "Enter 80 to 250 knots"),
  v2: within(100, 250, "Enter 100 to 250 knots"),
  vref: within(100, 250, "Enter 100 to 250 knots"),
  cg: within(0, 50, "Enter 0 to 50% MAC"),
  takeoff_trim: within(0, 15, "Enter 0.00 to 15.00 units"),
  passengers: within(0, 500, "Enter 0 to 500"),
  // Weights are stored in thousands, as 3.0's ranges were; the box shows them in full.
  zfw: (v, c) => within(0, 400, `Enter 0 to 400,000 ${c.unit}`)(v),
  takeoff_weight: (v, c) => within(0, 400, `Enter 0 to 400,000 ${c.unit}`)(v),
  reserves: (v, c) => within(0, 100, `Enter 0 to 100,000 ${c.unit}`)(v),
  cargo: (v, c) => within(0, 100, `Enter 0 to 100,000 ${c.unit}`)(v),
  cost_index: within(0, 9999, "Enter 0 to 9999"),
  isa_deviation: within(-60, 60, "Enter −60 to +60"),
  transition_altitude: within(1000, 20000, "Enter 1,000 to 20,000 ft"),
  transition_level: within(10, 600, "Enter FL10 to FL600"),
  landing_altitude: within(-1000, 14000, "Enter −1,000 to 14,000 ft"),
  minimums: within(0, 14000, "Enter 0 to 14,000 ft"),
};

/** The message for a field's value, or null when it's in range or empty. */
export function rangeMessage(field: FillField, c: Ctx): string | null {
  const value = c.values[field] ?? "";
  return value ? (RANGES[field]?.(value, c) ?? null) : null;
}
