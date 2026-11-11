// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Builds the deployable site into apps/web/dist. For now it copies the placeholder page; later
// Phase 2 steps replace this with the real build (decision 0010). Run with `just site`.

import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const here = path.dirname(fileURLToPath(import.meta.url));
const out = path.join(here, "dist");
const files = ["index.html", "site.css"];

fs.rmSync(out, { recursive: true, force: true });
fs.mkdirSync(out, { recursive: true });
for (const file of files) {
  fs.copyFileSync(path.join(here, file), path.join(out, file));
}
process.stdout.write(`site: ${files.length} files in ${path.relative(process.cwd(), out)}\n`);
