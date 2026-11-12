<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The one overlay: on a phone a bottom sheet over a scrim, on wider screens a pop-out under its
anchor (or a dialog when it has none). It takes the safe areas into account, closes on Escape
and on the scrim, and keeps its own scroll.
-->
<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    open,
    title,
    onClose,
    anchored = false,
    children,
    footer,
  }: {
    open: boolean;
    title: string;
    onClose: () => void;
    /** On wider screens, sit under the element it opened from instead of in the middle. */
    anchored?: boolean;
    children: Snippet;
    footer?: Snippet;
  } = $props();
</script>

<svelte:window onkeydown={(e) => open && e.key === "Escape" && onClose()} />

{#if open}
  <div class="scrim" role="presentation" onclick={onClose}></div>
  <div class="sheet" class:anchored role="dialog" aria-label={title}>
    <header>
      <h2>{title}</h2>
      <button class="close" type="button" onclick={onClose}>Close</button>
    </header>
    <div class="content">{@render children()}</div>
    {#if footer}<footer>{@render footer()}</footer>{/if}
  </div>
{/if}

<style>
  .scrim {
    position: fixed;
    inset: 0;
    z-index: 40;
    background: var(--scrim);
  }
  .sheet {
    position: fixed;
    right: 0;
    bottom: 0;
    left: 0;
    z-index: 41;
    display: flex;
    flex-direction: column;
    max-height: 85dvh;
    border-top: var(--border-thin) solid var(--line);
    border-radius: var(--radius-lg) var(--radius-lg) 0 0;
    background: var(--surface);
    box-shadow: var(--shadow-overlay);
    padding-bottom: env(safe-area-inset-bottom);
  }
  header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-4);
    min-height: var(--space-12);
    padding: 0 var(--space-4);
    border-bottom: var(--border-thin) solid var(--line);
  }
  h2 {
    margin: 0;
    font: var(--weight-bold) var(--text-md) / var(--leading-tight) var(--sans);
  }
  .close {
    min-height: var(--control-h);
    padding: 0 var(--space-2);
    color: var(--muted);
    font-size: var(--text-sm);
  }
  .content {
    overflow: auto;
    padding: var(--space-4);
  }
  footer {
    display: flex;
    flex-wrap: wrap;
    justify-content: flex-end;
    gap: var(--space-3);
    padding: var(--space-3) var(--space-4);
    border-top: var(--border-thin) solid var(--line);
  }
  @media (min-width: 600px) {
    .scrim {
      background: transparent;
    }
    .sheet {
      top: 50%;
      right: auto;
      bottom: auto;
      left: 50%;
      width: min(var(--dialog-w), calc(100vw - 2 * var(--gutter)));
      max-height: min(85dvh, var(--dialog-max-h));
      border: var(--border-thin) solid var(--line);
      border-radius: var(--radius-lg);
      transform: translate(-50%, -50%);
    }
    .sheet.anchored {
      position: absolute;
      top: calc(100% + var(--space-2));
      left: 0;
      transform: none;
    }
  }
</style>
