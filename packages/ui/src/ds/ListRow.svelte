<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The one list row: a leading control (a tick box), the main button with a number, title, chips,
a meta line and a value, an optional trailing control (a chevron), and an expanded body below.
Marked rows carry the bar (the step the pane shows); selected rows a tint. Rows in a card run
edge to edge and share one rule between them.
-->
<script lang="ts">
  import type { Snippet } from "svelte";

  let {
    number,
    title,
    value,
    valueBelow = false,
    meta,
    marked = false,
    selected = false,
    muted = false,
    large = false,
    leading,
    chips,
    trailing,
    body,
    onSelect,
  }: {
    number?: number | string | undefined;
    title: string;
    /** The setting, in the monospaced face, on the right (or below when long). */
    value?: string | undefined;
    valueBelow?: boolean | undefined;
    meta?: string | undefined;
    marked?: boolean | undefined;
    selected?: boolean | undefined;
    /** A done step: the words step back. */
    muted?: boolean | undefined;
    /** Non-normal and reference lists: one step larger type. */
    large?: boolean | undefined;
    leading?: Snippet | undefined;
    chips?: Snippet | undefined;
    trailing?: Snippet | undefined;
    body?: Snippet | undefined;
    onSelect?: (() => void) | undefined;
  } = $props();
</script>

<li class="lr" class:marked class:selected class:muted class:large>
  {#if leading}<div class="lead">{@render leading()}</div>{/if}
  <div class="main">
    <button class="hit" type="button" onclick={onSelect} aria-current={selected ? "true" : undefined}>
      <span class="title">
        {#if number != null}<small class="n">{number}</small>{/if}<span class="t">{title}</span>
        {#if chips}<span class="chips">{@render chips()}</span>{/if}
      </span>
      {#if value}<span class="v" class:below={valueBelow}>{value}</span>{/if}
    </button>
    {#if meta}<p class="meta">{meta}</p>{/if}
    {#if body}<div class="body">{@render body()}</div>{/if}
  </div>
  {#if trailing}<div class="trail">{@render trailing()}</div>{/if}
</li>

<style>
  .lr {
    display: grid;
    grid-template-columns: var(--space-12) minmax(0, 1fr) auto;
    align-items: start;
    border-top: var(--border-thin) solid var(--line);
    list-style: none;
  }
  .lr:first-child {
    border-top: 0;
  }
  .marked {
    box-shadow: inset var(--border-thick) 0 0 var(--accent);
  }
  .selected {
    background: var(--accent-soft);
  }
  .lead {
    display: grid;
    justify-items: center;
    padding-top: var(--space-2);
  }
  .main {
    min-width: 0;
    padding: var(--space-3) 0;
  }
  .lr:not(:has(.lead)) .main {
    grid-column: 1 / 3;
    padding-left: var(--space-4);
  }
  .hit {
    display: flex;
    flex-wrap: wrap;
    align-items: baseline;
    justify-content: space-between;
    gap: var(--space-1) var(--space-4);
    width: 100%;
    min-height: var(--space-6);
    text-align: left;
  }
  .title {
    min-width: 0;
    font: var(--weight-medium) var(--text-md) / var(--leading-snug) var(--sans);
  }
  .n {
    margin-right: var(--space-2);
    color: var(--muted);
    font: var(--weight-regular) var(--text-xs) var(--mono);
  }
  .chips {
    display: inline-flex;
    flex-wrap: wrap;
    gap: var(--space-1);
    margin-left: var(--space-2);
  }
  .v {
    min-width: 0;
    max-width: 60%;
    margin-left: auto;
    font: var(--weight-regular) var(--text-sm) / var(--leading-snug) var(--mono);
    text-align: right;
  }
  .v.below {
    flex-basis: 100%;
    max-width: none;
    margin-left: 0;
    text-align: left;
  }
  .meta {
    margin: var(--space-1) 0 0;
    color: var(--muted);
    font: var(--weight-regular) var(--text-sm) / var(--leading-snug) var(--sans);
  }
  .body {
    margin-top: var(--space-3);
  }
  .trail {
    display: grid;
    place-items: center;
    min-width: var(--control-h);
    min-height: var(--control-h);
    padding-top: var(--space-1);
  }
  .muted .t,
  .muted .v {
    color: var(--muted);
  }
  .large .title {
    font-size: var(--text-lg);
  }
  .large .v {
    font-size: var(--text-md);
  }
  @media (max-width: 599px) {
    .v {
      flex-basis: 100%;
      max-width: none;
      margin-left: 0;
      text-align: left;
    }
  }
</style>
