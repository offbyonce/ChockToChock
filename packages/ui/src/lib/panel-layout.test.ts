// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import type { PanelItem, PanelSection } from "@chocktochock/bindings";
import { describe, expect, it } from "vitest";
import { knobY, layoutSection, nameFit, TOGGLE } from "./panel-layout";
import { em, widest } from "./text-width";

const lever = (label: string, x: number) =>
  ({
    kind: "toggle_switch",
    label,
    x,
    y: 40,
    controls: [],
    positions: "IDLE, CUTOFF",
    description: "",
  }) as PanelItem;

// Close-set controls, like the start levers beside the parking brake.
const section = {
  id: "lo",
  title: "Parking brake and start levers",
  x: 0,
  y: 0,
  width: 200,
  height: 90,
  items: [
    lever("PARKING BRAKE", 30),
    lever("START LEVER 1", 90),
    lever("START LEVER 2", 120),
    lever("STAB TRIM PRI", 170),
  ],
} as unknown as PanelSection;

describe("nameFit", () => {
  it("keeps every name within the room to its neighbors, so names never overlap", () => {
    for (const k of [1, 1.6, 2.4]) {
      for (const item of section.items) {
        const others = section.items.filter((o) => o !== item).map((o) => Math.abs(o.x - item.x));
        const room = (Math.min(...others) - 4) * k;
        const fit = nameFit(item, section, k);
        const longest = widest(fit.lines);
        // At the smallest size a very long word may still need more room; it then shrinks to it.
        expect(longest * fit.px, `${item.label} at ${k}`).toBeLessThanOrEqual(Math.max(room, longest * 8.5));
      }
    }
  });

  it("leaves a name at full size on one line when there is room", () => {
    const roomy = {
      ...section,
      width: 600,
      items: [lever("BATTERY", 100), lever("CAB/UTIL", 300)],
    } as PanelSection;
    expect(nameFit(roomy.items[0] as PanelItem, roomy, 1.6)).toEqual({ lines: ["BATTERY"], px: 11.8 });
  });

  it("never calls a layout clear while names overlap, even with a button set between two levers", () => {
    const crowded = {
      ...section,
      items: [
        ...section.items,
        {
          kind: "push_button",
          label: "HORN CUTOUT",
          x: 105,
          y: 58,
          controls: [],
          description: "",
        } as PanelItem,
      ],
    } as PanelSection;
    for (const k of [1.2, 1.8]) {
      const all = crowded.items.map((_, i) => i);
      const { items, clear } = layoutSection(crowded, all, () => ({ on: false, t: null, rest: -1 }), k, 0);
      const boxes = all.map((i) => {
        const it = crowded.items[i] as PanelItem;
        const n = items.get(i)?.name;
        if (!n) return null;
        const line = (n.px * 1.15) / k;
        const w = Math.max(...n.lines.map((l) => l.length)) * 0.64 * (n.px / k);
        const top = it.y + n.y - line * 0.82;
        return { x0: it.x - w / 2, x1: it.x + w / 2, y0: top, y1: top + n.lines.length * line };
      });
      for (const [a, ba] of boxes.entries()) {
        for (const [b, bb] of boxes.entries()) {
          if (a >= b || !ba || !bb) continue;
          const hit = ba.x0 < bb.x1 && bb.x0 < ba.x1 && ba.y0 < bb.y1 && bb.y0 < ba.y1;
          // Text never shrinks, so a crowded layout overlaps; it must then say it is not clear.
          if (hit)
            expect(clear, `${crowded.items[a]?.label} and ${crowded.items[b]?.label} at ${k}`).toBe(false);
        }
      }
    }
  });

  it("keeps every label on a crowded section at full size, and says when they collide", () => {
    const all = section.items.map((_, i) => i);
    const out = layoutSection(section, all, () => ({ on: false, t: null, rest: 0 }), 1.6, 0);
    for (const i of all) {
      const it = section.items[i] as PanelItem;
      expect(
        out.items.get(i)?.positions.map((l) => l.n),
        it.label,
      ).toEqual([0, 1]);
      expect(out.items.get(i)?.name?.lines.join(" "), it.label).toBe(it.label);
    }
    // Text never shrinks: every label in every panel is the same size.
    expect(out.scale).toBe(1);
    // A roomy section places every label clear.
    const roomy = {
      ...section,
      width: 600,
      items: [lever("BATTERY", 100), lever("CAB/UTIL", 300)],
    } as PanelSection;
    expect(layoutSection(roomy, [0, 1], () => ({ on: false, t: null, rest: 0 }), 1.6, 0).clear).toBe(true);
  });

  it("keeps full-size labels on a lone selector: its own outline does not block its labels", () => {
    const wiper = {
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
          controls: [],
          positions: "PARK, INT, LOW, HIGH",
          description: "",
        },
      ],
    } as unknown as PanelSection;
    const out = layoutSection(wiper, [0], () => ({ on: true, t: 0, rest: -1 }), 1.75, 0);
    expect(out.clear).toBe(true);
    expect(out.items.get(0)?.positions.map((l) => l.n)).toEqual([0, 1, 2, 3]);
  });
});

