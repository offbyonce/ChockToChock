// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// What the interface needs from wherever it runs (decisions 0010 and 0012). The web app passes a
// host built on its WebAssembly module and browser APIs; a desktop build would pass one built on
// Tauri commands. Entry points pass it with hostContext(); components read it with getHost().
// Every rule (what shows, what counts, what an action does) is decided behind the host, in Rust.

import type {
  Action,
  AppInfo,
  Decoded,
  FlatPack,
  Flight,
  Loaded,
  OpenStep,
  PackSummary,
  PhaseView,
  PlanValues,
  Rail,
  SavedPlan,
  Settings,
} from "@chocktochock/bindings";
import { getContext } from "svelte";

export interface Host {
  /** Basic facts for the About box and the window title. */
  appInfo(): Promise<AppInfo>;
  /** The packs this build offers. */
  packs(): Promise<PackSummary[]>;
  /** Loads a pack; it stays loaded for the calls below. */
  loadPack(id: string): Promise<FlatPack>;
  newFlight(pack: string): Promise<Flight>;
  /** Applies an action and returns the changed flight. */
  apply(pack: string, flight: Flight, action: Action): Promise<Flight>;
  /** The phase rail for a flight, for the pilot's settings (Full or Essentials). */
  rail(pack: string, flight: Flight, settings: Settings): Promise<Rail>;
  /** One phase as shown: its lists, step states, progress and neighbors. */
  phaseView(pack: string, flight: Flight, settings: Settings, phase: string): Promise<PhaseView>;
  /** Open steps: undone steps of started phases; with `before`, only from phases before it. */
  openSteps(pack: string, flight: Flight, settings: Settings, before?: string): Promise<OpenStep[]>;
  /**
   * What is saved on this device for a pack (decision 0012): each read by Rust as ready (migrated
   * if older), newer or unreadable, with the saved text so nothing is lost. Null when none.
   */
  loadSaved(pack: string): Promise<Saved>;
  /** Saves the flight or the settings on this device; returns when, in milliseconds. */
  save(pack: string, what: "flight" | "settings", value: Flight | Settings): Promise<number>;
  /** Moves saved text the app cannot use aside (kept, not deleted), so a new save can start. */
  setAside(pack: string, what: "flight" | "settings"): Promise<void>;
  defaultSettings(): Promise<Settings>;
  /**
   * The latest SimBrief plan for a username or Pilot ID, fetched from the pilot's browser and
   * read into the compact plan (decision 0019); nothing else is kept.
   */
  fetchPlan(user: string): Promise<SavedPlan>;
  /** A saved OFP file the pilot chose, read once on this device into the compact plan. */
  readPlanFile(text: string): Promise<SavedPlan>;
  /** The flight values a plan fills in. */
  planValues(plan: SavedPlan): Promise<PlanValues>;
  /** The plan kept on this device for a pack, or null. */
  loadPlan(pack: string): Promise<SavedPlan | null>;
  /** Keeps the plan on this device, or forgets it (null). */
  savePlan(pack: string, plan: SavedPlan | null): Promise<void>;
  /** A METAR or TAF decoded from the code table. */
  decodeMetar(text: string): Promise<Decoded>;
  decodeTaf(text: string): Promise<Decoded>;
}

export type SavedItem<T> = { loaded: Loaded<T>; text: string } | null;
export type Saved = { flight: SavedItem<Flight>; settings: SavedItem<Settings> };

const HOST = Symbol("host");

/** The context to pass to `mount` (or a test's `render`) so components can find the host. */
export function hostContext(host: Host): Map<symbol, Host> {
  return new Map([[HOST, host]]);
}

/** The host the app was mounted with. */
export function getHost(): Host {
  const host = getContext<Host | undefined>(HOST);
  if (!host) throw new Error("the app was mounted without a host");
  return host;
}
