// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import { describe, expect, it } from "vitest";
import { marksStep } from "./marked-step";

describe("the step bar", () => {
  it("marks the selected step, not the current one, once a step is selected", () => {
    // Opening door lights while Landing gear pins is current left the bar on Landing gear pins.
    expect(marksStep("doors", "doors", "todo")).toBe(true);
    expect(marksStep("doors", "pins", "current")).toBe(false);
  });

  it("marks the current step while nothing is selected", () => {
    expect(marksStep(null, "pins", "current")).toBe(true);
    expect(marksStep(null, "doors", "todo")).toBe(false);
  });
});
