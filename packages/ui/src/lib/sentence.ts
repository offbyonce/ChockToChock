// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import type { PackIndex } from "../pack-index";

/**
 * A step name in capitals as a sentence ("FUEL PUMP SWITCHES" as "Fuel pump switches"), keeping
 * glossary terms and codes ("MCP", "TERR (EFIS control panel)", "N1"); mixed case stays.
 */
export function sentence(index: PackIndex, text: string): string {
  if (text !== text.toUpperCase()) return text;
  const words = text
    .split(/(\s+|[(),])/)
    .map((w) => (index.glossary(w) !== undefined || /\d|\//.test(w) ? w : w.toLowerCase()));
  const out = words.join("");
  return out.charAt(0).toUpperCase() + out.slice(1);
}
