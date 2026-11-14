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

  it("opened from the keyboard, puts the cursor in Runway with its focus ring", async () => {
    render(FlightStrip, props({ runway: "16L" }));
    // Enter or Space on a button clicks it with detail 0.
    await fireEvent.click(document.querySelector(".fsline") as HTMLElement, { detail: 0 });
    await new Promise((go) => setTimeout(go, 0));
    const runway = document.getElementById("sp-runway");
    expect(document.activeElement).toBe(runway);
    expect(runway?.classList.contains("quiet")).toBe(false);
  });

  it("shows an initial altitude outside 3.0's range under the field, tied to it", async () => {
    render(FlightStrip, props({ initial_altitude: "45000" }));
    await fireEvent.click(document.querySelector(".fsline") as HTMLElement);
    const err = document.querySelector(".err");
    expect(err?.textContent?.trim()).toBe("Enter 0 to 41,000 ft");
    const input = document.getElementById("sp-initial_altitude");
    expect(input?.getAttribute("aria-invalid")).toBe("true");
    expect(input?.getAttribute("aria-describedby")).toBe(err?.id);
  });
});
