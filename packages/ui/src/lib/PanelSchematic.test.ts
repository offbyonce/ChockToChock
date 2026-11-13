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
      targets: [{ section: "elec", item: 0, position: 1, lit: false, open: false, resolved: false }],
      guardClosed: true,
    });

    const svg = screen.getByRole("img");
    expect(svg.getAttribute("aria-label")).toBe("Electrical panel, schematic. This step: BATTERY set to ON.");
    // No "SET TO" caption (no board draws one): the target position's label is the one marked.
    expect(screen.queryByText(/SET TO/)).toBeNull();
    expect([...svg.querySelectorAll(".pos.target")].map((t) => t.textContent)).toEqual(["ON"]);
    // One knob at a position, for the battery; every other switch shows no position.
    expect(svg.querySelectorAll(".knob.set")).toHaveLength(1);
    expect(svg.querySelectorAll(".knob")).toHaveLength(1);
    // The battery is drawn changed: blue fill and outline.
    expect(svg.querySelectorAll(".body.changed")).toHaveLength(1);
  });

  it("highlights a control without a target when the step names no single position", () => {
    render(PanelSchematic, {
      panel,
      focus: ["elec"],
      targets: [{ section: "elec", item: 0, position: null, lit: false, open: false, resolved: false }],
    });

    expect(screen.queryByText(/SET TO/)).toBeNull();
    expect(screen.queryByText("SEE STEP")).toBeNull();
    expect(screen.getByRole("img").getAttribute("aria-label")).toBe(
      "Electrical panel, schematic. This step: BATTERY.",
    );
  });

  it("draws a control whose position the step leaves to a condition dashed, with no caption", () => {
    // The center fuel pumps sat dimmed at rest with no caption, so they read as OFF; then a
    // "SEE STEP" caption stood in, replaced by the dashed outline (decision 0022).
    render(PanelSchematic, {
      panel,
      focus: ["elec"],
      targets: [{ section: "elec", item: 0, position: null, lit: false, open: true, resolved: false }],
    });

    expect(screen.queryByText("SEE STEP")).toBeNull();
    expect(screen.queryByText(/SET TO/)).toBeNull();
    expect(screen.getByRole("img").querySelectorAll(".body.conditional")).toHaveLength(1);
  });

  it("shows no position for it, not even its resting one", () => {
    // Drawn at rest (OFF), the center fuel pumps read as set OFF.
    const rests = {
      ...panel,
      sections: panel.sections.map((s) =>
        s.id === "elec"
          ? { ...s, items: s.items.map((it, n) => (n === 0 ? { ...it, rest: "OFF" } : it)) }
          : s,
      ),
    };
    const { container } = render(PanelSchematic, {
      panel: rests,
      focus: ["elec"],
      targets: [{ section: "elec", item: 0, position: null, lit: false, open: true, resolved: false }],
    });

    expect(container.querySelector(".on .knob")).toBeNull();
    expect(container.querySelector(".on .pos.at-rest")).toBeNull();
    expect(container.querySelector(".on .body.conditional")).not.toBeNull();
  });

  it("draws neighboring small sections together in one drawing", () => {
    const targets = [
      { section: "lwip", item: 0, position: 1, lit: false, open: false, resolved: false },
      { section: "rwip", item: 0, position: 1, lit: false, open: false, resolved: false },
    ];
    render(PanelSchematic, { panel, focus: ["lwip", "rwip"], targets });

    const svg = screen.getByRole("img");
    expect(svg.getAttribute("aria-label")).toBe(
      "L wiper and R wiper panel, schematic. This step: L WIPER set to ON; R WIPER set to ON.",
    );
    // Each section's title and its control's label.
    expect(screen.getAllByText("L WIPER")).toHaveLength(2);
    expect(screen.getAllByText("R WIPER")).toHaveLength(2);
    expect(svg.querySelectorAll(".pos.target")).toHaveLength(2);
  });

  it("labels every selector position at its own tick, so the pointer and its label agree", () => {
    render(PanelSchematic, {
      panel: wiperPanel,
      focus: ["lwip"],
      targets: [{ section: "lwip", item: 0, position: 0, lit: false, open: false, resolved: false }],
    });

    const svg = screen.getByRole("img");
    for (const p of ["PARK", "INT", "LOW", "HIGH"]) expect(screen.getByText(p)).toBeTruthy();
    // PARK is the first position: far left, where the pointer points.
    expect(Number(screen.getByText("PARK").getAttribute("x"))).toBeLessThan(0);
    expect(Number(screen.getByText("HIGH").getAttribute("x"))).toBeGreaterThan(0);
    // 3.0's angle for the first of four positions.
    expect(svg.querySelector(".bar.set")?.getAttribute("transform")).toBe("rotate(-100)");
    expect(screen.getByText("PARK").classList.contains("target")).toBe(true);
  });

  it("grows a section's frame to hold the labels under its controls", () => {
    render(PanelSchematic, {
      panel: wiperPanel,
      focus: ["lwip"],
      targets: [{ section: "lwip", item: 0, position: 0, lit: false, open: false, resolved: false }],
    });

    const frame = screen.getByRole("img").querySelector(".frame");
    const name = screen.getAllByText("L WIPER").find((t) => t.classList.contains("label"));
    const frameBottom = Number(frame?.getAttribute("y")) + Number(frame?.getAttribute("height"));
    // The name sits in the control's group, which is placed at the control's y (50).
    expect(50 + Number(name?.getAttribute("y"))).toBeLessThan(frameBottom);
  });

  it("draws an unused control at its resting position, dimmed and unlike the target", () => {
    const rested = structuredClone(panel);
    const cab = rested.sections[0]?.items[1];
    if (cab) cab.rest = "OFF";
    render(PanelSchematic, {
      panel: rested,
      focus: ["elec"],
      targets: [{ section: "elec", item: 0, position: 1, lit: false, open: false, resolved: false }],
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
      width: 250,
      height: 100,
      sections: [
        {
          id: "lo",
          title: "Parking brake",
          x: 0,
          y: 0,
          width: 250,
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
      targets: [{ section: "lo", item: 0, position: null, lit: true, open: false, resolved: false }],
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
      targets: [{ section: "lo", item: 0, position: 1, lit: false, open: false, resolved: false }],
    });

    const grip = screen.getByRole("img").querySelector(".grip");
    const set = [...screen.getByRole("img").querySelectorAll(".pos")].find((t) => t.textContent === "SET");
    const gripMid = Number(grip?.getAttribute("y")) + Number(grip?.getAttribute("height")) / 2;
    // The label's baseline sits about 3 units below the position it names.
    expect(Math.abs(gripMid - (Number(set?.getAttribute("y")) - 3))).toBeLessThan(1);
  });

  it("draws every light and push button as one standard key size", () => {
    // MAINT (a push button) was drawn smaller than the BAT DISCHARGE, TR UNIT and ELEC lights.
    const key = (kind: string, label: string, x: number) =>
      ({
        kind,
        label,
        x,
        y: 40,
        controls: ["c"],
        color: kind === "light" ? "amber" : undefined,
        description: "",
      }) as unknown;
    const keys = {
      id: "oh",
      title: "Overhead",
      width: 300,
      height: 80,
      sections: [
        {
          id: "elec",
          title: "Electrical",
          x: 0,
          y: 0,
          width: 300,
          height: 80,
          items: [key("light", "TR UNIT", 50), key("light", "ELEC", 130), key("push_button", "MAINT", 220)],
        },
      ],
    } as unknown as Panel;
    render(PanelSchematic, { panel: keys, focus: ["elec"], targets: [] });

    const sizes = [
      ...screen.getByRole("img").querySelectorAll(".light rect.lamp, .push_button rect.body"),
    ].map((r) => `${r.getAttribute("width")}x${r.getAttribute("height")}`);
    expect(sizes).toHaveLength(3);
    expect(new Set(sizes).size).toBe(1);
  });

  it("draws every selector dial the same size, however many positions it has", () => {
    // The DC (8 positions) and AC (7) meters dials grew with their positions and looked oversized
    // and unequal; only their labels may move out.
    const dial = (label: string, x: number, positions: string) =>
      ({ kind: "selector", label, x, y: 60, controls: ["c"], positions, description: "" }) as unknown;
    const dials = {
      id: "oh",
      title: "Overhead",
      width: 300,
      height: 140,
      sections: [
        {
          id: "meters",
          title: "Meters",
          x: 0,
          y: 0,
          width: 300,
          height: 140,
          items: [
            dial("DC METERS", 60, "STBY PWR, BAT BUS, BAT, AUX BAT, TR1, TR2, TR3, TEST"),
            dial("AC METERS", 160, "STBY PWR, GRD PWR, GEN1, APU GEN, GEN2, INV, TEST"),
            dial("WIPER", 250, "PARK, INT, LOW, HIGH"),
          ],
        },
      ],
    } as unknown as Panel;
    render(PanelSchematic, { panel: dials, focus: ["meters"], targets: [] });

    const radii = [...screen.getByRole("img").querySelectorAll(".selector circle.body")].map((c) =>
      c.getAttribute("r"),
    );
    expect(radii).toHaveLength(3);
    expect(new Set(radii).size).toBe(1);
  });

  it("draws a tall section whole, never cut to the rows around the step", () => {
    // A drawing is one piece (decision 0021): the meters display and the flight control lights
    // were cut off when tall sections were cropped to the step's rows.
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
      targets: [{ section: "meters", item: 3, position: 1, lit: false, open: false, resolved: false }],
    });
    expect(screen.getByText("BATTERY")).toBeTruthy();
    expect(screen.getByText("DC METERS")).toBeTruthy();
    expect(screen.getByText("METER DISPLAY")).toBeTruthy();
    expect(screen.getByText(/DISCHARGE/)).toBeTruthy();
    meters.unmount();

    render(PanelSchematic, {
      panel: tall,
      focus: ["fctl"],
      targets: [{ section: "fctl", item: 0, position: 1, lit: false, open: false, resolved: false }],
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
      targets: [{ section: "cap", item: 0, position: null, lit: false, open: false, resolved: false }],
    });
    const label = screen.getByText("INBOARD DU: ND AND ENGINES") as unknown as SVGTextElement;
    // Every drawing uses one scale, 1.8 pixels per unit.
    expect(Number.parseFloat(label.style.fontSize) * 1.8).toBeCloseTo(12.6, 1);
  });

  it("keeps position labels at 11 px above a dial in a tall section", () => {
    // The dials' top labels (AUX BAT, APU GEN) ran into the section title, so the whole section's
    // text shrank; position labels are now at the 11 px minimum (decision 0021).
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
      targets: [{ section: "elecu", item: 3, position: 1, lit: false, open: false, resolved: false }],
    });
    const aux = screen.getByText("AUX BAT") as unknown as SVGTextElement;
    // Laid out at 1.8 pixels per unit.
    expect(Number.parseFloat(aux.style.fontSize) * 1.8).toBeCloseTo(11, 1);
  });

  it("draws every panel at one scale, so controls and text are the same size in every drawing", () => {
    // Drawings used to scale to fit their card, so the same button came out larger in one panel
    // than in another.
    const panel = (width: number) =>
      ({
        id: "p",
        title: "P",
        width: width + 20,
        height: 100,
        sections: [
          {
            id: "s",
            title: "S",
            x: 0,
            y: 0,
            width,
            height: 80,
            items: [{ kind: "push_button", label: "VNAV", x: 40, y: 50, controls: ["c"], description: "" }],
          },
        ],
      }) as unknown as Panel;
    const sizeOf = (width: number) => {
      const { container, unmount } = render(PanelSchematic, {
        panel: panel(width),
        focus: ["s"],
        targets: [{ section: "s", item: 0, position: null, lit: false, open: false, resolved: false }],
      });
      const svg = container.querySelector("svg") as SVGSVGElement;
      const [, , w] = (svg.getAttribute("viewBox") ?? "").split(" ").map(Number);
      const px = Number.parseFloat(svg.style.width) / (w ?? 1);
      unmount();
      return px;
    };
    expect(sizeOf(120)).toBeCloseTo(1.8, 2);
    expect(sizeOf(300)).toBeCloseTo(1.8, 2);
  });

  it("draws a wide section in one piece, never split into parts", () => {
    // The MCP was cut into three parts drawn one under another, which changed the panel layout.
    const item = (label: string, x: number) =>
      ({ kind: "push_button", label, x, y: 40, controls: ["c"], description: "" }) as unknown;
    const panel = {
      id: "gl",
      title: "Glareshield",
      width: 700,
      height: 100,
      sections: [
        {
          id: "mcp",
          title: "Mode control panel",
          x: 0,
          y: 0,
          width: 660,
          height: 80,
          items: [item("VNAV", 60), item("APP", 600)],
        },
      ],
    } as unknown as Panel;
    const { container } = render(PanelSchematic, {
      panel,
      focus: ["mcp"],
      targets: [
        { section: "mcp", item: 0, position: null, lit: false, open: false, resolved: false },
        { section: "mcp", item: 1, position: null, lit: false, open: false, resolved: false },
      ],
    });
    const svgs = [...container.querySelectorAll("svg.schematic")];
    expect(svgs).toHaveLength(1);
    // The card's caption names a lone section, so the drawing repeats no title.
    expect(svgs[0]?.querySelectorAll(".title")).toHaveLength(0);
    expect(svgs[0]?.textContent).toContain("VNAV");
    expect(svgs[0]?.textContent).toContain("APP");
  });

  it("rests a step's switch where the pack says when the step leaves its position open", () => {
    // The center fuel pumps showed their knob midway, at a position that does not exist.
    const panel = {
      id: "oh",
      title: "Overhead",
      width: 200,
      height: 100,
      sections: [
        {
          id: "fuel",
          title: "Fuel",
          x: 0,
          y: 0,
          width: 180,
          height: 90,
          items: [
            {
              kind: "toggle_switch",
              label: "CTR L",
              x: 60,
              y: 50,
              controls: ["c"],
              positions: "OFF, ON",
              rest: "OFF",
              description: "",
            },
          ],
        },
      ],
    } as unknown as Panel;
    const { container } = render(PanelSchematic, {
      panel,
      focus: ["fuel"],
      targets: [{ section: "fuel", item: 0, position: null, lit: false, open: false, resolved: false }],
    });
    const knob = container.querySelector(".knob.rest");
    expect(knob).not.toBeNull();
    expect(container.querySelector(".knob.unknown")).toBeNull();
  });

  it("draws the whole section, not only the controls the step uses", () => {
    // V/S, CMD A, CWS A and the disengage bar vanished from the MCP when the step used none.
    const item = (label: string, x: number) =>
      ({ kind: "push_button", label, x, y: 40, controls: ["c"], description: "" }) as unknown;
    const panel = {
      id: "gl",
      title: "Glareshield",
      width: 800,
      height: 100,
      sections: [
        {
          id: "mcp",
          title: "Mode control panel",
          x: 0,
          y: 0,
          width: 760,
          height: 80,
          items: [item("VNAV", 60), item("V/S", 700)],
        },
      ],
    } as unknown as Panel;
    render(PanelSchematic, {
      panel,
      focus: ["mcp"],
      targets: [{ section: "mcp", item: 0, position: null, lit: false, open: false, resolved: false }],
    });
    expect(screen.getByText("V/S")).toBeTruthy();
  });

  it("draws a wide section's far control in the same piece", () => {
    // The captain displays' inboard display fell below the outboard one when every part was drawn.
    const item = (label: string, x: number) =>
      ({ kind: "push_button", label, x, y: 40, controls: ["c"], description: "" }) as unknown;
    const panel = {
      id: "mn",
      title: "Main panel",
      width: 800,
      height: 100,
      sections: [
        {
          id: "cd",
          title: "Captain displays",
          x: 0,
          y: 0,
          width: 760,
          height: 80,
          items: [item("OUTBD", 60), item("INBD", 700)],
        },
      ],
    } as unknown as Panel;
    const { container } = render(PanelSchematic, {
      panel,
      focus: ["cd"],
      targets: [{ section: "cd", item: 1, position: null, lit: false, open: false, resolved: false }],
    });
    expect(container.querySelector("svg.schematic")?.textContent).toContain("INBD");
  });

  it("draws the sections between two far-apart step sections, so the piece is never empty", () => {
    // TERR: the two EFIS panels at either end of the glareshield, with the MCP (taller than a
    // context neighbor) between them; the MCP was left out and the drawing opened on empty panel.
    const item = (label: string) => ({
      kind: "push_button",
      label,
      x: 40,
      y: 40,
      controls: ["c"],
      description: "",
    });
    const panel = {
      id: "gl",
      title: "Glareshield",
      width: 1000,
      height: 260,
      sections: [
        { id: "efisl", title: "EFIS L", x: 0, y: 0, width: 200, height: 120, items: [item("TERR")] },
        { id: "mcp", title: "MCP", x: 220, y: 0, width: 560, height: 228, items: [item("VNAV")] },
        { id: "efisr", title: "EFIS R", x: 800, y: 0, width: 200, height: 120, items: [item("TERR")] },
      ],
    } as unknown as Panel;
    const { container } = render(PanelSchematic, {
      panel,
      focus: ["efisl", "efisr"],
      targets: [
        { section: "efisl", item: 0, position: null, lit: false, open: false, resolved: false },
        { section: "efisr", item: 0, position: null, lit: false, open: false, resolved: false },
      ],
    });
    expect(container.querySelectorAll("svg.schematic")).toHaveLength(1);
    expect(container.textContent).toContain("VNAV");
  });

  it("draws a decided control resolved: at its position, as changed", () => {
    render(PanelSchematic, {
      panel,
      focus: ["elec"],
      targets: [{ section: "elec", item: 0, position: 0, lit: false, open: false, resolved: true }],
    });
    const svg = screen.getByRole("img");
    expect(svg.querySelectorAll(".resolved")).toHaveLength(1);
    expect(svg.querySelectorAll(".resolved .knob.set")).toHaveLength(1);
    // The set position's label carries the decision's diamond.
    expect(screen.getByText("◆ OFF").classList.contains("target")).toBe(true);
    expect(svg.querySelector(".conditional")).toBeNull();
  });

  it("draws no SET TO caption under any control, as no board does", () => {
    // The pane's fuel drawing grew past its card with a caption under every pump switch.
    render(PanelSchematic, {
      panel,
      focus: ["elec", "lwip"],
      targets: [
        { section: "elec", item: 0, position: 1, lit: false, open: false, resolved: false },
        { section: "lwip", item: 0, position: 1, lit: false, open: false, resolved: true },
      ],
    });
    expect(screen.queryByText(/SET TO/)).toBeNull();
    expect(screen.getByRole("img").querySelectorAll(".set-to")).toHaveLength(0);
  });

  it("draws a resting selector's pointing end, so it reads one way", () => {
    // SYS DSPL rested as a bar across the knob, pointing at L and R at once.
    const rested = structuredClone(wiperPanel);
    const wiper = rested.sections[0]?.items[0];
    if (wiper) wiper.rest = "PARK";
    render(PanelSchematic, { panel: rested, focus: ["lwip"], targets: [] });
    const svg = screen.getByRole("img");
    expect(svg.querySelectorAll(".bar.rest")).toHaveLength(1);
    expect(svg.querySelectorAll(".tip.rest")).toHaveLength(1);
  });

  it("opens the larger view on the step's controls, not the top of the panel", () => {
    // The fuel pumps sit at the foot of their panel; the larger view opened at its top.
    const item = (kind: string, label: string, x: number, y: number) =>
      ({ kind, label, x, y, controls: ["c"], positions: "OFF, ON", description: "" }) as unknown;
    const tall = {
      id: "oh",
      title: "Overhead",
      width: 300,
      height: 900,
      sections: [
        {
          id: "fuel",
          title: "Fuel",
          x: 0,
          y: 0,
          width: 240,
          height: 880,
          items: [item("light", "FILTER BYPASS", 60, 60), item("toggle_switch", "AFT 1", 60, 840)],
        },
      ],
    } as unknown as Panel;
    const frames: FrameRequestCallback[] = [];
    const raf = globalThis.requestAnimationFrame;
    globalThis.requestAnimationFrame = (cb) => (frames.push(cb), frames.length);
    const proto = HTMLDialogElement.prototype as unknown as { showModal: () => void };
    const showModal = proto.showModal;
    proto.showModal = function (this: HTMLDialogElement) {
      this.setAttribute("open", "");
    };
    try {
      render(PanelSchematic, {
        panel: tall,
        focus: ["fuel"],
        targets: [{ section: "fuel", item: 1, position: 1, lit: false, open: false, resolved: false }],
        around: true,
      });
      const sheet = document.querySelector("dialog.sheet") as HTMLDialogElement;
      const box = (top: number, height: number) =>
        ({
          top,
          bottom: top + height,
          height,
          left: 0,
          right: 0,
          width: 0,
          x: 0,
          y: top,
          toJSON: () => ({}),
        }) as DOMRect;
      sheet.getBoundingClientRect = () => box(0, 800);
      for (const g of sheet.querySelectorAll("g.item.on"))
        (g as Element).getBoundingClientRect = () => box(1500, 100);
      screen.getByRole("button", { name: "See the whole panel larger" }).click();
      for (const cb of frames.splice(0)) cb(0);
      expect(sheet.hasAttribute("open")).toBe(true);
      // Titled as the pane card's caption.
      expect(sheet.getAttribute("aria-label")).toBe("Fuel panel");
      expect(sheet.querySelector(".sheet-head span")?.textContent).toBe("Fuel panel");
      expect(sheet.scrollTop).toBe(1550 - 400);
    } finally {
      globalThis.requestAnimationFrame = raf;
      proto.showModal = showModal;
    }
  });

  it("draws the pane's one-section drawing without a frame, and a used screen as a bezel, not a blue box", () => {
    // Flow's display unit read as one large blue rectangle, with its cut frame as stray lines.
    const du = {
      id: "mn",
      title: "Main panel",
      width: 300,
      height: 200,
      sections: [
        {
          id: "cd",
          title: "Captain displays",
          x: 0,
          y: 0,
          width: 300,
          height: 200,
          items: [
            {
              kind: "display_unit",
              label: "INBOARD DU",
              x: 150,
              y: 100,
              width: 120,
              height: 90,
              controls: ["c"],
              description: "",
            },
          ],
        },
      ],
    } as unknown as Panel;
    const targets = [{ section: "cd", item: 0, position: null, lit: false, open: false, resolved: false }];
    const pane = render(PanelSchematic, { panel: du, focus: ["cd"], targets, around: true });
    const svg = screen.getByRole("img");
    expect(svg.querySelector(".frame")?.classList.contains("bare")).toBe(true);
    expect(svg.querySelectorAll(".screen.changed")).toHaveLength(1);
    expect(svg.querySelectorAll(".display")).toHaveLength(1);
    pane.unmount();
    render(PanelSchematic, { panel: du, focus: ["cd"], targets });
    expect(screen.getByRole("img").querySelector(".frame")?.classList.contains("bare")).toBe(false);
  });

  it("draws a guard shut over its switch, or raised clear of it on its hinge, so they read apart", () => {
    // Shut and open guards differed only by a line along the base.
    const t = { section: "elec", item: 0, position: 1, lit: false, open: false, resolved: false };
    const cover = () =>
      [...screen.getByRole("img").querySelectorAll(".cover")].map((c) =>
        [...c.classList].filter((n) => !n.startsWith("svelte-")).join(" "),
      );
    const resting = render(PanelSchematic, { panel, focus: ["elec"], targets: [] });
    expect(cover()).toEqual(["cover shut"]);
    resting.unmount();
    const opened = render(PanelSchematic, { panel, focus: ["elec"], targets: [t] });
    expect(cover()).toEqual(["cover raised step"]);
    opened.unmount();
    render(PanelSchematic, { panel, focus: ["elec"], targets: [t], guardClosed: true });
    expect(cover()).toEqual(["cover shut step"]);
  });
});
