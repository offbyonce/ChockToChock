<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

A panel drawing for one step. It draws the panel sections that hold the step's controls, in the
pack's own panel layout, with their neighbors in the same rows for context (so two small sections
side by side, such as the wiper selectors, share one drawing). The drawing is one piece: never
split, cropped or rearranged (decision 0021, docs/design/rules.md §5). It is as wide as its card,
keeping its proportions, unless that would draw a label under 11 px; then it is drawn at the
smallest size that keeps 11 px labels, scrolls sideways, and offers a full-width view.
-->
<script lang="ts">
  import { panelCaption } from "./panel-name";
  import type { ItemTarget, Panel, PanelSection as Section } from "@chocktochock/bindings";
  import { drawingScale, halfWidth, K, layoutSection, openingScroll, positionsOf } from "./panel-layout";
  import PanelSection from "./PanelSection.svelte";
  import Self from "./PanelSchematic.svelte";
  import type { PackIndex } from "../pack-index";

  let {
    panel,
    focus,
    targets,
    guardClosed = false,
    large = false,
    index,
    picked = $bindable(),
    around = false,
    named = "",
  }: {
    /** The panel the sections are on. */
    panel: Panel;
    /** Ids of the sections that hold the step's controls. */
    focus: string[];
    /** The step's targets (from Rust). */
    targets: ItemTarget[];
    /** The step asks for the guard closed. */
    guardClosed?: boolean;
    /** Drawn in the full-width view, which offers no larger one. */
    large?: boolean | undefined;
    /** With the pack's index, tapping a control opens its card: what it is and what it does. */
    index?: PackIndex | undefined;
    /** The side pane's drawing (Checklist board): a section drawn alone shows the rows around the
     * step's controls, whole across; the larger view shows the whole section. */
    around?: boolean | undefined;
    /** Text naming more controls to keep in the step's rows (its Check and If not: "ALIGN light
     * on, then off"). */
    named?: string | undefined;
    /** The control whose card is open. */
    picked?: { section: string; item: number } | null | undefined;
  } = $props();

  /** How far beside the step's sections a neighbor may sit and still be drawn. */
  const NEAR = 120;

  let focused = $derived(panel.sections.filter((s) => focus.includes(s.id)));
  /**
   * The step's sections and, for context, the sections beside and between them in the same rows.
   * A tall section stands alone.
   */
  let drawn = $derived.by((): Section[] => {
    if (focused.length === 1 && (focused[0]?.height ?? 0) > 200) return focused;
    const x0 = Math.min(...focused.map((s) => s.x)) - NEAR;
    const x1 = Math.max(...focused.map((s) => s.x + s.width)) + NEAR;
    const y0 = Math.min(...focused.map((s) => s.y));
    const y1 = Math.max(...focused.map((s) => s.y + s.height));
    const [left, right] = [x0 + NEAR, x1 - NEAR];
    return panel.sections.filter((s) => {
      if (focus.includes(s.id)) return true;
      if (!(s.y < y1 && s.y + s.height > y0)) return false;
      // Everything between the step's sections is part of the one piece (the MCP between the two
      // EFIS panels); small neighbors beside them are drawn for context.
      if (s.x >= left && s.x + s.width <= right) return true;
      return s.x >= x0 && s.x + s.width <= x1 && s.height <= 200;
    });
  });

  type Band = { y0: number; y1: number; x0: number; x1: number };
  const wholeOf = (s: Section): Band => ({ y0: 0, y1: s.height, x0: 0, x1: s.width });

  /**
   * The rows a lone section shows in the side pane: from the step's highest control, with room
   * above for its lights and labels, to its lowest, whole across. The whole section otherwise.
   */
  let band = $derived.by((): Band | null => {
    const only = drawn.length === 1 ? drawn[0] : undefined;
    if (!around || large || !only) return null;
    const words = ` ${named.toUpperCase().replace(/[^A-Z0-9/ ]+/g, " ")} `;
    const ys = only.items.flatMap((it, i) =>
      targets.some((t) => t.section === only.id && t.item === i) || (it.label && words.includes(` ${it.label.toUpperCase()} `))
        ? [it.y]
        : [],
    );
    if (!ys.length) return null;
    const y0 = Math.max(0, Math.min(...ys) - 62);
    const y1 = Math.min(only.height, Math.max(...ys) + 24);
    return y0 <= 0 && y1 >= only.height ? null : { x0: 0, x1: only.width, y0, y1 };
  });
  const bandOf = (s: Section): Band => (band && drawn.length === 1 ? band : wholeOf(s));

  /** How a section's labels lay out, title at `titleTop`. */
  function layoutFor(s: Section, titleTop: number) {
    const on = (i: number) => targets.find((t) => t.section === s.id && t.item === i);
    const b = bandOf(s);
    return layoutSection(
      s,
      s.items.flatMap((it, i) => (it.y >= b.y0 + 10 && it.y <= b.y1 - 10 ? [i] : [])),
      (i) => {
        const item = s.items[i] as (typeof s.items)[number];
        return {
          on: !!on(i),
          t: on(i)?.position ?? null,
          rest: !on(i) && item.rest ? positionsOf(item).indexOf(item.rest) : -1,
          lit: on(i)?.lit ?? false,
          resolved: on(i)?.resolved ?? false,
        };
      },
      K,
      titleTop,
    );
  }

  /**
   * The drawing's frames and box, in panel units. A section drawn alone is framed to what it
   * shows (its controls and labels), so the drawing fills the card instead of the section's empty
   * corners; the card's caption names it, so it needs no title of its own.
   */
  let view = $derived.by(() => {
    const frames = new Map<string, Band | undefined>();
    const only = drawn.length === 1 ? drawn[0] : undefined;
    if (only) {
      const e = layoutFor(only, -1000).extent;
      const f = { x0: e.x0 - 8, x1: e.x1 + 8, y0: e.y0 - 8, y1: e.y1 + 8 };
      frames.set(only.id, f);
      return {
        frames,
        titled: false,
        box: { x: only.x + f.x0 - 2, y: only.y + f.y0 - 2, w: f.x1 - f.x0 + 4, h: f.y1 - f.y0 + 4 },
      };
    }
    const x = Math.min(...drawn.map((s) => s.x)) - 2;
    const y = Math.min(...drawn.map((s) => s.y)) - 2;
    const right = Math.max(...drawn.map((s) => s.x + s.width)) + 2;
    const low = Math.max(...drawn.map((s) => s.y + Math.max(s.height, layoutFor(s, 0).bottom)));
    return { frames, titled: true, box: { x, y, w: right - x, h: low + 2 - y } };
  });

  let width = $state(0);
  /** The page's own scale (wide screens and Text size), so a drawing grows with the page. */
  let pageScale = $state(1);
  $effect(() => {
    if (!wrap) return;
    const read = () => {
      pageScale = Number.parseFloat(getComputedStyle(document.documentElement).fontSize) / 16 || 1;
    };
    read();
    window.addEventListener("resize", read);
    return () => window.removeEventListener("resize", read);
  });
  let scale = $derived(drawingScale(width, view.box.w, pageScale));

  /** A drawing wider than the card starts scrolled to the step's controls. */
  let wrap = $state<HTMLDivElement | null>(null);
  $effect(() => {
    if (!wrap || !scale.overflow) return;
    const el = wrap;
    void width;
    // After the drawing has taken its new size (wide screens rescale it after the first frame).
    const frame = requestAnimationFrame(() => openOnTargets(el));
    return () => cancelAnimationFrame(frame);
  });
  function openOnTargets(wrap: HTMLDivElement) {
    // The step's controls, edge to edge.
    const spans = drawn.flatMap((s) =>
      s.items.flatMap((it, i) =>
        targets.some((t) => t.section === s.id && t.item === i)
          ? [{ x0: s.x + it.x - halfWidth(it), x1: s.x + it.x + halfWidth(it) }]
          : [],
      ),
    );
    if (!spans.length) return;
    const room = wrap.clientWidth;
    const px = (x: number) => (x - view.box.x) * scale.k;
    const [first, last] = [Math.min(...spans.map((x) => x.x0)), Math.max(...spans.map((x) => x.x1))];
    wrap.scrollLeft = openingScroll(px(first), px(last), room);
  }

  let sheet = $state<HTMLDialogElement | null>(null);
  /**
   * Opens the larger view scrolled so the step's controls are in sight (the owner, batch 2). The
   * drawing takes its full size a frame or two after the sheet opens, so it centers again whenever
   * the drawing resizes in the first half second.
   */
  function openLarger() {
    const el = sheet;
    if (!el) return;
    el.showModal();
    const center = () => {
      const ons = [...el.querySelectorAll("g.item.on")].map((g) => g.getBoundingClientRect());
      if (!ons.length) return;
      const top = Math.min(...ons.map((r) => r.top));
      const bottom = Math.max(...ons.map((r) => r.bottom));
      const box = el.getBoundingClientRect();
      el.scrollTop += (top + bottom) / 2 - (box.top + box.height / 2);
    };
    requestAnimationFrame(center);
    const watch = new ResizeObserver(center);
    for (const svg of el.querySelectorAll("svg")) watch.observe(svg);
    setTimeout(() => watch.disconnect(), 500);
  }

  /** The open control card: the item, its light meaning, and what its control does. */
  let card = $derived.by(() => {
    if (!picked || !index) return null;
    const item = panel.sections.find((s) => s.id === picked?.section)?.items[picked.item];
    if (!item) return null;
    const meaning = item.kind === "light" ? index.lightMeaning(panel.id, picked.section, picked.item) : undefined;
    const does = (item.controls ?? []).map((c) => index?.control(c)?.function).find(Boolean);
    return { item, meaning, does };
  });

  let described = $derived(
    targets
      .map((t) => {
        const item = panel.sections.find((s) => s.id === t.section)?.items[t.item];
        const pos = t.position != null ? item?.positions?.split(",")[t.position]?.trim() : undefined;
        return item ? `${item.label}${pos ? ` set to ${pos}` : ""}` : null;
      })
      .filter(Boolean)
      .join("; "),
  );
  let title = $derived(focused.map((s) => s.title).join(" and "));
  /** The larger view's title: the pane card's caption ("Fuel panel"). */
  let caption = $derived(panelCaption(focused.map((s) => s.title)));
