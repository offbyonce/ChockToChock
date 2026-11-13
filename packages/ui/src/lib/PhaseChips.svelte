<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The phase track (Checklist board, 60 px): the stage's phases as tabs, each with its progress bar
(a complete phase green with a check, the current one boxed with its count and blue progress), an
amber dot on a phase holding an open step, then the Full · Detailed pill, the overall count and
the saved time. The bars' empty track uses the to-do ring color, 3:1 against the page (owner,
batch 3).
-->
<script lang="ts">
  import type { Detail, Progress, RailPhase } from "@chocktochock/bindings";
  import { phaseHref } from "../route.svelte";
  import Icon from "./Icon.svelte";

  let {
    phases,
    current,
    detail,
    essentials,
    total,
    savedAt,
    open = [],
  }: {
    phases: RailPhase[];
    current: string | null;
    detail: Detail;
    essentials: boolean;
    total: Progress;
    /** When progress was last saved on this device. */
    savedAt: number | null;
    /** Phases holding an open step, for the amber dots. */
    open?: string[];
  } = $props();

  let saved = $derived(
    savedAt === null ? null : new Date(savedAt).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" }),
  );
  const share = (p: RailPhase) => (p.progress.total ? (100 * p.progress.done) / p.progress.total : 0);
</script>

<div class="rail" class:none={!phases.length}>
  <nav class="phases" aria-label="Phases">
    {#each phases as p (p.id)}
      {@const here = p.id === current}
      {@const openHere = open.filter((o) => o === p.id).length}
      <a href={phaseHref(p.id)} class:here class:complete={p.complete} aria-current={here ? "step" : undefined}>
        <span class="label">
          <span class="name"
            >{#if p.complete}<span class="tick"><Icon name="check" /></span>{/if}{p.title}{#if !here}<span class="sr-only"
                >, {p.complete ? "complete" : `${p.progress.done} of ${p.progress.total} done`}</span
              >{/if}{#if openHere}<span
                class="dot"
                aria-label="{openHere} open step{openHere === 1 ? '' : 's'}"
              ></span>{/if}</span
          >
          {#if here}<span class="count">{p.progress.done}/{p.progress.total}</span>{/if}
        </span>
        <span class="bar" aria-hidden="true"><i style:width="{p.complete ? 100 : share(p)}%"></i></span>
      </a>
    {/each}
  </nav>
  <div class="meta">
    <a class="pill" href="#/settings" aria-label="Step mode: {essentials ? 'Essentials' : 'Full'} and {detail === 'expanded' ? 'Detailed' : 'Simple'}, change"
      >{essentials ? "Essentials" : "Full"} · {detail === "expanded" ? "Detailed" : "Simple"}</a
    >
    <span class="count-all" aria-label="{total.done} of {total.total} steps done">{total.done}/{total.total}</span>
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
    gap: var(--space-2);
    height: var(--size-phase-row);
    padding: 0 var(--space-6);
    border-bottom: var(--border-thin) solid var(--color-line);
    background: var(--color-bg);
  }
  .rail.none .phases {
    visibility: hidden;
  }
  .phases {
    display: flex;
    flex: 1;
    gap: var(--space-2);
    min-width: 0;
    overflow-x: auto;
    scrollbar-width: none;
  }
  .phases a {
    flex: none;
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: calc(var(--space-1) + var(--border-thick) / 2);
    min-width: calc(3 * var(--space-6));
    height: calc(var(--size-touch-min) + var(--space-1));
    padding: 0 calc(var(--space-2) + var(--border-thick) / 2);
    border-radius: var(--radius-small);
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
    text-decoration: none;
  }
  .phases a.complete {
    color: var(--color-text-secondary);
  }
  .phases a.here {
    min-width: calc(6 * var(--space-6) + var(--space-2));
    background: var(--color-accent-soft);
    box-shadow: inset 0 0 0 var(--size-focus-ring) var(--color-accent);
    color: var(--color-text-strong);
    font-weight: var(--weight-semibold);
  }
  .label {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-2);
    white-space: nowrap;
  }
  .name {
    display: flex;
    align-items: center;
    gap: calc(var(--space-1) + var(--border-thick) / 2);
  }
  .tick {
    display: inline-flex;
    color: var(--color-done);
  }
  .tick :global(svg) {
    width: var(--size-icon-small);
    height: var(--size-icon-small);
  }
  .dot {
    width: var(--size-dot);
    height: var(--size-dot);
    border-radius: 50%;
    background: var(--color-open-dot);
  }
  .count {
    color: var(--color-accent-text);
    font-family: var(--font-mono);
    font-weight: var(--weight-medium);
  }
  .bar {
    display: flex;
    height: var(--size-progress-bar);
    border-radius: var(--border-thick);
    background: var(--color-todo-ring);
  }
  .bar i {
    border-radius: var(--border-thick);
    background: var(--color-accent);
  }
  .complete .bar i {
    background: var(--color-done);
  }
  .meta {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    white-space: nowrap;
  }
  .pill {
    display: inline-flex;
    align-items: center;
    height: var(--size-touch-min);
    padding: 0 calc(var(--space-3) + var(--border-thick));
    border: var(--border-thin) solid var(--color-border);
    border-radius: var(--radius-pill);
    background: var(--color-button-bg);
    box-shadow: var(--color-button-shadow);
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
    text-decoration: none;
  }
  .count-all {
    color: var(--color-text-muted);
    font-family: var(--font-mono);
    font-size: var(--font-size-caption);
  }
  .saved {
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  @media (max-width: 899px) {
    .saved {
      display: none;
    }
  }
</style>
