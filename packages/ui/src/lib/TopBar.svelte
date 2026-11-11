<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The bar on every screen: the phases menu (narrow screens), the flight strip, search, settings,
and the Emergency button for the reference lists.
-->
<script lang="ts">
  import Icon from "./Icon.svelte";

  let {
    version,
    menuOpen,
    onMenu,
  }: { version: string | null; menuOpen: boolean; onMenu: () => void } = $props();
</script>

<header class="bar">
  <button
    class="icon-button menu"
    type="button"
    aria-expanded={menuOpen}
    aria-controls="phase-rail"
    onclick={onMenu}
  >
    <Icon name={menuOpen ? "x" : "menu"} />
    <span class="sr-only">{menuOpen ? "Close phases" : "Phases"}</span>
  </button>
  <a class="brand" href="#/">
    Chock to Chock
    {#if version}<span class="version">v{version}</span>{/if}
  </a>
  <div class="strip" title="Flight setup and SimBrief arrive in step 7, batch 2">
    <Icon name="plane" />
    <span>No flight plan loaded</span>
  </div>
  <div class="actions">
    <button class="icon-button" type="button" disabled title="Search arrives in step 7, batch 6">
      <Icon name="search" />
      <span class="sr-only">Search</span>
    </button>
    <a class="icon-button" href="#/settings">
      <Icon name="settings" />
      <span class="sr-only">Settings</span>
    </a>
    <a class="emergency" href="#/reference">
      <Icon name="alert-triangle" />
      <span>Emergency</span>
    </a>
  </div>
</header>

<style>
  .bar {
    position: sticky;
    z-index: 3;
    top: 0;
    display: flex;
    align-items: center;
    gap: 0.75rem;
    min-height: 64px;
    padding: 0 1rem;
    border-bottom: 1px solid var(--border);
    background: var(--panel);
  }
  .brand {
    display: flex;
    align-items: baseline;
    gap: 0.5rem;
    color: var(--text);
    font-size: 1.125rem;
    font-weight: 700;
    text-decoration: none;
    white-space: nowrap;
  }
  .version {
    color: var(--muted);
    font-size: 0.8125rem;
    font-weight: 400;
  }
  .strip {
    display: flex;
    flex: 1;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
    min-height: 48px;
    padding: 0 0.875rem;
    border: 1px solid var(--border);
    border-radius: 0.5rem;
    color: var(--muted);
  }
  .strip span {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
  .icon-button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 48px;
    height: 48px;
    border: 0;
    border-radius: 0.5rem;
    background: transparent;
    color: var(--text);
    cursor: pointer;
  }
  .icon-button:hover:not(:disabled) {
    background: var(--raised);
  }
  .icon-button:disabled {
    color: var(--muted);
    cursor: default;
  }
  .emergency {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    min-height: 48px;
    padding: 0 1rem;
    border-radius: 0.5rem;
    background: var(--danger);
    color: var(--on-danger);
    font-weight: 700;
    text-decoration: none;
  }
  @media (min-width: 1024px) {
    .menu {
      display: none;
    }
  }
  @media (max-width: 719px) {
    .strip span,
    .emergency span {
      display: none;
    }
  }
</style>
