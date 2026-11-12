// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// The Flight setup fields, in 3.0's groups and order, with 3.0's labels, examples and help lines.
// Each field is one of the app's fill fields (decision 0019); unit choices are fill fields too.

import type { FillField } from "@chocktochock/bindings";

/** A two-way unit choice beside a field, stored as its own fill field. */
export type UnitChoice = { field: FillField; values: [string, string] };

export type SetupField = {
  field: FillField;
  label: string;
  /** An example value, shown when the field is empty. */
  example: string;
  help: string;
  kind: "text" | "number" | "select";
  min?: number | undefined;
  max?: number | undefined;
  maxLength?: number | undefined;
  inputMode?: "numeric" | "decimal" | undefined;
  options?: string[] | undefined;
  unit?: UnitChoice | undefined;
  /** Spans the whole row. */
  wide?: boolean | undefined;
};

export const WEIGHT_UNIT: UnitChoice = { field: "weight_unit", values: ["LB", "KG"] };
export const QNH_UNIT: UnitChoice = { field: "qnh_unit", values: ["inHg", "hPa"] };
export const MINIMUMS_TYPE: UnitChoice = { field: "minimums_type", values: ["BARO", "RADIO"] };

/** A unit choice's value, or its first value when none is set. */
export const unitOf = (values: Partial<Record<FillField, string>>, unit: UnitChoice): string =>
  values[unit.field] ?? unit.values[0];

const FLAPS = ["0", "1", "2", "5", "10", "15", "25", "30", "40"];

const text = (
  field: FillField,
  label: string,
  example: string,
  help: string,
  more: Partial<SetupField> = {},
): SetupField => ({
  field,
  label,
  example,
  help,
  kind: "text",
  ...more,
});
const num = (
  field: FillField,
  label: string,
  example: string,
  help: string,
  min?: number,
  max?: number,
  more: Partial<SetupField> = {},
): SetupField => ({
  field,
  label,
  example,
  help,
  kind: "number",
  min,
  max,
  ...more,
});

/** The groups, in 3.0's order. Labels with a weight follow the chosen weight unit. */
export function setupGroups(
  values: Partial<Record<FillField, string>>,
): { title: string; fields: SetupField[] }[] {
  const w = unitOf(values, WEIGHT_UNIT);
  const hpa = unitOf(values, QNH_UNIT) === "hPa";
  return [
    {
      title: "Flight",
      fields: [
        text("flight_number", "Flight number", "1234", "Up to 7 characters", { maxLength: 7 }),
        text("origin", "From", "KSEA", "4-letter ICAO code"),
        text("destination", "To", "KSFO", "4-letter ICAO code"),
        num("cruise_level", "Cruise level", "350", "FL100 to FL410", 100, 410),
      ],
    },
    {
      title: "Departure clearance",
      fields: [
        text("runway", "Runway", "16L", "01 to 36, with L, C or R"),
        text("sid", "Departure (SID)", "SUMMA2", "As cleared, or blank for radar vectors", { maxLength: 8 }),
        text("transition", "Transition", "SEA", "The SID transition, if any", { maxLength: 8 }),
        num("initial_altitude", "Initial altitude", "5000", "Feet, in 100 ft steps", 0, 41000),
        text("squawk", "Squawk", "4521", "4 digits, 0 to 7", { maxLength: 4, inputMode: "numeric" }),
        text("ground_frequency", "Ground frequency", "121.700", "The frequency to call for taxi", {
          maxLength: 7,
          inputMode: "decimal",
        }),
        text("departure_frequency", "Departure frequency", "125.600", "Contact after takeoff", {
          maxLength: 7,
          inputMode: "decimal",
        }),
        num("heading", "Runway heading", "163", "001 to 360 degrees", 1, 360),
        hpa
          ? num("qnh", "QNH", "1013", "948 to 1050 hPa", 948, 1050, { unit: QNH_UNIT })
          : num("qnh", "QNH", "29.92", "28.00 to 31.00 inHg", 28, 31, { unit: QNH_UNIT }),
      ],
    },
    {
      title: "Takeoff data",
      fields: [
        {
          field: "takeoff_flaps",
          label: "Takeoff flaps",
          example: "",
          help: "From your takeoff data",
          kind: "select",
          options: FLAPS,
        },
        num("v1", "V1", "145", "From the iFly EFB takeoff performance", 80, 250),
        num("vr", "VR", "148", "From the iFly EFB takeoff performance", 80, 250),
        num("v2", "V2", "152", "100 to 250 knots", 100, 250),
        num("cg", "CG", "22.1", "% MAC, from the iFly EFB", 0, 50),
        num("takeoff_trim", "Stab trim", "5.25", "0.00 to 15.00 units", 0, 15),
        num("fuel", "Fuel", "12500", "Departing fuel", 0, undefined, { unit: WEIGHT_UNIT }),
        num("minimum_takeoff_fuel", "Minimum takeoff fuel", "9800", "From your flight plan", 0),
      ],
    },
    {
      title: "Performance (CDU)",
      fields: [
        num("zfw", `ZFW (1000 ${w})`, "62.5", "PERF INIT, in thousands", 0, 400),
        num("reserves", `Reserves (1000 ${w})`, "5.2", "Final reserve plus alternate", 0, 100),
        num("cost_index", "Cost index", "30", "PERF INIT", 0, 9999),
        text("cruise_wind", "Cruise wind", "270/045", "Direction/speed", { maxLength: 7 }),
        num("isa_deviation", "ISA deviation", "-3", "Degrees C at cruise", -60, 60),
        num("transition_altitude", "Transition altitude", "18000", "Feet, departure", 1000, 20000),
        num("transition_level", "Transition level", "180", "Flight level, destination", 10, 600),
        num(
          "takeoff_weight",
          `Planned takeoff weight (1000 ${w})`,
          "79.4",
          "Cross-check the FMC gross weight",
          0,
          400,
        ),
      ],
    },
    {
      title: "Route and loading",
      fields: [
        text("route", "Route (legs)", "SUMMA2 SUMMA J1 BTG", "Airways and waypoints from your flight plan", {
          wide: true,
          maxLength: 400,
        }),
        num("passengers", "Passengers", "150", "For the iFly EFB payload page", 0, 500),
        num("cargo", `Cargo (1000 ${w})`, "2.1", "For the iFly EFB payload page", 0, 100),
      ],
    },
  ];
}
