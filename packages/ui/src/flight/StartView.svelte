<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Start options (Start.dc.html): the Exterior inspection toggle with Preview the inspection (the
Exterior phase until batch 6's inspection view), Power source (also asked at Power up), the iFly
panel state, and Turnaround. The descriptions are the board's, on batch 7's validation list.
-->
<script lang="ts">
  import type { Flight, StartState, UserOption } from "@chocktochock/bindings";
  import { phaseHref } from "../route.svelte";
  import Toggle from "../lib/Toggle.svelte";
  import Icon from "../lib/Icon.svelte";

  let {
    flight,
    options,
    states,
    onOption,
    onStartState,
  }: {
    flight: Flight;
    options: UserOption[];
    states: StartState[];
    onOption: (option: string, value: string) => void;
    onStartState: (state: string | null) => void;
  } = $props();

  const opt = (id: string) => options.find((o) => o.id === id);
  const value = (id: string) => flight.options[id] ?? opt(id)?.default ?? "";
  let power = $derived(opt("power"));
</script>

<section class="card">
  <div class="text">
    <span class="t" id="st-ext">Exterior inspection</span>
    <span class="d">The walk around the airplane before you board: 18 numbered stops on a diagram. Off skips the stage; anything you ticked is kept.</span>
    <a class="link" href={phaseHref("exterior")}>Preview the inspection</a>
  </div>
  <Toggle on={value("exterior") !== "off"} labelledby="st-ext" onToggle={() => onOption("exterior", value("exterior") === "off" ? "on" : "off")} />
</section>
{#if power}
  <section class="card">
    <div class="text">
      <span class="t" id="st-power">{power.title}</span>
      <span class="d">On ground power, the APU starts before the Before start procedure. The Power up step asks the same question, so you can choose there too.</span>
    </div>
    <span class="seg" role="group" aria-labelledby="st-power">
      {#each power.values as v (v)}
        <button type="button" aria-pressed={value("power") === v} onclick={() => onOption("power", v)}>
          {power.choices?.find((c) => c.value === v)?.label ?? v}
        </button>
      {/each}
    </span>
  </section>
{/if}
{#if states.length}
  <section class="card">
    <div class="text">
      <span class="t">iFly panel state</span>
      <span class="d">Loaded a panel state from the iFly EFB? Pick it to tick the steps it has already set and start where the airplane is. Steps it can't set, like the FMC, MCP and radios, stay open.</span>
    </div>
    <label class="pick">
      Panel state
      <span class="sel">
        <select value={flight.startState ?? ""} onchange={(e) => onStartState(e.currentTarget.value || null)}>
          <option value="">Choose a panel state</option>
          {#each states as s (s.id)}<option value={s.id}>{s.title}</option>{/each}
        </select>
        <Icon name="chevron-down" />
      </span>
    </label>
  </section>
{/if}
<section class="card">
  <div class="text">
    <span class="t" id="st-turn">Turnaround</span>
    <span class="d">Next leg in the same airplane. Starts from the turnaround steps instead of cold and dark.</span>
  </div>
  <Toggle on={value("start") === "turn"} labelledby="st-turn" onToggle={() => onOption("start", value("start") === "turn" ? "cold" : "turn")} />
</section>

<style>
  .card {
    flex: none;
    margin: 0;
    display: flex;
    align-items: flex-start;
    gap: calc(var(--space-4) + var(--space-1));
    max-width: calc(26 * var(--space-10));
    padding: var(--space-4) calc(var(--space-4) + var(--space-1));
    border: var(--border-thin) solid var(--color-border);
    border-radius: var(--radius-panel);
    background: var(--color-surface);
  }
  .text {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }
  .t {
    font-size: var(--font-size-body);
    font-weight: var(--weight-semibold);
  }
  .d {
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
    line-height: var(--leading-normal);
  }
  .link {
    display: inline-flex;
    align-items: center;
    align-self: flex-start;
    min-height: var(--size-touch-min);
    color: var(--color-link);
    font-size: var(--font-size-secondary);
    text-decoration: none;
  }
  .seg {
    display: flex;
    flex: none;
    padding: var(--border-thick);
    border: var(--border-thin) solid var(--color-segment-border);
    border-radius: var(--radius-segment);
    background: var(--color-segment-track);
  }
  .seg button {
    height: var(--size-touch-min);
    padding: 0 calc(var(--space-3) + var(--space-1) / 2);
    border: 0;
    border-radius: var(--radius-control);
    background: transparent;
    color: var(--color-text-muted);
    font: inherit;
    font-size: var(--font-size-secondary);
  }
  .seg button[aria-pressed="true"] {
    background: var(--color-segment-on);
    box-shadow:
      inset 0 0 0 calc(var(--border-thick) * 3 / 4) var(--color-accent),
      var(--color-segment-shadow);
    color: var(--color-text-strong);
    font-weight: var(--weight-semibold);
  }
  .pick {
    display: flex;
    flex-direction: column;
    gap: calc(var(--space-1) + var(--border-thick) / 2);
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  .sel {
    position: relative;
    display: block;
  }
  select {
    appearance: none;
    height: var(--size-touch-min);
    min-width: calc(6 * var(--space-10));
    padding: 0 var(--space-10) 0 var(--space-3);
    border: var(--border-thin) solid var(--color-border);
    border-radius: calc(var(--radius-small) + var(--border-thick));
    background: var(--color-field);
    box-shadow: var(--color-button-shadow);
    color: var(--color-text);
    font: inherit;
    font-size: var(--font-size-secondary);
  }
  .sel :global(.icon) {
    position: absolute;
    top: calc((var(--size-touch-min) - var(--size-icon-small)) / 2);
    right: var(--space-3);
    width: var(--size-icon-small);
    height: var(--size-icon-small);
    color: var(--color-text-muted);
    pointer-events: none;
  }
</style>
