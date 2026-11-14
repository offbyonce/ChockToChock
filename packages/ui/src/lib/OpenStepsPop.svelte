<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

"2 open steps" and its pop-over (OpenSteps board): every open step from every stage, grouped by
phase and list, each with its setting and Go to step, and the footer saying skipped steps stay
open. Pop-over tokens; it closes on its X, Esc or a press outside.
-->
<script lang="ts">
  import type { OpenStep } from "@chocktochock/bindings";
  import type { PackIndex } from "../pack-index";
  import Icon from "./Icon.svelte";
  import { stepTitle } from "./sentence";

  let { index, open, onGo }: { index: PackIndex; open: OpenStep[]; onGo: (step: string) => void } = $props();

  let shown = $state(false);
  let box = $state<HTMLElement | null>(null);
  let groups = $derived.by(() => {
    const out: { title: string; steps: OpenStep[] }[] = [];
    for (const o of open) {
      const title = `${o.phaseTitle} · ${o.listTitle}`;
      const g = out.find((x) => x.title === title) ?? (out.push({ title, steps: [] }), out[out.length - 1]);
      g?.steps.push(o);
    }
    return out;
  });
  const label = (n: number) => `${n} open step${n === 1 ? "" : "s"}`;
  function outside(e: PointerEvent): void {
    if (shown && box && !box.contains(e.target as Node)) shown = false;
  }
  function key(e: KeyboardEvent): void {
    if (shown && e.key === "Escape") shown = false;
  }
</script>

<svelte:document onpointerdown={outside} onkeydown={key} />

{#if open.length}
  <div class="wrap" bind:this={box}>
    <button class="opener" type="button" aria-haspopup="dialog" aria-expanded={shown} onclick={() => (shown = !shown)}>
      {label(open.length)}<Icon name="chevron-down" />
    </button>
    {#if shown}
      <div class="pop" role="dialog" aria-labelledby="open-title">
        <span class="arrow" aria-hidden="true"></span>
        <div class="head">
          <h2 id="open-title">{label(open.length)}</h2>
          <button class="close" type="button" aria-label="Close" onclick={() => (shown = false)}><Icon name="x" /></button>
        </div>
        {#each groups as g (g.title)}
          <p class="group">{g.title}</p>
          {#each g.steps as o (o.step)}
            {@const step = index.step(o.step)}
            {#if step}
              <div class="row">
                <span class="status" class:skipped={o.skipped} aria-hidden="true"><i></i></span>
                <span class="txt">
                  <span class="nm">{stepTitle(index, step)}</span>
                  <span class="val">{step.expect}</span>
                </span>
                <button
                  class="go"
                  type="button"
                  onclick={() => {
                    shown = false;
                    onGo(o.step);
                  }}>Go to step</button
                >
              </div>
            {/if}
          {/each}
        {/each}
        <p class="foot">Skipped steps stay open until you do them. The Flight log lists any still open at the end.</p>
      </div>
    {/if}
  </div>
{/if}

<style>
  .wrap {
    position: relative;
  }
  .opener {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    height: var(--size-touch-min);
    padding: 0 calc(var(--space-3) + var(--border-thick));
    border: var(--border-thin) solid var(--color-open-border);
    border-radius: var(--radius-control);
    background: var(--color-open-bg);
    color: var(--color-open-text);
    font: inherit;
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
  }
  .opener :global(svg) {
    width: var(--size-icon-small);
    height: var(--size-icon-small);
  }
  .pop {
    position: absolute;
    top: calc(100% + var(--space-2));
    right: 0;
    z-index: 12;
    box-sizing: border-box;
    width: calc(var(--size-drawer) + 5 * var(--space-6));
    padding: var(--space-2) var(--space-2) var(--space-3);
    border: var(--border-thin) solid var(--color-popover-border);
    border-radius: var(--radius-popover);
    background: var(--color-popover-surface);
    box-shadow: var(--color-overlay-shadow);
  }
  .arrow {
    position: absolute;
    top: calc(-1 * var(--space-2) + var(--border-thin));
    right: calc(var(--space-10) + var(--space-6));
    width: var(--space-3);
    height: var(--space-3);
    border-top: var(--border-thin) solid var(--color-popover-border);
    border-left: var(--border-thin) solid var(--color-popover-border);
    background: var(--color-popover-surface);
    transform: rotate(45deg);
  }
  .head {
    display: flex;
    align-items: center;
    padding: 0 0 0 var(--space-3);
  }
  h2 {
    flex: 1;
    margin: 0;
    font-size: var(--font-size-body);
    font-weight: var(--weight-semibold);
  }
  .close {
    display: grid;
    place-items: center;
    width: var(--size-touch-min);
    height: var(--size-touch-min);
    border: 0;
    background: none;
    color: var(--color-text-secondary);
  }
  .group {
    margin: var(--space-2) var(--space-3) var(--space-1);
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-semibold);
  }
  .row {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: var(--space-2) var(--space-3);
    border-radius: var(--radius-control);
    background: var(--color-surface-2);
  }
  .status {
    display: grid;
    place-items: center;
    box-sizing: border-box;
    flex: none;
    width: var(--size-status-icon);
    height: var(--size-status-icon);
    border: var(--border-thick) solid var(--color-todo-ring);
    border-radius: 50%;
  }
  .status.skipped {
    border-style: dashed;
    border-color: var(--color-open-dot);
  }
  .status i {
    width: calc(var(--size-status-icon) / 2);
    height: var(--border-thick);
    background: var(--color-open-dot);
  }
  .txt {
    flex: 1;
    display: flex;
    flex-direction: column;
    min-width: 0;
  }
  .nm {
    font-size: var(--font-size-body);
  }
  .val {
    color: var(--color-text-secondary);
    font-family: var(--font-mono);
    font-size: var(--font-size-caption);
  }
  .go {
    min-height: var(--size-touch-min);
    border: 0;
    background: none;
    color: var(--color-link);
    font: inherit;
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
  }
  .foot {
    margin: var(--space-3) var(--space-3) 0;
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
    line-height: var(--leading-snug);
  }
</style>
