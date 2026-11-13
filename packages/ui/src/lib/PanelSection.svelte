<svelte:options namespace="svg" />

<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

One panel section inside a panel drawing (PanelSchematic): its frame and title and our own control
glyphs, drawn from scratch and never traced from a real panel, at the pack's own positions, in the
panel palette (decision 0021). Each control is in one state:
- changed: the step sets it; blue fill and outline, the knob at the target, "SET TO …" under it;
- resolved: changed, with the position from a decision step's answer (decision 0022);
- conditional: the step sets it only if a condition applies that the app does not know yet;
  a dashed blue outline and no position;
- resting: a control the step does not use, a neutral outline at its resting position;
- lit or unlit: a light the step expects on takes its real color; every other light is an outline
  and its legend, with no fill.
What the step asks for, never the simulator's state (decision 0006).
-->
<script lang="ts">
  import type { ItemTarget, PanelItem, PanelSection } from "@chocktochock/bindings";
  import { em } from "./text-width";
  import { angleOf, BUTTON, DIAL, half, halfWidth, withinAcross, knobY, LAMP, TOGGLE, labelled, lampText, layoutSection, leverY, PX, positionsOf, slotY } from "./panel-layout";

  let {
    section,
    targets,
    guardClosed,
    band,
    frame,
    titled = true,
    k,
    hit = 0,
    onPick,
  }: {
    section: PanelSection;
    targets: ItemTarget[];
    guardClosed: boolean;
    /** The part to draw, in section units: rows and, for a wide section, a span across. */
    band: { y0: number; y1: number; x0: number; x1: number };
    /** The outline drawn around the controls, fitted to them; by default the band and labels. */
    frame?: { y0: number; y1: number; x0: number; x1: number } | undefined;
    /** The section's title is drawn; a later part of a divided section continues without it. */
    titled?: boolean;
    /** Screen pixels per drawing unit, for the fixed text sizes. */
    k: number;
    /** The smallest hit area, in drawing units (44 px on screen); 0 draws none. */
    hit?: number;
    /** Tapping a control opens its detail card. */
    onPick?: ((section: string, item: number) => void) | undefined;
  } = $props();

  /** Room above a cut-down section for its title, so it never overlaps the top row. */
  const TITLE = 18;
  const fs = (px: number) => `${(px / k).toFixed(2)}px`;
  /** Half the guard's width and height around a toggle's track. */
  const gw = TOGGLE.w / 2 + TOGGLE.guard;
  const gh = TOGGLE.h / 2 + TOGGLE.guard;
  /** Light words shrink only if they would not fit inside the light. */
  /** A screen's label: 3.0's size, smaller only if it would not fit across the screen. */
  const screenFs = (label: string, boxWidth: number) =>
    `${Math.min(PX.screen / k, (boxWidth - 6) / em(label)).toFixed(2)}px`;

  let byItem = $derived(new Map(targets.filter((t) => t.section === section.id).map((t) => [t.item, t])));


  let top = $derived(frame ? frame.y0 : band.y0 === 0 ? 0 : band.y0 - TITLE);
  const shownItems = () =>
    section.items
      .map((item, i) => ({ item, i }))
      .filter(
        ({ item }) =>
          item.y >= band.y0 + 10 && item.y <= band.y1 - 10 && withinAcross(item, band),
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
          open: byItem.get(i)?.open ?? false,
          resolved: byItem.get(i)?.resolved ?? false,
        };
      },
      k,
      top,
      // Labels may reach past a crop's edge; the frame fits them, so check them against the section.
      { x0: 0, x1: section.width },
    ),
  );
  /** The frame reaches past the lowest label, so no text crosses its edge. */
  let frameBottom = $derived(frame ? frame.y1 : Math.max(band.y1, layout.bottom));
  let left = $derived(frame?.x0 ?? band.x0);
  let right = $derived(frame?.x1 ?? band.x1);
</script>

