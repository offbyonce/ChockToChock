<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

A panel drawing for one step, laid out like 3.0's panel cards. It draws the panel sections that
hold the step's controls, in the pack's own panel layout, with their neighbors in the same rows
for context (so two small sections side by side, such as the wiper selectors, share one drawing).
A tall single section is cut down to the rows around the step's controls. Controls scale to fill
the card; text keeps 3.0's sizes.
-->
<script lang="ts">
  import type { ItemTarget, Panel, PanelSection as Section } from "@chocktochock/bindings";
  import { layoutSection, positionsOf } from "./panel-layout";
  import PanelSection from "./PanelSection.svelte";

  let {
    panel,
    focus,
    targets,
    guardClosed = false,
  }: {
    /** The panel the sections are on. */
    panel: Panel;
    /** Ids of the sections that hold the step's controls. */
    focus: string[];
    /** The step's targets (from Rust). */
    targets: ItemTarget[];
    /** The step asks for the guard closed. */
    guardClosed?: boolean;
  } = $props();

  /** Fill the card's width, up to 2.6 times the pack's size. */
  const SCALE = 2.6;
  /**
   * Text keeps 3.0's pixel sizes up to this many pixels per unit, then grows with the drawing, as
   * 3.0's text does (it is sized in drawing units), so a large drawing never has small text.
   */
  const TEXT_K = 1.8;
  /** A section wider than this is cropped across to SPAN around the step's controls, as 3.0 does. */
  const WIDE = 320;
  const SPAN = 240;
  /** How far beside the step's sections a neighbor may sit and still be drawn. */
  const NEAR = 120;

  let focused = $derived(panel.sections.filter((s) => focus.includes(s.id)));
  // A small group of sections brings its neighbors in the same rows; a tall section stands alone.
  let drawn = $derived.by((): Section[] => {
    if (focused.length === 1 && (focused[0]?.height ?? 0) > 200) return focused;
    const x0 = Math.min(...focused.map((s) => s.x)) - NEAR;
    const x1 = Math.max(...focused.map((s) => s.x + s.width)) + NEAR;
    const y0 = Math.min(...focused.map((s) => s.y));
    const y1 = Math.max(...focused.map((s) => s.y + s.height));
    return panel.sections.filter(
      (s) => focus.includes(s.id) || (s.x >= x0 && s.x + s.width <= x1 && s.y < y1 && s.y + s.height > y0 && s.height <= 200),
    );
  });

  /** A section taller than this is cut down to the controls within REACH of the step's, as 3.0 does. */
  const TALL = 280;
  const REACH = 100;

  /**
   * The part of a section to draw, as 3.0 frames its panel cards, so the step's controls show in
   * first view and the drawing stays as large as 3.0's. A tall section keeps the controls whose
   * centers lie within REACH of the step's (more if the step's own controls spread further), and a
   * wide one is cut across to the controls around the step's. Sections drawn together show whole.
   */
  function bandOf(s: Section) {
    const mine = s.items.filter((_, i) => targets.some((t) => t.section === s.id && t.item === i));
    const whole = { y0: 0, y1: s.height, x0: 0, x1: s.width };
    if (!mine.length || drawn.length > 1) return whole;
    const xs = mine.map((it) => it.x);
    const ys = mine.map((it) => it.y);
    let rows = {};
    if (s.height > TALL) {
      const c = (Math.min(...ys) + Math.max(...ys)) / 2;
      const reach = Math.max(REACH, (Math.max(...ys) - Math.min(...ys)) / 2 + 30);
      const kept = s.items.map((it) => it.y).filter((y) => Math.abs(y - c) <= reach);
      // Room above and below the kept controls for their shapes; sections draw the items inside.
      rows = { y0: Math.max(0, Math.min(...kept) - 20), y1: Math.min(s.height, Math.max(...kept) + 20) };
    }
    let across = {};
    if (s.width > WIDE) {
      const c = (Math.min(...xs) + Math.max(...xs)) / 2;
      const half = Math.max(SPAN / 2, (Math.max(...xs) - Math.min(...xs)) / 2 + 60);
      const x0 = Math.max(0, Math.min(c - half, s.width - 2 * half));
      across = { x0, x1: Math.min(s.width, x0 + 2 * half) };
    }
    return { ...whole, ...rows, ...across };
  }
  let bands = $derived(new Map(drawn.map((s) => [s.id, bandOf(s)])));
  const top = (s: Section) => s.y + Math.max(0, (bands.get(s.id)?.y0 ?? 0) - ((bands.get(s.id)?.y0 ?? 0) > 0 ? 18 : 0));
  let x0 = $derived(Math.min(...drawn.map((s) => s.x + (bands.get(s.id)?.x0 ?? 0))) - 2);
  let w = $derived(Math.max(...drawn.map((s) => s.x + (bands.get(s.id)?.x1 ?? s.width))) + 2 - x0);

  let width = $state(0);
  // The drawing is as wide as its box, but never more than SCALE times the pack's size.
  let k = $derived(width > 0 ? Math.min(width, w * SCALE) / w : 1.6);
  /** Pixels per unit for text: the drawing's, but no more than TEXT_K. */
  let kText = $derived(Math.min(k, TEXT_K));

  /** A section's lowest point: its band, or past its lowest label (as PanelSection lays it out). */
  const bottom = (s: Section) => {
    const band = bands.get(s.id) ?? { y0: 0, y1: s.height, x0: 0, x1: s.width };
    const shown = s.items.flatMap((item, i) =>
      item.y >= band.y0 + 10 && item.y <= band.y1 - 10 && item.x >= band.x0 + 4 && item.x <= band.x1 - 4 ? [i] : [],
    );
    const on = (i: number) => targets.find((t) => t.section === s.id && t.item === i);
    const layout = layoutSection(
      s,
      shown,
      (i) => {
        const item = s.items[i] as (typeof s.items)[number];
        return {
          on: !!on(i),
          t: on(i)?.position ?? null,
          rest: !on(i) && item.rest ? positionsOf(item).indexOf(item.rest) : -1,
          lit: on(i)?.lit ?? false,
        };
      },
      kText,
      top(s) - s.y,
      { x0: band.x0, x1: band.x1 },
    );
    return s.y + Math.max(band.y1, layout.bottom);
  };
  let box = $derived.by(() => {
    const y = Math.min(...drawn.map(top)) - 2;
    const h = Math.max(...drawn.map(bottom)) + 2 - y;
    return { x: x0, y, w, h };
  });

  let described = $derived(
    targets
      .map((t) => {
        const item = panel.sections.find((s) => s.id === t.section)?.items[t.item];
        const pos = t.position != null ? item?.positions?.split(",")[t.position]?.trim() : undefined;
        return item ? `${item.label}${pos ? ` set to ${pos}` : ""}` : null;
      })
      .filter(Boolean)
      .join("; "),
  );
  let title = $derived(focused.map((s) => s.title).join(" and "));
</script>

<div class="wrap" bind:clientWidth={width}>
  <svg
    class="schematic"
    viewBox="{box.x} {box.y} {box.w} {box.h}"
    style:width="min(100%, {Math.round(box.w * SCALE)}px)"
    role="img"
    aria-label="{title} panel, schematic.{described ? ` This step: ${described}.` : ''}"
  >
    {#each drawn as s (s.id)}
      <PanelSection section={s} {targets} {guardClosed} band={bands.get(s.id) ?? { y0: 0, y1: s.height, x0: 0, x1: s.width }} k={kText} />
    {/each}
  </svg>
</div>

<style>
  .wrap {
    width: 100%;
  }
  .schematic {
    display: block;
    height: auto;
    margin: 0 auto;
  }
</style>
