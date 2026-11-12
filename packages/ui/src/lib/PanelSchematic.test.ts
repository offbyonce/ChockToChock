// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import type { Panel } from "@chocktochock/bindings";
import { cleanup, render, screen } from "@testing-library/svelte";
import { afterEach, describe, expect, it } from "vitest";
import PanelSchematic from "./PanelSchematic.svelte";

const toggle = (label: string, x: number, controls: string[], guarded = false) => ({
  kind: "toggle_switch",
  label,
  guarded,
  x,
  y: 50,
  controls,
  positions: "OFF, ON",
  description: "",
});

const panel = {
  id: "oh",
  title: "Overhead",
  width: 600,
  height: 300,
  sections: [
    {
      id: "elec",
      title: "Electrical",
      x: 0,
      y: 0,
      width: 200,
      height: 120,
      items: [toggle("BATTERY", 40, ["bat"], true), toggle("CAB/UTIL", 120, ["cab"])],
    },
    {
      id: "lwip",
      title: "L wiper",
      x: 210,
      y: 0,
      width: 90,
      height: 120,
      items: [toggle("L WIPER", 45, ["wip"])],
    },
    {
      id: "rwip",
      title: "R wiper",
      x: 310,
      y: 0,
      width: 90,
      height: 120,
      items: [toggle("R WIPER", 45, ["wip"])],
    },
  ],
} as unknown as Panel;

const wiperPanel = {
  id: "oh",
  title: "Overhead",
  width: 300,
  height: 120,
  sections: [
    {
      id: "lwip",
      title: "L wiper",
      x: 0,
      y: 0,
      width: 97,
      height: 84,
      items: [
        {
          kind: "selector",
          label: "L WIPER",
          x: 48,
          y: 50,
          controls: ["wiper"],
          positions: "PARK, INT, LOW, HIGH",
          description: "",
        },
      ],
    },
  ],
} as unknown as Panel;

afterEach(cleanup);

