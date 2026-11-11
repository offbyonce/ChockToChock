<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

A step's details, as 3.0 shows them: what to check, why, how to do it (a numbered list that opens,
from the pack's shared notes), a note, what to do if not, and the explanation of a test. Used in
an opened step, in Flow view and in the side panel.
-->
<script lang="ts">
  import type { Step } from "@chocktochock/bindings";
  import type { PackIndex } from "../pack-index";
  import Icon from "./Icon.svelte";

  let { index, step, large = false }: { index: PackIndex; step: Step; large?: boolean } = $props();

  let how = $derived(step.how ? index.how(step.how) : undefined);
  let explain = $derived(step.explain ? index.explain(step.explain) : undefined);
  let check = $derived(step.check ?? explain?.check ?? null);
</script>

<div class="detail" class:large>
  {#if check}<p class="check"><b>Check</b>{check}</p>{/if}
  {#if step.why}<p><b>Why</b> · {step.why}</p>{/if}
  {#if explain}
    <p><b>What it is</b> · {explain.what}</p>
    {#if !step.why}<p><b>Why</b> · {explain.why}</p>{/if}
    <p><b>Good result</b> · {explain.success}</p>
    {#if explain.failure}<p><b>If not</b> · {explain.failure}</p>{/if}
  {/if}
  {#if how?.length}
    <details>
      <summary>How to do it <Icon name="chevron-down" /></summary>
      <ol>
        {#each how as line, i (i)}<li>{line}</li>{/each}
      </ol>
    </details>
  {/if}
  {#if step.note}<p><b>Note</b> · {step.note}</p>{/if}
  {#if step.if_not}<p><b>If not</b> · {step.if_not}</p>{/if}
</div>

<style>
  .detail {
    display: flex;
    flex-direction: column;
    gap: 0.625rem;
    line-height: 1.45;
  }
  p {
    margin: 0;
  }
  b {
    font-weight: 600;
  }
  .check {
    padding: 0.625rem 0.875rem;
    border-radius: 0.5rem;
    background: var(--accent-soft);
    font-weight: 600;
  }
  .check b {
    display: block;
    color: var(--accent);
    font-size: 0.6875rem;
    letter-spacing: 0.06em;
    text-transform: uppercase;
  }
  .large .check {
    font-size: 1.05rem;
  }
  summary {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    min-height: 2.75rem;
    color: var(--accent);
    font-weight: 600;
    list-style: none;
    cursor: pointer;
  }
  summary::-webkit-details-marker {
    display: none;
  }
  details[open] summary :global(.icon) {
    transform: rotate(180deg);
  }
  ol {
    margin: 0;
    padding: 0.625rem 0.875rem 0.625rem 2rem;
    border-radius: 0.5rem;
    background: var(--zone);
  }
  li + li {
    margin-top: 0.25rem;
  }
</style>
