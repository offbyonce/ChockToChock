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
    margin: 0 0 0.5rem;
    font-size: 1.6875rem;
    font-weight: 650;
    letter-spacing: -0.02em;
  }
  .lead {
    max-width: 66ch;
    margin: 0 0 1.125rem;
    color: var(--muted);
  }
  .set {
    display: grid;
    grid-template-columns: minmax(0, 1fr) 15rem;
    align-items: center;
    gap: 1rem 2rem;
    padding: 1rem;
  }
  .set b {
    font-weight: 600;
  }
  .set p {
    margin: 0.125rem 0 0;
    color: var(--muted);
    font-size: 0.875rem;
  }
  .seg {
    display: grid;
    grid-auto-columns: 1fr;
    grid-auto-flow: column;
    gap: 0.25rem;
    padding: 0.25rem;
    border-radius: 0.625rem;
    background: var(--zone);
  }
  .seg button {
    min-width: 0;
    min-height: 2.75rem;
    padding: 0 0.25rem;
    border-radius: 0.5rem;
    color: var(--muted);
    font-weight: 500;
  }
  .seg button[aria-checked="true"] {
    background: var(--surface);
    box-shadow: 0 1px 2px rgb(0 0 0 / 25%);
    color: var(--ink);
  }
  summary {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    min-height: 2.75rem;
    box-sizing: border-box;
    padding: 0.875rem 1rem;
    cursor: pointer;
    font-weight: 600;
    list-style: none;
  }
  summary::-webkit-details-marker {
    display: none;
  }
  /* 3.0's chevron: down when closed, up when open, and a rule under an open group's title. */
  summary::after {
    content: "";
    flex: none;
    width: 0.5rem;
    height: 0.5rem;
    margin-right: 0.25rem;
    border-right: 2px solid var(--muted);
    border-bottom: 2px solid var(--muted);
    transform: rotate(45deg);
  }
  details[open] > summary {
    border-bottom: 1px solid var(--line);
  }
  details[open] > summary::after {
    transform: rotate(-135deg);
  }
  .sgrid {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 1rem;
    padding: 1rem;
  }
  .nav {
    display: flex;
    justify-content: flex-end;
    margin-top: 1rem;
  }
  @media (max-width: 900px) {
    .sgrid {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }
  @media (max-width: 640px) {
    .set,
    .sgrid {
      grid-template-columns: 1fr;
    }
  }
</style>