</script>

<div class="wrap" class:more={scale.overflow} bind:clientWidth={width} bind:this={wrap}>
  <svg
    class="schematic"
    viewBox="{view.box.x} {view.box.y} {view.box.w} {view.box.h}"
    style:width="{Math.floor(view.box.w * scale.k)}px"
    role="img"
    aria-label="{title} panel, schematic.{described ? ` This step: ${described}.` : ''}"
  >
    {#each drawn as s (s.id)}
      <PanelSection section={s} {targets} {guardClosed} band={bandOf(s)} frame={view.frames.get(s.id)} titled={view.titled} k={K} hit={index ? (44 * pageScale) / scale.k : 0} onPick={index ? (section, item) => (picked = { section, item }) : undefined} />
    {/each}
  </svg>
</div>
{#if card}
  <div class="control-card" role="region" aria-label="{card.item.label}">
    <div class="card-head">
      <b>{card.item.label}</b>
      <button type="button" class="card-close" aria-label="Close" onclick={() => (picked = null)}>×</button>
    </div>
    {#if card.item.description}<p>{card.item.description}</p>{/if}
    {#if card.meaning}<p><b>When lit</b> {card.meaning}</p>{/if}
    {#if card.does}<p><b>What it does</b> {card.does}</p>{/if}
  </div>
{/if}
{#if (scale.overflow || band) && !large}
  <button type="button" class="btn larger" onclick={openLarger}>See the whole panel larger</button>
  <dialog class="sheet" bind:this={sheet} aria-label={caption}>
    <div class="sheet-head">
      <span>{caption}</span>
      <button type="button" class="btn" onclick={() => sheet?.close()}>Close</button>
    </div>
    <Self {panel} {focus} {targets} {guardClosed} large />
  </dialog>
{/if}

<style>
  .wrap {
    width: 100%;
    overflow-x: auto;
  }
  /* A drawing wider than its card fades at its edges, so it is clear there is more. */
  .wrap.more {
    mask-image: linear-gradient(90deg, transparent, black var(--space-10), black calc(100% - var(--space-10)), transparent);
  }
  .schematic {
    display: block;
    height: auto;
    margin: 0 auto;
  }
  .control-card {
    margin-top: var(--space-2);
    padding: var(--space-2) var(--space-3);
    border: var(--border-thin) solid var(--panel-frame);
    border-radius: var(--radius-control);
    background: var(--panel-fill);
    color: var(--panel-text);
    font-size: var(--font-size-secondary);
  }
  .control-card p {
    margin: var(--space-1) 0 0;
  }
  .card-head {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .card-close {
    color: var(--panel-label);
    font-size: var(--font-size-panel-title);
  }
  .larger {
    margin-top: var(--space-2);
  }
  /* The full-width view: the whole window across, the drawing scrolling inside if needed. */
  .sheet {
    width: 100vw;
    max-width: 100vw;
    max-height: 100dvh;
    margin: auto 0 0;
    padding: var(--space-4) var(--gutter) var(--space-6);
    border: 0;
    border-top: var(--border-thin) solid var(--color-border-strong);
    border-radius: var(--radius-sheet) var(--radius-sheet) 0 0;
    background: var(--color-sheet);
    color: var(--color-text);
  }
  .sheet::backdrop {
    background: var(--color-scrim);
  }
  /* Stays in view (with Close) when the sheet opens scrolled to the step's controls. */
  .sheet-head {
    position: sticky;
    top: calc(-1 * var(--space-4));
    z-index: 1;
    padding: var(--space-2) 0;
    background: var(--color-sheet);
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-4);
    margin-bottom: var(--space-3);
    font-size: var(--font-size-panel-title);
    font-weight: var(--weight-semibold);
  }
</style>
