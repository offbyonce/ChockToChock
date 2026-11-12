// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import { describe, expect, it } from "vitest";
import { fitSize, labelLines } from "./map-labels";

describe("labelLines", () => {
  const full = 12;

  it("keeps a label on one line at full size when it fits", () => {
    expect(labelLines("FUEL", 80, 30, full)).toEqual(["FUEL"]);
  });

  it("keeps one line in a short box, where two lines would draw smaller (R WIPER)", () => {
    // Wrapping first made R WIPER two tiny lines in its short box on the overhead map.
    const lines = labelLines("R WIPER", 40, 14, full);
    expect(lines).toEqual(["R WIPER"]);
    expect(fitSize(lines, 40, 14, full)).toBeGreaterThan(fitSize(["R", "WIPER"], 40, 14, full));
  });

  it("lets one line of capitals fill most of a short box's height (VOICE REC)", () => {
    // A full line height for one line drew VOICE REC smaller than 3.0 in its short cell.
    expect(fitSize(["VOICE REC"], 200, 11, full)).toBeGreaterThan(9.5);
  });

  it("wraps in a narrow, tall box, splitting at a slash", () => {
    expect(labelLines("NAV / DISPLAY", 50, 40, full)).toEqual(["NAV /", "DISPLAY"]);
  });
});
