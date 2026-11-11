<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

One panel section as a schematic, laid out like 3.0's panel cards: the section frame and title,
our own control glyphs (drawn from scratch, never traced from a real panel) at the pack's own
positions, and lights with their words inside.

The controls a step uses are highlighted, and each shows the step's target position, labeled
"SET TO …" (Phase 2 brief, step 7). That is what the step asks for, never the simulator's state,
which the app does not know (decision 0006). Controls the step does not use show no position.
Highlighting uses a thicker outline and fill as well as color.
-->
<script lang="ts">
  import type { ItemTarget, PanelItem, PanelSection } from "@chocktochock/bindings";

  let {
    section,
    targets,
    guardClosed = false,
  }: {
    section: PanelSection;
    /** The step's targets in this section (from Rust). */
    targets: ItemTarget[];
    /** The step asks for the guard closed. */
    guardClosed?: boolean;
  } = $props();

  let byItem = $derived(new Map(targets.filter((t) => t.section === section.id).map((t) => [t.item, t])));
  const positionsOf = (item: PanelItem) =>
    (item.positions ?? "")
      .split(",")
      .map((p) => p.trim())
      .filter(Boolean);
  /** The target position's name, if the step names one. */
  const targetOf = (i: number, item: PanelItem) => {
    const t = byItem.get(i);
    return t?.position != null ? (positionsOf(item)[t.position] ?? null) : null;
  };

  /** Half the height a glyph takes, so its label sits under it. */
  function half(item: PanelItem): number {
    switch (item.kind) {
      case "toggle_switch":
        return item.guarded ? 19 : 15;
      case "selector":
      case "gauge":
        return 15;
      case "push_button":
        return 11;
      case "light":
        return 10;
      default:
        return (item.height ?? 30) / 2;
    }
  }

  /** Long labels go on two lines, split near the middle. */
  function lines(label: string, max = 12): string[] {
    if (label.length <= max) return [label];
    const words = label.split(" ");
    if (words.length < 2) return [label];
    let best = 1;
    let bestDiff = Number.POSITIVE_INFINITY;
    for (let i = 1; i < words.length; i++) {
      const diff = Math.abs(words.slice(0, i).join(" ").length - words.slice(i).join(" ").length);
      if (diff < bestDiff) [best, bestDiff] = [i, diff];
    }
    return [words.slice(0, best).join(" "), words.slice(best).join(" ")];
  }

  /** A toggle's lever position: first position at the top, last at the bottom. */
  const leverY = (index: number, count: number) => (count < 2 ? 0 : -9 + (18 * index) / (count - 1));
  /** A selector's pointer angle in degrees from straight up, positions spread over 240°. */
  const angle = (index: number, count: number) => (count < 2 ? 0 : -120 + (240 * index) / (count - 1));

  /** Fill the card's width, up to 1.8 times the pack's size, so a small section is not tiny. */
  const scale = 1.8;
  /** Room above a cut-down section for its title, so it never overlaps the top row. */
  const TITLE = 18;

  /**
   * The part of the section to draw. Like 3.0, a tall section is cut down to the rows around the
   * step's controls (the row above for context), so the drawing stays large enough to read; with
   * no control of the step in it, the whole section shows.
   */
  let band = $derived.by(() => {
    const ys = section.items.filter((_, i) => byItem.has(i)).map((item) => item.y);
    if (!ys.length || section.height <= 200) return { y0: 0, y1: section.height };
    const y0 = Math.max(0, Math.min(...ys) - 95);
    const y1 = Math.min(section.height, Math.max(...ys) + 72);
    return { y0, y1 };
  });
  let top = $derived(band.y0 === 0 ? 0 : band.y0 - TITLE);
  let shown = $derived(
    section.items.map((item, i) => ({ item, i })).filter(({ item }) => item.y >= band.y0 + 10 && item.y <= band.y1 - 10),
  );

  let described = $derived(
    section.items
      .map((item, i) => {
        const t = targetOf(i, item);
        return byItem.has(i) ? `${item.label}${t ? ` set to ${t}` : ""}` : null;
      })
      .filter(Boolean)
      .join("; "),
  );
</script>

<svg
  class="schematic"
  viewBox="-2 {top - 2} {section.width + 4} {band.y1 - top + 4}"
  style:width="min(100%, {Math.round((section.width + 4) * scale)}px)"
  role="img"
  aria-label="{section.title} panel, schematic.{described ? ` This step: ${described}.` : ''}"
