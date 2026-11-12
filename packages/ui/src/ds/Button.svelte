<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The one button (docs/design/system.md). A button, or a link styled as one when it has an href.
Variants: primary (the one main action on a screen), secondary (the default), quiet (low
emphasis, in rows and footers), danger (destroys something; asks first elsewhere), emergency (the
Emergency button only), link (text only). Every variant is at least the touch height; compact is for fine pointers inside dense rows.
-->
<script lang="ts">
  import type { Snippet } from "svelte";
  import Icon, { type IconName } from "../lib/Icon.svelte";

  let {
    variant = "secondary",
    compact = false,
    href,
    icon,
    iconEnd,
    disabled = false,
    pressed,
    label,
    onclick,
    children,
  }: {
    variant?: "primary" | "secondary" | "quiet" | "danger" | "emergency" | "link" | undefined;
    compact?: boolean | undefined;
    href?: string | undefined;
    icon?: IconName | undefined;
    iconEnd?: IconName | undefined;
    disabled?: boolean | undefined;
    /** A toggle button's state. */
    pressed?: boolean | undefined;
    /** The accessible name, when the visible text is not enough. */
    label?: string | undefined;
    onclick?: ((e: MouseEvent) => void) | undefined;
    children?: Snippet | undefined;
  } = $props();
</script>

{#if href && !disabled}
  <a class="b {variant}" class:compact {href} aria-label={label} {onclick}>
    {#if icon}<Icon name={icon} />{/if}{@render children?.()}{#if iconEnd}<Icon name={iconEnd} />{/if}
  </a>
{:else}
  <button class="b {variant}" class:compact type="button" {disabled} aria-pressed={pressed} aria-label={label} {onclick}>
    {#if icon}<Icon name={icon} />{/if}{@render children?.()}{#if iconEnd}<Icon name={iconEnd} />{/if}
  </button>
{/if}

<style>
  .b {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-2);
    min-height: var(--control-h);
    padding: 0 var(--space-4);
    border: var(--border-thin) solid var(--line);
    border-radius: var(--radius-md);
    background: var(--surface);
    color: var(--ink);
    font: var(--weight-medium) var(--text-sm) / var(--leading-tight) var(--sans);
    text-decoration: none;
    white-space: nowrap;
    cursor: pointer;
  }
  .b:hover:not(:disabled) {
    border-color: var(--muted);
  }
  .b:disabled {
    opacity: 0.5;
    cursor: default;
  }
  .primary {
    border-color: var(--accent-surface);
    background: var(--accent-surface);
    color: var(--on-accent);
  }
  .primary:hover:not(:disabled) {
    border-color: var(--accent-surface);
    filter: brightness(1.08);
  }
  .quiet {
    border-color: transparent;
    background: transparent;
    color: var(--muted);
  }
  .quiet:hover:not(:disabled) {
    border-color: transparent;
    background: var(--zone);
    color: var(--ink);
  }
  .danger {
    border-color: var(--stop);
    color: var(--stop);
  }
  .danger:hover:not(:disabled) {
    border-color: var(--stop);
    background: var(--stop-soft);
  }
  .emergency {
    border-color: var(--emergency);
    background: var(--emergency);
    color: var(--on-emergency);
    font-weight: var(--weight-bold);
  }
  .emergency:hover:not(:disabled) {
    border-color: var(--emergency);
    filter: brightness(1.1);
  }
  .link {
    padding: 0 var(--space-2);
    border-color: transparent;
    background: transparent;
    color: var(--accent);
  }
  .link:hover:not(:disabled) {
    border-color: transparent;
    text-decoration: underline;
  }
  .b[aria-pressed="true"] {
    border-color: var(--accent);
    background: var(--accent-soft);
  }
  .b :global(svg) {
    flex: none;
    width: var(--icon-sm);
    height: var(--icon-sm);
  }
  @media (pointer: fine) {
    .compact {
      min-height: var(--control-h-compact);
      padding: 0 var(--space-3);
    }
  }
</style>
