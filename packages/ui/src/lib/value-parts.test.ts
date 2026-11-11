// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import { describe, expect, it } from "vitest";
import { fillParts } from "./value-parts";

describe("fillParts", () => {
  it("keeps numbers with a thousands comma whole", () => {
    // Splitting at every comma once showed "1, 000 LB".
    const parts = fillParts(
      "REQUIRED ___ KG/LBS, ON BOARD ___ KG/LBS, FUEL PUMPS ON, CENTER PUMPS ON IF MORE THAN 1,000 LB (ABOUT 450 KG) IN THE CENTER TANK",
    );
    expect(parts).toEqual({
      blanks: "REQUIRED ___ KG/LBS, ON BOARD ___ KG/LBS",
      rest: "FUEL PUMPS ON, CENTER PUMPS ON IF MORE THAN 1,000 LB (ABOUT 450 KG) IN THE CENTER TANK",
    });
  });

  it("leaves a value with no blanks alone", () => {
    expect(fillParts("ON, GUARD CLOSED")).toBeNull();
  });
});
