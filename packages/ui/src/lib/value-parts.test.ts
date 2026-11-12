// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import { describe, expect, it } from "vitest";
import { fields, fillParts } from "./value-parts";

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

describe("fields", () => {
  it("labels each blank with the words before it and, up to a comma, after it, as 3.0 did", () => {
    expect(fields("REQUIRED ___ KG/LBS, ON BOARD ___ KG/LBS")).toEqual([
      { before: "REQUIRED", after: "KG/LBS" },
      { before: "ON BOARD", after: "KG/LBS" },
    ]);
    expect(fields("CDU ___, PWB ___, FLAPS ___ INDICATED")).toEqual([
      { before: "CDU", after: "" },
      { before: "PWB", after: "" },
      { before: "FLAPS", after: "INDICATED" },
    ]);
    expect(fields("___ SET")).toEqual([{ before: "", after: "SET" }]);
    expect(fields("V2 ___, HEADING ___, ALTITUDE ___")).toHaveLength(3);
  });
});
