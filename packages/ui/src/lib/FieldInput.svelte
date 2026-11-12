<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

One flight value, as 3.0 drew it: the label, the input (or a list of choices) with an optional
two-way unit choice beside it, and a help line. Every change goes to the core as an action.
-->
<script lang="ts">
  import type { FillField } from "@chocktochock/bindings";
  import { type SetupField, unitOf } from "./flight-fields";

  let {
    spec,
    values,
    onSet,
    idPrefix = "f",
  }: {
    spec: SetupField;
    values: Partial<Record<FillField, string>>;
    onSet: (field: FillField, value: string) => void;
    idPrefix?: string;
  } = $props();

  let id = $derived(`${idPrefix}-${spec.field}`);
  let value = $derived(values[spec.field] ?? "");
</script>

<div class="sf" class:wide={spec.wide}>
  <label for={id}>{spec.label}</label>
  <div class="sfr">
    {#if spec.kind === "select"}
      <select {id} {value} onchange={(e) => onSet(spec.field, e.currentTarget.value)}>
        <option value=""></option>
        {#each spec.options ?? [] as o (o)}
          <option value={o}>{o}</option>
        {/each}
      </select>
    {:else}
      <input
        {id}
        type={spec.kind}
        step={spec.kind === "number" ? "any" : undefined}
        min={spec.min}
        max={spec.max}
        maxlength={spec.maxLength}
        inputmode={spec.inputMode}
        placeholder={spec.example}
        {value}
        oninput={(e) => onSet(spec.field, e.currentTarget.value)}
        onchange={(e) => {
          const v = e.currentTarget.value.trim();
          if (v !== e.currentTarget.value) onSet(spec.field, v);
        }}
      />
    {/if}
    {#if spec.unit}
      {@const unit = spec.unit}
      <div class="mode" role="radiogroup" aria-label="{spec.label} unit">
        {#each unit.values as u (u)}
          <button type="button" role="radio" aria-checked={unitOf(values, unit) === u} onclick={() => onSet(unit.field, u)}>{u}</button>
        {/each}
      </div>
    {/if}
  </div>
  <small>{spec.help}</small>
</div>

<style>
  .sf {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    min-width: 0;
  }
  .sf.wide {
    grid-column: 1 / -1;
  }
  label {
    font-size: var(--font-size-body);
    font-weight: var(--weight-medium);
  }
  .sfr {
    display: flex;
    gap: var(--space-2);
  }
  input,
  select {
    flex: 1;
    min-width: 0;
    height: var(--size-touch-min);
    box-sizing: border-box;
    padding: 0 var(--space-2);
    border: var(--border-thin) solid var(--color-border);
    border-radius: var(--radius-control);
    background: var(--color-surface);
    color: var(--color-text);
    font: var(--weight-medium) var(--font-size-value-mono) var(--font-mono);
  }
  input:invalid {
    border-color: var(--color-danger-text);
  }
  .mode {
    display: grid;
    flex: none;
    grid-template-columns: auto auto;
    gap: var(--space-1);
    height: var(--size-touch-min);
    box-sizing: border-box;
    padding: var(--space-1);
    border-radius: var(--radius-control);
    background: var(--color-bg);
  }
  .mode button {
    min-width: var(--size-touch-min);
    height: 100%;
    padding: 0 var(--space-3);
    border-radius: var(--radius-small);
    color: var(--color-text-muted);
    font-size: var(--font-size-body);
    font-weight: var(--weight-medium);
  }
  .mode button[aria-checked="true"] {
    background: var(--color-surface);
    box-shadow: inset 0 0 0 var(--size-focus-ring) var(--color-accent);
    color: var(--color-text);
  }
  small {
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
    line-height: 1.3;
  }
</style>
