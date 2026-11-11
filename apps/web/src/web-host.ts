// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// The web host: the interface's host, built on the WebAssembly module (./core.ts) and browser
// APIs (decision 0010).

import type { Host } from "@chocktochock/ui";
import { appInfo, exampleChecklist } from "./core";

export const webHost: Host = {
  appInfo,
  exampleChecklist,
};
