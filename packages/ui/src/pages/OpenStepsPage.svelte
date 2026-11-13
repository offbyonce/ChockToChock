<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Open steps for the whole flight: on every phase the pilot started, the steps not yet done,
marked Skipped or Missed (decision 0012). Each can be ticked here, a skipped one put back to do,
or found in its phase.
-->
<script lang="ts">
  import type { OpenStep } from "@chocktochock/bindings";
  import Icon from "../lib/Icon.svelte";
  import type { PackIndex } from "../pack-index";
  import { phaseHref } from "../route.svelte";

  let {
    index,
    open,
    onTick,
    onUnskip,
  }: {
    index: PackIndex;
    open: OpenStep[];
    onTick: (id: string) => void;
    onUnskip: (id: string) => void;
  } = $props();

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

<h1>Open steps</h1>
<p class="lead">
  {#if open.length}
    {open.length} {open.length === 1 ? "step is" : "steps are"} still open on phases you started. A
    list with an open step is never complete.
  {:else}
    No open steps. Steps you skip, or leave behind on a phase you started, appear here.
  {/if}
</p>

{#each groups as g (g.phase)}
  <section class="card">
    <header>
      <h2>{g.title}</h2>
      <a class="link" href={phaseHref(g.phase)}>Go to phase <Icon name="arrow-right" /></a>
    </header>
    <ul>
      {#each g.steps as o (o.step)}
        {@const step = index.step(o.step)}
        <li>
          <div class="txt">
            <b>{step?.text ?? o.step}</b>
            <span class="val">{step?.expect}</span>
            <small>{o.listTitle}</small>
          </div>
          <span class="badge" class:sk={o.skipped}>{o.skipped ? "Skipped" : "Missed"}</span>
          <div class="acts">
            {#if o.skipped}<button class="btn" type="button" onclick={() => onUnskip(o.step)}>Unskip</button>{/if}
            <button class="btn primary" type="button" onclick={() => onTick(o.step)}>
              <Icon name="check" /> Tick
            </button>
          </div>
        </li>
      {/each}
    </ul>
  </section>
{/each}

<style>
  h1 {
    margin: 0 0 var(--space-2);
    font-size: var(--font-size-section-title);
    font-weight: 650;
  }
  .lead {
    margin: 0 0 var(--space-4);
    color: var(--color-text-muted);
  }
  header {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-3) var(--space-4);
    border-bottom: var(--border-thin) solid var(--color-border);
  }
  h2 {
    margin: 0;
    font-size: var(--font-size-body);
    font-weight: var(--weight-semibold);
  }
  .link {
    display: inline-flex;
    align-items: center;
    gap: var(--space-1);
    min-height:  var(--size-touch-min);
    margin-left: auto;
    color: var(--color-accent);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-medium);
    text-decoration: none;
  }
  ul {
    margin: 0;
    padding: 0;
    list-style: none;
  }
  li {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--space-2) var(--space-4);
    padding: var(--space-3) var(--space-4);
    border-top: var(--border-thin) solid var(--color-border);
  }
  li:first-child {
    border-top: 0;
  }
  .txt {
    display: flex;
    flex: 1;
    flex-wrap: wrap;
    align-items: baseline;
    gap: var(--space-1) var(--space-4);
    min-width: 0;
  }
  b {
    font-weight: var(--weight-medium);
  }
  .val {
    margin-left: auto;
    font: var(--weight-regular) var(--font-size-value-mono) var(--font-mono);
  }
  small {
    flex-basis: 100%;
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  .badge {
    padding: 0 var(--space-2);
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
  .acts {
    display: flex;
    gap: var(--space-2);
  }
</style>
