// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import { describe, expect, it } from "vitest";
import { contrast, DIMMING, palette, TEXT_PAIRS } from "./tokens";

describe("theme tokens", () => {
  it("measures contrast like WCAG", () => {
    expect(contrast("#000000", "#ffffff")).toBeCloseTo(21, 5);
    expect(contrast("#777777", "#777777")).toBeCloseTo(1, 5);
  });

  for (const dark of [true, false]) {
    for (const [step] of DIMMING.entries()) {
      it(`keeps text contrast at least 4.5:1 (${dark ? "dark" : "light"}, dimming ${step})`, () => {
        const p = palette(dark, step);
        for (const [fg, bg] of TEXT_PAIRS) {
          const ratio = contrast(p[fg], p[bg]);
          expect(ratio, `${fg} on ${bg}: ${ratio.toFixed(2)}`).toBeGreaterThanOrEqual(4.5);
        }
      });
    }
  }
});
