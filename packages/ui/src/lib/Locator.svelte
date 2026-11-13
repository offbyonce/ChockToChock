<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

"Where is this?" (WherePop board, decision 0023). Beside a side pane (tablet landscape and up) it
is a pop-over that opens leftward over the step list, its arrow at the link, so the side pane with
Check, If not and Why stays in view; elsewhere (tablet portrait, phone) it is a dialog until the
TabWhere and PhoneWhere boards arrive in batch 5. Inside: the region's title and where to look
from the seat, the side and seat views, the legend, and the whole flight deck behind "Whole flight
deck" until the Flight deck page exists (batch 6). All from the pack, in the panel palette.
-->
<script lang="ts">
  import type { Zone } from "@chocktochock/bindings";
  import type { PackIndex } from "../pack-index";
  import Icon from "./Icon.svelte";
  import ViewDrawing from "./ViewDrawing.svelte";

  let {
    index,
    zone,
    link = false,
  }: {
    index: PackIndex;
    zone: Zone;
    /** Drawn as a plain link (the side pane's "Where is this?"), not a pill button. */
    link?: boolean;
  } = $props();

  let dialog = $state<HTMLDialogElement | null>(null);
  let trigger = $state<HTMLButtonElement | null>(null);
  let heading = $state<HTMLHeadingElement | null>(null);
  let open = $state(false);
  let popover = $state(false);
  let deck = $state(false);
  let phone = $state(false);
  /** Where the pop-over sits, and where its arrow points, in pixels. */
  let place = $state({ top: 0, right: 0, arrow: 0 });

  let map = $derived(index.region(zone));
  let region = $derived(map?.region ?? null);
  let overview = $derived(index.overview);
  let byId = $derived(new Map(index.zones.map((z) => [z.id, z])));
  let title = $derived((phone ? region?.short_title : region?.title) ?? region?.title ?? "Where is this?");

  function show() {
    if (!dialog || !trigger) return;
    phone = !window.matchMedia("(min-width: 600px)").matches;
    const pane = trigger.closest("aside");
    popover = !!pane && window.matchMedia("(min-width: 900px)").matches;
    deck = false;
    if (popover && pane) {
      dialog.show();
      position(pane);
    } else {
      dialog.showModal();
    }
    open = true;
    heading?.focus();
  }
  /** Beside the pane, its arrow on the link: the board's arrow sits 168 + 6 px from the top. */
  function position(pane: Element) {
    if (!dialog || !trigger) return;
    const link = trigger.getBoundingClientRect();
    const paneLeft = pane.getBoundingClientRect().left;
    const mid = link.top + link.height / 2;
    const height = dialog.offsetHeight;
    const top = Math.max(8, Math.min(mid - 174, window.innerHeight - height - 8));
    place = { top, right: window.innerWidth - paneLeft + 8, arrow: mid - top - 6 };
  }
  function close() {
    dialog?.close();
  }
  $effect(() => {
    if (!open) return;
    const onKey = (e: KeyboardEvent) => e.key === "Escape" && close();
    const onDown = (e: PointerEvent) => {
      const t = e.target as Node;
      if (popover && dialog && !dialog.contains(t) && !trigger?.contains(t)) close();
    };
    window.addEventListener("keydown", onKey);
    window.addEventListener("pointerdown", onDown);
    return () => {
      window.removeEventListener("keydown", onKey);
      window.removeEventListener("pointerdown", onDown);
    };
  });
</script>

