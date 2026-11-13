// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// The one Node function the design token test uses to read stylesheets (Vitest returns empty text
// for a raw CSS import). Declared here so the package needs no Node type definitions.
declare module "node:fs" {
  export function readFileSync(path: string, encoding: "utf8"): string;
}

interface ImportMeta {
  /** The folder of this module (Node and Vitest). */
  readonly dirname: string;
}
