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
    gap: 0.875rem;
    height: var(--top-h);
    padding: 0 1rem;
    border-bottom: 1px solid var(--line);
    background: var(--surface);
  }
  .menu {
    display: grid;
    place-items: center;
    width: 2.75rem;
    height: 2.75rem;
    margin-left: -0.375rem;
    border-radius: 0.5rem;
  }
  .brand {
    font-weight: 600;
    letter-spacing: -0.01em;
    line-height: 1.2;
    white-space: nowrap;
  }
  .brand small {
    display: block;
    color: var(--muted);
    font-size: 0.6875rem;
    font-weight: 400;
  }
  .stages {
    display: flex;
    gap: 0.2rem;
    margin-left: 0.5rem;
  }
  .stages a {
    position: relative;
    display: flex;
    align-items: center;
    height: 2.5rem;
    padding: 0 0.8rem;
    border-radius: 0.6rem;
    color: var(--muted);
    font-weight: 500;
    text-decoration: none;
  }
  .stages a:hover {
    color: var(--ink);
  }
  .stages a[aria-current="true"] {
    background: var(--accent-soft);
    color: var(--accent);
  }
  .end {
    display: flex;
    align-items: center;
    gap: 0.625rem;
    margin-left: auto;
  }
  .tool {
    position: relative;
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.3125rem 0.625rem;
    border: 1px solid var(--line);
    border-radius: 0.5rem;
    font-size: 0.8125rem;
    font-weight: 500;
    white-space: nowrap;
  }
  /* 3.0's sizes on screen, with the touch area grown to 44px (step 7). */
  .tool::after,
  .stages a::after,
  .emg::after {
    position: absolute;
    inset: 50% 0 auto;
    height: 2.75rem;
    content: "";
    transform: translateY(-50%);
  }
  .tool:hover:not(:disabled) {
    border-color: var(--muted);
  }
  .find:disabled {
    color: var(--muted);
  }
  kbd {
    color: var(--muted);
    font: 0.6875rem var(--mono);
  }
  .emg {
    position: relative;
    display: inline-flex;
    align-items: center;
    height: 2.5rem;
    padding: 0 1rem;
    border-radius: 0.6rem;
    background: var(--emergency);
    color: var(--on-emergency);
    font-weight: 600;
    text-decoration: none;
    white-space: nowrap;
  }
  @media (max-width: 1100px) {
    kbd {
      display: none;
    }
  }
  @media (max-width: 900px) {
    .stages {
      display: none;
    }
    .brand {
      margin-right: auto;
    }
  }
  @media (max-width: 560px) {
    .brand {
      display: none;
    }
  }
</style>