{#if overview}
  <button class={link ? "where-link" : "where-btn"} type="button" aria-haspopup="dialog" aria-expanded={open} bind:this={trigger} onclick={show}>
    {#if !link}<Icon name="map-pin" />{/if} Where is this?
  </button>
  <dialog
    bind:this={dialog}
    class:popover
    aria-labelledby="locator-title"
    style:top={popover ? `${place.top}px` : undefined}
    style:right={popover ? `${place.right}px` : undefined}
    onclose={() => {
      open = false;
      trigger?.focus();
    }}
    onclick={(e) => !popover && e.target === dialog && close()}
  >
    {#if popover}<span class="arrow" aria-hidden="true" style:top="{place.arrow}px"></span>{/if}
    <div class="head">
      <!-- Focus lands on the title, not the close button, so no focus ring shows on opening. -->
      <h2 id="locator-title" tabindex="-1" bind:this={heading}>{title}</h2>
      <button class="close" type="button" aria-label="Close" onclick={close}><Icon name="x" /></button>
    </div>
    {#if region?.locate}<p class="locate">{region.locate}</p>{/if}
    {#if overview.views?.side || overview.views?.seat}
      <div class="views">
        {#if overview.views?.side}
          <ViewDrawing frame={overview.views.side} region={region?.name ?? null} kind="side" title="From the side: how high, how far" />
        {/if}
        {#if overview.views?.seat}
          <ViewDrawing frame={overview.views.seat} region={region?.name ?? null} kind="seat" title="From the captain seat: which side" />
        {/if}
      </div>
    {/if}
    <div class="legend">
      <span class="key"><i class="sw here"></i>This area</span>
      <span class="key"><i class="sw other"></i>Other panels</span>
      <span class="fill"></span>
      <button class="deck-toggle" type="button" aria-expanded={deck} onclick={() => (deck = !deck)}>
        Whole flight deck <Icon name="chevron-down" />
      </button>
    </div>
    {#if deck}
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
  /* The side pane's link, as the board draws it. */
  .where-link {
    display: inline-flex;
    align-items: center;
    min-width: 0;
    color: var(--color-link);
    font-size: var(--font-size-secondary);
  }
  dialog {
    width: min(var(--size-content-max), calc(100vw - 2 * var(--gutter)));
    max-height: calc(100dvh - 2 * var(--gutter));
    padding: var(--space-4) var(--space-6) var(--space-4);
    border: var(--border-thin) solid var(--color-popover-border);
    border-radius: var(--radius-popover);
    background: var(--color-surface);
    color: var(--color-text);
    box-shadow: var(--color-overlay-shadow);
  }
  dialog[open] {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }
  dialog::backdrop {
    background: var(--color-scrim);
  }
  /* WherePop: beside the side pane, over the step list, as wide as the board's pop-over. */
  dialog.popover {
    position: fixed;
    left: auto;
    margin: 0;
    width: min(calc(var(--side-pane) + 8 * var(--space-8)), calc(100vw - var(--side-pane) - 2 * var(--gutter)));
    overflow: visible;
  }
  .arrow {
    position: absolute;
    right: calc(-1 * var(--size-dot));
    width: var(--space-3);
    height: var(--space-3);
    border-top: var(--border-thin) solid var(--color-popover-border);
    border-right: var(--border-thin) solid var(--color-popover-border);
    background: var(--color-surface);
    transform: rotate(45deg);
  }
  .head {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  h2 {
    outline: none;
    margin: 0;
    font-size: var(--font-size-panel-title);
    font-weight: var(--weight-bold);
  }
  .close {
    display: grid;
    place-items: center;
    margin-right: calc(-1 * var(--space-3));
    color: var(--color-text-secondary);
  }
  .locate {
    margin: 0;
    font-size: var(--font-size-body);
    line-height: var(--leading-normal);
  }
  .views {
    display: grid;
    gap: var(--space-3);
  }
  @media (min-width: 600px) {
    .views {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }
  .legend {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--space-6);
    color: var(--color-text-secondary);
    font-size: var(--font-size-caption);
  }
  .key {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }
  .sw {
    width: calc(var(--space-3) + var(--border-thick));
    height: calc(var(--space-2) + var(--border-thick));
    border: var(--border-thin) solid var(--panel-stroke);
    border-radius: var(--border-thick);
    background: var(--panel-fill);
  }
  .sw.here {
    border: var(--border-thick) solid var(--panel-changed);
    background: var(--panel-changed-fill);
  }
  .fill {
    flex: 1;
  }
  .deck-toggle {
    display: inline-flex;
    align-items: center;
    gap: var(--space-1);
    color: var(--color-link);
    font-size: var(--font-size-secondary);
  }
  .deck-toggle[aria-expanded="true"] :global(.icon) {
    transform: rotate(180deg);
  }
  .zone {
    margin: 0;
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
  }
  /* The whole flight deck, on the panel palette. */
  .deck {
    display: block;
    width: 100%;
    max-height: 55vh;
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
  /* Map units, about one to a pixel here: 11 or more. */
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
