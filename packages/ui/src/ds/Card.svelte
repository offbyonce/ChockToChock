<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The one card: a surface with an optional header (a title, a kind tag, an action on the right) and
a body. Lists inside a card run edge to edge (flush); everything else gets the card's padding.
-->
<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    title,
    kind,
    flush = false,
    actions,
    children,
  }: {
    title?: string | undefined;
    /** A small tag before the title ("Procedure", "Checklist"). */
    kind?: string | undefined;
    flush?: boolean | undefined;
    actions?: Snippet | undefined;
    children: Snippet;
  } = $props();
</script>

<section class="card" class:flush>
  {#if title || actions}
    <header>
      {#if kind}<span class="kind">{kind}</span>{/if}
      {#if title}<h2>{title}</h2>{/if}
      {#if actions}<div class="actions">{@render actions()}</div>{/if}
    </header>
  {/if}
  <div class="body">{@render children()}</div>
</section>

<style>
  .card {
    min-width: 0;
    border: var(--border-thin) solid var(--line);
    border-radius: var(--radius-lg);
    background: var(--surface);
  }
  header {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    min-height: var(--space-12);
    padding: var(--space-2) var(--space-4);
    border-bottom: var(--border-thin) solid var(--line);
  }
  .kind {
    padding: 0 var(--space-2);
    border-radius: var(--radius-sm);
    background: var(--zone);
    color: var(--zone-ink);
    font: var(--weight-medium) var(--text-xs) / var(--leading-normal) var(--sans);
  }
  h2 {
    flex: 1 1 auto;
    min-width: 0;
    margin: 0;
    font: var(--weight-bold) var(--text-md) / var(--leading-tight) var(--sans);
  }
  .actions {
    display: flex;
    flex: none;
    align-items: center;
    gap: var(--space-2);
    margin-left: auto;
    white-space: nowrap;
  }
  .body {
    padding: var(--space-4);
  }
  .flush .body {
    padding: 0;
  }
</style>
