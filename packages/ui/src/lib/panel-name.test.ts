// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import { describe, expect, it } from "vitest";
import { panelCaption } from "./panel-name";

describe("panelCaption", () => {
  it("names one section as a panel, once", () => {
    expect(panelCaption(["Electrical meters"])).toBe("Electrical meters panel");
    expect(panelCaption(["Doors panel"])).toBe("Doors panel");
  });

  it("shares the common last words of several sections", () => {
    // Was "Captain EFIS and First officer EFIS panel".
    expect(panelCaption(["Captain EFIS", "First officer EFIS"])).toBe(
      "Captain and First officer EFIS panels",
    );
    expect(panelCaption(["Left CDU", "Right CDU"])).toBe("Left and Right CDU panels");
    expect(panelCaption(["Fuel", "Hydraulic"])).toBe("Fuel and Hydraulic panels");
  });
});
