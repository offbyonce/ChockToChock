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
    // 3.0's angle for the first of four positions.
    expect(svg.querySelector(".pointer")?.getAttribute("transform")).toBe("rotate(-100)");
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

  it("cuts a tall section to the controls near the step's, as 3.0 does, keeping lights within reach", () => {
    // Placed as on the real panels: the electrical meters section (battery step) and the flight
    // control section (alternate flaps step). Round 2 cut off the lights 99 units below ALT FLAPS;
    // drawing sections whole then pushed the battery switch out of first view.
    const item = (kind: string, label: string, x: number, y: number) =>
      ({ kind, label, x, y, controls: ["c"], positions: "OFF, ON", description: "" }) as unknown;
    const tall = {
      id: "oh",
      title: "Overhead",
      width: 300,
      height: 400,
      sections: [
        {
          id: "meters",
          title: "Electrical meters",
          x: 0,
          y: 0,
          width: 225,
          height: 318,
          items: [
            item("display_unit", "METER DISPLAY", 112, 74),
            item("light", "BAT DISCHARGE", 42, 148),
            item("selector", "DC METERS", 60, 206),
            item("toggle_switch", "BATTERY", 60, 276),
          ],
        },
        {
          id: "fctl",
          title: "Flight control",
          x: 0,
          y: 0,
          width: 235,
          height: 322,
          items: [item("toggle_switch", "ALT FLAPS", 205, 134), item("light", "MACH TRIM FAIL", 195, 232)],
        },
      ],
    } as unknown as Panel;
    const meters = render(PanelSchematic, {
      panel: tall,
      focus: ["meters"],
      targets: [{ section: "meters", item: 3, position: 1, lit: false }],
    });
    expect(screen.getByText("BATTERY")).toBeTruthy();
    expect(screen.getByText("DC METERS")).toBeTruthy();
    expect(screen.queryByText("METER DISPLAY")).toBeNull();
    expect(screen.queryByText("BAT")).toBeNull();
    meters.unmount();

    render(PanelSchematic, {
      panel: tall,
      focus: ["fctl"],
      targets: [{ section: "fctl", item: 0, position: 1, lit: false }],
    });
    expect(screen.getByText(/MACH/)).toBeTruthy();
  });

  it("labels a screen at 3.0's size, 12.6px, when the label fits across it", () => {
    // Screen labels were drawn at the light-text size, smaller than 3.0's.
    const panel = {
      id: "mn",
      title: "Main panel",
      width: 300,
      height: 200,
      sections: [
        {
          id: "cap",
          title: "Captain displays",
          x: 0,
          y: 0,
          width: 240,
          height: 160,
          items: [
            {
              kind: "display_unit",
              label: "INBOARD DU: ND AND ENGINES",
              x: 120,
              y: 80,
              width: 180,
              height: 110,
              controls: ["du"],
              description: "",
            },
          ],
        },
      ],
    } as unknown as Panel;
    render(PanelSchematic, {
      panel,
      focus: ["cap"],
      targets: [{ section: "cap", item: 0, position: null, lit: false }],
    });
    const label = screen.getByText("INBOARD DU: ND AND ENGINES") as unknown as SVGTextElement;
    // jsdom lays nothing out, so the drawing uses its default 1.6 pixels per unit.
    expect(Number.parseFloat(label.style.fontSize) * 1.6).toBeCloseTo(12.6, 1);
  });

  it("keeps full-size text when a tall section is cut above a dial with labels on top", () => {
    // The electrical meters section cut to the battery step: the dials' top labels (AUX BAT,
    // APU GEN) ran into the section title, so the whole section's text shrank to its minimum.
    const it = (kind: string, label: string, x: number, y: number, positions: string, extra = {}) =>
      ({ kind, label, x, y, controls: ["c"], positions, description: "", ...extra }) as unknown;
    const panel = {
      id: "oh",
      title: "Overhead",
      width: 300,
      height: 400,
      sections: [
        {
          id: "elecu",
          title: "Electrical meters",
          x: 0,
          y: 0,
          width: 225,
          height: 318,
          items: [
            it("display_unit", "METER DISPLAY", 112, 74, "", { width: 150, height: 60 }),
            it("selector", "DC METERS", 60, 206, "STBY PWR, BAT BUS, BAT, AUX BAT, TR1, TR2, TR3, TEST", {
              angles: [-135, -90, -45, 0, 45, 90, 135, 180],
            }),
            it("selector", "AC METERS", 168, 206, "STBY PWR, GRD PWR, GEN1, APU GEN, GEN2, INV, TEST", {
              angles: [-135, -90, -45, 0, 45, 90, 135],
            }),
            it("toggle_switch", "BATTERY", 60, 276, "OFF, ON", { guarded: true }),
            it("toggle_switch", "CAB/UTIL", 140, 276, "OFF, ON"),
            it("toggle_switch", "IFE/PASS SEAT", 196, 276, "OFF, ON"),
          ],
        },
      ],
    } as unknown as Panel;
    render(PanelSchematic, {
      panel,
      focus: ["elecu"],
      targets: [{ section: "elecu", item: 3, position: 1, lit: false }],
    });
    const aux = screen.getByText("AUX BAT") as unknown as SVGTextElement;
    // jsdom lays nothing out, so the drawing uses its default 1.6 pixels per unit.
    expect(Number.parseFloat(aux.style.fontSize) * 1.6).toBeCloseTo(10.8, 1);
  });
});
