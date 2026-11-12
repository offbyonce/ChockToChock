// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Screens have their own address after `#`, so Back works and a screen can be bookmarked,
// offline too. `#/phase/<id>`, `#/reference` (optionally `/<phase id>`), `#/settings`, `#/setup`
// (Flight setup), `#/open`
// (open steps); anything
// else is the start.

export type Route =
  | { screen: "start" }
  | { screen: "phase"; id: string }
  | { screen: "reference"; id: string | null }
  | { screen: "settings" }
  | { screen: "setup" }
  | { screen: "open" };

export function parseRoute(hash: string): Route {
  const parts = hash.replace(/^#\/?/, "").split("/").filter(Boolean).map(decodeURIComponent);
  if (parts[0] === "phase" && parts[1]) return { screen: "phase", id: parts[1] };
  if (parts[0] === "reference") return { screen: "reference", id: parts[1] ?? null };
  if (parts[0] === "settings") return { screen: "settings" };
  if (parts[0] === "setup") return { screen: "setup" };
  if (parts[0] === "open") return { screen: "open" };
  return { screen: "start" };
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
