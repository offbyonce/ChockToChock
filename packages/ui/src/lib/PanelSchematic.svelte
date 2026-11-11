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

  /** Fill the card's width, up to 1.8 times the pack's size. */
  const SCALE = 1.8;
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

  /** The rows of a section to draw: a tall one is cut to the rows around the step's controls. */
  function bandOf(s: Section) {
    const ys = s.items.filter((_, i) => targets.some((t) => t.section === s.id && t.item === i)).map((it) => it.y);
    if (!ys.length || s.height <= 200 || drawn.length > 1) return { y0: 0, y1: s.height };
    return { y0: Math.max(0, Math.min(...ys) - 95), y1: Math.min(s.height, Math.max(...ys) + 72) };
  }
  let bands = $derived(new Map(drawn.map((s) => [s.id, bandOf(s)])));
  let box = $derived.by(() => {
    const top = (s: Section) => s.y + Math.max(0, (bands.get(s.id)?.y0 ?? 0) - ((bands.get(s.id)?.y0 ?? 0) > 0 ? 18 : 0));
    const x = Math.min(...drawn.map((s) => s.x)) - 2;
    const y = Math.min(...drawn.map(top)) - 2;
    const w = Math.max(...drawn.map((s) => s.x + s.width)) + 2 - x;
    const h = Math.max(...drawn.map((s) => s.y + (bands.get(s.id)?.y1 ?? s.height))) + 2 - y;
    return { x, y, w, h };
  });

  let width = $state(0);
  // The drawing is as wide as its box, but never more than SCALE times the pack's size.
  let k = $derived(width > 0 ? Math.min(width, box.w * SCALE) / box.w : 1.6);

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
      <PanelSection section={s} {targets} {guardClosed} band={bands.get(s.id) ?? { y0: 0, y1: s.height }} {k} />
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
    max-height: 21rem;
    margin: 0 auto;
  }
</style>
