// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Layout shared by the panel drawing (PanelSchematic) and its sections (PanelSection): glyph
// sizes, label lines and how far labels reach, so a section's frame always holds its labels.
// Drawing units are the pack's own; text keeps fixed pixel sizes, so its size in units depends
// on `k`, the screen pixels per unit.

import type { PanelItem, PanelSection } from "@chocktochock/bindings";
import { em, widest } from "./text-width";

/**
 * 3.0's rendered text sizes, in pixels: names and titles 11.8, positions and light words 10.8.
 * "SET TO …" sits under the name, a little smaller so the name reads first. A screen's label is
 * 3.0's measured 12.6, its name size at the scale 3.0 draws a screen in the step pane.
 */
export const PX = { name: 11.8, title: 11.8, pos: 10.8, lamp: 10.8, setTo: 9.5, screen: 12.6 } as const;
/** Line height for labels under a control, as a multiple of the text size. */
const LINE = 1.15;

export const positionsOf = (item: PanelItem): string[] =>
  (item.positions ?? "")
    .split(",")
    .map((p) => p.trim())
    .filter(Boolean);

/**
 * A toggle switch in 3.0's proportions: a slim track `w` wide and `h` tall with round ends, a red
 * guard `guard` outside it, and position labels `gap` beyond the guard or track.
 */
export const TOGGLE = { w: 7, h: 22, guard: 2.8, gap: 3.6, knob: 2.3 } as const;
/** How far a toggle reaches either side of its center, guard included. */
const toggleHalfWidth = (guarded: boolean) => TOGGLE.w / 2 + (guarded ? TOGGLE.guard : 0);

/** Half the height a glyph takes, so its label sits under it. */
export function half(item: PanelItem): number {
  switch (item.kind) {
    case "toggle_switch":
      return TOGGLE.h / 2 + (item.guarded ? TOGGLE.guard : 0);
    case "selector":
    case "gauge":
      return 15;
    case "push_button":
      return 11;
    case "light":
      return 10;
    case "lever":
      return 18;
    case "handle":
      return 12;
    default:
      return (item.height ?? 30) / 2;
  }
}

/** Every light is the same size, as in 3.0, so lights side by side line up. */
export const LAMP = { w: 50, h: 20 } as const;

/**
 * A light's words wrapped to its fixed width, up to three lines as 3.0 does, and their size in
 * pixels: 3.0's size, smaller only when a single word would not fit across the light.
 */
export function lampText(item: PanelItem, k: number, px: number): { lines: string[]; px: number } {
  const room = (LAMP.w - 4) * k;
  const words = item.label.split(" ");
  const size = Math.min(px, room / widest(words));
  const out: string[] = [];
  for (const w of words) {
    const last = out[out.length - 1];
    if (last !== undefined && em(`${last} ${w}`) * size <= room) out[out.length - 1] = `${last} ${w}`;
    else out.push(w);
  }
  return { lines: out.slice(0, 3), px: size };
}

/** A lever's knob position along its slot: first position at the top, last at the bottom. */
export const slotY = (index: number, count: number) => (count < 2 ? 0 : -13 + (26 * index) / (count - 1));

/** Long labels go on two lines, split near the middle. */
export function lines(label: string, max = 12): string[] {
  if (label.length <= max) return [label];
  const words = label.split(" ");
  if (words.length < 2) return [label];
  let best = 1;
  let bestDiff = Number.POSITIVE_INFINITY;
  for (let i = 1; i < words.length; i++) {
    const diff = Math.abs(words.slice(0, i).join(" ").length - words.slice(i).join(" ").length);
    if (diff < bestDiff) [best, bestDiff] = [i, diff];
  }
  return [words.slice(0, best).join(" "), words.slice(best).join(" ")];
}

/** Whether a control has its name written under it (lights and screens carry it inside). */
export const labelled = (item: PanelItem) =>
  item.kind !== "light" && item.kind !== "readout" && item.kind !== "display_unit";

/** The smallest a name may shrink to, in pixels, when its neighbors leave little room. */
const MIN_NAME = 8.5;

/**
 * How a control's name fits under it: the room is the distance to the nearest control in the same
 * row (or the section's edge), so names never run into each other. One line if it fits, two if
 * that fits, and only then smaller, never below MIN_NAME pixels.
 */
