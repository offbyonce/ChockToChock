// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import { svelte } from "@sveltejs/vite-plugin-svelte";
import { defineConfig } from "vite";

// The web app's dev server for now. `base: "./"` keeps paths relative, so the built site works
// under the Pages project path (decision 0010). The production build joins `just site` in
// Phase 2, step 6; until then `just site` publishes the placeholder in ./placeholder.
export default defineConfig({
  plugins: [svelte()],
  base: "./",
  server: { port: 1420, strictPort: true },
  build: { target: "es2022" },
});
