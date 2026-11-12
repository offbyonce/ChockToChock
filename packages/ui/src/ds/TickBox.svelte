<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The tick box at the start of a step row, a full touch target. States read by shape as well as
color: empty (to do), outlined in the accent (current), filled with a check (done), dashed with
the skip mark (skipped).
-->
<script lang="ts">
  import Icon from "../lib/Icon.svelte";

  let {
    state,
    label,
    onclick,
  }: {
    state: "to_do" | "current" | "done" | "skipped";
    /** "Confirm DOOR LIGHTS: ALL OUT". */
    label: string;
    onclick?: (() => void) | undefined;
  } = $props();
</script>

<button class="tb {state}" type="button" aria-label={label} aria-pressed={state === "done"} {onclick}>
  <span class="box">
    {#if state === "done"}<Icon name="check" />{:else if state === "skipped"}<Icon name="skip" />{/if}
  </span>
</button>

<style>
  .tb {
    display: grid;
    place-items: center;
    width: var(--control-h);
    height: var(--control-h);
  }
  .box {
    display: grid;
    place-items: center;
    width: var(--icon);
    height: var(--icon);
    border: var(--border-thick) solid var(--muted);
    border-radius: var(--radius-sm);
  }
  .current .box {
    border-color: var(--accent);
  }
  .done .box {
    border-color: var(--go);
    background: var(--go);
    color: var(--on-accent);
  }
  .skipped .box {
    border-style: dashed;
    border-color: var(--warn);
    color: var(--warn);
  }
  .box :global(svg) {
    width: var(--icon-sm);
    height: var(--icon-sm);
  }
</style>
