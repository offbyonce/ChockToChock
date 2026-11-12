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
    gap: 0.375rem;
    min-width: 0;
  }
  .sf.wide {
    grid-column: 1 / -1;
  }
  label {
    font-size: 0.9375rem;
    font-weight: 500;
  }
  .sfr {
    display: flex;
    gap: 0.5rem;
  }
  input,
  select {
    flex: 1;
    min-width: 0;
    height: 2.875rem;
    box-sizing: border-box;
    padding: 0 0.6rem;
    border: 1px solid var(--line);
    border-radius: 0.5rem;
    background: var(--surface);
    color: var(--ink);
    font: 500 1rem var(--mono);
  }
  input:invalid {
    border-color: var(--stop);
  }
  .mode {
    display: grid;
    flex: none;
    grid-template-columns: auto auto;
    gap: 2px;
    height: 2.875rem;
    box-sizing: border-box;
    padding: 3px;
    border-radius: 9px;
    background: var(--bg);
  }
  .mode button {
    min-width: 3rem;
    height: 100%;
    padding: 0 0.5rem;
    border-radius: 7px;
    color: var(--muted);
    font-size: 0.9375rem;
    font-weight: 500;
  }
  .mode button[aria-checked="true"] {
    background: var(--surface);
    box-shadow: 0 1px 2px rgb(0 0 0 / 14%);
    color: var(--ink);
  }
  small {
    color: var(--muted);
    font-size: 0.8125rem;
    line-height: 1.3;
  }
</style>
