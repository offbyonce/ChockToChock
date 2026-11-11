// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// The web host: the interface's host, built on the WebAssembly module and browser APIs
// (decision 0010). The module arrives in Phase 2, step 5; until then every call fails with this
// message, which the app shows.

import type { Host } from "@chocktochock/ui";

const notBuiltYet = () => Promise.reject(new Error("The web host is not built yet (Phase 2, step 5)."));

export const webHost: Host = {
  appInfo: notBuiltYet,
  exampleChecklist: notBuiltYet,
};
