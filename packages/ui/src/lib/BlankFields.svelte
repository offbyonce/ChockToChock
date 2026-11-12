<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

A step's blanks, as 3.0 drew them in the list and in Flow: each blank a field with the words
before and after it, the parts to fill in on one line in the monospaced face, and the rest of the
instruction on its own muted line. A blank shows the flight value (decision 0019) until the pilot
types over it.
-->
<script lang="ts">
  import { fields } from "./value-parts";

  let {
    name,
    blanks,
    rest,
    values,
    onBlank,
  }: {
    /** The step's name, for each field's accessible label. */
    name: string;
    blanks: string;
    rest: string;
    values: string[];
    onBlank: (index: number, value: string) => void;
  } = $props();
</script>

<div class="flds">
  {#each fields(blanks) as f, n (n)}
    <label class="fld">
      {#if f.before}<span>{f.before}</span>{/if}
      <input
        aria-label="{name} {f.before}"
        value={values[n] ?? ""}
        autocomplete="off"
        oninput={(e) => onBlank(n, e.currentTarget.value)}
        onchange={(e) => {
          const v = e.currentTarget.value.trim();
          if (v !== e.currentTarget.value) onBlank(n, v);
        }}
      />
      {#if f.after}<span>{f.after}</span>{/if}
    </label>
  {/each}
</div>
{#if rest}<div class="rest">{rest}</div>{/if}

<style>
  .flds {
    display: flex;
    flex-wrap: wrap;
    gap: 0.45rem 1.1rem;
    margin-top: 0.15rem;
  }
  .fld {
    display: inline-flex;
    align-items: baseline;
    gap: 0.4rem;
    font: 500 0.84rem var(--mono);
  }
  input {
    width: 5.2rem;
    padding: 0.05rem 0.1rem;
    border: 0;
    border-bottom: 1.5px solid var(--muted);
    border-radius: 0;
    background: transparent;
    color: var(--ink);
    font: 500 0.9rem var(--mono);
    text-align: center;
  }
  .rest {
    color: var(--muted);
    font: 400 0.875rem/1.45 var(--sans);
  }
  @media (pointer: coarse) {
    input {
      width: 5.6rem;
      min-height: 2.25rem;
    }
  }
</style>
