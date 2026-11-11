<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Where a control is, as in 3.0: the pack's flight deck overview (our own layout) cut down to the
region the zone is in, with the region's edge captions ("AFT, TOWARD THE CABIN", "CAPTAIN SIDE")
from the pack, and the zone marked by a thicker outline, fill and bold label as well as color.
-->
<script lang="ts">
  import type { Zone } from "@chocktochock/bindings";
  import type { PackIndex } from "../pack-index";

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
  let sides = $derived(!!(region?.left || region?.right));
  // Room around the areas for the captions, as 3.0 lays them out.
  let pad = $derived({
    top: region?.top || (sides && short) ? 28 : 6,
    bottom: region?.bottom ? 28 : 6,
    left: sides && !short ? 26 : 6,
    right: sides && !short ? 26 : 6,
  });
  let box = $derived({
    x: bounds.x0 - pad.left,
    y: bounds.y0 - pad.top,
    w: bounds.x1 - bounds.x0 + pad.left + pad.right,
    h: bounds.y1 - bounds.y0 + pad.top + pad.bottom,
  });
  /**
   * Labels keep 3.0's rendered sizes (measured: about 9.5px for areas, 9.3px for the edge
   * captions) however large the map is drawn; `width` is the map's width on screen.
   */
  const PX = { label: 9.5, edge: 9.3 };
  let width = $state(0);
  let k = $derived(width > 0 ? width / box.w : 0.75);
  let LABEL = $derived(PX.label / k);
  let cx = $derived((bounds.x0 + bounds.x1) / 2);
  let cy = $derived((bounds.y0 + bounds.y1) / 2);

  /** A label that fits its box: one line, or two split at " / " or near the middle. */
  function lines(label: string, width: number): string[] {
    // One line if it fits, even at the smallest size; split only when it cannot.
    if (label.length * (8.5 / k) * 0.6 <= width - 4) return [label];
    const slash = label.indexOf(" / ");
    if (slash > 0) return [`${label.slice(0, slash)} /`, label.slice(slash + 3)];
    const mid = label.lastIndexOf(" ", Math.ceil(label.length / 2));
    return mid > 0 ? [label.slice(0, mid), label.slice(mid + 1)] : [label];
  }
  /** Shrinks a label that still does not fit, so it never spills out of its box. */
  const size = (text: string[], width: number, height: number) =>
    Math.max(
      8.5 / k,
      Math.min(LABEL, (width - 4) / (Math.max(...text.map((t) => t.length)) * 0.6), (height - 2) / (text.length * 1.15)),
    );
</script>

{#if map && map.areas.length}
  <div class="wrap" bind:clientWidth={width}>
  <svg
    class="map"
    viewBox="{box.x} {box.y} {box.w} {box.h}"
    style:max-width="min({Math.round(box.w * 1.4)}px, 32rem)"
    role="img"
    aria-label="Flight deck map, {region?.name ?? zone.area}: {zone.title} marked"
  >
    {#each map.areas as area (area.zone + area.x + area.y)}
      {@const z = byId.get(area.zone)}
      {@const on = area.zone === zone.id}
      {@const text = lines(area.label ?? z?.short_title ?? "", area.width)}
      {@const fs = size(text, area.width, area.height)}
      <g class:on>
        <rect x={area.x} y={area.y} width={area.width} height={area.height} rx="3" />
        {#each text as line, n (n)}
          <text
            x={area.x + area.width / 2}
            y={area.y + area.height / 2 + (n - (text.length - 1) / 2) * fs * 1.1}
            style:font-size="{fs}px"
          >{line}</text>
        {/each}
      </g>
    {/each}
    {#if region?.top}
      <text class="edge" x={cx} y={bounds.y0 - 13} style:font-size="{PX.edge / k}px">↑ {region.top}</text>
    {/if}
    {#if region?.bottom}
      <text class="edge" x={cx} y={bounds.y1 + 15} style:font-size="{PX.edge / k}px">↓ {region.bottom}</text>
    {/if}
    {#if sides && short}
      {#if region?.left}<text class="edge" x={bounds.x0} y={bounds.y0 - 13} text-anchor="start" style:font-size="{PX.edge / k}px">{region.left}</text>{/if}
      {#if region?.right}<text class="edge" x={bounds.x1} y={bounds.y0 - 13} text-anchor="end" style:font-size="{PX.edge / k}px">{region.right}</text>{/if}
    {:else if sides}
      {#if region?.left}<text class="edge" transform="translate({bounds.x0 - 14} {cy}) rotate(-90)" style:font-size="{PX.edge / k}px">{region.left}</text>{/if}
      {#if region?.right}<text class="edge" transform="translate({bounds.x1 + 14} {cy}) rotate(90)" style:font-size="{PX.edge / k}px">{region.right}</text>{/if}
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
    max-height: 20rem;
    margin: 0 auto;
  }
  rect {
    fill: var(--surface);
    stroke: var(--muted);
    stroke-width: 1;
  }
  text {
    font-family: var(--sans);
    font-weight: 600;
    fill: var(--ink);
    text-anchor: middle;
    dominant-baseline: central;
  }
  .edge {
    letter-spacing: 0.05em;
    fill: var(--muted);
  }
  .on rect {
    fill: var(--accent-soft);
    stroke: var(--accent);
    stroke-width: 2.5;
  }
  .on text {
    fill: var(--accent);
  }
</style>
