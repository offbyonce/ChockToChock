<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

A step's details, in 3.0's order: the highlighted CHECK cell and the IF NOT cell side by side,
why, what to say, how to do it (a numbered list that opens, from the pack's shared notes; open at
the start for tests and completion steps, as in 3.0), then a note. Used in an opened step, in
Flow view and in the side panel. Each item shows once.
-->
<script lang="ts">
  import type { Step } from "@chocktochock/bindings";
  import type { PackIndex } from "../pack-index";
  import Icon from "./Icon.svelte";

  let { index, step, large = false }: { index: PackIndex; step: Step; large?: boolean } = $props();

  let how = $derived(step.how ? index.how(step.how) : undefined);
  let explain = $derived(step.explain ? index.explain(step.explain) : undefined);
  let check = $derived(step.check ?? explain?.check ?? null);
  let checkDetail = $derived(step.check ? step.check_detail : explain?.check_detail);
  let ifNot = $derived(step.if_not ?? explain?.if_not ?? null);
  let ifNotDetail = $derived(step.if_not ? step.if_not_detail : explain?.if_not_detail);
  let why = $derived(step.why ?? explain?.why ?? null);
  let howOpen = $derived(/COMPLETE|TEST/.test(step.expect));
</script>

<div class="detail" class:large>
  {#if check || ifNot}
    <div class="cells" style:--n={check && ifNot ? 2 : 1}>
      {#if check}
        <div class="cell ck"><small>CHECK</small><b>{check}</b>{#if checkDetail}<span>{checkDetail}</span>{/if}</div>
      {/if}
      {#if ifNot}
        <div class="cell if"><small>IF NOT</small><b>{ifNot}</b>{#if ifNotDetail}<span>{ifNotDetail}</span>{/if}</div>
      {/if}
    </div>
  {/if}
  {#if why}<p class="line"><b>Why</b> · {why}</p>{/if}
  {#if explain}
    <p class="line"><b>What it is</b> · {explain.what}</p>
    <p class="line"><b>Good result</b> · {explain.success}</p>
    {#if explain.failure}<p class="line"><b>If it fails</b> · {explain.failure}</p>{/if}
  {/if}
  {#if step.callout}<p class="line"><b>Say</b> · {step.callout}</p>{/if}
  {#if how?.length}
    <details open={howOpen || large}>
      <summary>How to do it <Icon name="chevron-down" /></summary>
      <ol>
        {#each how as line, i (i)}<li>{line}</li>{/each}
      </ol>
    </details>
  {/if}
  {#if step.note}<p class="line"><b>Note</b> · {step.note}</p>{/if}
</div>

<style>
  /* Sizes and colors follow 3.0's opened step. */
  .detail {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    line-height: 1.45;
  }
  .cells {
    display: grid;
    grid-template-columns: repeat(var(--n), minmax(0, 1fr));
    gap: var(--space-1);
  }
  .cell {
    padding: var(--space-3) var(--space-3);
    border-radius: var(--radius-control);
    background: var(--color-bg);
  }
  .cell small {
    display: block;
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-semibold);
  }
  .cell b {
    display: block;
    margin-top: var(--space-1);
    font-size: var(--font-size-body);
    font-weight: var(--weight-semibold);
    line-height: 1.25;
  }
  .cell span {
    display: block;
    margin-top: var(--space-1);
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
    line-height: 1.3;
  }
  .cell.ck {
    background: color-mix(in srgb, var(--color-accent) 16%, var(--color-surface));
  }
  .cell.ck small {
    color: var(--color-accent);
  }
  .cell.if {
    background: var(--color-open-bg);
  }
  .cell.if small {
    color: var(--color-open-text);
  }
  .large .cell b {
    font-size: var(--font-size-body);
  }
  .line {
    margin: 0;
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
  }
  .line b {
    color: var(--color-text);
    font-weight: var(--weight-semibold);
  }
  .large .line {
    font-size: var(--font-size-body);
  }
  summary {
    display: inline-flex;
    align-items: center;
    gap: var(--space-2);
    min-height:  var(--size-touch-min);
    color: var(--color-accent);
    font-weight: var(--weight-medium);
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
    margin: var(--space-1) 0 var(--space-2);
    padding: var(--space-3) var(--space-4) var(--space-3) var(--space-8);
    border-radius: var(--radius-control);
    background: var(--color-surface-2);
  }
  li + li {
    margin-top: var(--space-1);
  }
</style>
