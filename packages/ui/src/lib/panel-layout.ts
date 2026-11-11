// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Layout shared by the panel drawing (PanelSchematic) and its sections (PanelSection): glyph
// sizes, label lines and how far labels reach, so a section's frame always holds its labels.
// Drawing units are the pack's own; text keeps fixed pixel sizes, so its size in units depends
// on `k`, the screen pixels per unit.

import type { PanelItem } from "@chocktochock/bindings";

/**
 * 3.0's rendered text sizes, in pixels: names and titles 11.8, positions and light words 10.8.
 * "SET TO …" sits under the name, a little smaller so the name reads first.
 */
export const PX = { name: 11.8, title: 11.8, pos: 10.8, lamp: 10.8, setTo: 9.5 } as const;
/** Line height for labels under a control, as a multiple of the text size. */
const LINE = 1.15;

export const positionsOf = (item: PanelItem): string[] =>
  (item.positions ?? "")
    .split(",")
    .map((p) => p.trim())
    .filter(Boolean);

/** Half the height a glyph takes, so its label sits under it. */
export function half(item: PanelItem): number {
  switch (item.kind) {
    case "toggle_switch":
      return item.guarded ? 19 : 15;
    case "selector":
    case "gauge":
      return 15;
    case "push_button":
      return 11;
    case "light":
      return 10;
    default:
      return (item.height ?? 30) / 2;
  }
}

/** Long labels go on two lines, split near the middle. */
export function lines(label: string, max = 12): string[] {
  if (label.length <= max) return [label];
  const words = label.split(" ");
  if (words.length < 2) return [label];
  let best = 1;
  let bestDiff = Number.POSITIVE_INFINITY;
  for (let i = 1; i < words.length; i++) {
    const diff = Math.abs(words.slice(0, i).join(" ").length - words.slice(i).join(" ").length);
    if (diff < bestDiff) [best, bestDiff] = [i, diff];
  }
  return [words.slice(0, best).join(" "), words.slice(best).join(" ")];
}

/** Whether a control has its name written under it (lights and screens carry it inside). */
export const labelled = (item: PanelItem) =>
  item.kind !== "light" && item.kind !== "readout" && item.kind !== "display_unit";

/** The baseline of label line `n` (0 is the name's first line) under a control, in units. */
export const labelY = (item: PanelItem, n: number, k: number) =>
  half(item) + 4 + (n + 1) * ((PX.name * LINE) / k);

/** How far below the section's top a control and its labels reach, in units. */
export function bottomOf(item: PanelItem, hasTarget: boolean, k: number): number {
  if (!labelled(item)) return item.y + half(item);
  const count = lines(item.label).length + (hasTarget ? 1 : 0);
  return item.y + labelY(item, count - 1, k) + (PX.name * 0.35) / k;
}

/**
 * A selector's tick angle, in degrees from straight up, clockwise: positions spread across the
 * top from left (first) to right (last), as a rotary selector is read.
 */
export const angle = (index: number, count: number) => (count < 2 ? 0 : -90 + (180 * index) / (count - 1));

/** Which selector positions get a label: all of up to four; otherwise the ends and the target. */
export const labelledPositions = (count: number, target: number | null): number[] =>
  count <= 4
    ? [...Array(count).keys()]
    : [...new Set([0, count - 1, ...(target == null ? [] : [target])])].sort((a, b) => a - b);
