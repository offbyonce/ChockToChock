// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import { describe, expect, it } from "vitest";
import {
  colors,
  contrast,
  DIMMING,
  dim,
  GRAPHIC_PAIRS,
  PANEL_GRAPHIC_PAIRS,
  PANEL_TEXT_PAIRS,
  panelColors,
  scaleFor,
  TEXT_PAIRS,
} from "./tokens";

describe("theme tokens", () => {
  it("measures contrast like WCAG", () => {
    expect(contrast("#000000", "#ffffff")).toBeCloseTo(21, 5);
    expect(contrast("#777777", "#777777")).toBeCloseTo(1, 5);
  });

  it("dims only plain colors", () => {
    expect(dim("transparent", 0.5)).toBe("transparent");
    expect(dim("rgba(6, 8, 12, 0.55)", 0.5)).toBe("rgba(6, 8, 12, 0.55)");
    expect(dim("#ffffff", 1)).toBe("#ffffff");
    expect(dim("#ffffff", 0.5)).not.toBe("#ffffff");
  });

  it("scales desktop layouts with the window, never below 100 %", () => {
    expect(scaleFor(1440, 900)).toBe(1);
    expect(scaleFor(2560, 1440)).toBe(1.6);
    expect(scaleFor(1280, 720)).toBe(1);
    expect(scaleFor(390, 844)).toBe(1);
    // Ultrawide: the height limits it, and no columns are added.
    expect(scaleFor(3440, 1440)).toBe(1.6);
  });

  for (const dark of [true, false]) {
    for (const [step] of DIMMING.entries()) {
      const theme = `${dark ? "dark" : "light"}, dimming ${step}`;
      const check = (pairs: [string, string][], set: Record<string, string>, min: number) => {
        for (const [fg, bg] of pairs) {
          const [a, b] = [set[fg] ?? "", set[bg] ?? ""];
          if (!a.startsWith("#") || !b.startsWith("#")) continue;
          const ratio = contrast(a, b);
          expect(ratio, `${fg} on ${bg}: ${ratio.toFixed(2)}`).toBeGreaterThanOrEqual(min);
        }
      };
      it(`keeps text at 4.5:1 and icons at 3:1 (${theme})`, () => {
        check(TEXT_PAIRS, colors(dark, step), 4.5);
        check(GRAPHIC_PAIRS, colors(dark, step), 3);
      });
      it(`keeps drawing labels at 4.5:1 and outlines at 3:1 (${theme})`, () => {
        check(PANEL_TEXT_PAIRS, panelColors(dark, step), 4.5);
        check(PANEL_GRAPHIC_PAIRS, panelColors(dark, step), 3);
      });
    }
  }
});
