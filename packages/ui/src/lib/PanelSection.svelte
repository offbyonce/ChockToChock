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
  import { angle, LAMP, labelled, lampText, layoutSection, leverY, PX, positionsOf, slotY } from "./panel-layout";

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
    /** The part to draw, in section units: rows and, for a wide section, a span across. */
    band: { y0: number; y1: number; x0: number; x1: number };
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


  let top = $derived(band.y0 === 0 ? 0 : band.y0 - TITLE);
  const shownItems = () =>
    section.items
      .map((item, i) => ({ item, i }))
      .filter(
        ({ item }) =>
          item.y >= band.y0 + 10 && item.y <= band.y1 - 10 && item.x >= band.x0 + 4 && item.x <= band.x1 - 4,
      );
  let shown = $derived(shownItems());
  /** Every label placed clear of the controls and of each other (panel-layout.ts). */
  let layout = $derived(
    layoutSection(
      section,
      shown.map((s) => s.i),
      (i) => {
        const item = section.items[i] as PanelItem;
        const on = byItem.has(i);
        return {
          on,
          t: byItem.get(i)?.position ?? null,
          rest: !on && item.rest ? positionsOf(item).indexOf(item.rest) : -1,
          lit: byItem.get(i)?.lit ?? false,
        };
      },
      k,
      top,
      { x0: band.x0, x1: band.x1 },
    ),
  );
  /** The frame reaches past the lowest label, so no text crosses its edge. */
  let frameBottom = $derived(Math.max(band.y1, layout.bottom));
</script>

