// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

/**
 * Panel section titles as one card caption, as 3.0 named panels: "Electrical meters panel". Two or
 * more sections share their common last words and take the plural: "Captain EFIS" and "First
 * officer EFIS" are "Captain and First officer EFIS panels".
 */
export function panelCaption(titles: string[]): string {
  const strip = (t: string) => t.replace(/\s+panel$/i, "");
  const names = titles.map(strip);
  if (names.length <= 1) return `${names[0] ?? ""} panel`;
  const words = names.map((n) => n.split(/\s+/));
  let shared = 0;
  const first = words[0] ?? [];
  while (
    shared < first.length - 1 &&
    words.every((w) => w.length - shared > 1 && w[w.length - 1 - shared] === first[first.length - 1 - shared])
  ) {
    shared++;
  }
  const tail = first.slice(first.length - shared).join(" ");
  const heads = words.map((w) => w.slice(0, w.length - shared).join(" "));
  const list =
    heads.length === 2 ? heads.join(" and ") : `${heads.slice(0, -1).join(", ")} and ${heads.at(-1)}`;
  return `${list}${tail ? ` ${tail}` : ""} panels`;
}
