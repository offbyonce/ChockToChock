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
    essentials,
    total,
    savedAt,
  }: {
    phases: RailPhase[];
    current: string | null;
    detail: Detail;
    essentials: boolean;
    total: Progress;
    /** When progress was last saved on this device. */
    savedAt: number | null;
  } = $props();

  let saved = $derived(
    savedAt === null ? null : new Date(savedAt).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" }),
  );

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
    <a class="chip" href="#/settings">{essentials ? "Essentials" : "Full"} · {detail === "expanded" ? "Detailed" : "Simple"}</a>
    <div class="bar" aria-hidden="true">
      <i style:width={total.done ? `max(0.375rem, ${(100 * total.done) / total.total}%)` : "0"}></i>
    </div>
    <span class="count mono" aria-label="{total.done} of {total.total} steps done">{#if essentials}Essentials{" "}{/if}{total.done}/{total.total}</span>
    {#if saved}<span class="saved" role="status">Saved {saved}</span>{/if}
  </div>
</div>

<style>
  .rail {
    position: sticky;
    top: var(--size-header);
    z-index: 19;
    display: flex;
    align-items: center;
    gap: var(--space-4);
    height: var(--size-phase-row);
    padding: 0 var(--space-4);
    border-bottom: var(--border-thin) solid var(--color-border);
    background: var(--color-surface);
  }
  .chips {
    display: flex;
    flex: 1;
    gap: var(--space-1);
    min-width: 0;
    overflow-x: auto;
    scrollbar-width: none;
  }
  .chips a {
    position: relative;
    display: flex;
    flex: none;
    align-items: center;
    gap: var(--space-2);
    height: var(--size-touch-min);
    padding: 0 var(--space-2);
    border-radius: var(--radius-control);
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
    text-decoration: none;
    white-space: nowrap;
  }
  .chips a:hover {
    color: var(--color-text);
  }
  .chips a[aria-current="true"] {
    background: var(--color-accent-soft);
    color: var(--color-accent);
    font-weight: var(--weight-semibold);
  }
  /* The touch area grows to 44px without changing 3.0's look (step 7). */
  .chips a::after,
  .chip::after {
    position: absolute;
    inset: 50% 0 auto;
    height:  var(--size-touch-min);
    content: "";
    transform: translateY(-50%);
  }
  .chips {
    padding: var(--space-1) 0;
  }
  /* 3.0's small filled dots. State reads without color too: half filled when started, a check
     ring when complete, dashed when a step is skipped. */
  .dot {
    position: relative;
    width: var(--size-dot);
    height: var(--size-dot);
    border-radius: 50%;
    background: var(--color-border);
  }
  [aria-current="true"] .dot {
    background: var(--color-accent);
  }
  .dot.started {
    background: linear-gradient(90deg, var(--color-accent) 50%, var(--color-border) 50%);
  }
  .dot.complete {
    background: var(--color-done);
    box-shadow: 0 0 0 var(--border-thick) var(--color-surface), 0 0 0 calc(var(--border-thick) + var(--border-thin)) var(--color-done);
  }
  .dot.skipped {
    background: none;
    border: var(--size-focus-ring) dashed var(--color-open-text);
  }
  /* A row of chips wider than the screen fades at its edge, so more is clearly there. */
  @media (max-width: 1439px) {
    .chips {
      mask-image: linear-gradient(90deg, black calc(100% - var(--space-8)), transparent);
    }
  }
  .meta {
    display: flex;
    flex: none;
    align-items: center;
    gap: var(--space-2);
  }
  .chip {
    position: relative;
    display: inline-flex;
    align-items: center;
    min-height: var(--size-touch-min);
    padding: var(--space-1) var(--space-3);
    border: var(--border-thin) solid var(--color-border);
    border-radius: var(--radius-pill);
    background: var(--color-bg);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-medium);
    text-decoration: none;
    white-space: nowrap;
  }
  .bar {
    width: calc(4 * var(--space-8));
    height: var(--size-progress-bar);
    overflow: hidden;
    border-radius: var(--radius-control);
    background: var(--color-surface-2);
  }
  .bar i {
    display: block;
    height: 100%;
    background: var(--color-done);
  }
  .saved {
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
    white-space: nowrap;
  }
  .count {
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
  }
  @media (max-width: 1439px) {
    .bar {
      display: none;
    }
  }
  @media (max-width: 600px) {
    .saved {
      display: none;
    }
  }
  /* As in 3.0, narrow screens drop the bar when there are no phases to show. */
  @media (max-width: 899px) {
    .rail.none {
      display: none;
    }
  }
</style>
