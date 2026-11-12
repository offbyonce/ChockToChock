<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The one segmented control: a choice of two to five, all visible, one chosen. Segments share the
width equally. When choices have descriptions ("Ground power at the gate · Start the APU later")
the cards layout shows each as a card. Inline is the compact form beside a field (inHg | hPa).
-->
<script lang="ts" module>
  export type Choice<V> = { value: V; label: string; description?: string | undefined };
</script>

<script lang="ts" generics="T extends string | number | boolean">

  let {
    choices,
    value,
    onChange,
    label,
    labelledby,
    layout = "bar",
  }: {
    choices: Choice<T>[];
    value: T;
    onChange: (value: T) => void;
    /** The accessible name, or the id of the element that names it. */
    label?: string;
    labelledby?: string;
    layout?: "bar" | "inline" | "cards";
  } = $props();
</script>

<div class="seg {layout}" role="radiogroup" aria-label={label} aria-labelledby={labelledby}>
  {#each choices as c (String(c.value))}
    <button type="button" role="radio" aria-checked={c.value === value} onclick={() => onChange(c.value)}>
      <span class="l">{c.label}</span>
      {#if layout === "cards" && c.description}<span class="d">{c.description}</span>{/if}
    </button>
  {/each}
</div>

<style>
  .seg {
    display: grid;
    grid-auto-columns: minmax(0, 1fr);
    grid-auto-flow: column;
    gap: var(--space-1);
    padding: var(--space-1);
    border-radius: var(--radius-md);
    background: var(--zone);
  }
  button {
    min-width: 0;
    min-height: calc(var(--control-h) - 2 * var(--space-1));
    padding: 0 var(--space-3);
    border-radius: var(--radius-sm);
    color: var(--muted);
    font: var(--weight-medium) var(--text-sm) / var(--leading-tight) var(--sans);
  }
  button[aria-checked="true"] {
    background: var(--surface);
    box-shadow: var(--shadow-raised);
    color: var(--ink);
  }
  .inline {
    flex: none;
    grid-auto-columns: auto;
  }
  .cards {
    grid-auto-flow: row;
    grid-template-columns: repeat(auto-fit, minmax(var(--card-min), 1fr));
    gap: var(--space-3);
    padding: 0;
    background: none;
  }
  .cards button {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    min-height: var(--space-16);
    padding: var(--space-3) var(--space-4);
    border: var(--border-thin) solid var(--line);
    border-radius: var(--radius-lg);
    background: var(--surface);
    text-align: left;
  }
  .cards button[aria-checked="true"] {
    border-color: var(--accent);
    box-shadow: inset 0 0 0 var(--border-thin) var(--accent);
  }
  .cards .l {
    color: var(--ink);
    font-size: var(--text-md);
    font-weight: var(--weight-bold);
  }
  .cards .d {
    color: var(--muted);
    font-weight: var(--weight-regular);
    line-height: var(--leading-snug);
  }
</style>
