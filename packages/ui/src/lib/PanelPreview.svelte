<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The label-free panel preview (round 6, ROUND6.md): the whole panel the step uses, drawn by the same
renderer from the same pack data as the full panel, with the same glyphs and states (changed
filled blue, conditional dashed, lights outline only), simply without text, and a thick blue halo
around the step's controls (dashed for conditional ones) so they read at small sizes. The whole
panel is scaled to fit a box, its card's width and at most `size.preview-max` tall (the owner,
gate review), and centered in it; it is never cropped or stretched.
-->
<script lang="ts">
  import type { ItemTarget, Panel } from "@chocktochock/bindings";
  import { K } from "./panel-layout";
  import PanelSection from "./PanelSection.svelte";

  let {
    panel,
    focus,
    targets,
    guardClosed = false,
    label,
    view,
    marker,
    svg = $bindable(),
  }: {
    panel: Panel;
    /** Ids of the sections that hold the step's controls: the panel the preview shows. */
    focus: string[];
    targets: ItemTarget[];
    guardClosed?: boolean;
    /** What the preview shows, for screen readers ("The whole fuel panel"). */
    label: string;
    /** The view box to show, in panel units, when it must match another drawing's (the
     * navigator matches the labeled panel's); by default the sections' own box. */
    view?: { x: number; y: number; w: number; h: number } | undefined;
    /** A frame over part of the drawing, as fractions of the view box (the navigator's view). */
    marker?: { x0: number; y0: number; x1: number; y1: number } | undefined;
    /** Out: the svg element, for mapping pointer positions. */
    svg?: SVGSVGElement | null | undefined;
  } = $props();

  let sections = $derived(panel.sections.filter((s) => focus.includes(s.id)));
  let box = $derived.by(() => {
    if (view) return view;
    const x = Math.min(...sections.map((s) => s.x)) - 2;
    const y = Math.min(...sections.map((s) => s.y)) - 2;
    const x1 = Math.max(...sections.map((s) => s.x + s.width)) + 2;
    const y1 = Math.max(...sections.map((s) => s.y + s.height)) + 2;
    return { x, y, w: x1 - x, h: y1 - y };
  });
  const whole = (s: { width: number; height: number }) => ({ x0: 0, y0: 0, x1: s.width, y1: s.height });
</script>

<svg class="preview" viewBox="{box.x} {box.y} {box.w} {box.h}" role="img" aria-label={label} bind:this={svg}>
  {#each sections as s (s.id)}
    <PanelSection section={s} {targets} {guardClosed} band={whole(s)} frame={whole(s)} titled={false} textless halo k={K} />
  {/each}
  {#if marker}
    <rect
      class="marker"
      x={box.x + marker.x0 * box.w}
      y={box.y + marker.y0 * box.h}
      width={(marker.x1 - marker.x0) * box.w}
      height={(marker.y1 - marker.y0) * box.h}
      rx="3"
    />
  {/if}
</svg>

<style>
  /* The navigator's frame: the part of the panel the labeled view shows. */
  .marker {
    fill: var(--panel-changed-fill);
    fill-opacity: 0.25;
    stroke: var(--panel-changed);
    stroke-width: var(--border-thick);
    vector-effect: non-scaling-stroke;
  }
  .preview {
    display: block;
    width: 100%;
    height: auto;
    max-height: var(--size-preview-max);
  }
</style>
