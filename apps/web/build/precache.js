// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Writes the service worker after the site is built (decision 0010). It lists the files to
// precache and derives the version, and so the cache name, from their paths and contents: any
// change to any file gives a new version, and an unchanged build gives the same one.

import { createHash } from "node:crypto";
import fs from "node:fs";
import path from "node:path";

/** Where the worker is written, relative to the built site. Its folder is its scope. */
export const WORKER = "app/sw.js";

/** Built pages outside the app that the worker does not precache. */
const OUTSIDE_APP = new Set(["index.html", "simbrief-probe.html"]);
/** The drawing gallery (design/) and its script: for review only, never part of the app a pilot installs. */
const DESIGN = (file) => file.startsWith("design/") || /^assets\/design-[^/]*$/.test(file);

function filesIn(dir, base = dir) {
  return fs.readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
    const full = path.join(dir, entry.name);
    return entry.isDirectory() ? filesIn(full, base) : [path.relative(base, full).split(path.sep).join("/")];
  });
}

/** The files to precache (relative to the site) and the version they give. */
export function precacheList(dist) {
  const files = filesIn(dist)
    .filter((file) => file !== WORKER && !OUTSIDE_APP.has(file) && !DESIGN(file))
    .sort();
  const hash = createHash("sha256");
  for (const file of files) {
    hash.update(`${file}\0`);
    hash.update(fs.readFileSync(path.join(dist, file)));
  }
  return { files, version: hash.digest("hex").slice(0, 16) };
}

/** Fills in the worker template and writes it into the built site. Returns what it wrote. */
export function writeServiceWorker(root, outDir) {
  const dist = path.resolve(root, outDir);
  const { files, version } = precacheList(dist);
  const workerDir = path.posix.dirname(WORKER);
  const urls = files.map((file) => path.posix.relative(workerDir, file));
  const template = fs.readFileSync(path.resolve(root, "src/sw/sw.js"), "utf8");
  const worker = template
    .replace('"__CHOCK_VERSION__"', JSON.stringify(version))
    .replace('["__CHOCK_FILES__"]', JSON.stringify(urls));
  if (worker.includes("__CHOCK_")) throw new Error("service worker template placeholders not found");
  fs.writeFileSync(path.join(dist, WORKER), worker);
  return { version, files: urls };
}
