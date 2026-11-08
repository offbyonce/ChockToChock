<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
<script lang="ts">
  import { type AppInfo, type Checklist, appInfo, exampleChecklist, isAppError } from "@chocktochock/bindings";
  import ChecklistView from "./lib/ChecklistView.svelte";

  let info = $state<AppInfo | null>(null);
  let list = $state<Checklist | null>(null);
  let error = $state<string | null>(null);

  async function load(): Promise<void> {
    try {
      [info, list] = await Promise.all([appInfo(), exampleChecklist()]);
    } catch (e) {
      error = isAppError(e) ? `${e.kind}: ${e.message}` : String(e);
    }
  }

  load();
</script>

<header>
  <h1>Chock to Chock</h1>
  {#if info}<span class="version">v{info.version}</span>{/if}
</header>

<main>
  {#if error}
    <p class="error" role="alert">{error}</p>
  {:else if list}
    <ChecklistView {list} />
  {:else}
    <p class="muted">Loading…</p>
  {/if}
</main>

<style>
  header {
    display: flex;
    align-items: baseline;
    gap: 0.75rem;
    padding: 1rem 1.5rem;
    border-bottom: 1px solid var(--panel);
  }
  h1 {
    margin: 0;
    font-size: 1.25rem;
  }
  .version,
  .muted {
    color: var(--muted);
  }
  main {
    padding: 1.5rem;
    max-width: 48rem;
  }
  .error {
    color: var(--accent);
  }
</style>
