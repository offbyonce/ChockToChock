// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// The web host: the interface's host, built on the WebAssembly module (./core.ts) and browser
// APIs (decisions 0010 and 0012). Packs and their index are files beside the app, written by
// `pack build`; this page is app/index.html, so they are one folder up.

import type { PackSummary, SavedPlan } from "@chocktochock/bindings";
import type { Host } from "@chocktochock/ui";
import {
  appInfo,
  apply,
  decodeMetar,
  decodeTaf,
  defaultSettings,
  loadPack,
  loadPlan,
  newFlight,
  openSteps,
  parsePlan,
  phaseView,
  planFormat,
  planUrl,
  planValues,
  rail,
} from "./core";
import { loadSaved, save, setAside } from "./saved";

const planKey = (pack: string) => `chocktochock.plan.${pack}`;

/** The compact plan from a SimBrief reply or a saved file (decision 0019): the raw text is dropped. */
async function compact(text: string): Promise<SavedPlan> {
  return { format: await planFormat(), plan: await parsePlan(text), loadedAt: Date.now() };
}

async function fetchText(path: string): Promise<string> {
  const response = await fetch(path);
  if (!response.ok) throw new Error(`Could not load ${path} (${response.status}).`);
  return response.text();
}

export const webHost: Host = {
  appInfo,
  packs: async () => JSON.parse(await fetchText("../packs/index.json")) as PackSummary[],
  loadPack: async (id) => loadPack(await fetchText(`../packs/${encodeURIComponent(id)}.json`)),
  newFlight,
  apply,
  rail,
  phaseView,
  openSteps,
  loadSaved,
  save,
  setAside,
  defaultSettings,
  fetchPlan: async (user) => {
    // Straight from the pilot's browser to SimBrief: only the username or Pilot ID goes with it.
    const response = await fetch(await planUrl(user), { credentials: "omit", referrerPolicy: "no-referrer" });
    return compact(await response.text());
  },
  readPlanFile: compact,
  planValues: (plan) => planValues(plan.plan),
  loadPlan: async (pack) => {
    let text: string | null = null;
    try {
      text = window.localStorage.getItem(planKey(pack));
    } catch {
      return null;
    }
    if (text === null) return null;
    const loaded = await loadPlan(text);
    return loaded.state === "ready" ? loaded.value : null;
  },
  savePlan: async (pack, plan) => {
    try {
      if (plan) window.localStorage.setItem(planKey(pack), JSON.stringify(plan));
      else window.localStorage.removeItem(planKey(pack));
    } catch {
      // Storage refused (private window): the plan lasts for this visit only.
    }
  },
  decodeMetar,
  decodeTaf,
};
