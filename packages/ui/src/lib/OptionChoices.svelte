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
    gap: var(--space-3);
    margin-bottom: var(--space-4);
  }
  .cards button {
    padding: var(--space-3) var(--space-4);
    border: var(--border-thin) solid var(--color-border);
    border-radius: var(--radius-panel);
    background: var(--color-surface);
    text-align: left;
  }
  .cards button:hover {
    border-color: var(--color-text-muted);
  }
  .cards b {
    display: block;
    font-weight: var(--weight-semibold);
  }
  .cards span {
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
  }
  .cards button[aria-pressed="true"] {
    border-color: var(--color-accent);
    box-shadow: 0 0 0 var(--size-focus-ring) var(--color-accent);
  }
  .row {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    gap: var(--space-4) var(--space-8);
    padding: var(--space-4);
  }
  .row b {
    font-weight: var(--weight-semibold);
  }
  .row p {
    margin: var(--space-1) 0 0;
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
  }
  .seg {
    display: flex;
    gap: var(--space-1);
    padding: var(--space-1);
    border-radius: var(--radius-control);
    background: var(--color-surface-2);
  }
  .seg button {
    flex: 1;
    min-width: 0;
    min-height:  var(--size-touch-min);
    padding: 0 var(--space-4);
    border-radius: var(--radius-control);
    color: var(--color-text-muted);
    font-weight: var(--weight-medium);
  }
  .seg button[aria-checked="true"] {
    background: var(--color-surface);
    box-shadow: var(--color-segment-shadow);
    color: var(--color-text);
  }
  @media (max-width: 600px) {
    .cards,
    .row {
      grid-template-columns: 1fr;
    }
  }
</style>
