<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

One list as a card, as in 3.0: kind and title, the steps, and at the end the completion call
(step 7). The call shows only when no step is still open; skipped steps keep a list open and say
so instead.
-->
<script lang="ts">
  import type { Checklist, Detail, Flight, PhaseList } from "@chocktochock/bindings";
  import type { PackIndex } from "../pack-index";
  import Icon from "./Icon.svelte";
  import StepItem from "./StepItem.svelte";

  let {
    index,
    list,
    source,
    flight,
    detail,
    selected,
    large = false,
    full = true,
    fullList = null,
    onTick,
    onSkip,
    onUnskip,
    onSelect,
    onClear,
  }: {
    index: PackIndex;
    list: PhaseList;
    source: Checklist | undefined;
    flight: Flight;
    detail: Detail;
    selected: string | null;
    large?: boolean;
    /** Every step shows (not Essentials only): Essentials steps carry an E. */
    full?: boolean;
    /** In Essentials, this list in Full, for "Show N more steps from Full". */
    fullList?: PhaseList | null;
    onTick: (id: string, done: boolean) => void;
    onSkip: (id: string) => void;
    onUnskip: (id: string) => void;
    onSelect: (id: string) => void;
    onClear: (list: string) => void;
  } = $props();

  /** Clear asks once: the first press turns it into "Clear all?" for a few seconds. */
  let confirming = $state(false);
  let timer: ReturnType<typeof setTimeout> | undefined;
  function clear(): void {
    if (!confirming) {
      confirming = true;
      timer = setTimeout(() => (confirming = false), 4000);
      return;
    }
    clearTimeout(timer);
    confirming = false;
    onClear(list.id);
  }
  let touched = $derived(list.progress.done + list.progress.skipped > 0);
  /** Showing the whole list in Essentials, as 3.0's "Show 3 more steps from Full". */
  let more = $state(false);
  let rows = $derived(more && fullList ? fullList.steps : list.steps);

  const KIND = { checklist: "Checklist", flow: "Procedure", "non-normal": "Non-normal" } as const;
</script>

<section class="card" aria-labelledby="list-{list.id}">
  <header>
    <span class="tag {list.kind}">{KIND[list.kind]}</span>
    <h2 id="list-{list.id}">{list.title}</h2>
    <span class="count mono" aria-label="{list.progress.done} of {list.progress.total} done">
      {list.progress.done}/{list.progress.total}
    </span>
    <button class="clear" class:confirming type="button" disabled={!touched} onclick={clear}>
      {confirming ? "Clear all?" : "Clear"}
    </button>
  </header>
  {#if source?.when}
    <p class="when"><b>When</b> · {source.when}</p>
  {/if}
  <ol class="items">
    {#each rows as row (row.id)}
      {@const step = index.step(row.id)}
      {#if step}
        <StepItem
          {index}
          {step}
          {row}
          {detail}
          {large}
          full={full || more}
          doneAt={flight.session.done[row.id]}
          selected={selected === row.id}
          onTick={() => onTick(row.id, row.state === "done")}
          onSkip={() => onSkip(row.id)}
          onUnskip={() => onUnskip(row.id)}
          onSelect={() => onSelect(row.id)}
        />
      {/if}
    {/each}
  </ol>
  {#if list.hidden > 0 && fullList}
    <button class="more" type="button" aria-expanded={more} onclick={() => (more = !more)}>
      {more ? "Show Essentials only" : `Show ${list.hidden} more ${list.hidden === 1 ? "step" : "steps"} from Full`}
    </button>
  {/if}
  {#if list.complete}
    <p class="call done" role="status"><Icon name="check" /> {list.title} complete</p>
  {:else if list.progress.toDo === 0 && list.progress.skipped > 0}
    <p class="call open" role="status">
      <Icon name="alert-triangle" />
      {list.progress.skipped} skipped {list.progress.skipped === 1 ? "step" : "steps"} still open. {list.title} is not complete.
    </p>
  {/if}
</section>

<style>
  header {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    padding: 0.75rem 1rem;
    border-bottom: 1px solid var(--line);
  }
  h2 {
    margin: 0;
    font-size: 0.9375rem;
    font-weight: 600;
  }
  .tag {
    padding: 0.0625rem 0.5rem;
    border-radius: 99rem;
    background: var(--zone);
    color: var(--zone-ink);
    font-size: 0.7188rem;
    font-weight: 600;
    white-space: nowrap;
  }
  .tag.checklist {
    background: var(--go-soft);
    color: var(--go);
  }
  .tag.non-normal {
    background: var(--stop-soft);
    color: var(--stop);
  }
  .clear {
    position: relative;
    color: var(--muted);
    font-size: 0.8125rem;
  }
  .clear:hover:not(:disabled) {
    color: var(--ink);
  }
  .clear:disabled {
    opacity: 0.5;
  }
  .clear.confirming {
    color: var(--stop);
    font-weight: 600;
  }
  .clear::after {
    position: absolute;
    inset: 50% -0.5rem auto;
    height: 2.75rem;
    content: "";
    transform: translateY(-50%);
  }
  .count {
    margin-left: auto;
    color: var(--muted);
    font-size: 0.8125rem;
  }
  .when {
    margin: 0;
    padding: 0.6rem 1rem;
    border-bottom: 1px solid var(--line);
    background: var(--bg);
    font-size: 0.86rem;
  }
  .items {
    margin: 0;
    padding: 0;
    list-style: none;
  }
  .more {
    min-height: 2.75rem;
    padding: 0 1rem 0.5rem;
    color: var(--accent);
    font-size: 0.875rem;
    font-weight: 500;
    text-align: left;
  }
  .call {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin: 0;
    padding: 0.875rem 1rem;
    border-top: 1px solid var(--line);
    font-weight: 600;
  }
  .call.done {
    background: var(--go-soft);
    color: var(--go);
    font-size: 1.0625rem;
    letter-spacing: 0.02em;
    text-transform: uppercase;
  }
  .call.open {
    background: var(--warn-soft);
    color: var(--warn);
  }
</style>
