<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

An option the pack asks for on this phase, as in 3.0: cards when its choices have descriptions
("Ground power at the gate"), or a row with a segmented control ("Approach type"). Choosing
applies the option in Rust, which decides what shows.
-->
<script lang="ts">
  import type { UserOption } from "@chocktochock/bindings";

  let {
    option,
    value,
    onChoose,
  }: { option: UserOption; value: string; onChoose: (value: string) => void } = $props();

  let choices = $derived(
    option.values.map((v) => option.choices?.find((c) => c.value === v) ?? { value: v, label: v, description: null }),
  );
  let cards = $derived(choices.some((c) => c.description));
</script>

{#if cards}
  <div class="cards" role="group" aria-label={option.title}>
    {#each choices as c (c.value)}
      <button type="button" aria-pressed={c.value === value} onclick={() => onChoose(c.value)}>
        <b>{c.label}</b>
        {#if c.description}<span>{c.description}</span>{/if}
      </button>
    {/each}
  </div>
{:else}
  <section class="card row">
    <div>
      <b id="opt-{option.id}">{option.title}</b>
      {#if option.description}<p>{option.description}</p>{/if}
    </div>
    <div class="seg" role="radiogroup" aria-labelledby="opt-{option.id}">
      {#each choices as c (c.value)}
        <button type="button" role="radio" aria-checked={c.value === value} onclick={() => onChoose(c.value)}>{c.label}</button>
      {/each}
    </div>
  </section>
{/if}

<style>
  /* 3.0's option cards: two columns, the chosen one outlined in the accent color. */
  .cards {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.625rem;
    margin-bottom: 1.125rem;
  }
  .cards button {
    padding: 0.8125rem 0.9375rem;
    border: 1px solid var(--line);
    border-radius: 0.75rem;
    background: var(--surface);
    text-align: left;
  }
  .cards button:hover {
    border-color: var(--muted);
  }
  .cards b {
    display: block;
    font-weight: 600;
  }
  .cards span {
    color: var(--muted);
    font-size: 0.8125rem;
  }
  .cards button[aria-pressed="true"] {
    border-color: var(--accent);
    box-shadow: 0 0 0 1px var(--accent);
  }
  .row {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    gap: 1rem 2rem;
    padding: 1rem;
  }
  .row b {
    font-weight: 600;
  }
  .row p {
    margin: 0.125rem 0 0;
    color: var(--muted);
    font-size: 0.875rem;
  }
  .seg {
    display: flex;
    gap: 0.25rem;
    padding: 0.25rem;
    border-radius: 0.625rem;
    background: var(--zone);
  }
  .seg button {
    min-width: 3rem;
    min-height: 2.75rem;
    padding: 0 1rem;
    border-radius: 0.5rem;
    color: var(--muted);
    font-weight: 500;
  }
  .seg button[aria-checked="true"] {
    background: var(--surface);
    box-shadow: 0 1px 2px rgb(0 0 0 / 25%);
    color: var(--ink);
  }
  @media (max-width: 760px) {
    .cards,
    .row {
      grid-template-columns: 1fr;
    }
  }
</style>
