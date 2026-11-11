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
    onSelect,
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
    onSelect: (id: string) => void;
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
        {onSelect}
      />
    </details>
  {/each}
{/if}

<style>
  h1 {
    margin: 0 0 1.125rem;
    font-size: 1.6875rem;
    font-weight: 650;
    line-height: 1.2;
    letter-spacing: -0.02em;
  }
  .tabs {
    display: grid;
    grid-auto-columns: 1fr;
    grid-auto-flow: column;
    gap: 0.25rem;
    margin-bottom: 1rem;
    padding: 0.25rem;
    border: 1px solid var(--line);
    border-radius: 0.75rem;
    background: transparent;
  }
  .tabs a {
    display: grid;
    place-items: center;
    min-height: 2.75rem;
    padding: 0 0.5rem;
    border-radius: 0.5rem;
    color: var(--muted);
    font-weight: 500;
    text-align: center;
    text-decoration: none;
  }
  .tabs a[aria-current="page"] {
    background: var(--surface);
    box-shadow: 0 1px 2px rgb(0 0 0 / 20%);
    color: var(--accent);
  }
  .intro {
    padding: 0.75rem 1rem;
    font-size: 0.9375rem;
  }
  .intro p {
    margin: 0.25rem 0;
  }
  summary {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    min-height: 3rem;
    padding: 0.5rem 1rem;
    font-size: 0.9375rem;
    font-weight: 600;
    list-style: none;
    cursor: pointer;
  }
  summary::-webkit-details-marker {
    display: none;
  }
  .n {
    margin-left: auto;
    color: var(--muted);
    font-size: 0.8125rem;
    font-weight: 400;
  }
  details[open] summary {
    border-bottom: 1px solid var(--line);
  }
  summary :global(.icon) {
    color: var(--muted);
    font-size: 1.15rem;
    transition: transform 0.12s;
  }
  summary :global(.icon svg) {
    stroke-width: 2.5;
  }
  details[open] summary :global(.icon) {
    transform: rotate(180deg);
  }
  .intro b {
    font-weight: 600;
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
