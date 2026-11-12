// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Every section of every panel in the bundled packs lays out cleanly: no control's shape runs
// into the section's title, another control or the frame, and all text keeps 3.0's size. Reads
// the flattened packs that `just web-packs` writes (`just test` runs it first).

import type { Panel, PanelItem, PanelSection } from "@chocktochock/bindings";
import { describe, expect, it } from "vitest";
import { half, halfWidth, LAMP, layoutSection, PX } from "./panel-layout";
import { em } from "./text-width";

/** The flattened packs, by file; the pack index file has no `content` and is skipped below. */
const files = import.meta.glob<{ content?: { panels?: Panel[] } }>(
  "../../../../apps/web/public/packs/*.json",
  {
    eager: true,
    import: "default",
  },
);

type Box = { x0: number; y0: number; x1: number; y1: number };
const overlaps = (a: Box, b: Box) => a.x0 < b.x1 && b.x0 < a.x1 && a.y0 < b.y1 && b.y0 < a.y1;

/** A control's shape, as the drawing sizes it. */
function shape(item: PanelItem): Box {
  const lamp = item.kind === "light";
  const hw = lamp ? LAMP.w / 2 : halfWidth(item);
  const h = lamp ? LAMP.h / 2 : half(item);
  return { x0: item.x - hw, y0: item.y - h, x1: item.x + hw, y1: item.y + h };
}

/** What is wrong with a section's layout at `k` pixels per unit, if anything. */
function problems(s: PanelSection, k: number): string[] {
  const out: string[] = [];
  const title = { x0: 6, y0: 2, x1: 8 + em(s.title.toUpperCase()) * (PX.title / k), y1: 16 };
  s.items.forEach((item, i) => {
    const b = shape(item);
    if (overlaps(b, title)) out.push(`${item.label} runs into the title`);
    if (b.x0 < 2 || b.x1 > s.width - 2 || b.y0 < 2 || b.y1 > s.height - 2)
      out.push(`${item.label} is outside the frame`);
    s.items.forEach((other, j) => {
      if (j > i && overlaps(b, shape(other))) out.push(`${item.label} overlaps ${other.label}`);
    });
  });
  const all = s.items.map((_, i) => i);
  const none = layoutSection(s, all, () => ({ on: false, t: null, rest: -1 }), k, 0);
  if (none.scale < 0.999) out.push(`text shrinks to ${none.scale.toFixed(2)}`);
  s.items.forEach((item, i) => {
    const target = (j: number) => (j === i ? { on: true, t: 0, rest: -1 } : { on: false, t: null, rest: -1 });
    const one = layoutSection(s, all, target, k, 0);
    if (one.scale < 0.999) out.push(`text shrinks to ${one.scale.toFixed(2)} with ${item.label} targeted`);
  });
  return [...new Set(out)];
}

const packs = Object.entries(files)
  .map(([file, pack]) => ({ file: file.split("/").pop() ?? file, panels: pack.content?.panels ?? [] }))
  .filter((p) => p.panels.length > 0);

describe("bundled pack panels", () => {
  it("are built (run `just web-packs` first)", () => {
    expect(packs.length).toBeGreaterThan(0);
  });

  for (const { file, panels } of packs) {
    for (const panel of panels) {
      for (const section of panel.sections) {
        it(`${file} ${panel.id}/${section.id} lays out cleanly`, () => {
          const found = [1.4, 1.8].flatMap((k) => problems(section, k).map((p) => `at ${k}: ${p}`));
          expect(found).toEqual([]);
        });
      }
    }
  }
});