export function nameFit(item: PanelItem, section: PanelSection, k: number): { lines: string[]; px: number } {
  const row = section.items.filter((o) => o !== item && Math.abs(o.y - item.y) < 30 && labelled(o));
  const gap = Math.min(
    ...row.map((o) => Math.abs(o.x - item.x)),
    2 * Math.min(item.x, section.width - item.x),
  );
  const room = Math.max(10, gap - 4) * k;
  const width = (text: string[], px: number) => widest(text) * px;
  const one = [item.label];
  if (width(one, PX.name) <= room) return { lines: one, px: PX.name };
  const two = lines(item.label, 1);
  const best = width(two, PX.name) < width(one, PX.name) ? two : one;
  return { lines: best, px: Math.max(MIN_NAME, Math.min(PX.name, room / widest(best))) };
}

/** The baseline of label line `n` (0 is the name's first line) under a control, in units. */
export const labelY = (item: PanelItem, n: number, k: number, px: number = PX.name) =>
  half(item) + 4 + (n + 1) * ((px * LINE) / k);

/** How far below the section's top a control and its labels reach, in units. */
export function bottomOf(item: PanelItem, section: PanelSection, hasTarget: boolean, k: number): number {
  if (!labelled(item)) return item.y + half(item);
  const fit = nameFit(item, section, k);
  const count = fit.lines.length + (hasTarget ? 1 : 0);
  return item.y + labelY(item, count - 1, k, fit.px) + (PX.name * 0.35) / k;
}

/**
 * A selector's tick angle, in degrees from straight up, clockwise, from left (first) to right
 * (last), as a rotary selector is read. 3.0's angles: two to five positions at set places, more
 * spread over 250 degrees, so the end positions sit low at the sides, close to the knob.
 */
const ANGLES: Record<number, number[]> = {
  2: [-90, 90],
  3: [-60, 0, 60],
  4: [-100, -35, 35, 100],
  5: [-110, -55, 0, 55, 110],
};
export const angle = (index: number, count: number) =>
  count < 2 ? 0 : (ANGLES[count]?.[index] ?? -125 + (250 * index) / (count - 1));

/** A selector position's angle: the pack's, as on the panel, or else spread over the top. */
export const angleOf = (item: PanelItem, index: number) =>
  item.angles?.[index] ?? angle(index, positionsOf(item).length);

/** Which selector positions get a label: all of up to four; otherwise the ends and the target. */
export const labelledPositions = (count: number, target: number | null): number[] =>
  count <= 4
    ? [...Array(count).keys()]
    : [...new Set([0, count - 1, ...(target == null ? [] : [target])])].sort((a, b) => a - b);

/** A rectangle in a section's own units. */
type Box = { x0: number; y0: number; x1: number; y1: number };
const overlaps = (a: Box, b: Box) => a.x0 < b.x1 && b.x0 < a.x1 && a.y0 < b.y1 && b.y0 < a.y1;

/** How wide a control's shape is, either side of its center. */
export function halfWidth(item: PanelItem): number {
  switch (item.kind) {
    case "toggle_switch":
      return toggleHalfWidth(!!item.guarded);
    case "selector":
      return 15.5;
    case "push_button":
      return item.guarded ? 14 : 11;
    case "light":
      return 19;
    case "lever":
      return 8;
    case "handle":
      return 13;
    case "gauge":
      return 14;
    default:
      return (item.width ?? 60) / 2;
  }
}

/**
 * Where a toggle's knob sits, first position at the top: on the centers of the track's round
 * ends and evenly between, as 3.0 draws it, so the knob never touches the outline. Each position's
 * label is level with it.
 */
export const knobY = (index: number, count: number) => {
  const end = TOGGLE.h / 2 - TOGGLE.w / 2;
  return count < 2 ? 0 : -end + (2 * end * index) / (count - 1);
};

/** A pull handle's grip position and its labels: in at the top, pulled out at the bottom. */
export const leverY = (index: number, count: number) => (count < 2 ? 0 : -9.5 + (19 * index) / (count - 1));

export type PositionLabel = { n: number; x: number; y: number; anchor: "start" | "middle" | "end" };
export type ItemLayout = {
  /** The name's lines, size in pixels, and the first line's baseline below the control's center. */
  name: { lines: string[]; px: number; y: number } | null;
  /** Position labels, relative to the control's center. */
  positions: PositionLabel[];
};
export type SectionLayout = {
  items: Map<number, ItemLayout>;
  /** The share of 3.0's text sizes this section uses: 1, or less where controls sit close. */
  scale: number;
  bottom: number;
};

