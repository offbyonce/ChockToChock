// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// How a flight deck map's area label fits its box. Sizes are in map units; `full` is 3.0's label
// size in those units.

/**
 * The size a label's lines can take inside a box, never above `full`. Capitals need about 0.9 of
 * the size in height, and each further line 1.2, so a short box keeps 3.0's larger one-line text.
 */
export const fitSize = (text: string[], width: number, height: number, full: number): number =>
  Math.min(
    full,
    (width - 7) / (Math.max(...text.map((t) => t.length)) * 0.62),
    (height - 2) / (0.9 + (text.length - 1) * 1.2),
  );

/**
 * A label's lines: one line at full size if it fits; otherwise one line or two (split at " / " or
 * near the middle), whichever draws larger. A short box keeps one line, where two would come out
 * smaller; a narrow, tall one wraps, as 3.0 does.
 */
export function labelLines(label: string, width: number, height: number, full: number): string[] {
  const one = [label];
  if (fitSize(one, width, height, full) >= full || !label.includes(" ")) return one;
  const slash = label.indexOf(" / ");
  const mid = label.lastIndexOf(" ", Math.ceil(label.length / 2));
  const two =
    slash > 0
      ? [`${label.slice(0, slash)} /`, label.slice(slash + 3)]
      : mid > 0
        ? [label.slice(0, mid), label.slice(mid + 1)]
        : one;
  return fitSize(two, width, height, full) > fitSize(one, width, height, full) ? two : one;
}
