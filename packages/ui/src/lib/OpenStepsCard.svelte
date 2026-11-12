<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Open steps from earlier pages, as in 3.0: a card at the top of a phase saying how many steps are
still open on the phases before it that the pilot started, which opens into the list. Each step
can be ticked here, and is marked Skipped or Missed in words. Shows nothing when none are open.
-->
<script lang="ts">
  import type { OpenStep } from "@chocktochock/bindings";
  import type { PackIndex } from "../pack-index";
  import { phaseHref } from "../route.svelte";
  import Icon from "./Icon.svelte";

  let { index, open, onTick }: { index: PackIndex; open: OpenStep[]; onTick: (id: string) => void } = $props();

  let shown = $state(false);
  let groups = $derived.by(() => {
    const out: { phase: string; title: string; steps: OpenStep[] }[] = [];
    for (const o of open) {
      const last = out[out.length - 1];
      if (last?.phase === o.phase) last.steps.push(o);
      else out.push({ phase: o.phase, title: o.phaseTitle, steps: [o] });
    }
    return out;
  });
</script>

{#if open.length}
  <section class="card opn" class:shown>
    <button class="line" type="button" aria-expanded={shown} aria-controls="open-list" onclick={() => (shown = !shown)}>
      <b><i class="dot" aria-hidden="true"></i>Open steps</b>
      <span class="sum">
        {open.length} from earlier pages{#each groups as g (g.phase)}<i>·</i>{g.title} {g.steps.length}{/each}
      </span>
      <span class="ed">{shown ? "Hide" : "Show"}</span>
    </button>
    {#if shown}
      <div id="open-list" class="list">
        <div class="hd">
          <h2>Open steps</h2>
          <button class="close" type="button" onclick={() => (shown = false)}>Close</button>
        </div>
        {#each groups as g (g.phase)}
          <a class="go" href={phaseHref(g.phase)}>{g.title} <Icon name="arrow-right" /></a>
          {#each g.steps as o (o.step)}
            {@const step = index.step(o.step)}
            <div class="row">
              <button class="tick" type="button" aria-label="Tick {step?.text ?? o.step}" onclick={() => onTick(o.step)}>
                <span></span>
              </button>
              <span class="st"><b>{step?.text}</b><span>{step?.expect}</span></span>
              <span class="badge" class:sk={o.skipped}>{o.skipped ? "Skipped" : "Missed"}</span>
            </div>
          {/each}
        {/each}
        <div class="ft"><button class="btn primary" type="button" onclick={() => (shown = false)}>Done</button></div>
      </div>
    {/if}
  </section>
{/if}

<style>
  /* 3.0's open steps card, under the flight strip. */
  .opn {
    margin-bottom: var(--space-3);
  }
  .line {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    width: 100%;
    min-height:  var(--size-touch-min);
    padding: var(--space-2) var(--space-4);
    text-align: left;
  }
  .line b {
    display: inline-flex;
    flex: none;
    align-items: center;
    gap: var(--space-2);
    font-weight: var(--weight-semibold);
  }
  .dot {
    width: var(--size-dot);
    height: var(--size-dot);
    border-radius: 50%;
    background: var(--color-open-text);
  }
  .sum {
    flex: 1;
    min-width: 0;
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
  }
  .sum i {
    margin: 0 var(--space-2);
    font-style: normal;
  }
  .ed {
    flex: none;
    color: var(--color-accent);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-medium);
  }
  .list {
    padding: var(--space-1) var(--space-4) var(--space-3);
    border-top: var(--border-thin) solid var(--color-border);
  }
  .go {
    display: inline-flex;
    align-items: center;
    gap: var(--space-1);
    min-height:  var(--size-touch-min);
    color: var(--color-accent);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-medium);
    text-decoration: none;
  }
  .row {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-2) 0;
    border-top: var(--border-thin) solid var(--color-border);
  }
  .tick {
    display: grid;
    place-items: center;
    width:  var(--size-touch-min);
    height:  var(--size-touch-min);
    flex: none;
  }
  .tick span {
    width: var(--size-status-icon);
    height: var(--size-status-icon);
    border: var(--border-ring) solid var(--color-text-muted);
    border-radius: var(--radius-small);
  }
  .st {
    display: flex;
    flex: 1;
    flex-direction: column;
    min-width: 0;
  }
  .st b {
    font-weight: var(--weight-medium);
  }
  .st span {
    color: var(--color-text-muted);
    font: var(--weight-regular) var(--font-size-value-mono) var(--font-mono);
  }
  .hd {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding-top: var(--space-2);
  }
  .hd h2 {
    margin: 0;
    font-size: var(--font-size-body);
    font-weight: var(--weight-semibold);
  }
  .close {
    min-height:  var(--size-touch-min);
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
  }
  .badge {
    flex: none;
    padding: var(--space-1) var(--space-2);
    border: var(--border-thin) solid var(--color-border);
    border-radius: var(--radius-pill);
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-semibold);
  }
  .badge.sk {
    border: var(--border-thin) dashed var(--color-open-text);
    color: var(--color-open-text);
  }
  .ft {
    display: flex;
    justify-content: flex-end;
    padding-top: var(--space-2);
  }
</style>
