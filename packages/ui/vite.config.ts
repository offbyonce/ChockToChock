// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import { svelte } from "@sveltejs/vite-plugin-svelte";
import { defineConfig } from "vitest/config";

// The shared package is built by the apps that use it; this config is for its tests.
export default defineConfig(({ mode }) => ({
  plugins: [svelte()],
  // Vitest runs in "test" mode; Svelte's browser build is needed for component tests.
  resolve: mode === "test" ? { conditions: ["browser"] } : {},
  test: {
    environment: "jsdom",
    include: ["src/**/*.test.ts"],
    setupFiles: ["src/test-setup.ts"],
  },
}));
