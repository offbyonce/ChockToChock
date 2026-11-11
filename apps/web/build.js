// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Builds the deployable site into apps/web/dist. For now it copies the placeholder and the
// SimBrief probe page from ./placeholder; later Phase 2 steps replace this with the real build
// (decision 0010).
// Run with `just site`.

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const source = path.join(here, "placeholder");
const out = path.join(here, "dist");
const files = ["index.html", "site.css", "simbrief-probe.html", "simbrief-probe.js", "ofp-probe.js"];

fs.rmSync(out, { recursive: true, force: true });
fs.mkdirSync(out, { recursive: true });
for (const file of files) {
  fs.copyFileSync(path.join(source, file), path.join(out, file));
}
process.stdout.write(`site: ${files.length} files in ${path.relative(process.cwd(), out)}\n`);
