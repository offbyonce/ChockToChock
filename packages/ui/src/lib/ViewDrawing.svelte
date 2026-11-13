<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

A side or seat view of the flight deck for "Where is this?" (decision 0023), drawn from the pack's
frame: its shapes by role (structure, glass, seat, eye, sight), every region's place as a quiet
outline, and the step's region highlighted and named. In the side view a sight line runs from the
eye to the region. Panel palette, both themes, sized by width with labels at 11 px or more; the
caption says it is a schematic.
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
  /** Pixels per frame unit on screen; labels are LABEL_MIN pixels (scaled with the page). */
  let k = $derived(width > 0 ? width / frame.width : 1);
  let fs = $derived((LABEL_MIN * page) / k);

  let places = $derived(frame.places ?? []);
  let shapes = $derived(frame.shapes ?? []);
  let here = $derived(places.find((p) => p.region === region) ?? null);
  let eye = $derived(shapes.find((s) => s.role === "eye")?.points[0] ?? null);
  const path = (points: number[][], closed: boolean) =>
    `M ${points.map(([x, y]) => `${x} ${y}`).join(" L ")}${closed ? " Z" : ""}`;
  /** Where a shape's label goes: inside a closed shape, just above a dot, beside a vertical line
   * (rotated along it), and off a sloping or level line on the side away from the frame's middle. */
  const labelAt = (points: number[][], closed: boolean) => {
    const xs = points.map((p) => p[0] ?? 0);
    const ys = points.map((p) => p[1] ?? 0);
    const x = (Math.min(...xs) + Math.max(...xs)) / 2;
    const y = (Math.min(...ys) + Math.max(...ys)) / 2;
    if (points.length === 1) return { x, y: y - fs * 1.1, rotate: 0 };
    if (closed) return { x, y, rotate: 0 };
    if (xs.every((v) => v === xs[0])) {
      const side = x < frame.width / 2 ? 1 : -1;
      return { x: x + side * fs * 0.9, y, rotate: side * 90 };
    }
    const dx = x - frame.width / 2;
    const dy = y - frame.height / 2;
    const len = Math.hypot(dx, dy) || 1;
    return { x: x + (dx / len) * fs * 2.4, y: y + (dy / len) * fs * 1.2, rotate: 0 };
  };
  /** The highlighted region's name: inside its place when it fits, else above a place in the top
   * half of the frame and below one in the bottom half. */
  let nameAt = $derived.by(() => {
    if (!here || !region) return null;
    const textW = region.length * fs * 0.62;
    const cx = here.x + here.width / 2;
    if (here.height >= fs * 1.6 && here.width >= textW + fs) return { x: cx, y: here.y + here.height / 2, inside: true };
    const top = here.y + here.height / 2 < frame.height / 2;
    return { x: cx, y: top ? here.y + here.height + fs * 1.1 : here.y - fs * 0.8, inside: false };
  });
</script>

<figure class="view">
  <figcaption>{title}</figcaption>
  <div class="wrap" bind:clientWidth={width}>
    <svg viewBox="0 0 {frame.width} {frame.height}" role="img" aria-label="{title}{here ? `, ${region} highlighted` : ''}. Schematic, not to scale.">
      {#each places as place (place.region)}
        <rect class="place" class:here={place.region === region} x={place.x} y={place.y} width={place.width} height={place.height} rx="2" />
      {/each}
      {#each shapes as shape, n (n)}
        {#if shape.points.length === 1}
          {@const [x, y] = shape.points[0] ?? [0, 0]}
          <circle class="shape {shape.role}" cx={x} cy={y} r={4} />
        {:else}
          <path class="shape {shape.role}" d={path(shape.points, shape.closed ?? false)} />
        {/if}
      {/each}
      {#if kind === "side" && eye && here}
        <line class="sight" x1={eye[0]} y1={eye[1]} x2={here.x + here.width / 2} y2={here.y + here.height / 2} />
      {/if}
      {#each shapes.filter((s) => s.label) as shape, n (n)}
        {@const at = labelAt(shape.points, shape.closed ?? false)}
        <text class="tag" x={at.x} y={at.y} style:font-size="{fs}px" transform={at.rotate ? `rotate(${at.rotate} ${at.x} ${at.y})` : undefined}>{shape.label}</text>
      {/each}
      {#if nameAt && region}
        <text class="name" class:inside={nameAt.inside} x={nameAt.x} y={nameAt.y} style:font-size="{fs}px">{region.toUpperCase()}</text>
      {/if}
    </svg>
  </div>
  <p class="note">Schematic, not to scale.</p>
</figure>

<style>
  .view {
    margin: 0;
    padding: var(--space-3);
    border: var(--border-thin) solid var(--panel-frame);
    border-radius: var(--radius-panel);
    background: var(--panel-bg);
  }
  figcaption {
    margin-bottom: var(--space-2);
    color: var(--color-text);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
  }
  svg {
    display: block;
    width: 100%;
    height: auto;
    overflow: visible;
  }
  .place {
    fill: var(--panel-fill);
    stroke: var(--panel-stroke);
    stroke-width: 1;
  }
  .place.here {
    fill: var(--panel-changed-fill);
    stroke: var(--panel-changed);
    stroke-width: 2.5;
  }
  .shape {
    fill: none;
    stroke: var(--panel-stroke);
    stroke-width: 1.5;
    stroke-linejoin: round;
  }
  .shape.glass {
    stroke: var(--panel-label);
    stroke-width: 2.5;
  }
  .shape.seat {
    fill: var(--panel-seat);
  }
  .shape.eye {
    fill: var(--panel-text);
    stroke: none;
  }
  .sight {
    stroke: var(--panel-changed);
    stroke-width: 1.5;
    stroke-dasharray: 4 3;
  }
  text {
    font-family: var(--font-sans);
    font-weight: var(--weight-semibold);
    text-anchor: middle;
    dominant-baseline: central;
  }
  .tag {
    fill: var(--panel-label);
  }
  .name {
    fill: var(--panel-text);
  }
  .name.inside {
    fill: var(--panel-changed-text);
  }
  .note {
    margin: var(--space-2) 0 0;
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
</style>
