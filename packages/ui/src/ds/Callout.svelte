<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

A step's CHECK and IF NOT cells: a small caption, the thing to check, and a line of detail. Accent
for what to check, warn for what it means if not.
-->
<script lang="ts">
  let {
    tone = "accent",
    caption,
    title,
    detail,
  }: {
    tone?: "accent" | "warn" | undefined;
    caption: string;
    title: string;
    detail?: string | undefined;
  } = $props();
</script>

<div class="co {tone}">
  <small>{caption}</small>
  <b>{title}</b>
  {#if detail}<span>{detail}</span>{/if}
</div>

<style>
  .co {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    min-width: 0;
    padding: var(--space-3) var(--space-4);
    border: var(--border-thin) solid var(--accent);
    border-radius: var(--radius-md);
    background: var(--accent-soft);
  }
  .warn {
    border-color: var(--warn);
    background: var(--warn-soft);
  }
  small {
    color: var(--accent);
    font: var(--weight-bold) var(--text-xs) / var(--leading-normal) var(--sans);
    letter-spacing: var(--tracking-caps);
    text-transform: uppercase;
  }
  .warn small {
    color: var(--warn);
  }
  b {
    font: var(--weight-bold) var(--text-md) / var(--leading-snug) var(--sans);
  }
  span {
    color: var(--muted);
    font: var(--weight-regular) var(--text-sm) / var(--leading-snug) var(--sans);
  }
</style>
