<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Behind the Emergency button, as in 3.0: a tab for each reference phase (memory items, quick
reference, non-normal), its introduction, and its lists as cards that open. Steps here use larger
type (step 7 requirement for non-normal lists).
-->
<script lang="ts">
  import type { Checklist, Detail, Flight, PhaseView, RailPhase } from "@chocktochock/bindings";
  import Icon from "../lib/Icon.svelte";
  import ListCard from "../lib/ListCard.svelte";
  import type { PackIndex } from "../pack-index";

  let {
    index,
    phases,
    view,
    intro,
    lists,
    flight,
    detail,
    selected,
    onTick,
    onSkip,
    onUnskip,
    onBlank,
    onSelect,
    onClear,
    full = true,
  }: {
    index: PackIndex;
    phases: RailPhase[];
    view: PhaseView | null;
    intro: { heading?: string | null; text: string }[];
    lists: Map<string, Checklist>;
    flight: Flight;
    detail: Detail;
    selected: string | null;
    onTick: (id: string, done: boolean) => void;
    onSkip: (id: string) => void;
    onUnskip: (id: string) => void;
    onBlank: (id: string, index: number, value: string) => void;
    onSelect: (id: string) => void;
    onClear: (list: string) => void;
    full?: boolean;
  } = $props();

  // The notice is under every screen already; a pack introduction that repeats it is skipped.
  let shown = $derived(intro.filter((p) => p.heading || !/^Simulator training only\./.test(p.text)));
</script>

<h1>Emergency</h1>
<nav class="tabs" aria-label="Reference">
  {#each phases as p (p.id)}
    <a href="#/reference/{encodeURIComponent(p.id)}" aria-current={p.id === view?.id ? "page" : undefined}>{p.title}</a>
  {/each}
</nav>

{#if shown.length}
  <div class="intro card">
    {#each shown as para, i (i)}
      <p>{#if para.heading}<b>{para.heading}:</b>{" "}{/if}{para.text}</p>
    {/each}
  </div>
{/if}

{#if view}
  {#each view.lists as list (list.id)}
    <details class="card group">
      <summary>
        <span>{list.title}</span><span class="n mono">{list.steps.length} steps</span><Icon name="chevron-down" />
      </summary>
      <ListCard
        {index}
        {list}
        source={lists.get(list.id)}
        {flight}
        {detail}
        {selected}
        large
        {onTick}
        {onSkip}
        {onUnskip}
    {onBlank}
        {onSelect}
        {onClear}
      />
    </details>
  {/each}
{/if}

<style>
  h1 {
    margin: 0 0 var(--space-4);
    font-size: var(--font-size-section-title);
    font-weight: 650;
    line-height: 1.2;
  }
  .tabs {
    display: grid;
    grid-auto-columns: 1fr;
    grid-auto-flow: column;
    gap: var(--space-1);
    margin-bottom: var(--space-4);
    padding: var(--space-1);
    border: var(--border-thin) solid var(--color-border);
    border-radius: var(--radius-panel);
    background: transparent;
  }
  .tabs a {
    display: grid;
    place-items: center;
    min-height:  var(--size-touch-min);
    padding: 0 var(--space-2);
    border-radius: var(--radius-control);
    color: var(--color-text-muted);
    font-weight: var(--weight-medium);
    text-align: center;
    text-decoration: none;
  }
  .tabs a[aria-current="page"] {
    background: var(--color-surface);
    box-shadow: var(--color-segment-shadow);
    color: var(--color-accent);
  }
  .intro {
    padding: var(--space-3) var(--space-4);
    font-size: var(--font-size-body);
  }
  .intro p {
    margin: var(--space-1) 0;
  }
  summary {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    min-height: var(--size-touch-min);
    padding: var(--space-2) var(--space-4);
    font-size: var(--font-size-body);
    font-weight: var(--weight-semibold);
    list-style: none;
    cursor: pointer;
  }
  summary::-webkit-details-marker {
    display: none;
  }
  .n {
    margin-left: auto;
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-regular);
  }
  details[open] summary {
    border-bottom: var(--border-thin) solid var(--color-border);
  }
  summary :global(.icon) {
    color: var(--color-text-muted);
    font-size: var(--font-size-body);
    transition: transform 0.12s;
  }
  details[open] summary :global(.icon) {
    transform: rotate(180deg);
  }
  .intro b {
    font-weight: var(--weight-semibold);
  }
  /* The card inside a card drops its own frame and title row. */
  .group :global(section.card) {
    margin: 0;
    border: 0;
    border-radius: 0;
  }
  .group :global(section.card > header) {
    display: none;
  }
</style>
