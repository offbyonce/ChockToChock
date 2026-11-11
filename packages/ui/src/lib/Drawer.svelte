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
    version,
    onClose,
  }: {
    open: boolean;
    rail: Rail;
    stage: string | null;
    total: Progress;
    detail: Detail;
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
  <p class="grp">Reference</p>
  <a class="ref" href="#/reference" onclick={onClose}>Emergency</a>
  <div class="foot">
    <a class="ref" href="#/settings" onclick={onClose}>Settings <span>Full · {detail === "expanded" ? "Expanded" : "Quick"}</span></a>
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
    width: 18rem;
    padding: 0.75rem 0.75rem 1rem;
    overflow-y: auto;
    border-right: 1px solid var(--line);
    background: var(--surface);
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
    background: rgb(0 0 0 / 45%);
  }
  .continue {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    padding: 0.75rem;
    border-radius: 0.75rem;
    background: var(--bg);
    text-decoration: none;
  }
  .continue small {
    color: var(--muted);
    font-size: 0.75rem;
  }
  .continue b {
    font-size: 1.0625rem;
    font-weight: 600;
  }
  .bar {
    display: block;
    height: 0.375rem;
    margin: 0.25rem 0;
    overflow: hidden;
    border-radius: 0.5rem;
    background: var(--zone);
  }
  .bar i {
    display: block;
    height: 100%;
    background: var(--go);
  }
  .stages {
    display: flex;
    flex-wrap: wrap;
    gap: 0.25rem;
    margin: 0.75rem 0;
  }
  .stages a {
    display: flex;
    align-items: center;
    min-height: 2.75rem;
    padding: 0 0.625rem;
    border-radius: 0.5rem;
    background: var(--bg);
    color: var(--muted);
    font-size: 0.875rem;
    font-weight: 500;
    text-decoration: none;
  }
  .stages a[aria-current="true"] {
    background: var(--accent-soft);
    color: var(--accent);
  }
  .grp {
    margin: 0;
    padding: 0.875rem 0.625rem 0.375rem;
    color: var(--muted);
    font-size: 0.7188rem;
    font-weight: 600;
    letter-spacing: 0.05em;
    text-transform: uppercase;
  }
  .ref {
    display: flex;
    align-items: center;
    justify-content: space-between;
    min-height: 2.75rem;
    padding: 0 0.625rem;
    border-radius: 0.5rem;
    font-weight: 500;
    text-decoration: none;
  }
  .ref:hover {
    background: var(--bg);
  }
  .ref span {
    color: var(--muted);
    font-size: 0.75rem;
    font-weight: 400;
  }
  .foot {
    margin-top: auto;
    padding-top: 0.75rem;
    border-top: 1px solid var(--line);
  }
  .ver {
    margin: 0.5rem 0.625rem 0;
    color: var(--muted);
    font-size: 0.75rem;
  }
  @media (prefers-reduced-motion: reduce) {
    nav {
      transition: none;
    }
  }
</style>
