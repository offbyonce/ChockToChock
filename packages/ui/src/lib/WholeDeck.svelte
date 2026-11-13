<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The whole flight deck from the pack's overview, the zone's region outlined and the zone marked:
behind the slide-out's "Whole flight deck" disclosure until the Flight deck page (batch 6, owner).
Panel palette; map units are about one to a pixel here, so labels stay at 11 px or more.
-->
<script lang="ts">
  import type { Zone } from "@chocktochock/bindings";
  import type { PackIndex } from "../pack-index";

  let { index, zone }: { index: PackIndex; zone: Zone } = $props();

  let overview = $derived(index.overview);
  let region = $derived(index.region(zone)?.region ?? null);
  let byId = $derived(new Map(index.zones.map((z) => [z.id, z])));
</script>

{#if overview}
  <p class="zone">{zone.title}, marked on the map.</p>
  <svg class="deck" viewBox="0 0 {overview.width} {overview.height}" role="img" aria-label="The whole flight deck, {zone.title} marked">
    {#if region}
      <rect class="region" x={region.x} y={region.y} width={region.width} height={region.height} rx="6" />
    {/if}
    {#each overview.areas as area (area.zone + area.x + area.y)}
      <rect class="area" class:on={area.zone === zone.id} x={area.x} y={area.y} width={area.width} height={area.height} rx="2" />
    {/each}
    {#each overview.captions ?? [] as caption (caption.text)}
      <text class="cap" x={caption.x} y={caption.y}>{caption.text}</text>
    {/each}
    {#each overview.areas.filter((a) => a.zone === zone.id) as area (area.x + area.y)}
      <text class="mark" x={area.x + area.width / 2} y={area.y + area.height / 2}>{area.label ?? byId.get(area.zone)?.short_title}</text>
    {/each}
  </svg>
{/if}

<style>
  .zone {
    margin: 0;
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
  }
  .deck {
    display: block;
    width: 100%;
    border-radius: var(--radius-panel);
    background: var(--panel-bg);
  }
  .area {
    fill: var(--panel-fill);
    stroke: var(--panel-stroke);
  }
  .area.on {
    fill: var(--panel-changed);
    stroke: var(--panel-changed);
  }
  .region {
    fill: none;
    stroke: var(--panel-changed);
    stroke-width: 2.5;
    stroke-dasharray: 6 4;
  }
  text {
    font-family: var(--font-sans);
    font-weight: var(--weight-bold);
  }
  .cap {
    font-size: 11.5px;
    fill: var(--panel-label);
  }
  .mark {
    font-size: 11.5px;
    fill: var(--color-on-accent);
    text-anchor: middle;
    dominant-baseline: central;
  }
</style>
