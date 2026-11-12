// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Typed calls into the WebAssembly module (crates/web-host). Every export answers with a JSON
// Reply; a reply with an error is thrown as the tagged AppError, so callers can branch on `kind`.
// Run `just wasm` to build the module into ./wasm.

import type {
  Action,
  AppInfo,
  Checklist,
  FlatPack,
  Flight,
  Loaded,
  Ofp,
  OpenStep,
  PhaseView,
  Rail,
  Reply,
  Settings,
} from "@chocktochock/bindings";
import init, * as wasm from "./wasm/web_host.js";

let ready: Promise<unknown> | undefined;

/** Loads the module once; later calls wait for the same load. */
function load(): Promise<unknown> {
  ready ??= init();
  return ready;
}

function unwrap<T>(answer: string): T {
  const reply = JSON.parse(answer) as Reply<T>;
  if ("error" in reply) throw reply.error;
  return reply.ok;
}

export async function appInfo(): Promise<AppInfo> {
  await load();
  return unwrap<AppInfo>(wasm.app_info());
}

export async function exampleChecklist(): Promise<Checklist> {
  await load();
  return unwrap<Checklist>(wasm.example_checklist());
}

/** Reads a flattened pack's JSON text, as written by `pack build`; the module keeps it loaded. */
export async function loadPack(json: string): Promise<FlatPack> {
  await load();
  return unwrap<FlatPack>(wasm.load_pack(json));
}

export async function newFlight(pack: string): Promise<Flight> {
  await load();
  return unwrap<Flight>(wasm.new_flight(pack));
}

export async function apply(pack: string, flight: Flight, action: Action): Promise<Flight> {
  await load();
  return unwrap<Flight>(wasm.apply(pack, JSON.stringify(flight), JSON.stringify(action)));
}

export async function rail(pack: string, flight: Flight, settings: Settings): Promise<Rail> {
  await load();
  return unwrap<Rail>(wasm.rail(pack, JSON.stringify(flight), JSON.stringify(settings)));
}

export async function openSteps(
  pack: string,
  flight: Flight,
  settings: Settings,
  before?: string,
): Promise<OpenStep[]> {
  await load();
  return unwrap<OpenStep[]>(
    wasm.open_steps(pack, JSON.stringify(flight), JSON.stringify(settings), before ?? ""),
  );
}

export async function phaseView(
  pack: string,
  flight: Flight,
  settings: Settings,
  phase: string,
): Promise<PhaseView> {
  await load();
  return unwrap<PhaseView>(wasm.phase_view(pack, JSON.stringify(flight), JSON.stringify(settings), phase));
}

/** Reads a saved flight: ready (migrated if older), newer, or unreadable (decision 0012). */
export async function loadFlight(json: string): Promise<Loaded<Flight>> {
  await load();
  return unwrap<Loaded<Flight>>(wasm.load_flight(json));
}

/** Reads saved settings: ready (migrated if older), newer, or unreadable (decision 0012). */
export async function loadSettings(json: string): Promise<Loaded<Settings>> {
  await load();
  return unwrap<Loaded<Settings>>(wasm.load_settings(json));
}

export async function defaultSettings(): Promise<Settings> {
  await load();
  return unwrap<Settings>(wasm.default_settings());
}

/** The address of the latest SimBrief plan for a username or Pilot ID. */
export async function planUrl(user: string): Promise<string> {
  await load();
  return unwrap<string>(wasm.plan_url(user));
}

/** Reads a SimBrief plan: fetched JSON, or a saved JSON or XML file. */
export async function parsePlan(text: string): Promise<Ofp> {
  await load();
  return unwrap<Ofp>(wasm.parse_plan(text));
}
