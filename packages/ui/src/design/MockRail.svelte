<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Mockup: the phase rail on the system. One chip per phase in the stage, a dot for its state, the
current phase filled; the detail level and progress on the right from tablet up.
-->
<script lang="ts">
  import Chip from "../ds/Chip.svelte";
  import { PHASES } from "./data";
</script>

<ol class="chips">
  {#each PHASES as p (p.title)}
    <li>
      <a href="#{p.title}" class={p.state} aria-current={p.state === "current" ? "page" : undefined}>
        <span class="dot" aria-hidden="true"></span>{p.title}
      </a>
    </li>
  {/each}
</ol>
<span class="meta">
  <Chip>Full · Detailed</Chip>
  <span class="count">12 / 356</span>
</span>

<style>
  .chips {
    display: flex;
    gap: var(--space-1);
    margin: 0;
    padding: 0;
    list-style: none;
  }
  a {
    display: inline-flex;
    align-items: center;
    gap: var(--space-2);
    min-height: var(--control-h-compact);
    padding: 0 var(--space-3);
    border-radius: var(--radius-full);
    color: var(--muted);
    font-size: var(--text-sm);
    font-weight: var(--weight-medium);
    text-decoration: none;
    white-space: nowrap;
  }
  .dot {
    width: var(--space-2);
    height: var(--space-2);
    border: var(--border-thin) solid var(--muted);
    border-radius: var(--radius-full);
  }
  .done .dot {
    border-color: var(--go);
    background: var(--go);
  }
  a[aria-current="page"] {
    background: var(--accent-soft);
    color: var(--accent);
  }
  a[aria-current="page"] .dot {
    border-color: var(--accent);
    background: var(--accent);
  }
  .meta {
    display: none;
    flex: none;
    align-items: center;
    gap: var(--space-3);
    margin-left: auto;
  }
  .count {
    color: var(--muted);
    font: var(--weight-regular) var(--text-xs) var(--mono);
  }
  @media (min-width: 900px) {
    .meta {
      display: flex;
    }
  }
</style>
