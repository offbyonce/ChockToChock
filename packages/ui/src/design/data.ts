// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Sample content for the design mockups, taken from the iFly pack's own Before start phase.

export type MockStep = {
  n: number;
  text: string;
  value: string;
  where?: string;
  state: "to_do" | "current" | "done" | "skipped";
  essential?: boolean;
  open?: boolean;
  check?: { title: string; detail?: string };
  ifNot?: { title: string; detail?: string };
  why?: string;
};

export const STAGES = ["Preflight", "Departure", "Enroute", "Arrival", "Post-flight"];
export const PHASES = [
  { title: "APU start", state: "done" },
  { title: "Before start", state: "current" },
  { title: "Pushback", state: "to_do" },
  { title: "Engine start", state: "to_do" },
  { title: "Before taxi", state: "to_do" },
  { title: "Taxi", state: "to_do" },
  { title: "Before takeoff", state: "to_do" },
  { title: "Takeoff", state: "to_do" },
];

export const PROCEDURE: MockStep[] = [
  { n: 1, text: "LANDING GEAR PINS", value: "REMOVED AND STOWED ON BOARD", state: "done" },
  {
    n: 2,
    text: "N1 BUGS",
    value: "MATCH THE GREEN N1 REFERENCE",
    where: "Main instrument panel · Display units",
    state: "current",
  },
  {
    n: 3,
    text: "MCP",
    value: "SET V2, HEADING, ALTITUDE, ARM A/T, ARM LNAV AND VNAV FOR AN RNAV DEPARTURE",
    where: "Glareshield (P7) · Mode control panel",
    state: "to_do",
    essential: true,
  },
  {
    n: 4,
    text: "DOOR LIGHTS",
    value: "ALL OUT",
    where: "Forward overhead (P5) · Door warning panel",
    state: "to_do",
    essential: true,
    open: true,
    check: { title: "All door lights out" },
    ifNot: { title: "Light on", detail: "That door is not closed" },
    why: "All closed before pushback",
  },
  {
    n: 5,
    text: "TERR (EFIS CONTROL PANEL)",
    value: "ON",
    where: "Glareshield (P7) · EFIS control panels",
    state: "skipped",
  },
  {
    n: 6,
    text: "FUEL PUMP SWITCHES",
    value: "ALL ON, CENTER PUMPS ON IF MORE THAN 1,000 LB (ABOUT 450 KG) IN THE CENTER TANK",
    where: "Forward overhead (P5) · Fuel panel",
    state: "to_do",
    essential: true,
  },
];

export const FUEL_STEP = {
  text: "Fuel pump switches",
  where: "Forward overhead (P5) · Fuel panel",
  value: "ALL ON, CENTER PUMPS ON IF MORE THAN 1,000 LB (ABOUT 450 KG) IN THE CENTER TANK",
  check: { title: "LOW PRESSURE lights out", detail: "Center lights out only if center pumps are on" },
  ifNot: {
    title: "Both center lights on",
    detail:
      "Normal if the center tank has less than 1,000 LB (about 450 KG) and its pumps are OFF. Otherwise check the center pumps are ON.",
  },
  why: "Feeds fuel to the engines under pressure",
  location:
    "Fuel panel, left side of the forward overhead, below the navigation switches. Six switches: L and R forward and aft, plus two center tank switches.",
  does: "Each tank has pumps feeding the engines. Low pressure lights sit beside each switch.",
};
