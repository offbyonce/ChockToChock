<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The one field: a label, the input or list of choices with an optional unit choice beside it, and
a help line. Inline is the step-blank form: the words around it on one line and an underlined
field the width of the value. Values are kept as typed and trimmed when the pilot leaves.
-->
<script lang="ts">
  import Segmented from "./Segmented.svelte";

  let {
    id,
    label,
    value,
    onInput,
    help,
    example,
    kind = "text",
    options,
    min,
    max,
    maxLength,
    inputMode,
    unit,
    variant = "box",
    before,
    after,
    wide = false,
  }: {
    id: string;
    label: string;
    value: string;
    onInput: (value: string) => void;
    help?: string | undefined;
    example?: string | undefined;
    kind?: "text" | "number" | "select" | undefined;
    options?: string[] | undefined;
    min?: number | undefined;
    max?: number | undefined;
    maxLength?: number | undefined;
    inputMode?: "numeric" | "decimal" | undefined;
    unit?: { values: string[]; value: string; onChange: (v: string) => void };
    variant?: "box" | "inline" | undefined;
    /** Inline only: the words before and after the field ("FLAPS", "INDICATED"). */
    before?: string | undefined;
    after?: string | undefined;
    wide?: boolean | undefined;
  } = $props();

  const trimmed = (el: HTMLInputElement) => {
    const v = el.value.trim();
    if (v !== el.value) onInput(v);
  };
</script>

{#if variant === "inline"}
  <label class="inline">
    {#if before}<span>{before}</span>{/if}
    <input {id} aria-label={label} {value} autocomplete="off" inputmode={inputMode} oninput={(e) => onInput(e.currentTarget.value)} onchange={(e) => trimmed(e.currentTarget)} />
    {#if after}<span>{after}</span>{/if}
  </label>
{:else}
  <div class="f" class:wide>
    <!-- A unit choice sits beside the label, so the value keeps the column's full width. -->
    <div class="head">
      <label for={id}>{label}</label>
      {#if unit}
        <Segmented layout="inline" label="{label} unit" choices={unit.values.map((v) => ({ value: v, label: v }))} value={unit.value} onChange={unit.onChange} />
      {/if}
    </div>
    <div class="row">
      {#if kind === "select"}
        <select {id} {value} onchange={(e) => onInput(e.currentTarget.value)}>
          <option value=""></option>
          {#each options ?? [] as o (o)}<option value={o}>{o}</option>{/each}
        </select>
      {:else}
        <input
          {id}
          type={kind}
          step={kind === "number" ? "any" : undefined}
          {min}
          {max}
          maxlength={maxLength}
          inputmode={inputMode}
          placeholder={example}
          {value}
          autocomplete="off"
          oninput={(e) => onInput(e.currentTarget.value)}
          onchange={(e) => trimmed(e.currentTarget)}
        />
      {/if}
    </div>
    {#if help}<small>{help}</small>{/if}
  </div>
{/if}

<style>
  .f {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    min-width: 0;
  }
  .wide {
    grid-column: 1 / -1;
  }
  label {
    font: var(--weight-medium) var(--text-sm) / var(--leading-tight) var(--sans);
  }
  .head {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-2);
    /* Every label row is the unit choice's height, so inputs in a row line up. */
    min-height: var(--control-h);
  }
  .head :global(.seg button) {
    padding: 0 var(--space-2);
  }
  /* Compact beside the label on fine pointers; a full touch target on touch screens. */
  @media (pointer: fine) {
    .head {
      min-height: var(--control-h-compact);
    }
    .head :global(.seg) {
      padding: 0;
    }
    .head :global(.seg button) {
      min-height: var(--control-h-compact);
    }
  }
  .row {
    display: flex;
    gap: var(--space-2);
  }
  input,
  select {
    flex: 1;
    min-width: 0;
    height: var(--control-h);
    padding: 0 var(--space-3);
    border: var(--border-thin) solid var(--line);
    border-radius: var(--radius-md);
    background: var(--surface);
    color: var(--ink);
    font: var(--weight-medium) var(--text-md) var(--mono);
  }
  input:hover,
  select:hover {
    border-color: var(--muted);
  }
  input:invalid {
    border-color: var(--stop);
  }
  small {
    color: var(--muted);
    font: var(--weight-regular) var(--text-xs) / var(--leading-snug) var(--sans);
  }
  .inline {
    display: inline-flex;
    align-items: baseline;
    gap: var(--space-2);
    font: var(--weight-medium) var(--text-sm) var(--mono);
  }
  .inline input {
    flex: none;
    width: 6ch;
    min-width: var(--space-16);
    height: auto;
    min-height: var(--space-8);
    padding: 0 var(--space-1);
    border: 0;
    border-bottom: var(--border-thick) solid var(--muted);
    border-radius: 0;
    background: transparent;
    font-size: var(--text-md);
    text-align: center;
  }
  .inline input:focus-visible {
    border-bottom-color: var(--accent);
    outline-offset: var(--space-1);
  }
  @media (pointer: coarse) {
    .inline input {
      min-height: var(--control-h);
    }
  }
</style>
