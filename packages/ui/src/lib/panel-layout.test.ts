// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import type { PanelItem, PanelSection } from "@chocktochock/bindings";
import { describe, expect, it } from "vitest";
import { layoutSection, nameFit } from "./panel-layout";

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
        const longest = Math.max(...fit.lines.map((l) => l.length));
        // At the smallest size a very long word may still need more room; it then shrinks to it.
        expect(longest * 0.64 * fit.px, `${item.label} at ${k}`).toBeLessThanOrEqual(
          Math.max(room, longest * 0.64 * 8.5),
        );
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

  it("places names so none overlaps another, even with a button set between two levers", () => {
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
    for (const k of [1.2, 1.6]) {
      const all = crowded.items.map((_, i) => i);
      const { items } = layoutSection(crowded, all, () => ({ on: false, t: null, rest: -1 }), k, 0);
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
          expect(hit, `${crowded.items[a]?.label} and ${crowded.items[b]?.label} at ${k}`).toBe(false);
        }
      }
    }
  });

  it("keeps every label on a crowded section, shrinking the text rather than dropping any", () => {
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
    expect(out.scale).toBeLessThanOrEqual(1);
    expect(out.scale).toBeGreaterThanOrEqual(0.62);
    // A roomy section keeps 3.0's full sizes.
    const roomy = {
      ...section,
      width: 600,
      items: [lever("BATTERY", 100), lever("CAB/UTIL", 300)],
    } as PanelSection;
    expect(layoutSection(roomy, [0, 1], () => ({ on: false, t: null, rest: 0 }), 1.6, 0).scale).toBe(1);
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
    expect(out.scale).toBe(1);
    expect(out.items.get(0)?.positions.map((l) => l.n)).toEqual([0, 1, 2, 3]);
  });
});
