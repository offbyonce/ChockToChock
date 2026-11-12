// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// The shared interface, used by apps/web now and a desktop build later (decision 0010).
// Styles: import "@chocktochock/ui/app.css" in the entry point.

export { default as App } from "./App.svelte";
export { getHost, type Host, hostContext, type Saved, type SavedItem } from "./host";
