<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
<script lang="ts">
  import type { Checklist } from "@chocktochock/bindings";
  import { SvelteSet } from "svelte/reactivity";

  let { list }: { list: Checklist } = $props();

  // Phase 0 keeps ticks in the window only. Phase 2 moves sessions to Rust (store crate).
  const done = new SvelteSet<string>();

  function toggle(id: string): void {
    if (done.has(id)) done.delete(id);
    else done.add(id);
  }
</script>

<section aria-labelledby="list-title">
  <h2 id="list-title">{list.title}</h2>
  <p class="progress">{done.size} of {list.steps.length} done</p>
  <ol>
    {#each list.steps as step (step.id)}
      <li class:done={done.has(step.id)}>
        <label>
          <input type="checkbox" checked={done.has(step.id)} onchange={() => toggle(step.id)} />
          <span class="text">{step.text}</span>
          <span class="dots" aria-hidden="true"></span>
          <span class="expect">{step.expect}</span>
        </label>
        {#if step.note}<p class="note">{step.note}</p>{/if}
      </li>
    {/each}
  </ol>
</section>

<style>
  h2 {
    margin: 0 0 0.25rem;
    font-size: 1.1rem;
  }
  .progress {
    margin: 0 0 1rem;
    color: var(--muted);
  }
  ol {
    list-style: none;
    margin: 0;
    padding: 0;
  }
  li {
    padding: 0.6rem 0.75rem;
    border-radius: 6px;
    background: var(--panel);
    margin-bottom: 0.5rem;
  }
  label {
    display: flex;
    align-items: baseline;
    gap: 0.6rem;
    cursor: pointer;
  }
  .dots {
    flex: 1;
    border-bottom: 1px dotted var(--muted);
  }
  .expect {
    font-weight: 600;
    color: var(--accent);
  }
  .done .text,
  .done .expect {
    color: var(--done);
  }
  .note {
    margin: 0.3rem 0 0 1.9rem;
    color: var(--muted);
    font-size: 0.9rem;
  }
</style>
