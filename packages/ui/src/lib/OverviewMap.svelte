<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Where a control is, as in 3.0: the pack's flight deck overview (our own layout) cut down to the
region the zone is in, with the region's edge captions ("AFT, TOWARD THE CABIN", "CAPTAIN SIDE")
from the pack, and the zone marked by a thicker outline, fill and label as well as color. Panel
palette, labels at 11 px or more (decision 0021): an area whose name cannot fit at 11 px goes
unlabeled; the marked area is always named.
-->
<script lang="ts">
  import type { Zone } from "@chocktochock/bindings";
  import { type PackIndex, splitArea } from "../pack-index";
  import { fitSize, labelLines } from "./map-labels";
  import { LABEL_MIN } from "./panel-layout";

  let { index, zone }: { index: PackIndex; zone: Zone } = $props();

  let byId = $derived(new Map(index.zones.map((z) => [z.id, z])));
  let map = $derived(index.region(zone));
  let region = $derived(map?.region ?? null);
  let bounds = $derived.by(() => {
    const areas = map?.areas ?? [];
    return {
      x0: Math.min(...areas.map((a) => a.x)),
      y0: Math.min(...areas.map((a) => a.y)),
      x1: Math.max(...areas.map((a) => a.x + a.width)),
      y1: Math.max(...areas.map((a) => a.y + a.height)),
    };
  });
  let short = $derived(bounds.y1 - bounds.y0 < 100);
  /**
   * Where a region stacks several panels one above another (the center console's P9, control
   * stand and P8), each gets a label down the left side, as 3.0 has, from the pack's area names.
   */
  let groups = $derived.by(() => {
    const out = new Map<string, { y0: number; y1: number }>();
    for (const a of map?.areas ?? []) {
      const name = byId.get(a.zone)?.area ?? "";
      const g = out.get(name) ?? { y0: a.y, y1: a.y + a.height };
      out.set(name, { y0: Math.min(g.y0, a.y), y1: Math.max(g.y1, a.y + a.height) });
    }
    const list = [...out.entries()].map(([name, g]) => ({ name, ...g })).sort((a, b) => a.y0 - b.y0);
    const stacked = list.length > 1 && list.every((g, n) => n === 0 || g.y0 >= (list[n - 1]?.y1 ?? 0) - 1);
    return stacked
      ? list.map((g) => {
          const [name, code] = splitArea(g.name);
          const words = name.toUpperCase().replace(/ PANEL$/, "").split(" ");
          return { ...g, lines: code ? [code, ...words] : words };
        })
      : [];
  });
  let sides = $derived(!!(region?.left || region?.right));
  // Room around the areas for the captions, as 3.0 lays them out (its 20 + 2 at the sides and
  // 22 + 2 at the top and bottom), so the areas fill as much of the map as in 3.0.
  let pad = $derived({
    top: region?.top || (sides && short) ? 24 : 6,
    bottom: region?.bottom ? 24 : 6,
    left: (sides && !short ? 22 : 6) + (groups.length ? 62 : 0),
    right: sides && !short ? 22 : 6,
  });
  let box = $derived({
    x: bounds.x0 - pad.left,
    y: bounds.y0 - pad.top,
    w: bounds.x1 - bounds.x0 + pad.left + pad.right,
    h: bounds.y1 - bounds.y0 + pad.top + pad.bottom,
  });
  /**
   * As 3.0 draws its maps (the area coordinates are 3.0's): area labels are 11.6 map units, so
   * they grow with the map, and a map is at most 230px tall, or 360px for a tall one such as the
   * center console. Edge captions keep 3.0's measured 9.3px. `width` is the map's width on screen.
   */
  const PX = { edge: LABEL_MIN };
  const LABEL = 11.6;
  let width = $state(0);
  /** The page's own scale (wide screens and Text size): the map's limits grow with the page. */
  let page = $state(1);
  $effect(() => {
    const read = () => {
      page = Number.parseFloat(getComputedStyle(document.documentElement).fontSize) / 16 || 1;
    };
    read();
    window.addEventListener("resize", read);
    return () => window.removeEventListener("resize", read);
  });
  let cap = $derived((bounds.y1 - bounds.y0 > bounds.x1 - bounds.x0 ? 360 : 230) * page);
  // The map is as wide as its box unless its height cap makes it narrower.
  let k = $derived(width > 0 ? Math.min(width, (cap * box.w) / box.h) / box.w : 0.75);
  let cx = $derived((bounds.x0 + bounds.x1) / 2);
  let cy = $derived((bounds.y0 + bounds.y1) / 2);

  /** An area's label size in map units: as large as fits its box, up to 3.0's size or 11 px. */
  const size = (text: string[], width: number, height: number) =>
    fitSize(text, width, height, Math.max(LABEL, (LABEL_MIN * page) / k));
  /** A label is drawn only at 11 px or more on screen; the marked area's never smaller. */
  const shows = (fs: number) => (fs * k) / page >= LABEL_MIN - 0.01;
