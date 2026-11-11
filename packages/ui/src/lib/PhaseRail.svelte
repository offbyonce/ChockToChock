<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Stages and their phases, with progress. What shows, and what counts, comes from Rust.
-->
<script lang="ts">
  import type { Rail, RailPhase } from "@chocktochock/bindings";
  import { phaseHref } from "../route.svelte";
  import ProgressRing from "./ProgressRing.svelte";

  let {
    rail,
    current,
    onNavigate,
  }: { rail: Rail; current: string | null; onNavigate?: () => void } = $props();

  function status(phase: RailPhase): string {
    if (phase.complete) return "Complete";
    const { done, total } = phase.progress;
    return phase.skipped > 0 ? `${done} of ${total} done, ${phase.skipped} skipped` : `${done} of ${total} done`;
  }
</script>

<nav class="rail" aria-label="Phases">
  {#each rail.stages as stage (stage.title)}
    <section>
      <h2>{stage.title}</h2>
      <ul>
        {#each stage.phases as phase (phase.id)}
          <li>
            <a
              href={phaseHref(phase.id)}
              aria-current={phase.id === current ? "page" : undefined}
              onclick={() => onNavigate?.()}
            >
              <ProgressRing progress={phase.progress} complete={phase.complete} />
              <span class="name">{phase.title}<span class="sr-only">, {status(phase)}</span></span>
              {#if phase.skipped > 0}
                <!-- The spoken status above already says this. -->
                <span class="skipped" aria-hidden="true">{phase.skipped} skipped</span>
              {/if}
            </a>
          </li>
        {/each}
      </ul>
    </section>
  {/each}
</nav>

<style>
  .rail {
    padding: 0.5rem 0.75rem 2rem;
  }
  h2 {
    margin: 1.25rem 0.75rem 0.375rem;
    color: var(--muted);
    font-size: 0.8125rem;
    font-weight: 700;
    letter-spacing: 0.06em;
    text-transform: uppercase;
  }
  ul {
    margin: 0;
    padding: 0;
    list-style: none;
  }
  a {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    min-height: 48px;
    padding: 0 0.75rem;
    border-radius: 0.5rem;
    color: var(--text);
    text-decoration: none;
  }
  a:hover {
    background: var(--raised);
  }
  a[aria-current="page"] {
    background: var(--raised);
    box-shadow: inset 3px 0 0 var(--accent);
    font-weight: 700;
  }
  .name {
    flex: 1;
  }
  .skipped {
    padding: 0.125rem 0.5rem;
    border: 1px dashed var(--skipped);
    border-radius: 999px;
    color: var(--skipped);
    font-size: 0.8125rem;
    white-space: nowrap;
  }
</style>
