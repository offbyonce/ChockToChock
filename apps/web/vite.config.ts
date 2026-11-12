// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import { svelte } from "@sveltejs/vite-plugin-svelte";
import { defineConfig, type Plugin } from "vite";
import { writeServiceWorker } from "./build/precache.js";

// The site's content security policy (decision 0009): everything from this site only, except
// SimBrief, which the browser calls directly. 'wasm-unsafe-eval' lets the WebAssembly module
// compile; it allows no JavaScript eval. GitHub Pages cannot send headers, so it goes in a <meta>
// tag, added to the built pages only, because the dev server injects inline styles.
const CSP = [
  "default-src 'none'",
  "script-src 'self' 'wasm-unsafe-eval'",
  "style-src 'self'",
  "img-src 'self'",
  "font-src 'self'",
  "manifest-src 'self'",
  "worker-src 'self'",
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

// After the build, write the service worker with the list of files to precache and a version
// derived from them (decision 0010; see build/precache.js).
function serviceWorker(): Plugin {
  let root = "";
  let outDir = "";
  return {
    name: "service-worker",
    apply: "build",
    configResolved(config) {
      root = config.root;
      outDir = config.build.outDir;
    },
    closeBundle() {
      const { version, files } = writeServiceWorker(root, outDir);
      this.info(`service worker ${version}: ${files.length} files to precache`);
    },
  };
}

// `base: "./"` keeps paths relative, so the built site works under the Pages project path
// (decision 0010). `just site` builds the WebAssembly module and the packs first; the packs are
// copied from ./public. Pages: the placeholder front page, the app at app/ and the SimBrief
// probe page.
export default defineConfig({
  plugins: [svelte(), contentSecurityPolicy(), serviceWorker()],
  base: "./",
  server: { port: 1420, strictPort: true },
  build: {
    target: "es2022",
    rollupOptions: {
      // Relative to this folder, where the build runs. The front page stays the placeholder until
      // the app is ready for users; the app is at app/ meanwhile (Phase 2 brief, steps 10 and 11).
      input: {
        index: "index.html",
        app: "app/index.html",
        "simbrief-probe": "simbrief-probe.html",
        design: "design/index.html",
      },
    },
  },
});
