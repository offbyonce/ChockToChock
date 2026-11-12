// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Progress and settings saved on this device, in browser storage (decision 0012; Phase 2 brief,
// step 8, brought forward to step 7 batch 1). Rust reads what is saved: ready (migrated forward
// if older), newer or unreadable. Saved text the app cannot use is never overwritten or deleted:
// it stays until the pilot starts fresh, and then it is moved aside under its own key.
// Browser storage can be missing or blocked (private windows); every access is guarded.

import type { Flight, Settings } from "@chocktochock/bindings";
import type { Saved } from "@chocktochock/ui";
import { loadFlight, loadSettings } from "./core";

const key = (pack: string, what: "flight" | "settings") =>
  what === "settings" ? "chocktochock.settings" : `chocktochock.flight.${pack}`;

function read(k: string): string | null {
  try {
    return window.localStorage.getItem(k);
  } catch {
    return null;
  }
}

function write(k: string, text: string): void {
  try {
    window.localStorage.setItem(k, text);
  } catch {
    // Storage is full or blocked: the app keeps working for this visit.
  }
}

let persistAsked = false;

export async function loadSaved(pack: string): Promise<Saved> {
  const flightText = read(key(pack, "flight"));
  const settingsText = read(key(pack, "settings"));
  return {
    flight: flightText === null ? null : { loaded: await loadFlight(flightText), text: flightText },
    settings: settingsText === null ? null : { loaded: await loadSettings(settingsText), text: settingsText },
  };
}

export async function save(
  pack: string,
  what: "flight" | "settings",
  value: Flight | Settings,
): Promise<number> {
  write(key(pack, what), JSON.stringify(value));
  // Ask once for storage the browser will not clear on its own (Phase 2 brief, step 8).
  if (!persistAsked) {
    persistAsked = true;
    void navigator.storage?.persist?.().catch(() => false);
  }
  return Date.now();
}

export async function setAside(pack: string, what: "flight" | "settings"): Promise<void> {
  const text = read(key(pack, what));
  if (text === null) return;
  write(`${key(pack, what)}.kept-${new Date().toISOString()}`, text);
  try {
    window.localStorage.removeItem(key(pack, what));
  } catch {
    // Blocked storage: nothing was saved there either.
  }
}