<g transform="translate({section.x} {section.y})">
  <rect class="frame" x={left} y={top} width={right - left} height={frameBottom - top} rx="3" />
  {#if titled}<text class="title" x={left + 8} y={top + 13} style:font-size={fs(PX.title * layout.scale)}>{section.title.toUpperCase()}</text>{/if}
  {#each shown as { item, i } (i)}
    {@const on = byItem.has(i)}
    {@const positions = positionsOf(item)}
    {@const t = byItem.get(i)?.position ?? null}
    <!-- A control the step does not use rests where the pack says, drawn dimmed. -->
    <!-- A step's control whose position the step leaves to a condition shows no position at all
         (its caption says to see the step): drawn at rest, it read as set to that position. -->
    {@const open = on && (byItem.get(i)?.open ?? false)}
    {@const r = (!on || (t == null && !open)) && item.rest ? positions.indexOf(item.rest) : -1}
    <g class="item {item.kind}" class:on class:resolved={on && (byItem.get(i)?.resolved ?? false)} class:conditional={open} transform="translate({item.x} {item.y})">

      {#if item.kind === "toggle_switch"}
        {#if item.guarded}
          <!-- A guard: an open frame around the switch, with its hinge at the top; closed, a lid
               line across it. -->
          <path class="guard" d="M {-gw} {gh} V {-gh + 2} Q {-gw} {-gh} {-gw + 2} {-gh} H {gw - 2} Q {gw} {-gh} {gw} {-gh + 2} V {gh}" />
          <!-- A guard the step does not touch rests closed, as on the airplane; the step's guard
               shows closed (blue lid) when the step closes it, open otherwise. -->
          {#if !on}<line class="guard lid rest" x1={-gw} x2={gw} y1={gh} y2={gh} />{:else if guardClosed}<line class="guard lid" x1={-gw - 1.5} x2={gw + 1.5} y1={gh} y2={gh} />{/if}
        {/if}
        <rect class="body track" class:changed={on && !open} class:conditional={open} x={-TOGGLE.w / 2} y={-TOGGLE.h / 2} width={TOGGLE.w} height={TOGGLE.h} rx={TOGGLE.w / 2} />
        {#if on && t != null}
          <circle class="knob set" cy={knobY(t, positions.length)} r={TOGGLE.knob + 0.3} />
        {:else if r >= 0}
          <circle class="knob rest" cy={knobY(r, positions.length)} r={TOGGLE.knob} />
        {/if}
        {#each layout.items.get(i)?.positions ?? [] as l (l.n)}
          <text class="pos" class:target={on && l.n === t} class:at-rest={l.n === r} x={l.x} y={l.y} text-anchor={l.anchor} style:font-size={fs(PX.pos * layout.scale)}>{positions[l.n]}</text>
        {/each}
      {:else if item.kind === "selector"}
        <!-- A rotary selector: a tick for each labelled position, a round body and a pointer bar
             across the knob, turned to the target (or resting) position. -->
        {#each layout.items.get(i)?.positions.map((l) => l.n) ?? [] as n (n)}
          <line class="tick" class:target={on && n === t} x1="0" y1={-DIAL - 1.5} x2="0" y2={-DIAL - 5} transform="rotate({angleOf(item, n)})" />
        {/each}
        <circle class="body" class:changed={on && !open} class:conditional={open} r={DIAL} />
        {#if on && t != null}
          <rect class="bar set" x="-1.6" y={-DIAL + 1.5} width="3.2" height={2 * DIAL - 3} rx="1.6" transform="rotate({angleOf(item, t)})" />
          <circle class="tip set" cy={-DIAL + 3.2} r="1.3" transform="rotate({angleOf(item, t)})" />
        {:else if r >= 0}
          <rect class="bar rest" x="-1.6" y={-DIAL + 2.5} width="3.2" height={2 * DIAL - 5} rx="1.6" transform="rotate({angleOf(item, r)})" />
          <!-- The pointing end, so a resting knob reads one way (SYS DSPL L or R). -->
          <circle class="tip rest" cy={-DIAL + 3.4} r="2.1" transform="rotate({angleOf(item, r)})" />
        {/if}
        <!-- Each label sits at its own tick, so the pointer and its label always agree. -->
        {#each layout.items.get(i)?.positions ?? [] as l (l.n)}
          <text class="pos" class:target={on && l.n === t} class:at-rest={l.n === r} x={l.x} y={l.y} text-anchor={l.anchor} style:font-size={fs(PX.pos * layout.scale)}>{positions[l.n]}</text>
        {/each}
      {:else if item.kind === "push_button"}
        <!-- As 3.0 draws a push button: a key with its name inside and a line along its bottom. -->
        {@const words = lampText(item, k, PX.lamp * layout.scale, BUTTON.w)}
        {#if item.guarded}
          <rect class="guard" x={-BUTTON.w / 2 - 2} y={-BUTTON.h / 2 - 2} width={BUTTON.w + 4} height={BUTTON.h + 4} rx="4" />
        {/if}
        <rect class="body key" class:changed={on && !open} class:conditional={open} x={-BUTTON.w / 2} y={-BUTTON.h / 2} width={BUTTON.w} height={BUTTON.h} rx="3" />
        <line class="key-line" x1={-BUTTON.w / 2 + 2} x2={BUTTON.w / 2 - 2} y1={BUTTON.h / 2 - 1.3} y2={BUTTON.h / 2 - 1.3} />
        {#each words.lines as line, n (n)}
          <text
            class="lamp-text"
            y={(n - (words.lines.length - 1) / 2) * ((words.px * 1.12) / k) + (words.px * 0.35) / k - 0.6}
            style:font-size={fs(words.px)}>{line}</text
          >
        {/each}
      {:else if item.kind === "light"}
        <!-- Every light the same size, its words wrapped inside, as in 3.0. A light the step
             expects on is filled in its color and says so; it is never the simulator's state. -->
        {@const words = lampText(item, k, PX.lamp * layout.scale)}
        {@const lit = byItem.get(i)?.lit ?? false}
        <rect class="lamp {item.color ?? ''}" class:lit class:focus={on && !lit} x={-LAMP.w / 2} y={-LAMP.h / 2} width={LAMP.w} height={LAMP.h} rx="2.6" />
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
        <rect class="body slot" class:changed={on && !open} class:conditional={open} x="-3" y="-16" width="6" height="32" rx="3" />
        {#if on && t != null}
          <rect class="knob set" x="-8" y={slotY(t, positions.length) - 3} width="16" height="6" rx="2" />
        {:else if r >= 0}
          <rect class="knob rest" x="-8" y={slotY(r, positions.length) - 3} width="16" height="6" rx="2" />
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
        <rect class="body grip" class:changed={on && !open} class:conditional={open} x="-13" y={gy - 5} width="26" height="10" rx="4" />
        {#each layout.items.get(i)?.positions ?? [] as l (l.n)}
          <text class="pos" class:target={on && l.n === t} class:at-rest={l.n === r} x={l.x} y={l.y} text-anchor={l.anchor} style:font-size={fs(PX.pos * layout.scale)}>{positions[l.n]}</text>
        {/each}
      {:else if item.kind === "gauge"}
        <circle class="body" class:changed={on} r="14" />
        <path class="scale" d="M -9 6 A 11 11 0 1 1 9 6" />
      {:else}
        {@const w = item.width ?? 60}
        {@const hh = item.height ?? 30}
        <rect class="body screen" class:changed={on} x={-w / 2} y={-hh / 2} width={w} height={hh} rx="3" />
        <text class="lamp-text" y="2" style:font-size={screenFs(item.label, w)}>{item.label}</text>
      {/if}
      {#if labelled(item)}
        {@const name = layout.items.get(i)?.name}
        {#if name}
          {#each name.lines as line, n (n)}
            <text class="label" y={name.y + (n * name.px * 1.15) / k} style:font-size={fs(name.px)}>{line}</text>
          {/each}
          {#each name.caption as line, n (n)}
            <text class="set-to" y={name.y + ((name.lines.length + n) * name.px * 1.15) / k} style:font-size={fs(PX.setTo * layout.scale)}>{line}</text>
          {/each}
        {/if}
      {/if}
      {#if hit > 0 && onPick}
        {@const hw = Math.max(halfWidth(item), hit / 2)}
        {@const hh = Math.max(half(item), hit / 2)}
        <!-- A tap target of 44 px or more around the control (rules.md §3); the step's controls
             are in the tab order, the rest open by pointer. -->
        <rect
          class="hit"
          x={-hw}
          y={-hh}
          width={2 * hw}
          height={2 * hh}
          role="button"
          tabindex={on ? 0 : -1}
          aria-label="{item.label}: what it is"
          onclick={() => onPick(section.id, i)}
          onkeydown={(e) => (e.key === "Enter" || e.key === " ") && (e.preventDefault(), onPick(section.id, i))}
        />
      {/if}
    </g>
  {/each}
</g>

<style>
  /* Panel palette only (decision 0021): outlines, switch bodies and lens rims at 3:1 or better
     against the panel, labels at 4.5:1, in both themes. */
  /* Sections sit on the panel background, as on the boards, so the blue of a changed control
     stands out against it (design review F2). */
  .frame {
    fill: var(--panel-bg);
    stroke: var(--panel-stroke);
    stroke-width: 1;
  }
  .title {
    font-weight: var(--weight-semibold);
    fill: var(--panel-label);
  }
  text {
    font-family: var(--font-sans);
    fill: var(--panel-text);
  }
  .label {
    font-weight: var(--weight-semibold);
    text-anchor: middle;
  }
  .pos {
    fill: var(--panel-label);
  }
  /* The target position reads first by weight; color stays on the drawing (rules.md §4). */
  .pos.target {
    font-weight: var(--weight-bold);
    fill: var(--panel-text);
  }
  .set-to {
    font-weight: var(--weight-semibold);
    text-anchor: middle;
    fill: var(--panel-text);
  }
  /* Resting: a control the step does not use, a neutral outline. */
  .body,
  .tick,
  .scale {
    fill: none;
    stroke: var(--panel-resting);
    stroke-width: 1.4;
  }
  .track {
    stroke-width: 1.2;
  }
  .key {
    fill: var(--panel-fill);
  }
  .key-line {
    stroke: var(--panel-resting);
    stroke-width: 0.8;
  }
  .knob.rest,
  .bar.rest {
    fill: none;
    stroke: var(--panel-resting);
    stroke-width: 1.3;
  }
  .guard {
    fill: none;
    stroke: var(--panel-resting);
    stroke-width: 1.1;
  }
  .guard.lid.rest {
    stroke: var(--panel-resting);
    stroke-width: 1.6;
  }
  .guard.lid {
    stroke: var(--panel-changed);
    stroke-width: 2.4;
    stroke-linecap: round;
  }
  .screen {
    fill: var(--panel-glass);
    stroke: var(--panel-stroke);
  }
  /* Changed (and resolved): blue fill and outline, the knob or pointer at the target. */
  .body.changed,
  .on .tick.target {
    fill: var(--panel-changed-fill);
    stroke: var(--panel-changed);
    stroke-width: 2;
  }
  .on .tick.target {
    fill: none;
  }
  .set,
  .bar.set {
    fill: var(--panel-changed);
    stroke: none;
  }
  .tip.set {
    fill: var(--panel-changed-fill);
  }
  .tip.rest {
    fill: var(--panel-text);
  }

  .on .lamp-text {
    fill: var(--panel-changed-text);
  }
  /* Conditional: a dashed blue outline and no position, until the condition is known. */
  .body.conditional {
    fill: none;
    stroke: var(--panel-changed);
    stroke-width: 1.6;
    stroke-dasharray: 3 2;
  }
  /* Lights: unlit is an outline and its legend, no fill; lit takes its real color. */
  .lamp {
    fill: none;
    stroke: var(--panel-stroke);
    stroke-width: 1.2;
  }
  .lamp.focus {
    stroke: var(--panel-changed);
    stroke-width: 2;
  }
  .lamp-text {
    font-weight: var(--weight-semibold);
    text-anchor: middle;
    fill: var(--panel-label);
  }
  .lamp.lit {
    stroke: none;
  }
  .lamp.lit.amber {
    fill: var(--color-lit-amber);
  }
  .lamp.lit.red {
    fill: var(--color-danger);
  }
  .lamp.lit.green {
    fill: var(--color-done);
  }
  .lamp.lit.blue {
    fill: var(--color-accent);
  }
  .lamp-text.lit.amber {
    fill: var(--color-on-lit-amber);
  }
  .lamp-text.lit.red {
    fill: var(--color-on-accent);
  }
  .lamp-text.lit.green {
    fill: var(--color-on-done);
  }
  .lamp-text.lit.blue {
    fill: var(--color-on-accent);
  }
  .hit {
    fill: transparent;
    cursor: pointer;
  }
  .hit:focus-visible {
    outline: none;
    stroke: var(--panel-changed);
    stroke-width: 1.5;
    stroke-dasharray: 2 2;
  }
  .lamp-on {
    fill: var(--panel-text);
  }
</style>
