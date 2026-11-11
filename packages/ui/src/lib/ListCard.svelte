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
    onTick,
    onSkip,
    onSelect,
  }: {
    index: PackIndex;
    list: PhaseList;
    source: Checklist | undefined;
    flight: Flight;
    detail: Detail;
    selected: string | null;
    large?: boolean;
    onTick: (id: string, done: boolean) => void;
    onSkip: (id: string) => void;
    onSelect: (id: string) => void;
  } = $props();

  const KIND = { checklist: "Checklist", flow: "Procedure", "non-normal": "Non-normal" } as const;
</script>

<section class="card" aria-labelledby="list-{list.id}">
  <header>
    <span class="tag {list.kind}">{KIND[list.kind]}</span>
    <h2 id="list-{list.id}">{list.title}</h2>
    <span class="count mono" aria-label="{list.progress.done} of {list.progress.total} done">
      {list.progress.done}/{list.progress.total}
    </span>
  </header>
  {#if source?.when}
    <p class="when"><b>When</b> · {source.when}</p>
  {/if}
  <ol class="items">
    {#each list.steps as row (row.id)}
      {@const step = index.step(row.id)}
      {#if step}
        <StepItem
          {index}
          {step}
          {row}
          {detail}
          {large}
          doneAt={flight.session.done[row.id]}
          selected={selected === row.id}
          onTick={() => onTick(row.id, row.state === "done")}
          onSkip={() => onSkip(row.id)}
          onSelect={() => onSelect(row.id)}
        />
      {/if}
    {/each}
  </ol>
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
