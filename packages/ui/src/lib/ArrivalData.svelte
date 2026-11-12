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
    padding: 0.875rem 1rem;
    border-bottom: 1px solid var(--line);
  }
  header b {
    font-weight: 600;
  }
  /* 3.0's five-field row: four narrow fields and a wider one for minimums with its unit choice. */
  .sgrid5 {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr)) minmax(12rem, 1.4fr);
    gap: 1rem;
    padding: 1rem;
  }
  @media (max-width: 1100px) {
    .sgrid5 {
      grid-template-columns: repeat(3, minmax(0, 1fr));
    }
  }
  @media (max-width: 600px) {
    .sgrid5 {
      grid-template-columns: 1fr;
    }
  }
</style>