>
  <rect class="frame" y={top} width={section.width} height={band.y1 - top} rx="3" />
  <text class="title" x="8" y={top + 13}>{section.title.toUpperCase()}</text>
  {#each shown as { item, i } (i)}
    {@const on = byItem.has(i)}
    {@const target = targetOf(i, item)}
    {@const positions = positionsOf(item)}
    {@const t = byItem.get(i)?.position ?? null}
    {@const h = half(item)}
    <g class="item {item.kind}" class:on transform="translate({item.x} {item.y})">
      {#if item.kind === "toggle_switch"}
        {#if item.guarded}
          <rect class="guard" class:closed={on && guardClosed} x="-11" y="-19" width="22" height="38" rx="5" />
        {/if}
        <rect class="body" x="-6" y="-14" width="12" height="28" rx="6" />
        {#if on && t != null}
          <line class="lever" x1="0" y1="0" x2="0" y2={leverY(t, positions.length)} />
          <circle class="knob set" cy={leverY(t, positions.length)} r="4" />
        {:else}
          <circle class="knob unknown" r="2.6" />
        {/if}
        {#if positions.length <= 3}
          {#each positions as p, n (n)}
            <text class="pos" class:target={on && n === t} x={item.guarded ? 14 : 10} y={leverY(n, positions.length) + 2.5}>{p}</text>
          {/each}
        {/if}
      {:else if item.kind === "selector"}
        {#each positions as _, n (n)}
          <line class="tick" class:target={on && n === t} x1="0" y1="-12" x2="0" y2="-15" transform="rotate({angle(n, positions.length)})" />
        {/each}
        <circle class="body" r="10.5" />
        {#if on && t != null}
          <line class="pointer" x1="0" y1="0" x2="0" y2="-9" transform="rotate({angle(t, positions.length)})" />
          <circle class="knob set" r="2.4" />
        {:else}
          <circle class="knob-ring" r="4.5" />
        {/if}
        {#if positions.length >= 2}
          <text class="pos end" x="-14" y="-12" text-anchor="end">{positions[0]}</text>
          <text class="pos end" x="14" y="-12">{positions[positions.length - 1]}</text>
        {/if}
      {:else if item.kind === "push_button"}
        {#if item.guarded}
          <rect class="guard" class:closed={on && guardClosed} x="-14" y="-14" width="28" height="28" rx="6" />
        {/if}
        <rect class="body" class:pressed={on && t != null} x="-11" y="-11" width="22" height="22" rx="4" />
      {:else if item.kind === "light"}
        {@const words = lines(item.label, 9)}
        <rect class="body lamp" x="-19" y="-10" width="38" height="20" rx="2" />
        {#each words as line, n (n)}
          <text class="lamp-text" y={(n - (words.length - 1) / 2) * 7 + 2}>{line}</text>
        {/each}
      {:else if item.kind === "gauge"}
        <circle class="body" r="14" />
        <path class="scale" d="M -9 6 A 11 11 0 1 1 9 6" />
      {:else}
        {@const w = item.width ?? 60}
        {@const hh = item.height ?? 30}
        <rect class="body screen" x={-w / 2} y={-hh / 2} width={w} height={hh} rx="3" />
        <text class="lamp-text" y="2">{item.label}</text>
      {/if}
      {#if item.kind !== "light" && item.kind !== "readout" && item.kind !== "display_unit"}
        {#each lines(item.label) as line, n (n)}
          <text class="label" y={h + 12 + n * 11}>{line}</text>
        {/each}
        {#if on && target}
          <text class="set-to" y={h + 12 + lines(item.label).length * 11}>SET TO {target}</text>
        {/if}
      {/if}
    </g>
  {/each}
</svg>

<style>
  .schematic {
    display: block;
    height: auto;
    max-height: 21rem;
    margin: 0 auto;
  }
  .frame {
    fill: none;
    stroke: var(--line);
  }
  .title {
    font-size: 8.5px;
    font-weight: 600;
    letter-spacing: 0.06em;
    fill: var(--muted);
  }
  .body,
  .guard,
  .knob-ring,
  .scale,
  .tick {
    fill: none;
    stroke: var(--muted);
    stroke-width: 1.4;
  }
  .guard {
    stroke-dasharray: 3 2;
  }
  .unknown {
    fill: none;
    stroke: var(--muted);
    stroke-width: 1.2;
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
    font-size: 10px;
    font-weight: 600;
    text-anchor: middle;
  }
  .lamp-text {
    font-size: 7px;
    font-weight: 600;
    text-anchor: middle;
  }
  .pos {
    font-size: 8px;
    fill: var(--muted);
  }
  .on .body,
  .on .knob-ring,
  .on .scale {
    stroke: var(--accent);
    stroke-width: 2.4;
  }
  .on .body {
    fill: var(--accent-soft);
  }
  .on .guard {
    stroke: var(--accent);
  }
  .on .guard.closed {
    stroke-dasharray: none;
    stroke-width: 2;
  }
  .on .body.pressed {
    fill: var(--accent);
  }
  .lever,
  .pointer {
    stroke: var(--accent);
    stroke-width: 2.6;
    stroke-linecap: round;
  }
  .set {
    fill: var(--accent);
  }
  .on .label {
    fill: var(--accent);
    font-weight: 700;
  }
  .pos.target {
    fill: var(--accent);
    font-weight: 700;
  }
  .tick.target {
    stroke: var(--accent);
    stroke-width: 2.4;
  }
  .set-to {
    font-size: 9px;
    font-weight: 700;
    text-anchor: middle;
    fill: var(--accent);
    letter-spacing: 0.03em;
  }
</style>
