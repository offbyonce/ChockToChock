<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Behind the Emergency button: the reference phases, in larger type (step 7 requirement for
non-normal lists).
-->
<script lang="ts">
  import type { RailPhase } from "@chocktochock/bindings";
  import Icon from "../lib/Icon.svelte";
  import { phaseHref } from "../route.svelte";

  let { phases }: { phases: RailPhase[] } = $props();
</script>

<article class="page">
  <h1>Emergency</h1>
  <p class="lead">Memory items, quick reference checklists and non-normal procedures.</p>
  <ul>
    {#each phases as phase (phase.id)}
      <li>
        <a href={phaseHref(phase.id)}>
          <span>{phase.title}</span>
          <span class="count">{phase.progress.total} items</span>
          <Icon name="chevron-right" />
        </a>
      </li>
    {/each}
  </ul>
</article>

<style>
  .page {
    max-width: 52rem;
    padding: 1.5rem;
    font-size: 1.25rem;
  }
  h1 {
    margin: 0 0 0.5rem;
    font-size: 2rem;
  }
  .lead {
    margin: 0 0 1.5rem;
    color: var(--muted);
  }
  ul {
    display: grid;
    gap: 0.75rem;
    margin: 0;
    padding: 0;
    list-style: none;
  }
  a {
    display: flex;
    align-items: center;
    gap: 1rem;
    min-height: 64px;
    padding: 0 1.25rem;
    border: 1px solid var(--border);
    border-left: 6px solid var(--danger);
    border-radius: 0.5rem;
    background: var(--panel);
    color: var(--text);
    font-weight: 700;
    text-decoration: none;
  }
  a span:first-child {
    flex: 1;
  }
  .count {
    color: var(--muted);
    font-size: 1rem;
    font-weight: 400;
  }
</style>
