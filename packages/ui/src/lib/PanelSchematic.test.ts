// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import type { PanelSection } from "@chocktochock/bindings";
import { cleanup, render, screen } from "@testing-library/svelte";
import { afterEach, describe, expect, it } from "vitest";
import PanelSchematic from "./PanelSchematic.svelte";

const section = {
  id: "elec",
  title: "Electrical",
  x: 0,
  y: 0,
  width: 200,
  height: 120,
  items: [
    {
      kind: "toggle_switch",
      label: "BATTERY",
      guarded: true,
      x: 40,
      y: 50,
      controls: ["bat"],
      positions: "OFF, ON",
      description: "",
    },
    {
      kind: "toggle_switch",
      label: "CAB/UTIL",
      x: 120,
      y: 50,
      controls: ["cab"],
      positions: "OFF, ON",
      description: "",
    },
  ],
} as unknown as PanelSection;

afterEach(cleanup);

describe("PanelSchematic", () => {
  it("shows the step's target as the target, and no position on other controls", () => {
    render(PanelSchematic, {
      section,
      targets: [{ section: "elec", item: 0, position: 1 }],
      guardClosed: true,
    });

    const svg = screen.getByRole("img");
    expect(svg.getAttribute("aria-label")).toBe("Electrical panel, schematic. This step: BATTERY set to ON.");
    expect(screen.getByText("SET TO ON")).toBeTruthy();
    // One lever is drawn, for the battery; the other switch shows no position.
    expect(svg.querySelectorAll(".lever")).toHaveLength(1);
    expect(svg.querySelectorAll(".unknown")).toHaveLength(1);
  });

  it("highlights a control without a target when the step names no single position", () => {
    render(PanelSchematic, { section, targets: [{ section: "elec", item: 0, position: null }] });

    expect(screen.queryByText(/SET TO/)).toBeNull();
    expect(screen.getByRole("img").getAttribute("aria-label")).toBe(
      "Electrical panel, schematic. This step: BATTERY.",
    );
  });
});
