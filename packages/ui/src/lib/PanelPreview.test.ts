// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import type { Panel } from "@chocktochock/bindings";
import { cleanup, render, screen } from "@testing-library/svelte";
import { afterEach, describe, expect, it } from "vitest";
import PanelPreview from "./PanelPreview.svelte";

afterEach(cleanup);

const panel = {
  id: "oh",
  title: "Overhead",
  width: 300,
  height: 200,
  sections: [
    {
      id: "fuel",
      title: "Fuel",
      x: 10,
      y: 20,
      width: 200,
      height: 120,
      items: [
        {
          kind: "toggle_switch",
          label: "AFT 1",
          x: 40,
          y: 60,
          controls: ["c"],
          positions: "OFF, ON",
          description: "",
        },
        {
          kind: "toggle_switch",
          label: "CTR L",
          x: 100,
          y: 60,
          controls: ["c"],
          positions: "OFF, ON",
          description: "",
        },
        {
          kind: "light",
          label: "LOW PRESSURE",
          x: 160,
          y: 30,
          color: "amber",
          controls: ["c"],
          description: "",
        },
      ],
    },
  ],
} as unknown as Panel;

describe("PanelPreview", () => {
  it("draws the whole section with the same glyphs and states, and no text at all", () => {
    render(PanelPreview, {
      panel,
      focus: ["fuel"],
      targets: [
        { section: "fuel", item: 0, position: 1, lit: false, open: false, resolved: false },
        { section: "fuel", item: 1, position: null, lit: false, open: true, resolved: false },
        { section: "fuel", item: 2, position: null, lit: true, open: false, resolved: false },
      ],
      label: "The whole fuel panel",
    });
    const svg = screen.getByRole("img", { name: "The whole fuel panel" });
    // The whole section, never cropped to the step's rows.
    expect(svg.getAttribute("viewBox")).toBe("8 18 204 124");
    // Changed blue, conditional dashed, a lit light outline only.
    expect(svg.querySelectorAll(".body.changed")).toHaveLength(1);
    expect(svg.querySelectorAll(".body.conditional")).toHaveLength(1);
    expect(svg.querySelectorAll(".lamp.lit")).toHaveLength(0);
    // Text is in the drawing for layout but never shown.
    expect(svg.querySelector("g")?.classList.contains("textless")).toBe(true);
  });
});
