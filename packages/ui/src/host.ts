// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// What the interface needs from wherever it runs (decision 0010). The web app passes a host built
// on its WebAssembly module and browser APIs; a desktop build would pass one built on Tauri
// commands. Entry points pass it with hostContext(); components read it with getHost().

import type { AppInfo, Checklist } from "@chocktochock/bindings";
import { getContext } from "svelte";

export interface Host {
  /** Basic facts for the About box and the window title. */
  appInfo(): Promise<AppInfo>;
  /** The bundled format-demo checklist. */
  exampleChecklist(): Promise<Checklist>;
}

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
