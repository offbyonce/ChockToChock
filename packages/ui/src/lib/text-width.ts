// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// How wide a label is in the interface face, so drawings can leave room for it before the
// browser lays it out (and in tests, where nothing is laid out). Advance widths of Geist
// SemiBold, the widest weight our drawings use, as a share of the text size, read from the
// bundled font file. A flat average per letter was too narrow for wide capitals such as M and W,
// which let neighboring labels touch.

const WIDTHS: Record<string, number> = {
  A: 0.709,
  B: 0.695,
  C: 0.724,
  D: 0.708,
  E: 0.615,
  F: 0.6,
  G: 0.726,
  H: 0.719,
  I: 0.29,
  J: 0.617,
  K: 0.672,
  L: 0.586,
  M: 0.902,
  N: 0.748,
  O: 0.763,
  P: 0.664,
  Q: 0.757,
  R: 0.689,
  S: 0.668,
  T: 0.584,
  U: 0.699,
  V: 0.709,
  W: 0.991,
  X: 0.66,
  Y: 0.613,
  Z: 0.577,
  "0": 0.683,
  "1": 0.427,
  "2": 0.642,
  "3": 0.637,
  "4": 0.643,
  "5": 0.656,
  "6": 0.615,
  "7": 0.538,
  "8": 0.644,
  "9": 0.618,
  " ": 0.236,
  "/": 0.508,
  "-": 0.418,
  ".": 0.225,
  ",": 0.225,
  "(": 0.306,
  ")": 0.306,
  "&": 0.677,
  ":": 0.306,
  "+": 0.566,
  "'": 0.195,
  "%": 0.818,
  "°": 0.42,
};
/** Any other character: about a lowercase letter, rounded up. */
const OTHER = 0.6;

/** The width of `text` in ems of the interface face; multiply by the size to get the width. */
export function em(text: string): number {
  let w = 0;
  for (const c of text) w += WIDTHS[c] ?? OTHER;
  return w;
}

/** The widest of several lines, in ems. */
export const widest = (lines: string[]): number => Math.max(0, ...lines.map(em));
