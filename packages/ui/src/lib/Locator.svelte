<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

"Where is this?": where to look from the seat, the side and seat views (decision 0023) when the
pack has them, and the whole flight deck overview (our own layout) with the region outlined and
the zone marked, all from the pack, in the panel palette.
-->
<script lang="ts">
  import type { Zone } from "@chocktochock/bindings";
  import type { PackIndex } from "../pack-index";
  import Icon from "./Icon.svelte";
  import ViewDrawing from "./ViewDrawing.svelte";

  let { index, zone }: { index: PackIndex; zone: Zone } = $props();

  let dialog = $state<HTMLDialogElement | null>(null);
  let map = $derived(index.region(zone));
  let region = $derived(map?.region ?? null);
  let overview = $derived(index.overview);
  let byId = $derived(new Map(index.zones.map((z) => [z.id, z])));
</script>

{#if overview}
  <button class="where-btn" type="button" aria-haspopup="dialog" onclick={() => dialog?.showModal()}>
    <Icon name="map-pin" /> Where is this?
  </button>
  <dialog bind:this={dialog} aria-labelledby="locator-title" onclick={(e) => e.target === dialog && dialog?.close()}>
    <div class="head">
      <h2 id="locator-title">{region?.name ?? zone.area}{#if region?.code}<span class="code"> · {region.code}</span>{/if}</h2>
      <button class="close" type="button" aria-label="Close" onclick={() => dialog?.close()}><Icon name="x" /></button>
    </div>
    {#if region?.locate}<p>{region.locate}</p>{/if}
    {#if overview.views?.side || overview.views?.seat}
      <div class="views">
        {#if overview.views?.side}
          <ViewDrawing frame={overview.views.side} region={region?.name ?? null} kind="side" title="From the side: how high, how far" />
        {/if}
        {#if overview.views?.seat}
          <ViewDrawing frame={overview.views.seat} region={region?.name ?? null} kind="seat" title="From the captain's seat: which side" />
        {/if}
      </div>
    {/if}
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
    <p class="tiny">Schematic, not to scale.</p>
  </dialog>
{/if}

<style>
  .where-btn {
    position: relative;
    display: inline-flex;
    align-items: center;
    gap: var(--space-2);
    margin-left: auto;
    padding: 0 var(--space-3);
    border: var(--border-thin) solid var(--color-border-strong);
    border-radius: var(--radius-pill);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
    white-space: nowrap;
  }
  .where-btn:hover {
    border-color: var(--color-text-muted);
  }
  dialog {
    width: min(var(--size-content-max), calc(100vw - 2 * var(--gutter)));
    max-height: calc(100dvh - 2 * var(--gutter));
    padding: var(--space-4) var(--space-6);
    border: var(--border-thin) solid var(--color-border-strong);
    border-radius: var(--radius-sheet);
    background: var(--color-sheet);
    color: var(--color-text);
  }
  dialog::backdrop {
    background: var(--color-scrim);
  }
  .head {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }
  h2 {
    margin: 0;
    font-size: var(--font-size-panel-title);
    font-weight: var(--weight-semibold);
  }
  .code {
    color: var(--color-text-muted);
    font: var(--weight-regular) var(--font-size-secondary) var(--font-mono);
  }
  .close {
    display: grid;
    place-items: center;
    margin-left: auto;
    border-radius: var(--radius-control);
    color: var(--color-text-muted);
  }
  p {
    margin: var(--space-2) 0;
    font-size: var(--font-size-body);
  }
  .views {
    display: grid;
    gap: var(--space-3);
    margin: var(--space-3) 0;
  }
  @media (min-width: 900px) {
    .views {
      grid-template-columns: 1fr 1fr;
    }
  }
  .zone {
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
  }
  /* The whole flight deck, on the panel palette. */
  .deck {
    display: block;
    width: 100%;
    max-height: 55vh;
    margin: var(--space-2) 0;
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
  /* Map units, about one to a pixel in this dialog: 11 or more. */
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
  .tiny {
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
</style>