<g transform="translate({section.x} {section.y})">
  <rect class="frame" x={band.x0} y={top} width={band.x1 - band.x0} height={frameBottom - top} rx="3" />
  <text class="title" x={band.x0 + 8} y={top + 13} style:font-size={fs(PX.title * layout.scale)}>{section.title.toUpperCase()}</text>
  {#each shown as { item, i } (i)}
    {@const on = byItem.has(i)}
    {@const target = targetOf(i, item)}
    {@const positions = positionsOf(item)}
    {@const t = byItem.get(i)?.position ?? null}
    <!-- A control the step does not use rests where the pack says, drawn dimmed. -->
    {@const r = !on && item.rest ? positions.indexOf(item.rest) : -1}
    <g class="item {item.kind}" class:on transform="translate({item.x} {item.y})">
      {#if item.kind === "toggle_switch"}
        {#if item.guarded}
          <rect class="guard" class:closed={on && guardClosed} x="-11" y="-19" width="22" height="38" rx="5" />
        {/if}
        <rect class="body" x="-6" y="-14" width="12" height="28" rx="6" />
        {#if on && t != null}
          <circle class="knob set" cy={leverY(t, positions.length)} r="3.4" />
        {:else if r >= 0}
          <circle class="knob rest" cy={leverY(r, positions.length)} r="2.8" />
        {:else}
          <circle class="knob unknown" r="2.6" />
        {/if}
        {#each layout.items.get(i)?.positions ?? [] as l (l.n)}
          <text class="pos" class:target={on && l.n === t} class:at-rest={l.n === r} x={l.x} y={l.y} text-anchor={l.anchor} style:font-size={fs(PX.pos * layout.scale)}>{positions[l.n]}</text>
        {/each}
      {:else if item.kind === "selector"}
        <!-- A tick for each labelled position, so no tick is left without a name. -->
        {#each layout.items.get(i)?.positions.map((l) => l.n) ?? [] as n (n)}
          <line class="tick" class:target={on && n === t} x1="0" y1="-12" x2="0" y2="-15.5" transform="rotate({angle(n, positions.length)})" />
        {/each}
        <circle class="body" r="10.5" />
        {#if on && t != null}
          <line class="pointer" x1="0" y1="0" x2="0" y2="-9" transform="rotate({angle(t, positions.length)})" />
          <circle class="knob set" r="2.4" />
        {:else if r >= 0}
          <line class="pointer rest" x1="0" y1="0" x2="0" y2="-8" transform="rotate({angle(r, positions.length)})" />
          <circle class="knob-ring" r="2.2" />
        {:else}
          <circle class="knob-ring" r="4.5" />
        {/if}
        <!-- Each label sits at its own tick, so the pointer and its label always agree. -->
        {#each layout.items.get(i)?.positions ?? [] as l (l.n)}
          <text class="pos" class:target={on && l.n === t} class:at-rest={l.n === r} x={l.x} y={l.y} text-anchor={l.anchor} style:font-size={fs(PX.pos * layout.scale)}>{positions[l.n]}</text>
        {/each}
      {:else if item.kind === "push_button"}
        {#if item.guarded}
          <rect class="guard" class:closed={on && guardClosed} x="-14" y="-14" width="28" height="28" rx="6" />
        {/if}
        <rect class="body" class:pressed={on && t != null} x="-11" y="-11" width="22" height="22" rx="4" />
      {:else if item.kind === "light"}
        <!-- Every light the same size, its words wrapped inside, as in 3.0. A light the step
             expects on is filled in its color and says so; it is never the simulator's state. -->
        {@const words = lampText(item, k, PX.lamp * layout.scale)}
        {@const lit = byItem.get(i)?.lit ?? false}
        <rect class="body lamp {item.color ?? ''}" class:lit x={-LAMP.w / 2} y={-LAMP.h / 2} width={LAMP.w} height={LAMP.h} rx="2.6" />
        {#each words.lines as line, n (n)}
          <text
            class="lamp-text {item.color ?? ''}"
            class:lit
            y={(n - (words.lines.length - 1) / 2) * ((words.px * 1.12) / k) + (words.px * 0.35) / k}
            style:font-size={fs(words.px)}>{line}</text
          >
        {/each}
        {#if lit}
          <text class="set-to lamp-on {item.color ?? ''}" y={LAMP.h / 2 + (PX.setTo * layout.scale * 1.1) / k} style:font-size={fs(PX.setTo * layout.scale)}>EXPECTED ON</text>
        {/if}
      {:else if item.kind === "lever"}
        <!-- A slot and the lever's knob at the target or resting position. -->
        <rect class="body slot" x="-3" y="-16" width="6" height="32" rx="3" />
        {#if on && t != null}
          <rect class="knob set" x="-8" y={slotY(t, positions.length) - 3} width="16" height="6" rx="2" />
        {:else if r >= 0}
          <rect class="knob rest" x="-8" y={slotY(r, positions.length) - 3} width="16" height="6" rx="2" />
        {:else}
          <rect class="knob unknown" x="-6" y="-2.5" width="12" height="5" rx="2" />
        {/if}
        {#each layout.items.get(i)?.positions ?? [] as l (l.n)}
          <text class="pos" class:target={on && l.n === t} class:at-rest={l.n === r} x={l.x} y={l.y} text-anchor={l.anchor} style:font-size={fs(PX.pos * layout.scale)}>{positions[l.n]}</text>
        {/each}
      {:else if item.kind === "handle"}
        <!-- A T-shaped pull handle on a short stem. -->
        <!-- The grip sits at the target (or resting) position, beside its label: in at the top,
             pulled out at the bottom. -->
        {@const at = on && t != null ? t : r >= 0 ? r : -1}
        {@const gy = at >= 0 ? leverY(at, positions.length) : 0}
        <rect class="body stem" x="-2" y="-12" width="4" height={gy + 8} rx="1" />
        <rect class="body grip" class:pressed={on && t != null} x="-13" y={gy - 5} width="26" height="10" rx="4" />
        {#each layout.items.get(i)?.positions ?? [] as l (l.n)}
          <text class="pos" class:target={on && l.n === t} class:at-rest={l.n === r} x={l.x} y={l.y} text-anchor={l.anchor} style:font-size={fs(PX.pos * layout.scale)}>{positions[l.n]}</text>
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
        {@const name = layout.items.get(i)?.name}
        {#if name}
          {#each name.lines as line, n (n)}
            <text class="label" y={name.y + (n * name.px * 1.15) / k} style:font-size={fs(name.px)}>{line}</text>
          {/each}
          {#if on && target}
            <text class="set-to" y={name.y + (name.lines.length * name.px * 1.15) / k} style:font-size={fs(PX.setTo * layout.scale)}>{target === "SET" ? "SET" : `SET TO ${target}`}</text>
          {/if}
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
  /* Resting positions of controls the step does not use: hollow and muted, so they never look
     like the step's target (solid, in the accent color). */
  .knob.rest {
    fill: none;
    stroke: var(--muted);
    stroke-width: 1.4;
    opacity: 0.75;
  }
  .pointer.rest {
    stroke: var(--muted);
    stroke-width: 1.4;
    opacity: 0.75;
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
  .knob.rest:is(rect) {
    fill: none;
  }
  /* A light the step expects on: filled in its color, with its words in that color. */
  .lamp.lit {
    stroke-width: 1.8;
  }
  .lamp.lit.amber {
    fill: var(--warn-soft);
    stroke: var(--warn);
  }
  .lamp.lit.red {
    fill: var(--stop-soft);
    stroke: var(--stop);
  }
  .lamp.lit.green {
    fill: var(--go-soft);
    stroke: var(--go);
  }
  .lamp.lit.blue {
    fill: var(--accent-soft);
    stroke: var(--accent);
  }
  .lamp-on.amber {
    fill: var(--warn);
  }
  .lamp-on.red {
    fill: var(--stop);
  }
  .lamp-on.green {
    fill: var(--go);
  }
  .lamp-text.lit.amber {
    fill: var(--warn);
  }
  .lamp-text.lit.red {
    fill: var(--stop);
  }
  .lamp-text.lit.green {
    fill: var(--go);
  }
  .lamp-text.lit.blue {
    fill: var(--accent);
  }
  .on .grip.pressed {
    fill: var(--accent-soft);
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
