// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Screens have their own address after `#`, so Back works and a screen can be bookmarked,
// offline too. `#/phase/<id>`, `#/reference` (optionally `/<phase id>`), `#/settings`,
// `#/flight/<section>` (the Flight pages, batch 4; optionally `/<field>` to focus a field, or the
// Weather tab), `#/open` (open steps); anything else is the start. `#/setup` (Flight setup until
// batch 4) leads to Flight › Overview.

export type Route =
  | { screen: "start" }
  | { screen: "phase"; id: string }
  | { screen: "reference"; id: string | null }
  | { screen: "settings" }
  | { screen: "flight"; section: FlightSection; focus: string | null }
  | { screen: "open" };

export function parseRoute(hash: string): Route {
  const parts = hash.replace(/^#\/?/, "").split("/").filter(Boolean).map(decodeURIComponent);
  if (parts[0] === "phase" && parts[1]) return { screen: "phase", id: parts[1] };
  if (parts[0] === "reference") return { screen: "reference", id: parts[1] ?? null };
  if (parts[0] === "settings") return { screen: "settings" };
  if (parts[0] === "setup") return { screen: "flight", section: "overview", focus: null };
  if (parts[0] === "flight") {
    const section = (FLIGHT_SECTIONS as readonly string[]).includes(parts[1] ?? "")
      ? (parts[1] as FlightSection)
      : "overview";
    return { screen: "flight", section, focus: parts[2] ?? null };
  }
  if (parts[0] === "open") return { screen: "open" };
  return { screen: "start" };
}

/** The Flight pages' sections, in the section list's order (FlightSection boards). */
export const FLIGHT_SECTIONS = [
  "overview",
  "route",
  "fuel",
  "cruise",
  "weather",
  "clearance",
  "takeoff",
  "approach",
  "start",
  "simbrief",
] as const;
export type FlightSection = (typeof FLIGHT_SECTIONS)[number];

/** A Flight section, optionally with a field to focus or a Weather tab. */
export function flightHref(section: FlightSection, focus?: string): string {
  return `#/flight/${section}${focus ? `/${encodeURIComponent(focus)}` : ""}`;
}

export function phaseHref(id: string): string {
  return `#/phase/${encodeURIComponent(id)}`;
}

/** The current route, kept in step with the address. */
export function currentRoute(): { readonly value: Route } {
  let value = $state(parseRoute(window.location.hash));
  $effect(() => {
    const update = () => {
      value = parseRoute(window.location.hash);
    };
    window.addEventListener("hashchange", update);
    return () => window.removeEventListener("hashchange", update);
  });
  return {
    get value() {
      return value;
    },
  };
}