</script>

{#if map && map.areas.length}
  <div class="wrap" bind:clientWidth={width}>
  <svg
    class="map"
    viewBox="{box.x} {box.y} {box.w} {box.h}"
    style:max-width="min({Math.round(box.w * 1.4 * page)}px, 32rem)"
    style:max-height="{cap}px"
    role="img"
    aria-label="Flight deck map, {region?.name ?? zone.area}: {zone.title} marked"
  >
    {#each map.areas as area (area.zone + area.x + area.y)}
      {@const z = byId.get(area.zone)}
      {@const on = area.zone === zone.id}
      {@const text = labelLines(area.label ?? z?.short_title ?? "", area.width, area.height, Math.max(LABEL, (LABEL_MIN * page) / k))}
      {@const fit = size(text, area.width, area.height)}
      {@const fs = on ? Math.max(fit, (LABEL_MIN * page) / k) : fit}
      <g class:on>
        <rect x={area.x} y={area.y} width={area.width} height={area.height} rx="3" />
        {#each on || shows(fit) ? text : [] as line, n (n)}
          <text
            x={area.x + area.width / 2}
            y={area.y + area.height / 2 + (n - (text.length - 1) / 2) * fs * 1.1}
            style:font-size="{fs}px"
          >{line}</text>
        {/each}
      </g>
    {/each}
    {#each groups as g (g.name)}
      {#each g.lines as line, n (n)}
        <text
          class="edge group"
          x={bounds.x0 - (sides && !short ? 23 : 8)}
          y={(g.y0 + g.y1) / 2 + (n - (g.lines.length - 1) / 2) * ((PX.edge * page * 1.15) / k)}
          style:text-anchor="end"
          style:font-size="{(PX.edge * page) / k}px">{line}</text
        >
      {/each}
    {/each}
    {#if region?.top}
      <text class="edge" x={cx} y={bounds.y0 - 13} style:font-size="{(PX.edge * page) / k}px">↑ {region.top}</text>
    {/if}
    {#if region?.bottom}
      <text class="edge" x={cx} y={bounds.y1 + 15} style:font-size="{(PX.edge * page) / k}px">↓ {region.bottom}</text>
    {/if}
    {#if sides && short}
      {#if region?.left}<text class="edge" x={bounds.x0} y={bounds.y0 - 13} style:text-anchor="start" style:font-size="{(PX.edge * page) / k}px">{region.left}</text>{/if}
      {#if region?.right}<text class="edge" x={bounds.x1} y={bounds.y0 - 13} style:text-anchor="end" style:font-size="{(PX.edge * page) / k}px">{region.right}</text>{/if}
    {:else if sides}
      {#if region?.left}<text class="edge" transform="translate({bounds.x0 - 10} {cy}) rotate(-90)" style:font-size="{(PX.edge * page) / k}px">{region.left}</text>{/if}
      {#if region?.right}<text class="edge" transform="translate({bounds.x1 + 10} {cy}) rotate(90)" style:font-size="{(PX.edge * page) / k}px">{region.right}</text>{/if}
    {/if}
  </svg>
  </div>
{/if}

<style>
  .wrap {
    width: 100%;
  }
  .map {
    display: block;
    width: 100%;
    height: auto;
    margin: 0 auto;
  }
  /* Panel palette (decision 0021). */
  rect {
    fill: var(--panel-fill);
    stroke: var(--panel-stroke);
    stroke-width: 1;
  }
  text {
    font-family: var(--font-sans);
    font-weight: var(--weight-semibold);
    fill: var(--panel-text);
    text-anchor: middle;
    dominant-baseline: central;
  }
  .edge {
    fill: var(--panel-label);
  }
  .on rect {
    fill: var(--panel-changed-fill);
    stroke: var(--panel-changed);
    stroke-width: 2.5;
  }
  .on text {
    fill: var(--panel-changed-text);
  }
</style>
