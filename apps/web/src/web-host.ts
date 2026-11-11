// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// The web host: the interface's host, built on the WebAssembly module (./core.ts) and browser
// APIs (decisions 0010 and 0012). Packs and their index are files beside the app, written by
// `pack build`; this page is app/index.html, so they are one folder up.

import type { PackSummary } from "@chocktochock/bindings";
import type { Host } from "@chocktochock/ui";
import { appInfo, apply, defaultSettings, loadPack, newFlight, rail } from "./core";

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
  defaultSettings,
};
