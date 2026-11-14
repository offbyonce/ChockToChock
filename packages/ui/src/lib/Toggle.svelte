<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The boards' toggle switch (Views.dc.html Settings, Start.dc.html): On or Off beside a track and
knob, a 44 px target. Off, the white knob takes a muted ring so it reads at 3:1 on the light track.
-->
<script lang="ts">
  let {
    on,
    labelledby,
    label,
    onToggle,
  }: {
    on: boolean;
    labelledby?: string;
    label?: string;
    onToggle: () => void;
  } = $props();
</script>

<button class="switch" type="button" role="switch" aria-checked={on} aria-labelledby={labelledby} aria-label={label} onclick={onToggle}>
  {on ? "On" : "Off"}<span class="track"><span class="knob"></span></span>
</button>

<style>
  .switch {
    display: flex;
    flex: none;
    align-items: center;
    gap: var(--space-3);
    min-height: var(--size-touch-min);
    padding: 0 var(--border-thick);
    border: 0;
    background: transparent;
    color: var(--color-text-strong);
    font: inherit;
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
  }
  .switch[aria-checked="false"] {
    color: var(--color-text-secondary);
  }
  .track {
    position: relative;
    width: var(--size-toggle-width);
    height: var(--size-toggle-height);
    border-radius: var(--radius-pill);
    background: var(--color-toggle-off);
  }
  .switch[aria-checked="true"] .track {
    background: var(--color-accent);
  }
  .knob {
    position: absolute;
    top: calc((var(--size-toggle-height) - var(--size-toggle-knob)) / 2);
    left: calc((var(--size-toggle-height) - var(--size-toggle-knob)) / 2);
    width: var(--size-toggle-knob);
    height: var(--size-toggle-knob);
    box-sizing: border-box;
    border-radius: 50%;
    background: var(--color-toggle-knob);
  }
  .switch[aria-checked="false"] .knob {
    border: var(--border-thin) solid var(--color-text-muted);
  }
  .switch[aria-checked="true"] .knob {
    left: calc(var(--size-toggle-width) - var(--size-toggle-knob) - (var(--size-toggle-height) - var(--size-toggle-knob)) / 2);
  }
</style>
