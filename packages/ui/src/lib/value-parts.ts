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

/**
 * The fields of a value's blank parts, as 3.0 labeled them: each blank with the words before it
 * and, up to the next comma, the words after it. "REQUIRED ___ KG/LBS, ON BOARD ___ KG/LBS" is
 * REQUIRED [] KG/LBS and ON BOARD [] KG/LBS; "CDU ___, PWB ___" is CDU [] and PWB [].
 */
export function fields(blanks: string): { before: string; after: string }[] {
  const pieces = blanks.split("___");
  const out: { before: string; after: string }[] = [];
  let before = pieces[0]?.trim() ?? "";
  for (let i = 1; i < pieces.length; i++) {
    const piece = pieces[i] ?? "";
    const comma = piece.indexOf(",");
    if (i === pieces.length - 1) {
      out.push({ before, after: (comma < 0 ? piece : piece.slice(0, comma)).trim() });
    } else if (comma >= 0) {
      out.push({ before, after: piece.slice(0, comma).trim() });
      before = piece.slice(piece.lastIndexOf(",") + 1).trim();
    } else {
      out.push({ before, after: "" });
      before = piece.trim();
    }
  }
  return out;
}
