// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import { cleanup, fireEvent, render } from "@testing-library/svelte";
import { afterEach, describe, expect, it } from "vitest";
import FlightStrip from "./FlightStrip.svelte";

afterEach(cleanup);

describe("FlightStrip", () => {
  const props = (values: Record<string, string>, notes = "") => ({
    values,
    notes,
    onSet: () => {},
    onNotes: () => {},
    onClear: () => {},
    onImport: () => {},
  });

  it("shows the SID with its transition, as the Checklist board ('SUMMA2.SUMMA')", () => {
    render(FlightStrip, props({ sid: "SUMMA2", transition: "SUMMA", runway: "16L" }));
    expect(document.querySelector('[data-field="sid"] .val')?.textContent).toBe("SUMMA2.SUMMA");
  });

  it("shows the SID alone without a transition, and the Notes chip once there is a note", () => {
    render(FlightStrip, props({ sid: "SUMMA2" }, "Expect runway 16L"));
    expect(document.querySelector('[data-field="sid"] .val')?.textContent).toBe("SUMMA2");
    expect(document.querySelector(".notes")?.textContent).toBe("Notes 1");
  });

  it("opened with a pointer, puts the cursor in Runway without a ring until a key is pressed", async () => {
    render(FlightStrip, props({ runway: "16L" }));
    await fireEvent.click(document.querySelector(".fsline") as HTMLElement, { detail: 1 });
    await new Promise((go) => setTimeout(go, 0));
    const runway = document.getElementById("sp-runway");
    expect(document.activeElement).toBe(runway);
    expect(runway?.classList.contains("quiet")).toBe(true);
    await fireEvent.keyDown(document, { key: "Tab" });
    expect(runway?.classList.contains("quiet")).toBe(false);
  });

  it("shows a squawk outside 0 to 7 as a short message under the field", async () => {
    render(FlightStrip, props({ squawk: "7800" }));
    await fireEvent.click(document.querySelector(".fsline") as HTMLElement);
    expect(document.querySelector(".err")?.textContent).toBe("Use 4 digits, 0 to 7");
  });
});
