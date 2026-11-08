// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import { fireEvent, render, screen } from "@testing-library/svelte";
import { afterEach, describe, expect, it } from "vitest";
import App from "./App.svelte";

const checklist = {
  id: "before-start",
  title: "Before start",
  steps: [
    { id: "t.battery", text: "Battery switch", expect: "ON", sources: [] },
    { id: "t.doors", text: "Doors", expect: "CLOSED", note: "Check all doors.", sources: [] },
  ],
};

afterEach(() => clearMocks());

describe("App", () => {
  it("shows the version and the checklist, and ticks steps", async () => {
    mockIPC((cmd) => {
      if (cmd === "app_info") return { name: "Chock to Chock", version: "0.1.0", packSchemaVersion: 1 };
      if (cmd === "example_checklist") return checklist;
      throw new Error(`unexpected command ${cmd}`);
    });

    render(App);

    expect(await screen.findByText("v0.1.0")).toBeTruthy();
    expect(screen.getByText("Battery switch")).toBeTruthy();
    expect(screen.getByText("Check all doors.")).toBeTruthy();
    expect(screen.getByText("0 of 2 done")).toBeTruthy();

    await fireEvent.click(screen.getByRole("checkbox", { name: /Battery switch/ }));
    expect(screen.getByText("1 of 2 done")).toBeTruthy();
  });

  it("shows a tagged error from Rust", async () => {
    mockIPC((cmd) => {
      if (cmd === "app_info") return { name: "Chock to Chock", version: "0.1.0", packSchemaVersion: 1 };
      throw { kind: "PackInvalid", message: "line 3: unknown field" };
    });

    render(App);

    expect((await screen.findByRole("alert")).textContent).toBe("PackInvalid: line 3: unknown field");
  });
});
