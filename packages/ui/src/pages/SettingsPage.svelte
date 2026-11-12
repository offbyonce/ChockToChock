<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Settings, laid out as in 3.0: one row per choice with a segmented control. This batch has the
view, the detail level, the theme and dimming; they are kept for this visit only until batch 1
saves them on the device, and batch 7 adds the rest.
-->
<script lang="ts">
  import type { Settings } from "@chocktochock/bindings";

  let { settings, onChange }: { settings: Settings; onChange: (s: Settings) => void } = $props();

  type Row = {
    key: "listView" | "essentials" | "detail" | "theme" | "dimming";
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
            onclick={() => onChange({ ...settings, [row.key]: value })}>{label}</button
          >
        {/each}
      </div>
    </div>
  {/each}
</div>
<div class="done"><a class="btn primary" href="#/">Done</a></div>

<style>
  h1 {
    margin: 0 0 0.5rem;
    font-size: 1.6875rem;
    font-weight: 650;
    letter-spacing: -0.02em;
  }
  .lead {
    margin: 0 0 1.125rem;
    color: var(--muted);
  }
  .set {
    display: grid;
    grid-template-columns: minmax(0, 1fr) 15rem;
    align-items: center;
    gap: 1rem 2rem;
    padding: 1rem;
    border-top: 1px solid var(--line);
  }
  .set:first-child {
    border-top: 0;
  }
  .set b {
    font-weight: 600;
  }
  .set p {
    margin: 0.125rem 0 0;
    color: var(--muted);
    font-size: 0.875rem;
  }
  /* Equal segments, in one aligned column on every row. */
  .seg {
    display: grid;
    grid-auto-columns: 1fr;
    grid-auto-flow: column;
    gap: 0.25rem;
    padding: 0.25rem;
    border-radius: 0.625rem;
    background: var(--zone);
  }
  .seg button {
    min-width: 0;
    min-height: 2.75rem;
    padding: 0 0.25rem;
    border-radius: 0.5rem;
    color: var(--muted);
    font-weight: 500;
  }
  .seg button[aria-checked="true"] {
    background: var(--surface);
    box-shadow: 0 1px 2px rgb(0 0 0 / 25%);
    color: var(--ink);
  }
  .done {
    display: flex;
    justify-content: flex-end;
  }
  @media (max-width: 640px) {
    .set {
      grid-template-columns: 1fr;
    }
  }
</style>
