<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The header (Checklist board, 60 px): menu, the app's name and aircraft, the stage tabs (the
current one bold and underlined, an amber dot on any stage holding an open step), the Checklist /
Flow switch and Emergency. Find is hidden until it is built (batch 6, owner). Stage tabs hide in
narrow windows; the menu has them there.
-->
<script lang="ts">
  import type { ListView, Rail } from "@chocktochock/bindings";
  import { phaseHref } from "../route.svelte";
  import Icon from "./Icon.svelte";

  let {
    aircraft,
    rail,
    stage,
    view,
    menuOpen,
    open = [],
    onMenu,
    onView,
  }: {
    aircraft: string;
    rail: Rail | null;
    /** The stage of the phase on screen, if any. */
    stage: string | null;
    /** The list view, when a phase is on screen. */
    view: ListView | null;
    menuOpen: boolean;
    /** Phases holding an open step (skipped or missed), for the amber dots. */
    open?: string[];
    onMenu: () => void;
    onView: (view: ListView) => void;
  } = $props();

  /** A stage opens on its first phase that still has steps open. */
  const target = (phases: Rail["stages"][number]["phases"]) => (phases.find((p) => !p.complete) ?? phases[0])?.id;
</script>

<header class="top">
  <button class="menu" type="button" aria-label="Menu" aria-expanded={menuOpen} aria-controls="drawer" onclick={onMenu}>
    <Icon name="menu" />
  </button>
  <div class="brand">Chock to Chock<small>{aircraft}</small></div>
  {#if rail}
    <nav class="stages" aria-label="Stages">
      {#each rail.stages as s (s.title)}
        {@const id = target(s.phases)}
        <!-- The dot marks other stages holding an open step; the current stage's own show on its
             phase tabs (Checklist board). -->
        {@const openHere = s.title === stage ? 0 : s.phases.filter((p) => open.includes(p.id)).length}
        {#if id}
          <a href={phaseHref(id)} aria-current={s.title === stage ? "page" : undefined}
            >{s.title}{#if openHere}<span class="dot" aria-label="{openHere} open step{openHere === 1 ? '' : 's'}"></span>{/if}</a
          >
        {/if}
      {/each}
    </nav>
  {/if}
  <div class="end">
    {#if view}
      <div class="seg" role="group" aria-label="View">
        <button type="button" aria-pressed={view === "checklist"} onclick={() => onView("checklist")}>Checklist</button>
        <button type="button" aria-pressed={view === "flow"} onclick={() => onView("flow")}>Flow</button>
      </div>
    {/if}
    <a class="emg" href="#/reference">Emergency</a>
  </div>
</header>

<style>
  .top {
    position: sticky;
    top: 0;
    z-index: 20;
    display: flex;
    align-items: center;
    gap: var(--space-4);
    height: var(--size-header);
    padding: 0 var(--space-4);
    border-bottom: var(--border-thin) solid var(--color-line);
    background: var(--color-bg);
  }
  .menu {
    display: grid;
    place-items: center;
    width: var(--size-touch-min);
    height: var(--size-touch-min);
    color: var(--color-text-secondary);
  }
  .brand {
    display: flex;
    flex-direction: column;
    font-size: var(--font-size-body);
    font-weight: var(--weight-semibold);
    line-height: var(--leading-tight);
    white-space: nowrap;
  }
  .brand small {
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-regular);
  }
  .stages {
    display: flex;
    align-items: stretch;
    height: var(--size-header);
  }
  .stages a {
    display: flex;
    align-items: center;
    gap: calc(var(--space-1) + var(--border-thick) / 2);
    padding: 0 var(--space-3);
    color: var(--color-text-muted);
    font-size: var(--font-size-body);
    text-decoration: none;
  }
  .stages a:hover {
    color: var(--color-text);
  }
  .stages a[aria-current="page"] {
    color: var(--color-text-strong);
    font-weight: var(--weight-semibold);
    box-shadow: inset 0 calc(-1 * (var(--border-thick) + var(--border-thin))) 0 var(--color-accent);
  }
  .dot {
    width: var(--size-dot);
    height: var(--size-dot);
    border-radius: 50%;
    background: var(--color-open-dot);
  }
  .end {
    display: flex;
    align-items: center;
    gap: var(--space-4);
    margin-left: auto;
  }
  /* The segmented control (rules.md components). */
  .seg {
    display: flex;
    padding: var(--border-thick);
    border: var(--border-thin) solid var(--color-segment-border);
    border-radius: var(--radius-segment);
    background: var(--color-segment-track);
  }
  .seg button {
    height: var(--size-touch-min);
    padding: 0 var(--space-4);
    border: 0;
    border-radius: var(--radius-control);
    background: transparent;
    color: var(--color-text-muted);
    font: inherit;
    font-size: var(--font-size-secondary);
  }
  .seg button[aria-pressed="true"] {
    background: var(--color-segment-on);
    outline: var(--size-focus-ring) solid var(--color-accent);
    outline-offset: calc(-1 * var(--size-focus-ring));
    box-shadow: var(--color-segment-shadow);
    color: var(--color-text-strong);
    font-weight: var(--weight-semibold);
  }
  .emg {
    display: inline-flex;
    align-items: center;
    height: var(--size-touch-min);
    padding: 0 var(--space-4);
    border: var(--size-focus-ring) solid var(--color-danger);
    border-radius: var(--radius-control);
    background: var(--color-danger-bg);
    color: var(--color-danger-text);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
    text-decoration: none;
    white-space: nowrap;
  }
  @media (max-width: 899px) {
    .stages {
      display: none;
    }
    .brand {
      margin-right: auto;
    }
  }
  @media (max-width: 599px) {
    .brand {
      display: none;
    }
  }
</style>
