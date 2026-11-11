<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

A phase in Checklist view, as in 3.0: the flight strip, where the phase sits, the title, the
lists as cards, and buttons to the phases either side. Flow view shows the same phase one step at
a time (lib/FlowView.svelte).
-->
<script lang="ts">
  import type { Checklist, Detail, Flight, PhaseView } from "@chocktochock/bindings";
  import Icon from "../lib/Icon.svelte";
  import ListCard from "../lib/ListCard.svelte";
  import type { PackIndex } from "../pack-index";
  import { phaseHref } from "../route.svelte";

  let {
    index,
    view,
    lists,
    flight,
    detail,
    selected,
    onTick,
    onSkip,
    onSelect,
  }: {
    index: PackIndex;
    view: PhaseView;
    lists: Map<string, Checklist>;
    flight: Flight;
    detail: Detail;
    selected: string | null;
    onTick: (id: string, done: boolean) => void;
    onSkip: (id: string) => void;
    onSelect: (id: string) => void;
  } = $props();
</script>

{#if view.stage && view.position}
  <p class="crumb">{view.stage} · Phase {view.position[0]} of {view.position[1]}</p>
{/if}
<h1>{view.title}</h1>

{#each view.lists as list (list.id)}
  <ListCard
    {index}
    {list}
    source={lists.get(list.id)}
    {flight}
    {detail}
    {selected}
    large={list.kind === "non-normal"}
    {onTick}
    {onSkip}
    {onSelect}
  />
{/each}

<nav class="nav" aria-label="Phases either side">
  {#if view.previous}
    <a class="btn" href={phaseHref(view.previous.id)}><Icon name="arrow-left" /> {view.previous.title}</a>
  {:else}
    <span></span>
  {/if}
  {#if view.next}
    <a class="btn primary" href={phaseHref(view.next.id)}>{view.next.title} <Icon name="arrow-right" /></a>
  {/if}
</nav>

<style>
  .crumb {
    margin: 0 0 0.125rem;
    color: var(--muted);
    font-size: 0.8125rem;
  }
  h1 {
    margin: 0 0 1.125rem;
    font-size: 1.6875rem;
    font-weight: 650;
    line-height: 1.2;
    letter-spacing: -0.02em;
  }
  .nav {
    display: flex;
    justify-content: space-between;
    gap: 0.625rem;
    margin-top: 0.375rem;
  }
</style>
