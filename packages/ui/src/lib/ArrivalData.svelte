<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The Arrival data box, as 3.0 showed it on the descent page: the landing values the arrival steps
fill in. The core says which phase shows it (decision 0019).
-->
<script lang="ts">
  import type { FillField } from "@chocktochock/bindings";
  import FieldInput from "./FieldInput.svelte";
  import { ARRIVAL_FIELDS } from "./flight-fields";

  let {
    values,
    onSet,
  }: {
    values: Partial<Record<FillField, string>>;
    onSet: (field: FillField, value: string) => void;
  } = $props();
</script>

<section class="card appb">
  <header><b>Arrival data</b></header>
  <div class="sgrid5">
    {#each ARRIVAL_FIELDS as spec (spec.field)}
      <FieldInput {spec} {values} {onSet} idPrefix="a" />
    {/each}
  </div>
</section>

<style>
  header {
    padding: var(--space-4) var(--space-4);
    border-bottom: var(--border-thin) solid var(--color-border);
  }
  header b {
    font-weight: var(--weight-semibold);
  }
  /* Three fields across: five no longer fit beside the side pane with 44 px fields (batch 4
     redesigns this form on the Flight pages). */
  .sgrid5 {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: var(--space-4);
    padding: var(--space-4);
  }
  @media (max-width: 599px) {
    .sgrid5 {
      grid-template-columns: 1fr;
    }
  }
</style>
