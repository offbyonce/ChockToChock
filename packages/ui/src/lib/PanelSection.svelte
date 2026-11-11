<svelte:options namespace="svg" />

<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

One panel section inside a panel drawing (PanelSchematic): its frame and title and our own control
glyphs, drawn from scratch and never traced from a real panel, at the pack's own positions. The
controls a step uses are highlighted and show the step's target position, labeled "SET TO …"
(Phase 2 brief, step 7): what the step asks for, never the simulator's state (decision 0006).
Other controls show no position. Text sizes come from the parent, so text stays at 3.0's sizes
however large the drawing is.
-->
<script lang="ts">
  import type { ItemTarget, PanelItem, PanelSection } from "@chocktochock/bindings";
  import { angle, bottomOf, half, labelledPositions, labelled, labelY, lines, PX, positionsOf } from "./panel-layout";

  let {
    section,
    targets,
    guardClosed,
    band,
    k,
  }: {
    section: PanelSection;
    targets: ItemTarget[];
    guardClosed: boolean;
    /** The rows to draw, in section units; the whole section when not cut down. */
    band: { y0: number; y1: number };
    /** Screen pixels per drawing unit, for the fixed text sizes. */
    k: number;
  } = $props();

  /** Room above a cut-down section for its title, so it never overlaps the top row. */
  const TITLE = 18;
  const fs = (px: number) => `${(px / k).toFixed(2)}px`;
  /** Light words shrink only if they would not fit inside the light. */
  const lampFs = (words: string[], boxWidth: number) =>
    `${Math.min(PX.lamp / k, (boxWidth - 3) / (Math.max(...words.map((w) => w.length)) * 0.62)).toFixed(2)}px`;

  let byItem = $derived(new Map(targets.filter((t) => t.section === section.id).map((t) => [t.item, t])));
  /** The target position's name, if the step names one. */
  const targetOf = (i: number, item: PanelItem) => {
    const t = byItem.get(i);
    return t?.position != null ? (positionsOf(item)[t.position] ?? null) : null;
  };

  /** A toggle's lever position: first position at the top, last at the bottom. */
  const leverY = (index: number, count: number) => (count < 2 ? 0 : -8 + (16 * index) / (count - 1));


  let top = $derived(band.y0 === 0 ? 0 : band.y0 - TITLE);
  const shownItems = () =>
    section.items.map((item, i) => ({ item, i })).filter(({ item }) => item.y >= band.y0 + 10 && item.y <= band.y1 - 10);
  let shown = $derived(shownItems());
  /** The frame reaches past the lowest label, so no text crosses its edge. */
  let frameBottom = $derived(
    Math.max(band.y1, ...shownItems().map(({ item, i }) => bottomOf(item, targetOf(i, item) != null, k) + 4)),
  );
</script>

<g transform="translate({section.x} {section.y})">
  <rect class="frame" y={top} width={section.width} height={frameBottom - top} rx="3" />
  <text class="title" x="8" y={top + 13} style:font-size={fs(PX.title)}>{section.title.toUpperCase()}</text>
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
          <circle class="knob set" cy={leverY(t, positions.length)} r="3.4" />
        {:else}
          <circle class="knob unknown" r="2.6" />
        {/if}
        {#if positions.length <= 3}
          {#each positions as p, n (n)}
            <text class="pos" class:target={on && n === t} x={item.guarded ? 14 : 10} y={leverY(n, positions.length) + 2.5} style:font-size={fs(PX.pos)}>{p}</text>
          {/each}
        {/if}
      {:else if item.kind === "selector"}
        {#each positions as _, n (n)}
          <line class="tick" class:target={on && n === t} x1="0" y1="-12" x2="0" y2="-15.5" transform="rotate({angle(n, positions.length)})" />
        {/each}
        <circle class="body" r="10.5" />
        {#if on && t != null}
          <line class="pointer" x1="0" y1="0" x2="0" y2="-9" transform="rotate({angle(t, positions.length)})" />
          <circle class="knob set" r="2.4" />
        {:else}
          <circle class="knob-ring" r="4.5" />
        {/if}
        <!-- Each label sits at its own tick, so the pointer and its label always agree. -->
        {#each labelledPositions(positions.length, on ? t : null) as n (n)}
          {@const a = (angle(n, positions.length) * Math.PI) / 180}
          {@const r = 18.5}
          <text
            class="pos"
            class:target={on && n === t}
            x={Math.sin(a) * r}
            y={-Math.cos(a) * r + 3}
            text-anchor={Math.sin(a) < -0.3 ? "end" : Math.sin(a) > 0.3 ? "start" : "middle"}
            style:font-size={fs(PX.pos)}>{positions[n]}</text
          >
        {/each}
      {:else if item.kind === "push_button"}
        {#if item.guarded}
          <rect class="guard" class:closed={on && guardClosed} x="-14" y="-14" width="28" height="28" rx="6" />
        {/if}
        <rect class="body" class:pressed={on && t != null} x="-11" y="-11" width="22" height="22" rx="4" />
      {:else if item.kind === "light"}
        {@const words = lines(item.label, 9)}
        <rect class="body lamp" x="-19" y="-10" width="38" height="20" rx="2" />
        {#each words as line, n (n)}
          <text class="lamp-text" y={(n - (words.length - 1) / 2) * 7 + 2} style:font-size={lampFs(words, 38)}>{line}</text>
        {/each}
      {:else if item.kind === "gauge"}
        <circle class="body" r="14" />
        <path class="scale" d="M -9 6 A 11 11 0 1 1 9 6" />
      {:else}
        {@const w = item.width ?? 60}
        {@const hh = item.height ?? 30}
        <rect class="body screen" x={-w / 2} y={-hh / 2} width={w} height={hh} rx="3" />
        <text class="lamp-text" y="2" style:font-size={lampFs([item.label], w)}>{item.label}</text>
      {/if}
      {#if labelled(item)}
        {#each lines(item.label) as line, n (n)}
          <text class="label" y={labelY(item, n, k)} style:font-size={fs(PX.name)}>{line}</text>
        {/each}
        {#if on && target}
          <text class="set-to" y={labelY(item, lines(item.label).length, k)} style:font-size={fs(PX.setTo)}>SET TO {target}</text>
        {/if}
      {/if}
    </g>
  {/each}
</g>

<style>
  .frame {
    fill: none;
    stroke: var(--line);
  }
  .title {
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
    stroke: var(--stop);
    stroke-width: 1.2;
    stroke-dasharray: 2.5 2;
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
    font-weight: 600;
    text-anchor: middle;
  }
  .lamp-text {
    font-weight: 600;
    text-anchor: middle;
  }
  .pos {
    fill: var(--muted);
  }
  /* As 3.0, color alone marks the controls the step uses: outlines keep their weight, nothing is
     boxed or bolded. Without color, the knob still tells them apart: only they show a position. */
  .on .body,
  .on .knob-ring,
  .on .scale,
  .on .tick.target {
    stroke: var(--accent);
  }
  .on .body.pressed {
    stroke-width: 2;
  }
  .pointer {
    stroke: var(--accent);
    stroke-width: 1.8;
    stroke-linecap: round;
  }
  .set {
    fill: var(--accent);
  }
  .on .label,
  .pos.target {
    fill: var(--accent);
  }
  .set-to {
    font-weight: 600;
    text-anchor: middle;
    fill: var(--accent);
    letter-spacing: 0.02em;
  }
</style>
