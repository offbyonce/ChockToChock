<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

A small ring that fills as a phase's counted steps are done. Complete shows a check mark, so it
reads without color. The words are in the label beside it.
-->
<script lang="ts">
  import type { Progress } from "@chocktochock/bindings";

  let { progress, complete }: { progress: Progress; complete: boolean } = $props();
  const R = 9;
  const C = 2 * Math.PI * R;
  let filled = $derived(progress.total === 0 ? 0 : (progress.done / progress.total) * C);
</script>

<svg class="ring" viewBox="0 0 24 24" aria-hidden="true">
  <circle class="track" cx="12" cy="12" r={R} />
  {#if complete}
    <circle class="full" cx="12" cy="12" r={R} />
    <path class="check" d="M8 12.5l2.6 2.6L16 9.6" />
  {:else}
    <circle
      class="fill"
      cx="12"
      cy="12"
      r={R}
      stroke-dasharray={`${filled} ${C}`}
      transform="rotate(-90 12 12)"
    />
  {/if}
</svg>

<style>
  .ring {
    width: 1.5rem;
    height: 1.5rem;
    flex: none;
  }
  circle {
    fill: none;
    stroke-width: 2.5;
  }
  .track {
    stroke: var(--border);
  }
  .fill {
    stroke: var(--accent);
    stroke-linecap: round;
  }
  .full {
    stroke: var(--done);
    fill: var(--done);
  }
  .check {
    fill: none;
    stroke: var(--panel);
    stroke-width: 2.5;
    stroke-linecap: round;
    stroke-linejoin: round;
  }
</style>
