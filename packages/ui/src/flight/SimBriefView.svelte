<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

SimBrief and OFP file (SimBrief.dc.html): the username and Import latest flight, what is and is
never brought in, the offline drop zone with Choose file (the file is read once on this device and
only the compact plan is kept, decision 0019), the last import, and Reset flight plan.
-->
<script lang="ts">
  import type { SavedPlan } from "@chocktochock/bindings";
  import Icon from "../lib/Icon.svelte";

  let {
    user,
    plan,
    onImport,
    onFile,
    onClear,
  }: {
    user: string | null;
    plan: SavedPlan | null;
    /** Imports the newest plan for this username; resolves with an error to show, or null. */
    onImport: (user: string) => Promise<string | null>;
    /** Reads a saved OFP file's text; resolves with an error to show, or null. */
    onFile: (text: string) => Promise<string | null>;
    onClear: () => void;
  } = $props();

  // svelte-ignore state_referenced_locally
  let name = $state(user ?? "");
  let busy = $state(false);
  let problem = $state<string | null>(null);
  let over = $state(false);
  let picker: HTMLInputElement | undefined = $state();

  async function run(task: () => Promise<string | null>): Promise<void> {
    busy = true;
    problem = null;
    try {
      problem = await task();
    } finally {
      busy = false;
    }
  }
  async function readFile(file: File | undefined): Promise<void> {
    if (!file) return;
    const text = await file.text();
    await run(() => onFile(text));
  }
  const time = (ms: number) => new Date(ms).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit", hourCycle: "h23" });
  let route = $derived(plan ? `${plan.plan.origin?.icao ?? "—"} → ${plan.plan.destination?.icao ?? "—"}` : "");
</script>

<section class="card col">
  <h3>Import from SimBrief</h3>
  <form
    class="row"
    onsubmit={(e) => {
      e.preventDefault();
      if (name.trim()) void run(() => onImport(name.trim()));
      else problem = "Enter your SimBrief username or pilot ID.";
    }}
  >
    <label class="user">
      SimBrief username or pilot ID
      <input bind:value={name} placeholder="Your SimBrief username" autocomplete="off" autocapitalize="off" spellcheck="false" />
    </label>
    <button class="primary" type="submit" disabled={busy}>Import latest flight</button>
  </form>
  {#if problem}<p class="problem" role="alert">{problem}</p>{/if}
  <span class="d">No password. The username is saved on this device only, and the import reads your newest SimBrief plan. SimBrief import is the only part of the checklist that uses the internet.</span>
  <div class="two">
    <div class="pair">
      <span class="k">What's brought in</span>
      <span class="v">Route, runway, SID, fuel, zero fuel weight, reserves, cost index, cruise wind, ISA deviation, transition levels and the airport weather</span>
    </div>
    <div class="pair">
      <span class="k">Never brought in</span>
      <span class="v">Names, airline details and other personal fields. The full OFP text isn't stored.</span>
    </div>
  </div>
</section>
<section class="card col tight">
  <h3>Offline: use a saved OFP file</h3>
  <div
    class="drop"
    class:over
    role="region"
    aria-label="Drop a saved OFP file here"
    ondragover={(e) => {
      e.preventDefault();
      over = true;
    }}
    ondragleave={() => (over = false)}
    ondrop={(e) => {
      e.preventDefault();
      over = false;
      void readFile(e.dataTransfer?.files[0]);
    }}
  >
    <span class="up"><Icon name="file-upload" /></span>
    <span class="dt">
      <span class="t">Drop a saved OFP file here</span>
      <span class="d">The file is read once on this device. Only the flight data listed above is kept.</span>
    </span>
    <button class="btn choose" type="button" disabled={busy} onclick={() => picker?.click()}>Choose file</button>
    <input
      class="sr-only"
      type="file"
      accept=".json,.xml,application/json,text/xml,application/xml"
      tabindex="-1"
      bind:this={picker}
      onchange={(e) => {
        void readFile(e.currentTarget.files?.[0]);
        e.currentTarget.value = "";
      }}
    />
  </div>
</section>
{#if plan}
  <section class="card last">
    <span class="dt">
      <span class="t">Last import</span>
      <span class="d">{route}, imported {time(plan.loadedAt)}. Values you typed since then are kept unless you import again.</span>
    </span>
    <button class="reset" type="button" onclick={onClear}>Reset flight plan</button>
  </section>
{/if}

<style>
  .card {
    flex: none;
    margin: 0;
    max-width: calc(26 * var(--space-10));
    padding: calc(var(--space-4) + var(--space-1));
    border: var(--border-thin) solid var(--color-border);
    border-radius: var(--radius-panel);
    background: var(--color-surface);
  }
  .col {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }
  .col.tight {
    gap: var(--space-3);
  }
  h3 {
    margin: 0;
    font-size: var(--font-size-body);
    font-weight: var(--weight-semibold);
  }
  .row {
    display: flex;
    align-items: flex-end;
    gap: var(--space-3);
  }
  .user {
    flex: 1;
    max-width: calc(9 * var(--space-10));
    display: flex;
    flex-direction: column;
    gap: calc(var(--space-1) + var(--border-thick) / 2);
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  .user input {
    height: var(--size-touch-min);
    box-sizing: border-box;
    padding: 0 var(--space-3);
    border: var(--border-thin) solid var(--color-field-border);
    border-radius: calc(var(--radius-small) + var(--border-thick));
    background: var(--color-field);
    color: var(--color-text-strong);
    font: inherit;
    font-size: var(--font-size-body);
  }
  .primary {
    height: var(--size-touch-min);
    padding: 0 calc(var(--space-4) + var(--space-1));
    border: 0;
    border-radius: var(--radius-control);
    background: var(--color-accent);
    color: var(--color-on-accent);
    font: inherit;
    font-size: var(--font-size-body);
    font-weight: var(--weight-bold);
  }
  .primary:disabled {
    opacity: 0.6;
  }
  .problem {
    margin: 0;
    color: var(--color-danger-text);
    font-size: var(--font-size-secondary);
  }
  .d {
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
    line-height: var(--leading-normal);
  }
  .two {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: var(--space-4);
  }
  .pair {
    display: flex;
    flex-direction: column;
    gap: calc(var(--space-1) + var(--border-thick) / 2);
  }
  .k {
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-semibold);
  }
  .v {
    color: var(--color-text);
    font-size: var(--font-size-secondary);
    line-height: calc(var(--leading-normal) + 0.1);
  }
  .drop {
    display: flex;
    align-items: center;
    gap: calc(var(--space-4) + var(--space-1));
    padding: var(--space-6);
    border: var(--border-thick) dashed var(--color-drop-zone-border);
    border-radius: var(--radius-panel);
    background: var(--color-drop-zone);
  }
  .drop.over {
    border-color: var(--color-accent);
  }
  .up {
    display: inline-flex;
    color: var(--color-text-secondary);
  }
  .up :global(.icon) {
    width: calc(var(--space-8) + var(--space-1));
    height: calc(var(--space-8) + var(--space-1));
  }
  .dt {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }
  .t {
    font-size: var(--font-size-body);
    font-weight: var(--weight-semibold);
  }
  .choose {
    padding: 0 var(--space-4);
    border-color: var(--color-border);
  }
  .last {
    display: flex;
    align-items: center;
    gap: var(--space-4);
    padding: var(--space-4) calc(var(--space-4) + var(--space-1));
  }
  .reset {
    height: var(--size-touch-min);
    padding: 0 var(--space-3);
    border: 0;
    background: transparent;
    color: var(--color-danger-text);
    font: inherit;
    font-size: var(--font-size-secondary);
  }
</style>
