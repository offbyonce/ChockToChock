<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The one chip: a short status or label beside a name ("Current", "Done 10:11 PM", "Skipped, still
open", "Not modeled in the simulator", the E for Essentials, a count). Tones carry meaning with
words and shape as well as color: a solid chip for the current step, an outline for done, a dashed
outline for skipped.
-->
<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    tone = "neutral",
    solid = false,
    dashed = false,
    title,
    children,
  }: {
    tone?: "neutral" | "accent" | "go" | "warn" | "stop";
    solid?: boolean;
    dashed?: boolean;
    title?: string;
    children: Snippet;
  } = $props();
</script>

<span class="c {tone}" class:solid class:dashed {title}>{@render children()}</span>

<style>
  .c {
    display: inline-flex;
    align-items: center;
    gap: var(--space-1);
    padding: 0 var(--space-2);
    border: var(--border-thin) solid var(--line);
    border-radius: var(--radius-full);
    color: var(--muted);
    font: var(--weight-medium) var(--text-xs) / var(--leading-normal) var(--sans);
    white-space: nowrap;
    vertical-align: middle;
  }
  .accent {
    border-color: var(--accent);
    color: var(--accent);
  }
  .go {
    border-color: var(--go);
    color: var(--go);
  }
  .warn {
    border-color: var(--warn);
    color: var(--warn);
  }
  .stop {
    border-color: var(--stop);
    color: var(--stop);
  }
  .solid.accent {
    background: var(--accent-surface);
    color: var(--on-accent);
  }
  .solid.neutral {
    background: var(--zone);
    color: var(--zone-ink);
  }
  .dashed {
    border-style: dashed;
  }
</style>
