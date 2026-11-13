<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

A side or seat view of the flight deck for "Where is this?" (decision 0023), drawn as the approved
design draws it (the WherePop board) from the pack's frame: the airplane's outline, windows and
seat by role, every region's outline as a quiet panel, the step's region in blue with a heavy
outline, and in the side view a dashed sight line from the pilot's eye to it, ending in a dot.
Labels sit where the pack puts them, at the board's sizes and never under 11 px. Panel palette,
both themes, sized by width.
-->
<script lang="ts">
  import type { ViewFrame } from "@chocktochock/bindings";
  import { LABEL_MIN } from "./panel-layout";

  let {
    frame,
    region,
    kind,
    title,
  }: {
    frame: ViewFrame;
    /** The overview region to highlight, by name. */
    region: string | null;
    kind: "side" | "seat";
    /** What the view shows, e.g. "From the side: how high, how far". */
    title: string;
  } = $props();

  /** The board's label sizes in view units: fixed labels, and the highlighted region's name. */
  const LABEL = 14;
  const NAME = 17;

  let width = $state(0);
  /** The page's own scale (wide screens, Text size), so labels keep their size against the text. */
  let page = $state(1);
  $effect(() => {
    const read = () => {
      page = Number.parseFloat(getComputedStyle(document.documentElement).fontSize) / 16 || 1;
    };
    read();
    window.addEventListener("resize", read);
    return () => window.removeEventListener("resize", read);
  });
  /** Pixels per view unit on screen. */
  let k = $derived(width > 0 ? width / frame.width : 1);
  /** A size in view units, never drawn under 11 px (scaled with the page). */
  const atLeast = (units: number) => Math.max(units, (LABEL_MIN * page) / k);

  let places = $derived(frame.places ?? []);
  let shapes = $derived(frame.shapes ?? []);
  let labels = $derived(frame.labels ?? []);
  let here = $derived(places.find((p) => p.region === region) ?? null);
  let eye = $derived(shapes.find((s) => s.role === "eye")?.points[0] ?? null);
  const path = (points: number[][], closed: boolean) =>
    `M ${points.map(([x, y]) => `${x} ${y}`).join(" L ")}${closed ? " Z" : ""}`;
  /** The middle of the highlighted region's outlines, where the sight line ends. */
  let target = $derived.by(() => {
    const pts = here?.shapes.flat() ?? [];
    if (!pts.length) return null;
    const xs = pts.map((p) => p[0] ?? 0);
    const ys = pts.map((p) => p[1] ?? 0);
    return { x: (Math.min(...xs) + Math.max(...xs)) / 2, y: (Math.min(...ys) + Math.max(...ys)) / 2 };
  });
</script>

<figure class="view">
  <figcaption>{title}</figcaption>
  <div class="wrap" bind:clientWidth={width}>
    <svg viewBox="0 0 {frame.width} {frame.height}" role="img" aria-label="{title}{here ? `, ${region} highlighted` : ''}. Schematic, not to scale.">
      {#each shapes.filter((s) => s.role !== "eye") as shape, n (n)}
        <path class="shape {shape.role}" d={path(shape.points, shape.closed ?? false)} />
      {/each}
      {#each places as place (place.region)}
        {#each place.shapes as outline, n (n)}
          <path class="place" class:here={place === here} d={path(outline, true)} />
        {/each}
      {/each}
      {#if kind === "side" && eye && target}
        <line class="sight" x1={eye[0]} y1={eye[1]} x2={target.x} y2={target.y} />
        <circle class="sight-end" cx={target.x} cy={target.y} r="5" />
      {/if}
      {#each labels as label (label.text + label.x)}
        <text class="tag" x={label.x} y={label.y} style:font-size="{atLeast(LABEL)}px" style:text-anchor={label.anchor ?? "middle"}>{label.text}</text>
      {/each}
      {#if here?.label_at && region}
        <text class="name" x={here.label_at[0]} y={here.label_at[1]} style:font-size="{atLeast(NAME)}px">{region.toUpperCase()}</text>
      {/if}
    </svg>
  </div>
  <p class="note">Schematic, not to scale.</p>
</figure>

<style>
  .view {
    display: flex;
    flex-direction: column;
    margin: 0;
    padding: var(--space-3);
    border: var(--border-thin) solid var(--panel-frame);
    border-radius: var(--radius-panel);
    background: var(--panel-bg);
  }
  figcaption {
    margin-bottom: var(--space-2);
    color: var(--color-text-secondary);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-semibold);
  }
  svg {
    display: block;
    width: 100%;
    height: auto;
  }
  .shape {
    stroke: var(--panel-stroke);
    stroke-linejoin: round;
    stroke-linecap: round;
  }
  .structure {
    fill: var(--panel-fuselage);
    stroke-width: 1.5;
  }
  .glass {
    fill: var(--panel-glass);
    stroke-width: 1.2;
  }
  .seat {
    fill: var(--panel-seat);
    stroke-width: 1.2;
  }
  .place {
    fill: var(--panel-fill);
    stroke: var(--panel-stroke);
    stroke-width: 1.2;
    stroke-linejoin: round;
  }
  .place.here {
    fill: var(--panel-changed-fill);
    stroke: var(--panel-changed);
    stroke-width: 3;
  }
  .sight {
    stroke: var(--panel-changed);
    stroke-width: 2;
    stroke-dasharray: 5 5;
  }
  .sight-end {
    fill: var(--panel-changed);
  }
  text {
    font-family: var(--font-sans);
    font-weight: var(--weight-semibold);
    dominant-baseline: alphabetic;
  }
  .tag {
    fill: var(--panel-label);
  }
  .name {
    fill: var(--panel-changed-text);
    text-anchor: middle;
    dominant-baseline: central;
  }
  .note {
    margin: auto 0 0;
    padding-top: var(--space-2);
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
</style>
