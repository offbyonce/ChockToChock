<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Settings, laid out as in 3.0: one row per choice with a segmented control. This batch has the
view, the detail level, the theme and dimming; they are kept for this visit only until batch 1
saves them on the device, and batch 7 adds the rest.
-->
<script lang="ts">
  import Toggle from "../lib/Toggle.svelte";
  import type { Settings } from "@chocktochock/bindings";

  let { settings, onChange }: { settings: Settings; onChange: (s: Settings) => void } = $props();

  type Row = {
    key: "listView" | "essentials" | "detail" | "theme" | "textSize" | "dimming";
    title: string;
    text: string;
    choices: [string | number | boolean, string][];
  };
  const ROWS: Row[] = [
    {
      key: "listView",
      title: "View",
      text: "Checklist shows full lists you tick. Flow shows one step at a time in large text.",
      choices: [
        ["checklist", "Checklist"],
        ["flow", "Flow"],
      ],
    },
    {
      key: "essentials",
      title: "Steps",
      text: "Full is maximum immersion, with every step. Essentials is only what you need, in the right order, to get off the ground fast.",
      choices: [
        [false, "Full"],
        [true, "Essentials"],
      ],
    },
    {
      key: "detail",
      title: "Step info",
      text: "Detailed shows where each control is and lets you open a step for more. Simple shows only the step and its setting.",
      choices: [
        ["expanded", "Detailed"],
        ["quick", "Simple"],
      ],
    },
    {
      key: "theme",
      title: "Appearance",
      text: "Auto follows your system setting.",
      choices: [
        ["system", "Auto"],
        ["light", "Light"],
        ["dark", "Dark"],
      ],
    },
    {
      key: "textSize",
      title: "Text and controls size",
      text: "Scales everything, on top of the automatic wide-screen scaling.",
      choices: [
        ["100", "100%"],
        ["115", "115%"],
        ["130", "130%"],
      ],
    },
    {
      key: "dimming",
      title: "Dimming",
      text: "Dims the whole screen for a dark room. Text stays readable at every step.",
      choices: [
        [0, "Off"],
        [1, "1"],
        [2, "2"],
        [3, "3"],
        [4, "4"],
      ],
    },
  ];
</script>

<h1>Settings</h1>
<p class="lead">Choose how the checklist looks. Your choices stay in place when you clear a flight.</p>
{#snippet preview()}
  <!-- Settings › Panel preview in steps (round 6, R6Settings): On or Off. -->
  <div class="set">
    <div>
      <b id="set-preview">Panel preview in steps</b>
      <p>The small outline of the panel beside each step. Show panel and Where is this? always work.</p>
    </div>
    <Toggle on={settings.panelPreview} labelledby="set-preview" onToggle={() => onChange({ ...settings, panelPreview: !settings.panelPreview })} />
  </div>
{/snippet}
<div class="card">
  {#each ROWS as row (row.key)}
    <div class="set">
      <div>
        <b id="set-{row.key}">{row.title}</b>
        <p>{row.text}</p>
      </div>
      <div class="seg" role="radiogroup" aria-labelledby="set-{row.key}">
        {#each row.choices as [value, label] (String(value))}
          <button
            type="button"
            role="radio"
            aria-checked={settings[row.key] === value}
            onclick={() =>
              onChange({
                ...settings,
                [row.key]: value,
              })}>{label}</button
          >
        {/each}
      </div>
    </div>
    {#if row.key === "textSize"}{@render preview()}{/if}
  {/each}
</div>
<div class="done"><a class="btn primary" href="#/">Done</a></div>

<style>
  h1 {
    margin: 0 0 var(--space-2);
    font-size: var(--font-size-section-title);
    font-weight: 650;
  }
  .lead {
    margin: 0 0 var(--space-4);
    color: var(--color-text-muted);
  }
  .set {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    gap: var(--space-4) var(--space-8);
    padding: var(--space-4);
    border-top: var(--border-thin) solid var(--color-border);
  }
  .set:first-child {
    border-top: 0;
  }
  .set b {
    font-weight: var(--weight-semibold);
  }
  .set p {
    margin: var(--space-1) 0 0;
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
  }
  /* Equal segments, in one aligned column on every row. */
  .seg {
    display: grid;
    grid-auto-columns: 1fr;
    grid-auto-flow: column;
    gap: var(--space-1);
    padding: var(--space-1);
    border-radius: var(--radius-control);
    background: var(--color-surface-2);
  }
  .seg button {
    min-width: 0;
    min-height:  var(--size-touch-min);
    padding: 0 var(--space-3);
    border-radius: var(--radius-control);
    color: var(--color-text-muted);
    font-weight: var(--weight-medium);
  }
  .seg button[aria-checked="true"] {
    background: var(--color-surface);
    box-shadow: inset 0 0 0 var(--size-focus-ring) var(--color-accent);
    color: var(--color-text);
  }
  /* The toggle switch (rules.md components; R6Settings). */
  .done {
    display: flex;
    justify-content: flex-end;
  }
  @media (max-width: 599px) {
    .set {
      grid-template-columns: 1fr;
    }
  }
</style>