describe("PanelSchematic", () => {
  it("shows the step's target as the target, and no position on other controls", () => {
    render(PanelSchematic, {
      panel,
      focus: ["elec"],
      targets: [{ section: "elec", item: 0, position: 1, lit: false }],
      guardClosed: true,
    });

    const svg = screen.getByRole("img");
    expect(svg.getAttribute("aria-label")).toBe("Electrical panel, schematic. This step: BATTERY set to ON.");
    expect(screen.getByText("SET TO ON")).toBeTruthy();
    // One knob at a position, for the battery; every other switch shows no position.
    expect(svg.querySelectorAll(".knob.set")).toHaveLength(1);
    expect(svg.querySelectorAll(".unknown").length).toBeGreaterThan(0);
  });

  it("highlights a control without a target when the step names no single position", () => {
    render(PanelSchematic, {
      panel,
      focus: ["elec"],
      targets: [{ section: "elec", item: 0, position: null, lit: false }],
    });

    expect(screen.queryByText(/SET TO/)).toBeNull();
    expect(screen.getByRole("img").getAttribute("aria-label")).toBe(
      "Electrical panel, schematic. This step: BATTERY.",
    );
  });

  it("draws neighboring small sections together in one drawing", () => {
    const targets = [
      { section: "lwip", item: 0, position: 1, lit: false },
      { section: "rwip", item: 0, position: 1, lit: false },
    ];
    render(PanelSchematic, { panel, focus: ["lwip", "rwip"], targets });

    const svg = screen.getByRole("img");
    expect(svg.getAttribute("aria-label")).toBe(
      "L wiper and R wiper panel, schematic. This step: L WIPER set to ON; R WIPER set to ON.",
    );
    // Each section's title and its control's label.
    expect(screen.getAllByText("L WIPER")).toHaveLength(2);
    expect(screen.getAllByText("R WIPER")).toHaveLength(2);
    expect(screen.getAllByText("SET TO ON")).toHaveLength(2);
  });

  it("labels every selector position at its own tick, so the pointer and its label agree", () => {
    render(PanelSchematic, {
      panel: wiperPanel,
      focus: ["lwip"],
      targets: [{ section: "lwip", item: 0, position: 0, lit: false }],
    });

    const svg = screen.getByRole("img");
    for (const p of ["PARK", "INT", "LOW", "HIGH"]) expect(screen.getByText(p)).toBeTruthy();
    // PARK is the first position: far left, where the pointer points.
    expect(Number(screen.getByText("PARK").getAttribute("x"))).toBeLessThan(0);
    expect(Number(screen.getByText("HIGH").getAttribute("x"))).toBeGreaterThan(0);
    expect(svg.querySelector(".pointer")?.getAttribute("transform")).toBe("rotate(-90)");
    expect(screen.getByText("PARK").classList.contains("target")).toBe(true);
  });

  it("grows a section's frame to hold the labels under its controls", () => {
    render(PanelSchematic, {
      panel: wiperPanel,
      focus: ["lwip"],
      targets: [{ section: "lwip", item: 0, position: 0, lit: false }],
    });

    const frame = screen.getByRole("img").querySelector(".frame");
    const setTo = screen.getByText("SET TO PARK");
    const frameBottom = Number(frame?.getAttribute("y")) + Number(frame?.getAttribute("height"));
    // The label sits in the control's group, which is placed at the control's y (50).
    expect(50 + Number(setTo.getAttribute("y"))).toBeLessThan(frameBottom);
  });

  it("draws an unused control at its resting position, dimmed and unlike the target", () => {
    const rested = structuredClone(panel);
    const cab = rested.sections[0]?.items[1];
    if (cab) cab.rest = "OFF";
    render(PanelSchematic, {
      panel: rested,
      focus: ["elec"],
      targets: [{ section: "elec", item: 0, position: 1, lit: false }],
    });

    const svg = screen.getByRole("img");
    // The battery's target is solid; CAB/UTIL rests at OFF, hollow and muted.
    expect(svg.querySelectorAll(".knob.set")).toHaveLength(1);
    expect(svg.querySelectorAll(".knob.rest")).toHaveLength(1);
    const restLabels = [...svg.querySelectorAll(".pos.at-rest")].map((t) => t.textContent);
    expect(restLabels).toEqual(["OFF"]);
  });

  it("draws every light the same size, and a light the step expects on as lit", () => {
    const lights = {
      id: "con",
      title: "Console",
      width: 300,
      height: 100,
      sections: [
        {
          id: "lo",
          title: "Parking brake",
          x: 0,
          y: 0,
          width: 300,
          height: 100,
          items: [
            {
              kind: "light",
              label: "PARK BRAKE LIGHT",
              color: "amber",
              x: 80,
              y: 50,
              controls: ["pbrake"],
              description: "",
            },
            {
              kind: "light",
              label: "ELEC",
              color: "amber",
              x: 200,
              y: 50,
              controls: ["other"],
              description: "",
            },
          ],
        },
      ],
    } as unknown as Panel;
    render(PanelSchematic, {
      panel: lights,
      focus: ["lo"],
      targets: [{ section: "lo", item: 0, position: null, lit: true }],
    });

    const lamps = [...screen.getByRole("img").querySelectorAll("rect.lamp")];
    expect(lamps.map((r) => r.getAttribute("width"))).toEqual(["50", "50"]);
    expect(lamps[0]?.classList.contains("lit")).toBe(true);
    expect(lamps[1]?.classList.contains("lit")).toBe(false);
    expect(screen.getByText("EXPECTED ON")).toBeTruthy();
  });

  it("draws a handle's grip at its target, beside that position's label", () => {
    const handle = {
      id: "con",
      title: "Console",
      width: 200,
      height: 100,
      sections: [
        {
          id: "lo",
          title: "Parking brake",
          x: 0,
          y: 0,
          width: 200,
          height: 100,
          items: [
            {
              kind: "handle",
              label: "PARKING BRAKE",
              x: 60,
              y: 50,
              controls: ["pbrake"],
              positions: "RELEASED, SET",
              description: "",
            },
          ],
        },
      ],
    } as unknown as Panel;
    render(PanelSchematic, {
      panel: handle,
      focus: ["lo"],
      targets: [{ section: "lo", item: 0, position: 1, lit: false }],
    });

    const grip = screen.getByRole("img").querySelector(".grip");
    const set = [...screen.getByRole("img").querySelectorAll(".pos")].find((t) => t.textContent === "SET");
    const gripMid = Number(grip?.getAttribute("y")) + Number(grip?.getAttribute("height")) / 2;
    // The label's baseline sits about 3 units below the position it names.
    expect(Math.abs(gripMid - (Number(set?.getAttribute("y")) - 3))).toBeLessThan(1);
  });

  it("draws a tall section whole, as 3.0 does, so controls near its bottom are not cut off", () => {
    const tall = {
      id: "oh",
      title: "Overhead",
      width: 300,
      height: 400,
      sections: [
        {
          id: "fctl",
          title: "Flight control",
          x: 0,
          y: 0,
          width: 240,
          height: 320,
          items: [
            {
              kind: "toggle_switch",
              label: "ALT FLAPS",
              x: 60,
              y: 60,
              controls: ["altflap"],
              positions: "UP, OFF, DOWN",
              description: "",
            },
            {
              kind: "light",
              label: "MACH TRIM FAIL",
              color: "amber",
              x: 180,
              y: 280,
              controls: ["lights"],
              description: "",
            },
          ],
        },
      ],
    } as unknown as Panel;
    render(PanelSchematic, {
      panel: tall,
      focus: ["fctl"],
      targets: [{ section: "fctl", item: 0, position: 1, lit: false }],
    });

    expect(screen.getByText(/MACH/)).toBeTruthy();
  });
});
