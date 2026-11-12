<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The app frame, laid out per class (docs/design/system.md): the top bar and the phase rail, then
the screen and, when there is one, the side pane. The pane sits beside the screen from tablet
landscape up and widens with the class; on phones and tablets in portrait its content opens as a
sheet instead. On large screens the content keeps a maximum width and centers.
-->
<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    top,
    rail,
    pane,
    children,
  }: {
    top: Snippet;
    rail?: Snippet | undefined;
    /** The side pane, shown beside the screen when there is room. */
    pane?: Snippet | undefined;
    children: Snippet;
  } = $props();
</script>

<div class="shell">
  <header class="top">{@render top()}</header>
  {#if rail}<nav class="rail" aria-label="Phases">{@render rail()}</nav>{/if}
  <div class="body" class:with-pane={!!pane}>
    <main>{@render children()}</main>
    {#if pane}<aside class="pane" aria-label="About the step">{@render pane()}</aside>{/if}
  </div>
</div>

<style>
  .shell {
    min-height: 100dvh;
    background: var(--bg);
  }
  .top,
  .rail {
    position: sticky;
    z-index: 20;
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: 0 max(var(--gutter), env(safe-area-inset-right)) 0 max(var(--gutter), env(safe-area-inset-left));
    border-bottom: var(--border-thin) solid var(--line);
    background: var(--surface);
  }
  .top {
    top: 0;
    min-height: var(--top-h);
    padding-top: env(safe-area-inset-top);
  }
  .rail {
    top: var(--top-h);
    min-height: var(--rail-h);
    overflow-x: auto;
    scrollbar-width: none;
  }
  .body {
    display: grid;
    grid-template-columns: minmax(0, var(--main-max));
    justify-content: center;
    gap: var(--space-8);
    padding: var(--space-6) max(var(--gutter), env(safe-area-inset-right)) var(--space-16) max(var(--gutter), env(safe-area-inset-left));
  }
  main {
    min-width: 0;
  }
  .pane {
    display: none;
  }
  @media (min-width: 900px) {
    .body.with-pane {
      grid-template-columns: minmax(0, var(--main-max)) var(--pane-w);
    }
    .pane {
      position: sticky;
      top: calc(var(--top-h) + var(--rail-h) + var(--space-6));
      display: block;
      max-height: calc(100dvh - var(--top-h) - var(--rail-h) - var(--space-12));
      overflow: auto;
      align-self: start;
    }
  }
</style>
