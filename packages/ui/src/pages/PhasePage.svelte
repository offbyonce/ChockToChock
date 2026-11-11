<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

One phase. Batch 0 shows its title and progress; its lists arrive in step 7, batch 1.
-->
<script lang="ts">
  import type { RailPhase } from "@chocktochock/bindings";

  let { phase, stage }: { phase: RailPhase; stage: string | null } = $props();

  let summary = $derived.by(() => {
    const p = phase.progress;
    const parts = [`${p.done} of ${p.total} done`];
    if (p.skipped > 0) parts.push(`${p.skipped} skipped, still open`);
    if (p.toDo > 0) parts.push(`${p.toDo} to do`);
    return parts.join(" · ");
  });
</script>

<article class="page">
  {#if stage}<p class="stage">{stage}</p>{/if}
  <h1>{phase.title}</h1>
  <p class="summary">{phase.complete ? "Complete" : summary}</p>
  <p class="placeholder">The checklists and flows for this phase arrive in step 7, batch 1.</p>
</article>

<style>
  .page {
    max-width: 52rem;
    padding: 1.5rem;
  }
  .stage {
    margin: 0;
    color: var(--muted);
    font-size: 0.875rem;
    font-weight: 700;
    letter-spacing: 0.06em;
    text-transform: uppercase;
  }
  h1 {
    margin: 0.25rem 0 0.5rem;
    font-size: 1.75rem;
  }
  .summary {
    margin: 0 0 1.5rem;
    color: var(--muted);
  }
  .placeholder {
    padding: 1rem 1.25rem;
    border: 1px dashed var(--border);
    border-radius: 0.5rem;
    color: var(--muted);
  }
</style>
