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
    margin-bottom: 0.625rem;
  }
  .line {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    width: 100%;
    min-height: 2.75rem;
    padding: 0.5rem 1rem;
    text-align: left;
  }
  .line b {
    display: inline-flex;
    flex: none;
    align-items: center;
    gap: 0.5rem;
    font-weight: 600;
  }
  .dot {
    width: 0.5rem;
    height: 0.5rem;
    border-radius: 50%;
    background: var(--warn);
  }
  .sum {
    flex: 1;
    min-width: 0;
    color: var(--muted);
    font-size: 0.875rem;
  }
  .sum i {
    margin: 0 0.4rem;
    font-style: normal;
  }
  .ed {
    flex: none;
    color: var(--accent);
    font-size: 0.875rem;
    font-weight: 500;
  }
  .list {
    padding: 0.25rem 1rem 0.75rem;
    border-top: 1px solid var(--line);
  }
  .go {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    min-height: 2.75rem;
    color: var(--accent);
    font-size: 0.875rem;
    font-weight: 500;
    text-decoration: none;
  }
  .row {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    padding: 0.375rem 0;
    border-top: 1px solid var(--line);
  }
  .tick {
    display: grid;
    place-items: center;
    width: 2.75rem;
    height: 2.75rem;
    flex: none;
  }
  .tick span {
    width: 1.375rem;
    height: 1.375rem;
    border: 1.5px solid var(--muted);
    border-radius: 0.4375rem;
  }
  .st {
    display: flex;
    flex: 1;
    flex-direction: column;
    min-width: 0;
  }
  .st b {
    font-weight: 500;
  }
  .st span {
    color: var(--muted);
    font: 0.8125rem var(--mono);
  }
  .badge {
    flex: none;
    padding: 0 0.5rem;
    border: 1px solid var(--line);
    border-radius: 99rem;
    color: var(--muted);
    font-size: 0.72rem;
    font-weight: 600;
  }
  .badge.sk {
    border: 1px dashed var(--warn);
    color: var(--warn);
  }
  .ft {
    display: flex;
    justify-content: flex-end;
    padding-top: 0.5rem;
  }
</style>
