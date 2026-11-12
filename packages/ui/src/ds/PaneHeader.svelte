<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The one header for a page or pane: an optional crumb above ("Departure · Phase 2 of 8"), the
title, an optional line under it, and actions on the right. Pages use level 1, panes level 2.
-->
<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    title,
    crumb,
    lead,
    level = 1,
    actions,
  }: {
    title: string;
    crumb?: string;
    lead?: string;
    level?: 1 | 2;
    actions?: Snippet;
  } = $props();
</script>

<header class="ph" class:pane={level === 2}>
  <div class="text">
    {#if crumb}<p class="crumb">{crumb}</p>{/if}
    <svelte:element this={level === 1 ? "h1" : "h2"} class="title">{title}</svelte:element>
    {#if lead}<p class="lead">{lead}</p>{/if}
  </div>
  {#if actions}<div class="actions">{@render actions()}</div>{/if}
</header>

<style>
  .ph {
    display: flex;
    flex-wrap: wrap;
    align-items: flex-end;
    justify-content: space-between;
    gap: var(--space-2) var(--space-4);
    margin-bottom: var(--space-6);
  }
  .pane {
    margin-bottom: var(--space-4);
  }
  .text {
    min-width: 0;
    max-width: var(--measure);
  }
  .crumb {
    margin: 0 0 var(--space-1);
    color: var(--muted);
    font: var(--weight-regular) var(--text-sm) / var(--leading-normal) var(--sans);
  }
  .title {
    margin: 0;
    font: var(--weight-bold) var(--text-2xl) / var(--leading-tight) var(--sans);
    letter-spacing: var(--tracking-title);
  }
  .pane .title {
    font-size: var(--text-xl);
  }
  .lead {
    margin: var(--space-2) 0 0;
    color: var(--muted);
    font: var(--weight-regular) var(--text-md) / var(--leading-normal) var(--sans);
  }
  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-2);
  }
</style>
