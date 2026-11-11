<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The second bar, as in 3.0: the phases of the current stage as chips, then the detail setting and
overall progress. Each chip's mark reads without color: an empty ring for to do, a half ring when
started, a filled circle with a check when complete, and a dashed ring when a step is skipped.
-->
<script lang="ts">
  import type { Detail, Progress, RailPhase } from "@chocktochock/bindings";
  import { phaseHref } from "../route.svelte";

  let {
    phases,
    current,
    detail,
    total,
  }: { phases: RailPhase[]; current: string | null; detail: Detail; total: Progress } = $props();

  const mark = (p: RailPhase) =>
    p.complete ? "complete" : p.skipped > 0 ? "skipped" : p.progress.done > 0 ? "started" : "todo";
  const WORDS = { complete: "complete", skipped: "has skipped steps", started: "started", todo: "not started" };
</script>

<div class="rail" class:none={!phases.length}>
  <nav class="chips" aria-label="Phases">
    {#each phases as p (p.id)}
      {@const m = mark(p)}
      <a href={phaseHref(p.id)} aria-current={p.id === current ? "true" : undefined}>
        <i class="dot {m}" aria-hidden="true"></i>
        <span>{p.title}<span class="sr-only">, {WORDS[m]}</span></span>
      </a>
    {/each}
  </nav>
  <div class="meta">
    <a class="chip" href="#/settings">Full · {detail === "expanded" ? "Detailed" : "Simple"}</a>
    <div class="bar" aria-hidden="true"><i style:width="{total.total ? (100 * total.done) / total.total : 0}%"></i></div>
    <span class="count mono" aria-label="{total.done} of {total.total} steps done">{total.done}/{total.total}</span>
  </div>
</div>

<style>
  .rail {
    position: sticky;
    top: var(--top-h);
    z-index: 19;
    display: flex;
    align-items: center;
    gap: 1rem;
    height: var(--rail-h);
    padding: 0 1rem;
    border-bottom: 1px solid var(--line);
    background: var(--surface);
  }
  .chips {
    display: flex;
    flex: 1;
    gap: 0.15rem;
    min-width: 0;
    overflow-x: auto;
    scrollbar-width: none;
  }
  .chips a {
    position: relative;
    display: flex;
    flex: none;
    align-items: center;
    gap: 0.45rem;
    height: 2.25rem;
    padding: 0 0.6rem;
    border-radius: 0.5rem;
    color: var(--muted);
    font-size: 0.875rem;
    text-decoration: none;
    white-space: nowrap;
  }
  .chips a:hover {
    color: var(--ink);
  }
  .chips a[aria-current="true"] {
    background: var(--accent-soft);
    color: var(--accent);
    font-weight: 600;
  }
  /* The touch area grows to 44px without changing 3.0's look (step 7). */
  .chips a::after,
  .chip::after {
    position: absolute;
    inset: 50% 0 auto;
    height: 2.75rem;
    content: "";
    transform: translateY(-50%);
  }
  .chips {
    padding: 0.25rem 0;
  }
  /* 3.0's small filled dots. State reads without color too: half filled when started, a check
     ring when complete, dashed when a step is skipped. */
  .dot {
    position: relative;
    width: 0.5rem;
    height: 0.5rem;
    border-radius: 50%;
    background: var(--line);
  }
  [aria-current="true"] .dot {
    background: var(--accent);
  }
  .dot.started {
    background: linear-gradient(90deg, var(--accent) 50%, var(--line) 50%);
  }
  .dot.complete {
    background: var(--go);
    box-shadow: 0 0 0 2px var(--surface), 0 0 0 3px var(--go);
  }
  .dot.skipped {
    background: none;
    border: 1.5px dashed var(--warn);
  }
  /* A row of chips wider than the screen fades at its edge, so more is clearly there. */
  @media (max-width: 1100px) {
    .chips {
      mask-image: linear-gradient(90deg, #000 calc(100% - 2rem), transparent);
    }
  }
  .meta {
    display: flex;
    flex: none;
    align-items: center;
    gap: 0.6rem;
  }
  .chip {
    position: relative;
    display: inline-flex;
    align-items: center;
    min-height: 2rem;
    padding: 0.3rem 0.7rem;
    border: 1px solid var(--line);
    border-radius: 99rem;
    background: var(--bg);
    font-size: 0.8125rem;
    font-weight: 500;
    text-decoration: none;
    white-space: nowrap;
  }
  .bar {
    width: 9rem;
    height: 0.375rem;
    overflow: hidden;
    border-radius: 0.5rem;
    background: var(--zone);
  }
  .bar i {
    display: block;
    height: 100%;
    background: var(--go);
  }
  .count {
    color: var(--muted);
    font-size: 0.8125rem;
  }
  @media (max-width: 1100px) {
    .bar {
      display: none;
    }
  }
  /* As in 3.0, narrow screens drop the bar when there are no phases to show. */
  @media (max-width: 1023px) {
    .rail.none {
      display: none;
    }
  }
</style>
