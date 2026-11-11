// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import { svelte } from "@sveltejs/vite-plugin-svelte";
import { defineConfig, type Plugin } from "vite";

// The site's content security policy (decision 0009): everything from this site only, except
// SimBrief, which the browser calls directly. 'wasm-unsafe-eval' lets the WebAssembly module
// compile; it allows no JavaScript eval. GitHub Pages cannot send headers, so it goes in a <meta>
// tag, added to the built pages only, because the dev server injects inline styles.
const CSP = [
  "default-src 'none'",
  "script-src 'self' 'wasm-unsafe-eval'",
  "style-src 'self'",
  "img-src 'self'",
  "connect-src 'self' https://www.simbrief.com",
  "base-uri 'none'",
  "form-action 'none'",
].join("; ");

function contentSecurityPolicy(): Plugin {
  return {
    name: "content-security-policy",
    apply: "build",
    transformIndexHtml: () => [
      {
        tag: "meta",
        attrs: { "http-equiv": "Content-Security-Policy", content: CSP },
        injectTo: "head-prepend",
      },
    ],
  };
}

// `base: "./"` keeps paths relative, so the built site works under the Pages project path
// (decision 0010). `just site` builds the WebAssembly module and the packs first; the packs are
// copied from ./public.
export default defineConfig({
  plugins: [svelte(), contentSecurityPolicy()],
  base: "./",
  server: { port: 1420, strictPort: true },
  build: {
    target: "es2022",
    rollupOptions: {
      // Relative to this folder, where the build runs.
      input: { index: "index.html", "simbrief-probe": "simbrief-probe.html" },
    },
  },
});