/** The smallest share of 3.0's text sizes a crowded section may use, as 3.0 also shrank. */
const MIN_SCALE = 0.62;

/**
 * Places every label in a section so none overlaps a control or another label, keeping all of
 * them, as 3.0 did (Phase 2 brief, step 7). Controls stay where the pack puts them and are
 * placed first. Then, the step's controls
 * first and the rest from left to right: a switch's position labels go on its right, or its left
 * if the right is taken; a name goes under its control, or a line or more lower. If anything
 * still collides, the section's text shrinks a step and the placing starts again, down to
 * MIN_SCALE of 3.0's sizes, so a roomy panel keeps full size and a crowded one stays readable.
 * `target(i)` gives whether item `i` is one of the step's controls, its target and its rest.
 */
export function layoutSection(
  section: PanelSection,
  shown: number[],
  target: (i: number) => { on: boolean; t: number | null; rest: number; lit?: boolean },
  k: number,
  titleTop: number,
  /** The part of the section drawn across (a wide section is cropped around the step). */
  bounds: { x0: number; x1: number } = { x0: 0, x1: section.width },
): SectionLayout {
  let last: SectionLayout | null = null;
  for (let scale = 1; scale >= MIN_SCALE - 0.001; scale -= 0.04) {
    const attempt = place(section, shown, target, k, titleTop, scale, bounds);
    last = attempt.layout;
    if (attempt.clear) return attempt.layout;
  }
  return last as SectionLayout;
}

