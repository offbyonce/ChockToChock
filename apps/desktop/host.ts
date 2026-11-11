// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Parked with the desktop app (see PARKED.md): not built during Phase 2. The Tauri host for a
// later desktop build. One function per command, so command names are written once; these were
// the wrappers in packages/bindings before decision 0010.

import type { AppInfo, Checklist } from "@chocktochock/bindings";
import type { Host } from "@chocktochock/ui";
import { invoke } from "@tauri-apps/api/core";

export const tauriHost: Host = {
  appInfo: () => invoke<AppInfo>("app_info"),
  exampleChecklist: () => invoke<Checklist>("example_checklist"),
};
