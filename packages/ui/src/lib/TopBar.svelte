<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The top bar, as in 3.0: menu, name, the stage tabs, Find, the Flow and Checklist switch, and the
red Emergency button. Stage tabs hide in narrow windows; the menu has them there.
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
        {#if id}
          <a href={phaseHref(id)} aria-current={s.title === stage ? "true" : undefined}>{s.title}</a>
        {/if}
      {/each}
    </nav>
  {/if}
  <div class="end">
    <button class="tool find" type="button" disabled title="Search arrives in a later batch">
      <span>Find</span> <kbd>Ctrl K</kbd>
    </button>
    {#if view}
      <button class="tool" type="button" onclick={() => onView(view === "flow" ? "checklist" : "flow")}>
        {view === "flow" ? "Checklist view" : "Flow view"}
      </button>
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
    border-bottom: var(--border-thin) solid var(--color-border);
    background: var(--color-surface);
  }
  .menu {
    display: grid;
    place-items: center;
    width:  var(--size-touch-min);
    height:  var(--size-touch-min);
    margin-left: calc(-1 * var(--space-2));
    border-radius: var(--radius-control);
  }
  .brand {
    font-weight: var(--weight-semibold);
    line-height: 1.2;
    white-space: nowrap;
  }
  .brand small {
    display: block;
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-regular);
  }
  .stages {
    display: flex;
    gap: var(--space-1);
    margin-left: var(--space-2);
  }
  .stages a {
    position: relative;
    display: flex;
    align-items: center;
    height: var(--size-touch-min);
    padding: 0 var(--space-3);
    border-radius: var(--radius-control);
    color: var(--color-text-muted);
    font-weight: var(--weight-medium);
    text-decoration: none;
  }
  .stages a:hover {
    color: var(--color-text);
  }
  .stages a[aria-current="true"] {
    background: var(--color-accent-soft);
    color: var(--color-accent);
  }
  .end {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    margin-left: auto;
  }
  .tool {
    position: relative;
    display: inline-flex;
    align-items: center;
    gap: var(--space-1);
    padding: var(--space-1) var(--space-3);
    border: var(--border-thin) solid var(--color-border);
    border-radius: var(--radius-control);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-medium);
    white-space: nowrap;
  }
  /* 3.0's sizes on screen, with the touch area grown to 44px (step 7). */
  .tool::after,
  .stages a::after,
  .emg::after {
    position: absolute;
    inset: 50% 0 auto;
    height:  var(--size-touch-min);
    content: "";
    transform: translateY(-50%);
  }
  .tool:hover:not(:disabled) {
    border-color: var(--color-text-muted);
  }
  .find:disabled {
    color: var(--color-text-muted);
  }
  kbd {
    color: var(--color-text-muted);
    font: var(--weight-regular) var(--font-size-caption) var(--font-mono);
  }
  .emg {
    position: relative;
    display: inline-flex;
    align-items: center;
    height: var(--size-touch-min);
    padding: 0 var(--space-4);
    border: var(--size-focus-ring) solid var(--color-danger);
    border-radius: var(--radius-control);
    background: var(--color-danger-bg);
    color: var(--color-danger-text);
    font-weight: var(--weight-semibold);
    text-decoration: none;
    white-space: nowrap;
  }
  @media (max-width: 1439px) {
    kbd {
      display: none;
    }
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