describe("text width", () => {
  it("counts wide capitals as wide, so DOWN takes more room than ARM by its real share", () => {
    // Measured from the bundled Geist SemiBold: a flat 0.64 per letter made DOWN 2.56 ems.
    expect(em("DOWN")).toBeGreaterThan(3.1);
    expect(em("I")).toBeLessThan(0.3);
  });

  it("keeps a switch's position labels clear of its neighbor's, as with ARM beside DOWN", () => {
    const toggle = (label: string, x: number, y: number, positions: string, guarded = false) =>
      ({
        kind: "toggle_switch",
        label,
        x,
        y,
        guarded,
        controls: [],
        positions,
        description: "",
      }) as PanelItem;
    // The alternate flaps master switch and the alternate flaps switch where the pack puts them,
    // near the right edge of the flight control section: DOWN goes left, beside ARM.
    const flaps = {
      id: "af",
      title: "Flight control",
      x: 0,
      y: 0,
      width: 235,
      height: 322,
      items: [
        toggle("ALT FLAPS MASTER", 150, 132, "OFF, ARM", true),
        toggle("ALT FLAPS", 205, 134, "UP, OFF, DOWN"),
      ],
    } as unknown as PanelSection;
    for (const k of [1.6, 2, 2.4]) {
      const layout = layoutSection(flaps, [0, 1], () => ({ on: true, t: 0, rest: 0 }), k, 0);
      const boxes = [0, 1].flatMap((i) => {
        const it = flaps.items[i] as PanelItem;
        const pos = (it.positions ?? "").split(", ");
        return (layout.items.get(i)?.positions ?? []).map((l) => {
          const w = (em(pos[l.n] ?? "") * 10.8 * layout.scale) / k;
          const x0 = it.x + (l.anchor === "start" ? l.x : l.anchor === "end" ? l.x - w : l.x - w / 2);
          const h = (10.8 * layout.scale) / k;
          return { i, x0, x1: x0 + w, y0: it.y + l.y - h, y1: it.y + l.y };
        });
      });
      for (const a of boxes)
        for (const b of boxes)
          if (a.i < b.i)
            expect(a.x0 < b.x1 && b.x0 < a.x1 && a.y0 < b.y1 && b.y0 < a.y1, `labels overlap at ${k}`).toBe(
              false,
            );
    }
  });
});

describe("knobY", () => {
  it("sits a toggle's knob on its track's round ends, clear of the outline, as 3.0 does", () => {
    // At the old 9.5 the knob ran into the outline; 3.0 centers it on the end's curve.
    const end = TOGGLE.h / 2 - TOGGLE.w / 2;
    const stroke = 1.1 / 2;
    for (const n of [2, 3]) {
      expect(knobY(0, n)).toBeCloseTo(-end);
      expect(knobY(n - 1, n)).toBeCloseTo(end);
    }
    expect(TOGGLE.knob + stroke).toBeLessThan(TOGGLE.w / 2 - 0.5);
  });
});

describe("selector angles", () => {
  it("labels every meter position at full text size, at the dial's real angles, none overlapping", () => {
    // The electrical meters section as the pack places it, with each dial's real geometry:
    // STBY PWR lower left, over the top, TEST at the bottom (DC) or lower right (AC).
    const item = (kind: string, label: string, x: number, y: number, positions: string) =>
      ({ kind, label, x, y, controls: [], positions, description: "" }) as unknown as PanelItem;
    const meters = {
      id: "elecu",
      title: "Electrical meters",
      x: 0,
      y: 0,
      width: 225,
      height: 318,
      items: [
        {
          ...item("selector", "DC METERS", 60, 206, "STBY PWR, BAT BUS, BAT, AUX BAT, TR1, TR2, TR3, TEST"),
          angles: [-135, -90, -45, 0, 45, 90, 135, 180],
        },
        {
          ...item("selector", "AC METERS", 168, 206, "STBY PWR, GRD PWR, GEN1, APU GEN, GEN2, INV, TEST"),
          angles: [-135, -90, -45, 0, 45, 90, 135],
        },
        { ...item("toggle_switch", "BATTERY", 60, 276, "OFF, ON"), guarded: true },
        item("toggle_switch", "CAB/UTIL", 140, 276, "OFF, ON"),
        item("toggle_switch", "IFE/PASS SEAT", 196, 276, "OFF, ON"),
      ],
    } as unknown as PanelSection;
    const target = (i: number) => (i === 2 ? { on: true, t: 1, rest: 0 } : { on: false, t: null, rest: 0 });
    for (const k of [1.4, 1.8]) {
      const layout = layoutSection(meters, [0, 1, 2, 3, 4], target, k, 150);
      expect(layout.clear, `at ${k}`).toBe(true);
      expect(layout.items.get(0)?.positions).toHaveLength(8);
      expect(layout.items.get(1)?.positions).toHaveLength(7);
    }
  });
});
