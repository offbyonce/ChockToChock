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
  /** Where a shape's label goes: inside a closed shape, just above a dot, at the foot of a
   * vertical line (reading inward), and off a sloping or level line on the side away from the
   * frame's middle; a dot's to its left. */
  const labelAt = (points: number[][], closed: boolean) => {
    const xs = points.map((p) => p[0] ?? 0);
    const ys = points.map((p) => p[1] ?? 0);
    const x = (Math.min(...xs) + Math.max(...xs)) / 2;
    const y = (Math.min(...ys) + Math.max(...ys)) / 2;
    // A dot's label to its left, clear of sight lines, which leave the eye forward.
    if (points.length === 1) return { x: x - fs * 0.8, y, anchor: "end" };
    if (closed) return { x, y, anchor: "middle" };
    if (xs.every((v) => v === xs[0])) {
      const left = x < frame.width / 2;
      return { x: x + (left ? fs * 0.5 : -fs * 0.5), y: Math.max(...ys) - fs * 0.7, anchor: left ? "start" : "end" };
    }
    const dx = x - frame.width / 2;
    const dy = y - frame.height / 2;
    const len = Math.hypot(dx, dy) || 1;
    return { x: x + (dx / len) * fs * 3.2, y: y + (dy / len) * fs * 1.6, anchor: "middle" };
  };
  type Box = { x0: number; y0: number; x1: number; y1: number };
  const overlaps = (a: Box, b: Box) => a.x0 < b.x1 && b.x0 < a.x1 && a.y0 < b.y1 && b.y0 < a.y1;
  /** Where the sight line meets the region's place: its edge, on the way from the eye. */
  let sightEnd = $derived.by(() => {
    if (!eye || !here) return null;
    const [ex, ey] = [eye[0] ?? 0, eye[1] ?? 0];
    const [cx, cy] = [here.x + here.width / 2, here.y + here.height / 2];
    const [dx, dy] = [cx - ex, cy - ey];
    const t = Math.min(
      dx === 0 ? Number.POSITIVE_INFINITY : Math.abs(here.width / 2 / dx),
      dy === 0 ? Number.POSITIVE_INFINITY : Math.abs(here.height / 2 / dy),
    );
    return { x: cx - dx * t, y: cy - dy * t };
  });
  /** Thin boxes along a line, so labels can keep clear of it. */
  const segments = (points: number[][]): Box[] =>
    points.slice(1).flatMap((p, n) => {
      const q = points[n] ?? p;
      const steps = Math.max(1, Math.ceil(Math.hypot((p[0] ?? 0) - (q[0] ?? 0), (p[1] ?? 0) - (q[1] ?? 0)) / 6));
      return Array.from({ length: steps + 1 }, (_, i) => {
        const x = (q[0] ?? 0) + (((p[0] ?? 0) - (q[0] ?? 0)) * i) / steps;
        const y = (q[1] ?? 0) + (((p[1] ?? 0) - (q[1] ?? 0)) * i) / steps;
        return { x0: x - 1.5, y0: y - 1.5, x1: x + 1.5, y1: y + 1.5 };
      });
    });
  /** The highlighted region's name: inside its place when it fits, else the first spot around it
   * that stays inside the frame and clears every other place, every line and the sight line. */
  let nameAt = $derived.by(() => {
    if (!here || !region) return null;
    const w = region.length * fs * 0.64;
    const cx = here.x + here.width / 2;
    const cy = here.y + here.height / 2;
    if (here.height >= fs * 1.15 && here.width >= w + fs) return { x: cx, y: cy, inside: true };
    const blocked: Box[] = [
      ...places.filter((p) => p !== here).map((p) => ({ x0: p.x, y0: p.y, x1: p.x + p.width, y1: p.y + p.height })),
      ...shapes.flatMap((sh) => segments(sh.closed ? [...sh.points, sh.points[0] ?? [0, 0]] : sh.points)),
      ...(eye && sightEnd ? segments([eye, [sightEnd.x, sightEnd.y]]) : []),
    ];
    const pad = fs * 0.3;
    const at = (x: number, y: number) => ({
      x,
      y,
      box: { x0: x - w / 2 - pad, y0: y - fs / 2 - pad, x1: x + w / 2 + pad, y1: y + fs / 2 + pad },
    });
    const gap = fs * 0.9;
    const tries = [
      at(cx, here.y + here.height + gap),
      at(cx, here.y - gap),
      at(here.x - w / 2 - gap, cy),
      at(here.x + here.width + w / 2 + gap, cy),
      at(here.x + here.width - w / 2, here.y + here.height + gap),
      at(here.x + w / 2, here.y + here.height + gap),
      at(here.x - w / 2 - gap, here.y + here.height + gap),
      at(cx, here.y + here.height + gap * 2.2),
    ];
    const inFrame = (b: Box) => b.x0 >= 0 && b.y0 >= 0 && b.x1 <= frame.width && b.y1 <= frame.height;
    const free = tries.find((t) => inFrame(t.box) && !blocked.some((o) => overlaps(o, t.box))) ?? tries[0];
    return free ? { x: free.x, y: free.y, inside: false } : null;
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
        <line class="sight" x1={eye[0]} y1={eye[1]} x2={sightEnd?.x ?? here.x} y2={sightEnd?.y ?? here.y} />
      {/if}
      {#each shapes.filter((s) => s.label) as shape, n (n)}
        {@const at = labelAt(shape.points, shape.closed ?? false)}
        <text class="tag" x={at.x} y={at.y} style:font-size="{fs}px" style:text-anchor={at.anchor}>{shape.label}</text>
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
  /* A halo in the background color, so no line strikes through a label. */
  .tag,
  .name {
    paint-order: stroke;
    stroke: var(--panel-bg);
    stroke-width: 4;
    stroke-linejoin: round;
  }
  .tag {
    fill: var(--panel-label);
  }
  .name {
    fill: var(--panel-text);
  }
  .name.inside {
    fill: var(--panel-changed-text);
    stroke: var(--panel-changed-fill);
  }
  .note {
    margin: var(--space-2) 0 0;
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
</style>
