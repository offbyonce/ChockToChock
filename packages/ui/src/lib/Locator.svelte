<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

"Where is this?", as in 3.0: the whole flight deck overview (our own layout) with the region
outlined and the zone marked, and where to look from the seat, from the pack.
-->
<script lang="ts">
  import type { Zone } from "@chocktochock/bindings";
  import type { PackIndex } from "../pack-index";
  import Icon from "./Icon.svelte";

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
    gap: 0.375rem;
    margin-left: auto;
    padding: 0.375rem 0.75rem;
    border: 1px solid var(--line);
    border-radius: 99rem;
    font-size: 0.8125rem;
    font-weight: 600;
    white-space: nowrap;
  }
  .where-btn::after {
    position: absolute;
    inset: 50% 0 auto;
    height: 2.75rem;
    content: "";
    transform: translateY(-50%);
  }
  .where-btn:hover {
    border-color: var(--muted);
  }
  dialog {
    width: min(34rem, calc(100vw - 2rem));
    max-height: calc(100vh - 2rem);
    padding: 1rem 1.25rem;
    border: 1px solid var(--line);
    border-radius: 0.875rem;
    background: var(--surface);
    color: var(--ink);
    box-shadow: 0 18px 44px rgb(0 0 0 / 35%);
  }
  dialog::backdrop {
    background: rgb(0 0 0 / 45%);
  }
  .head {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  h2 {
    margin: 0;
    font-size: 1.0625rem;
    font-weight: 600;
  }
  .code {
    color: var(--muted);
    font: 0.8125rem var(--mono);
  }
  .close {
    display: grid;
    place-items: center;
    width: 2.75rem;
    height: 2.75rem;
    margin: -0.5rem -0.75rem -0.5rem auto;
    border-radius: 0.5rem;
    color: var(--muted);
  }
  p {
    margin: 0.5rem 0;
    font-size: 0.9375rem;
  }
  .zone {
    color: var(--muted);
    font-size: 0.875rem;
  }
  .deck {
    display: block;
    width: 100%;
    max-height: 55vh;
    margin: 0.5rem 0;
  }
  .area {
    fill: var(--zone);
    stroke: var(--line);
  }
  .area.on {
    fill: var(--accent);
    stroke: var(--accent);
  }
  .region {
    fill: none;
    stroke: var(--accent);
    stroke-width: 2.5;
    stroke-dasharray: 6 4;
  }
  text {
    font-family: var(--sans);
    font-weight: 700;
  }
  .cap {
    font-size: 9px;
    letter-spacing: 0.06em;
    fill: var(--muted);
  }
  .mark {
    font-size: 9px;
    fill: var(--on-accent);
    text-anchor: middle;
    dominant-baseline: central;
  }
  .tiny {
    color: var(--muted);
    font-size: 0.75rem;
  }
</style>
