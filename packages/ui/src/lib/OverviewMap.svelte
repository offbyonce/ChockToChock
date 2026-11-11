<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Where a control is: the pack's flight deck overview (our own layout), cut down to the area the
zone is in, with the zone marked by a thicker outline and bold label as well as color.
-->
<script lang="ts">
  import type { Overview, Zone } from "@chocktochock/bindings";

  let { overview, zones, zone }: { overview: Overview; zones: Zone[]; zone: Zone } = $props();

  const PAD = 8;
  let byId = $derived(new Map(zones.map((z) => [z.id, z])));
  let areas = $derived(overview.areas.filter((a) => byId.get(a.zone)?.area === zone.area));
  let box = $derived.by(() => {
    const xs = areas.flatMap((a) => [a.x, a.x + a.width]);
    const ys = areas.flatMap((a) => [a.y, a.y + a.height]);
    if (!xs.length) return { x: 0, y: 0, w: overview.width, h: overview.height };
    const x = Math.min(...xs) - PAD;
    const y = Math.min(...ys) - PAD;
    return { x, y, w: Math.max(...xs) + PAD - x, h: Math.max(...ys) + PAD - y };
  });

  /** A label that fits its box: one line, or two split at " / " or near the middle. */
  function lines(label: string, width: number): string[] {
    if (label.length * 6.4 <= width - 6) return [label];
    const slash = label.indexOf(" / ");
    if (slash > 0) return [`${label.slice(0, slash)} /`, label.slice(slash + 3)];
    const mid = label.lastIndexOf(" ", Math.ceil(label.length / 2));
    return mid > 0 ? [label.slice(0, mid), label.slice(mid + 1)] : [label];
  }
</script>

<svg
  class="map"
  viewBox="{box.x} {box.y} {box.w} {box.h}"
  style:max-width="{Math.round(box.w * 1.2)}px"
  role="img"
  aria-label="Flight deck map, {zone.area}: {zone.title} marked"
>
  {#each areas as area (area.zone + area.x + area.y)}
    {@const z = byId.get(area.zone)}
    {@const on = area.zone === zone.id}
    {@const text = lines(area.label ?? z?.short_title ?? "", area.width)}
    <g class:on>
      <rect x={area.x} y={area.y} width={area.width} height={area.height} rx="3" />
      {#each text as line, n (n)}
        <text
          x={area.x + area.width / 2}
          y={area.y + area.height / 2 + (n - (text.length - 1) / 2) * 11}
        >{line}</text>
      {/each}
    </g>
  {/each}
</svg>

<style>
  .map {
    display: block;
    width: 100%;
    margin: 0 auto;
    height: auto;
    max-height: 16rem;
  }
  rect {
    fill: var(--surface);
    stroke: var(--muted);
    stroke-width: 1;
  }
  text {
    font: 600 10.2px var(--sans);
    fill: var(--zone-ink);
    text-anchor: middle;
    dominant-baseline: central;
  }
  .on rect {
    fill: var(--accent-soft);
    stroke: var(--accent);
    stroke-width: 2.5;
  }
  .on text {
    fill: var(--accent);
    font-weight: 700;
  }
</style>
