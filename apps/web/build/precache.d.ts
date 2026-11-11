// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

/** Fills in the service worker template and writes it into the built site. */
export function writeServiceWorker(root: string, outDir: string): { version: string; files: string[] };
