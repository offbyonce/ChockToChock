<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

One panel section as a schematic, with our own control glyphs (Phase 2 brief, step 7): drawn from
scratch, never traced from a real panel. Positions come from the pack's own layout. The app has no
simulator link (decision 0006), so no glyph shows a switch position and lights are drawn unlit.
The controls a step uses are marked by a thicker outline and fill as well as color.
-->
<script lang="ts">
  import type { PanelItem, PanelSection } from "@chocktochock/bindings";

  let { section, controls }: { section: PanelSection; controls: string[] } = $props();

  const marked = (item: PanelItem) => (item.controls ?? []).some((c) => controls.includes(c));

  /** Half the height a glyph takes, so its label sits under it. */
  function half(item: PanelItem): number {
    switch (item.kind) {
      case "toggle_switch":
        return item.guarded ? 19 : 15;
      case "selector":
      case "gauge":
        return 14;
      case "push_button":
        return 11;
      case "light":
        return 9;
      default:
        return (item.height ?? 30) / 2;
    }
  }

  /** Long labels go on two lines, split near the middle. */
  function lines(label: string): string[] {
    if (label.length <= 12) return [label];
    const words = label.split(" ");
    let best = 1;
    let bestDiff = Infinity;
    for (let i = 1; i < words.length; i++) {
      const diff = Math.abs(words.slice(0, i).join(" ").length - words.slice(i).join(" ").length);
      if (diff < bestDiff) [best, bestDiff] = [i, diff];
    }
    return words.length > 1 ? [words.slice(0, best).join(" "), words.slice(best).join(" ")] : [label];
  }

  const ends = (positions: string | null | undefined) => {
    const all = (positions ?? "").split(",").map((p) => p.trim()).filter(Boolean);
    return all.length >= 2 ? [all[0], all[all.length - 1]] : null;
  };
</script>

<svg
  class="schematic"
  viewBox="0 0 {section.width} {section.height}"
  style:max-width="{Math.round(section.width * 1.4)}px"
  role="img"
  aria-label="{section.title}: schematic, not to scale"
>
  {#each section.items as item, i (i)}
    {@const on = marked(item)}
    {@const h = half(item)}
    <g class="item {item.kind}" class:on transform="translate({item.x} {item.y})">
      {#if item.kind === "toggle_switch"}
        {#if item.guarded}
          <rect class="guard" x="-11" y="-19" width="22" height="38" rx="5" />
        {/if}
        <rect class="body" x="-6" y="-14" width="12" height="28" rx="6" />
        <circle class="knob" r="3.2" />
        {@const e = ends(item.positions)}
        {#if e}
          <text class="pos" x={item.guarded ? 15 : 11} y="-9">{e[0]}</text>
          <text class="pos" x={item.guarded ? 15 : 11} y="13">{e[1]}</text>
        {/if}
      {:else if item.kind === "selector"}
        <circle class="body" r="13" />
        <circle class="knob-ring" r="6" />
      {:else if item.kind === "push_button"}
        <rect class="body" x="-11" y="-11" width="22" height="22" rx="4" />
        {#if item.guarded}
          <rect class="guard" x="-14" y="-14" width="28" height="28" rx="6" />
        {/if}
      {:else if item.kind === "light"}
        <rect class="body lamp" x="-17" y="-9" width="34" height="18" rx="2" />
      {:else if item.kind === "gauge"}
        <circle class="body" r="14" />
        <path class="scale" d="M -9 6 A 11 11 0 1 1 9 6" />
      {:else}
        {@const w = item.width ?? 60}
        {@const hh = item.height ?? 30}
        <rect class="body screen" x={-w / 2} y={-hh / 2} width={w} height={hh} rx="3" />
      {/if}
      {#each lines(item.label) as line, n (n)}
        <text class="label" y={h + 10 + n * 10}>{line}</text>
      {/each}
    </g>
  {/each}
</svg>

<style>
  /* Never drawn larger than 1.4 times the pack's own size, so a small section stays small. */
  .schematic {
    display: block;
    width: 100%;
    margin: 0 auto;
    height: auto;
    max-height: 26rem;
  }
  .body,
  .guard,
  .knob-ring,
  .scale {
    fill: none;
    stroke: var(--muted);
    stroke-width: 1.4;
  }
  .guard {
    stroke-dasharray: 3 2;
  }
  .knob {
    fill: var(--muted);
  }
  .lamp,
  .screen {
    fill: var(--zone);
  }
  text {
    font-family: var(--sans);
    fill: var(--zone-ink);
  }
  .label {
    font-size: 9px;
    font-weight: 600;
    text-anchor: middle;
  }
  .pos {
    font-size: 7px;
    fill: var(--muted);
  }
  .on .body,
  .on .guard,
  .on .knob-ring,
  .on .scale {
    stroke: var(--accent);
    stroke-width: 2.6;
  }
  .on .body {
    fill: var(--accent-soft);
  }
  .on .knob {
    fill: var(--accent);
  }
  .on .label,
  .on .pos {
    fill: var(--accent);
  }
</style>
