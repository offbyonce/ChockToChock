// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import { cleanup, render } from "@testing-library/svelte";
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
  });

  it("shows the SID with its transition, as the Checklist board ('SUMMA2.SEA')", () => {
    render(FlightStrip, props({ sid: "SUMMA2", transition: "SEA", runway: "16L" }));
    expect(document.querySelector('[data-field="sid"] .val')?.textContent).toBe("SUMMA2.SEA");
  });

  it("shows the SID alone without a transition, and the Notes chip once there is a note", () => {
    render(FlightStrip, props({ sid: "SUMMA2" }, "Expect runway 16L"));
    expect(document.querySelector('[data-field="sid"] .val')?.textContent).toBe("SUMMA2");
    expect(document.querySelector(".notes")?.textContent).toBe("Notes 1");
  });
});
