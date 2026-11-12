// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import assert from "node:assert/strict";
import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { test } from "node:test";
import { WORKER, writeServiceWorker } from "./precache.js";

const template = fs.readFileSync(path.join(import.meta.dirname, "../src/sw/sw.js"), "utf8");

/** A small built site in a temporary folder, with the real worker template. */
function site() {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "chock-sw-"));
  const files = {
    "src/sw/sw.js": template,
    "dist/index.html": "placeholder",
    "dist/simbrief-probe.html": "probe",
    "dist/app/index.html": "app",
    "dist/app/manifest.webmanifest": "{}",
    "dist/assets/app-1.js": "code",
    "dist/design/index.html": "design page",
    "dist/assets/design-1.js": "design code",
    "dist/packs/demo.json": "{}",
  };
  for (const [file, text] of Object.entries(files)) {
    fs.mkdirSync(path.dirname(path.join(root, file)), { recursive: true });
    fs.writeFileSync(path.join(root, file), text);
  }
  return root;
}

test("the worker precaches the app's files, relative to its own folder", () => {
  const root = site();
  const { files, version } = writeServiceWorker(root, "dist");
  // Sorted by path within the site, then made relative to the worker's folder (app/).
  assert.deepEqual(files, ["index.html", "manifest.webmanifest", "../assets/app-1.js", "../packs/demo.json"]);
  const worker = fs.readFileSync(path.join(root, "dist", WORKER), "utf8");
  assert.ok(worker.includes(`const VERSION = "${version}";`));
  assert.ok(worker.includes(`const FILES = ${JSON.stringify(files)};`));
  assert.ok(!worker.includes("__CHOCK_"));
  fs.rmSync(root, { recursive: true });
});

test("the version changes with any file and only then", () => {
  const root = site();
  const first = writeServiceWorker(root, "dist").version;
  assert.equal(
    writeServiceWorker(root, "dist").version,
    first,
    "rebuilding the same files keeps the version",
  );
  fs.writeFileSync(path.join(root, "dist/packs/demo.json"), '{"changed":true}');
  assert.notEqual(writeServiceWorker(root, "dist").version, first);
  fs.rmSync(root, { recursive: true });
});