function place(
  section: PanelSection,
  shown: number[],
  target: (i: number) => { on: boolean; t: number | null; rest: number; lit?: boolean },
  k: number,
  titleTop: number,
  scale: number,
  bounds: { x0: number; x1: number },
): { layout: SectionLayout; clear: boolean } {
  const placed: (Box & { owner?: number })[] = [];
  let clear = true;
  const posPx = PX.pos * scale;
  const moved = section;
  placed.push({
    x0: bounds.x0 + 6,
    y0: titleTop + 2,
    x1: bounds.x0 + 8 + em(section.title.toUpperCase()) * ((PX.title * scale) / k),
    y1: titleTop + 16,
  });
  const glyph = (i: number): Box => {
    const it = moved.items[i] as PanelItem;
    const lamp = it.kind === "light";
    const hw = lamp ? LAMP.w / 2 : halfWidth(it);
    const h = lamp ? LAMP.h / 2 : half(it);
    return { x0: it.x - hw, y0: it.y - h, x1: it.x + hw, y1: it.y + h };
  };
  for (const i of shown) placed.push({ ...glyph(i), owner: i });
  const order = [...shown].sort((a, b) => {
    const [ta, tb] = [target(a).on, target(b).on];
    if (ta !== tb) return ta ? -1 : 1;
    return (section.items[a]?.x ?? 0) - (section.items[b]?.x ?? 0);
  });
  const inside = (b: Box) => b.x0 >= bounds.x0 + 4 && b.x1 <= bounds.x1 - 4;
  /** Clear of everything placed; a control's own shape does not block its own position labels. */
  const free = (b: Box, own?: number) =>
    inside(b) && !placed.some((p) => (own === undefined || p.owner !== own) && overlaps(p, b));
  const items = new Map<number, ItemLayout>();
  for (const i of order) {
    const it = moved.items[i] as PanelItem;
    const { on, t, rest, lit } = target(i);
    const layout: ItemLayout = { name: null, positions: [] };
    if (it.kind === "light" && lit) {
      // "EXPECTED ON" under a light the step expects on.
      const px = PX.setTo * scale;
      const w = em("EXPECTED ON") * (px / k);
      placed.push({
        x0: it.x - w / 2,
        y0: it.y + LAMP.h / 2 + 1,
        x1: it.x + w / 2,
        y1: it.y + LAMP.h / 2 + 2 + (px * 1.2) / k,
      });
    }
    const pos = positionsOf(it);
    const boxOf = (l: PositionLabel): Box => {
      const w = em(pos[l.n] ?? "") * (posPx / k);
      const x0 = l.anchor === "start" ? l.x : l.anchor === "end" ? l.x - w : l.x - w / 2;
      return { x0: it.x + x0, y0: it.y + l.y - posPx / k, x1: it.x + x0 + w, y1: it.y + l.y + 1.5 };
    };
    if (it.kind === "lever" || it.kind === "handle") {
      // Right of the lever or handle, or left if that side is taken; a long lever labels its
      // ends and its target or resting position.
      const ns =
        pos.length <= 4
          ? pos.map((_, n) => n)
          : labelledPositions(pos.length, on ? t : rest >= 0 ? rest : null);
      const side = (right: boolean): PositionLabel[] =>
        ns.map((n) => ({
          n,
          x: right ? halfWidth(it) + 4 : -halfWidth(it) - 4,
          y: (it.kind === "lever" ? slotY(n, pos.length) : leverY(n, pos.length)) + 3,
          anchor: right ? "start" : "end",
        }));
      const right = side(true);
      const left = side(false);
      const chosen = right.every((l) => free(boxOf(l), i))
        ? right
        : left.every((l) => free(boxOf(l), i))
          ? left
          : right;
      if (!chosen.every((l) => free(boxOf(l), i))) clear = false;
      for (const l of chosen) placed.push(boxOf(l));
      layout.positions = chosen;
    } else if (it.kind === "toggle_switch" && pos.length <= 3) {
      // Right of the switch, or left of it if that side is taken: every position keeps its label.
      const side = (right: boolean): PositionLabel[] =>
        pos.map((_, n) => {
          const dx = toggleHalfWidth(!!it.guarded) + TOGGLE.gap;
          // Level with the knob: the baseline sits half a capital below the knob's center.
          const y = knobY(n, pos.length) + (0.36 * posPx) / k;
          return { n, x: right ? dx : -dx, y, anchor: right ? "start" : "end" };
        });
      const right = side(true);
      const left = side(false);
      const chosen = right.every((l) => free(boxOf(l), i))
        ? right
        : left.every((l) => free(boxOf(l), i))
          ? left
          : right;
      if (!chosen.every((l) => free(boxOf(l), i))) clear = false;
      for (const l of chosen) placed.push(boxOf(l));
      layout.positions = chosen;
    } else if (it.kind === "selector") {
      // Every position is labeled at its own angle, just beyond its tick: a label above or below
      // moves out by half a line, as 3.0 places it; one at the side stays level with its tick,
      // clear of a wide label at the top.
      const line = posPx / k;
      for (const n of pos.keys()) {
        const a = (angleOf(it, n) * Math.PI) / 180;
        const [sn, cs] = [Math.sin(a), Math.cos(a)];
        const anchor = sn < -0.35 ? "end" : sn > 0.35 ? "start" : "middle";
        const lift = anchor === "middle" ? (cs > 0 ? -0.55 : 0.55) : 0;
        const l: PositionLabel = {
          n,
          x: sn * 17.5,
          // The label's middle at its point; the baseline sits half a capital below.
          y: -cs * 17.5 + lift * line + 0.36 * line,
          anchor,
        };
        if (!free(boxOf(l), i)) clear = false;
        placed.push(boxOf(l));
        layout.positions.push(l);
      }
    }
    if (labelled(it)) {
      const fit = nameFit(it, moved, k);
      const px = fit.px * scale;
      const extra = on && t != null ? 1 : 0;
      const line = (px * LINE) / k;
      const boxFor = (dy: number): Box => {
        const setTo = extra ? (pos[t as number] === "SET" ? "SET" : `SET TO ${pos[t as number] ?? ""}`) : "";
        const w = Math.max(widest(fit.lines) * px, em(setTo) * PX.setTo * scale) / k;
        const top = it.y + half(it) + 3 + dy * line;
        return { x0: it.x - w / 2, y0: top, x1: it.x + w / 2, y1: top + (fit.lines.length + extra) * line };
      };
      let dy = [0, 1, 2].find((d) => free(boxFor(d)));
      if (dy === undefined) {
        clear = false;
        dy = 0;
      }
      layout.name = { lines: fit.lines, px, y: half(it) + 3 + dy * line + line * 0.82 };
      placed.push(boxFor(dy));
    }
    items.set(i, layout);
  }
  return { layout: { items, scale, bottom: Math.max(...placed.map((b) => b.y1)) + 4 }, clear };
}
