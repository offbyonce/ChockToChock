<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Flight setup, laid out as in 3.0: the Start card, then the flight values in five groups that the
steps fill in wherever they need them. Every change goes to the core as an action and is saved
with the flight. SimBrief import joins this page in a later step of batch 2; the iFly panel
state picker comes with panel states in batch 3 (decision 0019).
-->
<script lang="ts">
  import type { FillField, Flight } from "@chocktochock/bindings";
  import FieldInput from "../lib/FieldInput.svelte";
  import { setupGroups } from "../lib/flight-fields";
  import { phaseHref } from "../route.svelte";

  let {
    flight,
    first,
    onSet,
    onOption,
  }: {
    flight: Flight;
    /** The flight's first phase, where "Start at" leads. */
    first: { id: string; title: string } | null;
    onSet: (field: FillField, value: string) => void;
    onOption: (option: string, value: string) => void;
  } = $props();

  let values = $derived(flight.values as Partial<Record<FillField, string>>);
  let groups = $derived(setupGroups(values));
  let start = $derived(flight.options.start ?? "cold");
  const START: [string, string][] = [
    ["cold", "Cold and dark"],
    ["turn", "Turnaround"],
  ];
  /** On a phone only the first group starts open, as in 3.0. */
  const narrow = typeof window !== "undefined" && window.innerWidth <= 760;
</script>

<h1>Flight setup</h1>
<p class="lead">Enter your numbers once. The checklist fills them in wherever a step needs them.</p>
<section class="card">
  <div class="set">
    <div>
      <b id="set-start">Start</b>
      <p>Turnaround skips Power up, for a next leg with the airplane already powered.</p>
    </div>
    <div class="seg" role="radiogroup" aria-labelledby="set-start">
      {#each START as [v, label] (v)}
        <button type="button" role="radio" aria-checked={start === v} onclick={() => onOption("start", v)}>{label}</button>
      {/each}
    </div>
  </div>
</section>
{#each groups as g, i (g.title)}
  <details class="card sg" open={!narrow || i === 0}>
    <summary>{g.title}</summary>
    <div class="sgrid">
      {#each g.fields as spec (spec.field)}
        <FieldInput {spec} {values} {onSet} idPrefix="s" />
      {/each}
    </div>
  </details>
{/each}
{#if first}
  <div class="nav"><a class="btn primary" href={phaseHref(first.id)}>Start at {first.title} →</a></div>
{/if}

<style>
  h1 {
    margin: 0 0 var(--space-2);
    font-size: var(--font-size-section-title);
    font-weight: 650;
  }
  .lead {
    max-width: 66ch;
    margin: 0 0 var(--space-4);
    color: var(--color-text-muted);
  }
  .set {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    gap: var(--space-4) var(--space-8);
    padding: var(--space-4);
  }
  .set b {
    font-weight: var(--weight-semibold);
  }
  .set p {
    margin: var(--space-1) 0 0;
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
  }
  .seg {
    display: grid;
    grid-auto-columns: 1fr;
    grid-auto-flow: column;
    gap: var(--space-1);
    padding: var(--space-1);
    border-radius: var(--radius-control);
    background: var(--color-surface-2);
  }
  .seg button {
    min-width: 0;
    min-height:  var(--size-touch-min);
    padding: 0 var(--space-3);
    border-radius: var(--radius-control);
    color: var(--color-text-muted);
    font-weight: var(--weight-medium);
  }
  .seg button[aria-checked="true"] {
    background: var(--color-surface);
    box-shadow: inset 0 0 0 var(--size-focus-ring) var(--color-accent);
    color: var(--color-text);
  }
  summary {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-4);
    min-height:  var(--size-touch-min);
    box-sizing: border-box;
    padding: var(--space-4) var(--space-4);
    cursor: pointer;
    font-weight: var(--weight-semibold);
    list-style: none;
  }
  summary::-webkit-details-marker {
    display: none;
  }
  /* 3.0's chevron: down when closed, up when open, and a rule under an open group's title. */
  summary::after {
    content: "";
    flex: none;
    width: var(--size-dot);
    height: var(--size-dot);
    margin-right: var(--space-1);
    border-right: var(--border-thick) solid var(--color-text-muted);
    border-bottom: var(--border-thick) solid var(--color-text-muted);
    transform: rotate(45deg);
  }
  details[open] > summary {
    border-bottom: var(--border-thin) solid var(--color-border);
  }
  details[open] > summary::after {
    transform: rotate(-135deg);
  }
  .sgrid {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: var(--space-4);
    padding: var(--space-4);
  }
  .nav {
    display: flex;
    justify-content: flex-end;
    margin-top: var(--space-4);
  }
  @media (max-width: 899px) {
    .sgrid {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }
  @media (max-width: 599px) {
    .set,
    .sgrid {
      grid-template-columns: 1fr;
    }
  }
</style>
