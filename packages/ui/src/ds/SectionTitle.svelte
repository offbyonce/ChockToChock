<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The one section title: a small upper-case caption over a group ("THE CONTROLS", "ROUTE", menu
groups). One size, one weight, one tracking everywhere.
-->
<script lang="ts">
  import type { Snippet } from "svelte";

  let { id, children }: { id?: string; children: Snippet } = $props();
</script>

<h3 {id}>{@render children()}</h3>

<style>
  h3 {
    margin: 0 0 var(--space-2);
    color: var(--muted);
    font: var(--weight-bold) var(--text-xs) / var(--leading-normal) var(--sans);
    letter-spacing: var(--tracking-caps);
    text-transform: uppercase;
  }
</style>
