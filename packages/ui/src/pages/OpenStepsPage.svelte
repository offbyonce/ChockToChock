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
    margin: 0 0 0.5rem;
    font-size: 1.6875rem;
    font-weight: 650;
    letter-spacing: -0.02em;
  }
  .lead {
    margin: 0 0 1.125rem;
    color: var(--muted);
  }
  header {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    padding: 0.75rem 1rem;
    border-bottom: 1px solid var(--line);
  }
  h2 {
    margin: 0;
    font-size: 0.9375rem;
    font-weight: 600;
  }
  .link {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    min-height: 2.75rem;
    margin-left: auto;
    color: var(--accent);
    font-size: 0.8125rem;
    font-weight: 500;
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
    gap: 0.5rem 1rem;
    padding: 0.625rem 1rem;
    border-top: 1px solid var(--line);
  }
  li:first-child {
    border-top: 0;
  }
  .txt {
    display: flex;
    flex: 1;
    flex-wrap: wrap;
    align-items: baseline;
    gap: 0.125rem 1rem;
    min-width: 0;
  }
  b {
    font-weight: 500;
  }
  .val {
    margin-left: auto;
    font: 0.8125rem var(--mono);
  }
  small {
    flex-basis: 100%;
    color: var(--muted);
    font-size: 0.7812rem;
  }
  .badge {
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
  .acts {
    display: flex;
    gap: 0.5rem;
  }
</style>
