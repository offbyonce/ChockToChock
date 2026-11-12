<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

A phase in Checklist view, as in 3.0: the flight strip, where the phase sits, the title, the
lists as cards, and buttons to the phases either side. Flow view shows the same phase one step at
a time (lib/FlowView.svelte).
-->
<script lang="ts">
  import type { Checklist, Detail, FillField, Flight, PhaseView } from "@chocktochock/bindings";
  import Icon from "../lib/Icon.svelte";
  import ListCard from "../lib/ListCard.svelte";
  import ArrivalData from "../lib/ArrivalData.svelte";
  import OptionChoices from "../lib/OptionChoices.svelte";
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
    onUnskip,
    onBlank,
    onSelect,
    onClear,
    full = true,
    fullView = null,
    onOption,
    onValue,
  }: {
    index: PackIndex;
    view: PhaseView;
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
    /** In Essentials, this phase in Full. */
    fullView?: PhaseView | null;
    onOption: (option: string, value: string) => void;
    /** A flight value typed in a box on the page, such as Arrival data. */
    onValue: (field: FillField, value: string) => void;
  } = $props();

  let asked = $derived(index.options.filter((o) => (o.ask_in ?? []).includes(view.id)));

  /**
   * Keys, as in 3.0's Flow view: Down and Up arrows move between steps, Space ticks the selected
   * step (the current one if none is selected). Ignored while typing or on a control.
   */
  let rows = $derived(view.lists.flatMap((l) => l.steps));
  function onKey(e: KeyboardEvent): void {
    if (e.altKey || e.ctrlKey || e.metaKey) return;
    const t = e.target;
    if (t instanceof Element && t.closest("input, textarea, select, button, a, summary, [contenteditable]")) return;
    const at = rows.findIndex((r) => r.id === (selected ?? view.lists.find((l) => l.current)?.current));
    if (e.key === "ArrowDown" || e.key === "ArrowUp") {
      const next = rows[Math.min(rows.length - 1, Math.max(0, at + (e.key === "ArrowDown" ? 1 : -1)))];
      if (next) {
        e.preventDefault();
        onSelect(next.id);
      }
    } else if (e.key === " " && at >= 0) {
      const row = rows[at];
      if (row) {
        e.preventDefault();
        onTick(row.id, row.state === "done");
      }
    }
  }
</script>

<svelte:window onkeydown={onKey} />

{#if view.stage && view.position}
  <p class="crumb">{view.stage} · Phase {view.position[0]} of {view.position[1]}</p>
{/if}
<h1>{view.title}</h1>

{#each asked as option (option.id)}
  <OptionChoices
    {option}
    value={flight.options[option.id] ?? option.default}
    onChoose={(value) => onOption(option.id, value)}
  />
{/each}

{#if view.arrival}
  <ArrivalData values={flight.values} onSet={onValue} />
{/if}

{#each view.lists as list (list.id)}
  <ListCard
    {index}
    {list}
    source={lists.get(list.id)}
    {flight}
    {detail}
    {selected}
    large={list.kind === "non-normal"}
    {full}
    fullList={fullView?.lists.find((l) => l.id === list.id) ?? null}
    {onTick}
    {onSkip}
    {onUnskip}
    {onBlank}
    {onSelect}
    {onClear}
  />
{/each}

<nav class="nav" aria-label="Phases either side">
  {#if view.previous}
    <a class="btn" href={phaseHref(view.previous.id)}><Icon name="arrow-left" /> {view.previous.title}</a>
  {:else}
    <!-- The first phase leads back to Flight setup, as in 3.0. -->
    <a class="btn" href="#/setup"><Icon name="arrow-left" /> Flight setup</a>
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
