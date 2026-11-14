<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

A phase in Checklist view (Checklist board): where the phase sits and its title, "N open steps"
with the open-steps list, the lists as cards (a challenge-and-response checklist folded until
the procedure is done), and buttons to the phases either side. Flow view shows the same phase one
step at a time (lib/FlowView.svelte).
-->
<script lang="ts">
  import type { Checklist, Detail, FillField, Flight, OpenStep, PhaseView } from "@chocktochock/bindings";
  import OpenStepsPop from "../lib/OpenStepsPop.svelte";
  import Icon from "../lib/Icon.svelte";
  import ListCard from "../lib/ListCard.svelte";
  import ArrivalData from "../lib/ArrivalData.svelte";
  import OptionChoices from "../lib/OptionChoices.svelte";
  import type { PackIndex } from "../pack-index";
  import { phaseHref } from "../route.svelte";
  import { decisionKey, type Decisions } from "../lib/decision";

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
    open = [],
    onGo,
    onEssentials,
    decisions,
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
    /** Every open step, for "N open steps". */
    open?: OpenStep[];
    /** Goes to a step (the open-steps list). */
    onGo?: ((id: string) => void) | undefined;
    onEssentials?: ((id: string, add: boolean) => void) | undefined;
    /** Decision steps and the steps that depend on them (decision 0022). */
    decisions?: Decisions | undefined;
  } = $props();
  /** The phase's procedure is not done yet: its checklist stays folded (Checklist board). */
  let procedureOpen = $derived(view.lists.some((l) => l.kind === "flow" && !l.complete));

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
    // A decision step that asks takes 1, 2 … and Space (Decision.dc.html).
    const asking = at >= 0 ? index.step(rows[at]?.id ?? "") : undefined;
    if (asking && decisions && decisionKey(e.key, asking, decisions)) {
      e.preventDefault();
      return;
    }
    if (e.key === "ArrowDown" || e.key === "ArrowUp" || e.key === "ArrowLeft") {
      // ← back, as the current step's hint says: the step before.
      const next = rows[Math.min(rows.length - 1, Math.max(0, at + (e.key === "ArrowDown" ? 1 : -1)))];
      if (next) {
        e.preventDefault();
        onSelect(next.id);
      }
    } else if ((e.key === "s" || e.key === "S") && at >= 0) {
      const row = rows[at];
      if (row && row.state !== "done") {
        e.preventDefault();
        onSkip(row.id);
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

<div class="title">
  <div>
    {#if view.stage && view.position}
      <p class="crumb">{view.stage} · Phase {view.position[0]} of {view.position[1]}</p>
    {/if}
    <h1>{view.title}</h1>
  </div>
  {#if onGo}<OpenStepsPop {index} {open} {onGo} />{/if}
</div>

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
    {onEssentials}
    folded={list.kind === "checklist" && procedureOpen}
    {decisions}
  />
{/each}

<nav class="nav" aria-label="Phases either side">
  {#if view.previous}
    <a class="btn secondary" href={phaseHref(view.previous.id)}><Icon name="arrow-left" /> {view.previous.title}</a>
  {:else}
    <!-- The first phase leads back to Flight setup, as in 3.0. -->
    <a class="btn secondary" href="#/setup"><Icon name="arrow-left" /> Flight setup</a>
  {/if}
  {#if view.next}
    <a class="btn secondary" href={phaseHref(view.next.id)}>{view.next.title} <Icon name="arrow-right" /></a>
  {/if}
</nav>

<style>
  .title {
    display: flex;
    align-items: flex-end;
    justify-content: space-between;
    gap: var(--space-4);
    margin-bottom: var(--space-4);
  }
  .crumb {
    margin: 0 0 var(--space-1);
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
  }
  h1 {
    margin: 0;
    font-size: var(--font-size-page-title);
    font-weight: var(--weight-bold);
    line-height: var(--leading-tight);
  }
  .nav {
    display: flex;
    justify-content: space-between;
    gap: var(--space-3);
    margin-top: var(--space-2);
  }
</style>
