// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import type { AppInfo, Checklist } from "@chocktochock/bindings";
import { fireEvent, render, screen } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";
import App from "./App.svelte";
import { type Host, hostContext } from "./host";

const info: AppInfo = { name: "Chock to Chock", version: "0.1.0", packSchemaVersion: 1 };

// As Rust sends it: optional fields are left out.
const checklist = {
  id: "before-start",
  title: "Before start",
  kind: "checklist",
  basis: ["sim-observed"],
  steps: [
    { id: "t.battery", text: "Battery switch", expect: "ON" },
    { id: "t.doors", text: "Doors", expect: "CLOSED", note: "Check all doors." },
  ],
} as unknown as Checklist;

function renderWith(host: Host): void {
  render(App, { context: hostContext(host) });
}

describe("App", () => {
  it("shows the version and the checklist, and ticks steps", async () => {
    renderWith({
      appInfo: async () => info,
      exampleChecklist: async () => checklist,
    });

    expect(await screen.findByText("v0.1.0")).toBeTruthy();
    expect(screen.getByText("Battery switch")).toBeTruthy();
    expect(screen.getByText("Check all doors.")).toBeTruthy();
    expect(screen.getByText("0 of 2 done")).toBeTruthy();

    await fireEvent.click(screen.getByRole("checkbox", { name: /Battery switch/ }));
    expect(screen.getByText("1 of 2 done")).toBeTruthy();
  });

  it("shows a tagged error from Rust", async () => {
    renderWith({
      appInfo: async () => info,
      exampleChecklist: () => Promise.reject({ kind: "PackInvalid", message: "line 3: unknown field" }),
    });

    expect((await screen.findByRole("alert")).textContent).toBe("PackInvalid: line 3: unknown field");
  });
});
