<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The label-free panel preview (round 6, ROUND6.md): the whole panel the step uses, drawn by the same
renderer from the same pack data as the full panel, with the same glyphs and states (changed
filled blue, conditional dashed, lights outline only), simply without text. It takes its card's
width and keeps the panel's proportions; it never crops, centers on or stretches anything.
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
  }: {
    panel: Panel;
    /** Ids of the sections that hold the step's controls: the panel the preview shows. */
    focus: string[];
    targets: ItemTarget[];
    guardClosed?: boolean;
    /** What the preview shows, for screen readers ("The whole fuel panel"). */
    label: string;
  } = $props();

  let sections = $derived(panel.sections.filter((s) => focus.includes(s.id)));
  let box = $derived.by(() => {
    const x = Math.min(...sections.map((s) => s.x)) - 2;
    const y = Math.min(...sections.map((s) => s.y)) - 2;
    const x1 = Math.max(...sections.map((s) => s.x + s.width)) + 2;
    const y1 = Math.max(...sections.map((s) => s.y + s.height)) + 2;
    return { x, y, w: x1 - x, h: y1 - y };
  });
  const whole = (s: { width: number; height: number }) => ({ x0: 0, y0: 0, x1: s.width, y1: s.height });
</script>

<svg class="preview" viewBox="{box.x} {box.y} {box.w} {box.h}" role="img" aria-label={label}>
  {#each sections as s (s.id)}
    <PanelSection section={s} {targets} {guardClosed} band={whole(s)} frame={whole(s)} titled={false} textless k={K} />
  {/each}
</svg>

<style>
  .preview {
    display: block;
    width: 100%;
    height: auto;
  }
</style>
