<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

A Flight section's cards (FlightSection boards): each group a titled card, at most 1040 wide,
with its rows; Fuel's card header carries "Weights in lb / kg"; Approach has its approach type
above the cards. The selected row drives the OFP panel beside it.
-->
<script lang="ts">
  import type { FillField, UserOption } from "@chocktochock/bindings";
  import FieldRow from "./FieldRow.svelte";
  import { SECTIONS, toEnter, type Ctx, type Section } from "./sections";

  let {
    section,
    ctx,
    selected,
    descriptions,
    approach,
    onSelect,
    onSet,
    onOption,
    onNotes,
  }: {
    section: Section;
    ctx: Ctx;
    selected: string;
    descriptions: boolean;
    /** The pack's approach type option, with the flight's choice (Approach only). */
    approach?: { option: UserOption; value: string } | null;
    onSelect: (row: string) => void;
    onSet: (field: FillField, value: string) => void;
    onOption: (option: string, value: string) => void;
    onNotes: (notes: string) => void;
  } = $props();

  const title = (id: string) => SECTIONS.find((s) => s.id === id)?.title ?? id;
  let unit = $derived((ctx.values.weight_unit ?? "LB").toUpperCase());
  /** The section's "N to enter" sits on its last card with a value still needed (boards). */
  let lastNeeded = $derived.by(() => {
    const missing = toEnter(ctx.values, section.id);
    let at = -1;
    section.groups.forEach((g, n) => {
      if (g.rows.some((r) => missing.includes(r))) at = n;
    });
    return at;
  });
</script>

{#if section.id === "approach" && approach}
  <div class="atype">
    <span class="al" id="atype">{approach.option.title}</span>
    <span class="seg" role="group" aria-labelledby="atype">
      {#each approach.option.values as value (value)}
        <button type="button" aria-pressed={approach.value === value} onclick={() => onOption(approach.option.id, value)}>
          {approach.option.choices?.find((c) => c.value === value)?.label ?? value}
        </button>
      {/each}
    </span>
    <span class="ah">The Approach steps change with the type.</span>
  </div>
{/if}
{#each section.groups as group, g (group.title)}
  {@const count = g === lastNeeded ? toEnter(ctx.values, section.id).length : 0}
  <section class="card" aria-label={group.title}>
    <div class="ghead">
      <h3>{group.title}</h3>
      <span class="fill"></span>
      <span class="note">{count ? `${count} to enter` : (group.note?.(ctx) ?? "")}</span>
      {#if group.units}
        <span class="note">Weights in</span>
        <span class="seg small" role="group" aria-label="Weight unit">
          {#each ["LB", "KG"] as u (u)}
            <button type="button" aria-pressed={unit === u} onclick={() => onSet("weight_unit", u)}>{u.toLowerCase()}</button>
          {/each}
        </span>
      {/if}
    </div>
    {#each group.rows as row (row.id)}
      <FieldRow {row} {ctx} {descriptions} selected={selected === row.id} sectionTitle={title} onSelect={() => onSelect(row.id)} {onSet} {onNotes} />
    {/each}
  </section>
{/each}

<style>
  .atype {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--space-2) var(--space-3);
  }
  .al {
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
  }
  .ah {
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  .card {
    flex: none;
    margin: 0;
    max-width: calc(26 * var(--space-10));
    overflow: visible;
    border: var(--border-thin) solid var(--color-border);
    border-radius: var(--radius-panel);
    background: var(--color-surface);
  }
  /* The last row keeps the card's rounded corners when it is selected. */
  .card :global(.frow:last-child) {
    border-radius: 0 0 var(--radius-panel) var(--radius-panel);
  }
  .ghead {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    min-height: calc(var(--size-touch-min) + var(--space-2));
    padding: 0 calc(var(--space-4) + var(--space-1));
  }
  h3 {
    margin: 0;
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
  }
  .fill {
    flex: 1;
  }
  .note {
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  .seg {
    display: inline-flex;
    padding: var(--border-thick);
    border: var(--border-thin) solid var(--color-segment-border);
    border-radius: var(--radius-segment);
    background: var(--color-segment-track);
  }
  .seg button {
    height: var(--size-touch-min);
    padding: 0 var(--space-4);
    border: 0;
    border-radius: var(--radius-control);
    background: transparent;
    color: var(--color-text-muted);
    font: inherit;
    font-size: var(--font-size-secondary);
  }
  .seg.small {
    border-radius: var(--radius-control);
  }
  .seg.small button {
    height: calc(var(--size-touch-min) - var(--space-1));
    min-width: var(--size-touch-min);
    padding: 0 calc(var(--space-2) + var(--border-thick));
    border-radius: calc(var(--radius-small) + var(--border-thick));
  }
  .seg button[aria-pressed="true"] {
    background: var(--color-segment-on);
    outline: var(--size-focus-ring) solid var(--color-accent);
    outline-offset: calc(-1 * var(--size-focus-ring));
    box-shadow: var(--color-segment-shadow);
    color: var(--color-text-strong);
    font-weight: var(--weight-semibold);
  }
</style>
