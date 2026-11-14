<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The menu, as in 3.0: where to continue, the stages, the reference screens and settings. Later
batches add flight setup, the flight log, practice and the rest here.
-->
<script lang="ts">
  import type { Detail, Progress, Rail } from "@chocktochock/bindings";
  import { phaseHref } from "../route.svelte";

  let {
    open,
    rail,
    stage,
    total,
    detail,
    essentials,
    openCount,
    version,
    onClose,
  }: {
    open: boolean;
    rail: Rail;
    stage: string | null;
    total: Progress;
    detail: Detail;
    essentials: boolean;
    /** How many steps are skipped and still open. */
    openCount: number;
    version: string | null;
    onClose: () => void;
  } = $props();

  let resume = $derived(rail.stages.flatMap((s) => s.phases).find((p) => !p.complete) ?? null);
  const target = (phases: Rail["stages"][number]["phases"]) => (phases.find((p) => !p.complete) ?? phases[0])?.id;
</script>

<svelte:window
  onkeydown={(e) => {
    if (open && e.key === "Escape") onClose();
  }}
/>

{#if open}
  <button class="scrim" type="button" aria-label="Close menu" onclick={onClose}></button>
{/if}
<nav id="drawer" class:open aria-label="Menu" inert={!open}>
  {#if resume}
    <a class="continue" href={phaseHref(resume.id)} onclick={onClose}>
      <small>Continue</small>
      <b>{resume.title}</b>
      <span class="bar" aria-hidden="true"><i style:width="{total.total ? (100 * total.done) / total.total : 0}%"></i></span>
      <small>{total.done} of {total.total} steps</small>
    </a>
  {/if}
  <div class="stages">
    {#each rail.stages as s (s.title)}
      {@const id = target(s.phases)}
      {#if id}
        <a href={phaseHref(id)} aria-current={s.title === stage ? "true" : undefined} onclick={onClose}>{s.title}</a>
      {/if}
    {/each}
  </div>
  <p class="grp">Flight</p>
  <a class="ref" href="#/flight/overview" onclick={onClose}>Flight</a>
  <a class="ref" href="#/open" onclick={onClose}>Open steps <span>{openCount}</span></a>
  <p class="grp">Reference</p>
  <a class="ref" href="#/reference" onclick={onClose}>Emergency</a>
  <div class="foot">
    <a class="ref" href="#/settings" onclick={onClose}>Settings <span>{essentials ? "Essentials" : "Full"} · {detail === "expanded" ? "Detailed" : "Simple"}</span></a>
    {#if version}<p class="ver">Version {version}</p>{/if}
  </div>
</nav>

<style>
  nav {
    position: fixed;
    top: 0;
    bottom: 0;
    left: 0;
    z-index: 50;
    display: flex;
    flex-direction: column;
    width: var(--size-drawer);
    padding: var(--space-3) var(--space-3) var(--space-4);
    overflow-y: auto;
    border-right: var(--border-thin) solid var(--color-border);
    background: var(--color-surface);
    transform: translateX(-102%);
    transition: transform 0.18s;
  }
  nav.open {
    transform: none;
  }
  .scrim {
    position: fixed;
    inset: 0;
    z-index: 45;
    background: var(--color-scrim);
  }
  .continue {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    padding: var(--space-3);
    border-radius: var(--radius-panel);
    background: var(--color-bg);
    text-decoration: none;
  }
  .continue small {
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  .continue b {
    font-size: var(--font-size-body);
    font-weight: var(--weight-semibold);
  }
  .bar {
    display: block;
    height: var(--size-progress-bar);
    margin: var(--space-1) 0;
    overflow: hidden;
    border-radius: var(--radius-control);
    background: var(--color-surface-2);
  }
  .bar i {
    display: block;
    height: 100%;
    background: var(--color-done);
  }
  .stages {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-1);
    margin: var(--space-3) 0;
  }
  .stages a {
    display: flex;
    align-items: center;
    min-height:  var(--size-touch-min);
    padding: 0 var(--space-3);
    border-radius: var(--radius-control);
    background: var(--color-bg);
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-medium);
    text-decoration: none;
  }
  .stages a[aria-current="true"] {
    background: var(--color-accent-soft);
    color: var(--color-accent);
  }
  .grp {
    margin: 0;
    padding: var(--space-4) var(--space-3) var(--space-2);
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-semibold);
    text-transform: uppercase;
  }
  .ref {
    display: flex;
    align-items: center;
    justify-content: space-between;
    min-height:  var(--size-touch-min);
    padding: 0 var(--space-3);
    border-radius: var(--radius-control);
    font-weight: var(--weight-medium);
    text-decoration: none;
  }
  .ref:hover {
    background: var(--color-bg);
  }
  .ref span {
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-regular);
  }
  .foot {
    margin-top: auto;
    padding-top: var(--space-3);
    border-top: var(--border-thin) solid var(--color-border);
  }
  .ver {
    margin: var(--space-2) var(--space-3) 0;
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  @media (prefers-reduced-motion: reduce) {
    nav {
      transition: none;
    }
  }
</style>
