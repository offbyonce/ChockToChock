// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

/**
 * A value with blanks to fill (`___`), split as 3.0 shows it: the parts with blanks on one line,
 * the rest of the instruction on its own. Parts are separated by a comma and a space, so a
 * number such as "1,000" stays whole. Null for a value with no blanks.
 */
export function fillParts(expect: string): { blanks: string; rest: string } | null {
  if (!expect.includes("___")) return null;
  const parts = expect.split(/,\s+/);
  return {
    blanks: parts.filter((p) => p.includes("___")).join(", "),
    rest: parts.filter((p) => !p.includes("___")).join(", "),
  };
}
