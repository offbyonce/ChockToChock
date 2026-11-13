<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

One list as a card (Checklist board): the title with its done and skipped counts and, once a step
is ticked, Clear ticks (Undo after, owner); the steps, the done ones at the top folded to one line
with Show; and at the end the completion call (3.0). A challenge-and-response checklist stays
folded to one line with its count until the phase's procedure is done.
-->
<script lang="ts">
  import type { Checklist, Detail, Flight, PhaseList } from "@chocktochock/bindings";
  import { marksStep } from "./marked-step";
  import type { PackIndex } from "../pack-index";
  import Icon from "./Icon.svelte";
  import { stepTitle } from "./sentence";
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
    onBlank,
    onSelect,
    onClear,
    onEssentials,
    folded = false,
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
    onBlank: (id: string, index: number, value: string) => void;
    onSelect: (id: string) => void;
    onClear: (list: string) => void;
    /** Adds a step to Essentials or removes it (the row menu). */
    onEssentials?: ((id: string, add: boolean) => void) | undefined;
    /** A checklist whose procedure is not done yet: folded to one line until opened. */
    folded?: boolean;
  } = $props();

  let touched = $derived(list.progress.done + list.progress.skipped > 0);
  /** Showing the whole list in Essentials, as 3.0's "Show 3 more steps from Full". */
  let more = $state(false);
  let rows = $derived(more && fullList ? fullList.steps : list.steps);

  /** The done steps at the top fold into one line, with Show (Checklist board). */
  let lead = $derived.by(() => {
    let n = 0;
    while (rows[n]?.state === "done") n++;
    return n;
  });
  let showDone = $state(false);
  let shown = $derived(showDone || lead < 2 ? rows : rows.slice(lead));
  let first = $derived(lead ? index.step(rows[0]?.id ?? "") : undefined);
  let last = $derived(lead ? index.step(rows[lead - 1]?.id ?? "") : undefined);
  // svelte-ignore state_referenced_locally
  let openList = $state(!folded);
  $effect(() => {
    if (!folded) openList = true;
  });
  let counts = $derived(
    `${list.progress.done} of ${list.progress.total} done${list.progress.skipped ? ` · ${list.progress.skipped} skipped` : ""}`,
  );
</script>

{#if !openList}
  <!-- Folded until the procedure is done (Checklist board): one line with its count. -->
  <button class="card folded" type="button" aria-expanded="false" onclick={() => (openList = true)}>
    <span class="ft">{list.title}</span>
    <span class="fd">Challenge and response · opens when the procedure is done</span>
    <span class="fill"></span>
    <span class="fd">{list.progress.done} of {list.progress.total} ⌄</span>
  </button>
{:else}
<section class="card" aria-labelledby="list-{list.id}">
  <header>
    <h2 id="list-{list.id}">{list.title}</h2>
    <span class="count">{counts}</span>
    {#if touched}<button class="clear" type="button" onclick={() => onClear(list.id)}>Clear ticks</button>{/if}
  </header>
  {#if source?.when || source?.why}
    <div class="when">
      {#if source?.when}<p><b>When</b> · {source.when}</p>{/if}
      {#if source?.why}<p><b>Why</b> · {source.why}</p>{/if}
    </div>
  {/if}
  {#if lead >= 2}
    <div class="done-group">
      <span class="ok"><Icon name="check" /></span>
      <span class="dg">{lead} done · {first ? stepTitle(index, first) : ""} to {last ? stepTitle(index, last) : ""}</span>
      <button class="show" type="button" aria-expanded={showDone} onclick={() => (showDone = !showDone)}>{showDone ? "Hide" : "Show"}</button>
    </div>
  {/if}
  <ol class="items">
    {#each shown as row (row.id)}
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
          marked={marksStep(selected, row.id, row.state)}
          onTick={() => onTick(row.id, row.state === "done")}
          onSkip={() => onSkip(row.id)}
          onUnskip={() => onUnskip(row.id)}
          onBlank={(n, v) => onBlank(row.id, n, v)}
          onSelect={() => onSelect(row.id)}
          onEssentials={onEssentials ? (add) => onEssentials(row.id, add) : undefined}
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
{/if}

<style>
  /* The card header (Checklist board): 52 tall, title and counts. */
  header {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    min-height: calc(var(--size-touch-min) + var(--space-2));
    padding: 0 var(--space-4) 0 calc(var(--space-4) + var(--space-1));
    border-bottom: var(--border-thin) solid var(--color-line);
  }
  h2 {
    flex: 1;
    margin: 0;
    font-size: var(--font-size-body);
    font-weight: var(--weight-semibold);
  }
  .count {
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
  }
  .clear {
    min-height: var(--size-touch-min);
    padding: 0 var(--space-1);
    border: 0;
    background: none;
    color: var(--color-link);
    font: inherit;
    font-size: var(--font-size-secondary);
  }
  .done-group {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    min-height: var(--size-touch-min);
    padding: 0 var(--space-4) 0 calc(var(--space-4) + var(--space-1));
    border-bottom: var(--border-thin) solid var(--color-line);
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
  }
  .ok {
    display: inline-flex;
    color: var(--color-done);
  }
  .ok :global(svg) {
    width: var(--size-icon-small);
    height: var(--size-icon-small);
  }
  .dg {
    flex: 1;
  }
  .show {
    min-height: var(--size-touch-min);
    border: 0;
    background: none;
    color: var(--color-link);
    font: inherit;
  }
  .folded {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    width: 100%;
    min-height: calc(var(--size-touch-min) + var(--space-2));
    padding: 0 var(--space-4) 0 calc(var(--space-4) + var(--space-1));
    color: var(--color-text);
    font: inherit;
    text-align: left;
  }
  .ft {
    font-size: var(--font-size-body);
    font-weight: var(--weight-semibold);
  }
  .fd {
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
  }
  .fill {
    flex: 1;
  }
  .when {
    margin: 0;
    padding: var(--space-2) var(--space-4);
    border-bottom: var(--border-thin) solid var(--color-border);
    background: var(--color-bg);
    font-size: var(--font-size-secondary);
  }
  .when p {
    margin: 0;
  }
  .when p + p {
    margin-top: var(--space-1);
  }
  .items {
    margin: 0;
    padding: 0;
    list-style: none;
  }
  .more {
    min-height:  var(--size-touch-min);
    padding: 0 var(--space-4) var(--space-2);
    color: var(--color-accent);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-medium);
    text-align: left;
  }
  .call {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    margin: 0;
    padding: var(--space-4) var(--space-4);
    border-top: var(--border-thin) solid var(--color-border);
    font-weight: var(--weight-semibold);
  }
  .call.done {
    background: var(--color-surface-2);
    color: var(--color-done);
    font-size: var(--font-size-body);
    text-transform: uppercase;
  }
  .call.open {
    background: var(--color-open-bg);
    color: var(--color-open-text);
  }
</style>
