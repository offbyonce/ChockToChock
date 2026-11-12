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
    gap: 0.5rem;
    line-height: 1.45;
  }
  .cells {
    display: grid;
    grid-template-columns: repeat(var(--n), minmax(0, 1fr));
    gap: 0.35rem;
  }
  .cell {
    padding: 0.65rem 0.8rem;
    border-radius: 0.6rem;
    background: var(--bg);
  }
  .cell small {
    display: block;
    color: var(--muted);
    font-size: 0.72rem;
    font-weight: 600;
    letter-spacing: 0.03em;
  }
  .cell b {
    display: block;
    margin-top: 0.2rem;
    font-size: 1rem;
    font-weight: 600;
    line-height: 1.25;
  }
  .cell span {
    display: block;
    margin-top: 0.2rem;
    color: var(--muted);
    font-size: 0.8125rem;
    line-height: 1.3;
  }
  .cell.ck {
    background: color-mix(in srgb, var(--accent) 16%, var(--surface));
  }
  .cell.ck small {
    color: var(--accent);
  }
  .cell.if {
    background: var(--warn-soft);
  }
  .cell.if small {
    color: var(--warn);
  }
  .large .cell b {
    font-size: 1.1rem;
  }
  .line {
    margin: 0;
    color: var(--muted);
    font-size: 0.875rem;
  }
  .line b {
    color: var(--ink);
    font-weight: 600;
  }
  .large .line {
    font-size: 1rem;
  }
  summary {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    min-height: 2.75rem;
    color: var(--accent);
    font-weight: 500;
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
    margin: 0.1rem 0 0.4rem;
    padding: 0.625rem 0.875rem 0.625rem 2rem;
    border-radius: 0.5rem;
    background: var(--zone);
  }
  li + li {
    margin-top: 0.25rem;
  }
</style>
