<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

A panel drawing for one step, laid out like 3.0's panel cards. It draws the panel sections that
hold the step's controls, in the pack's own panel layout, with their neighbors in the same rows
for context (so two small sections side by side, such as the wiper selectors, share one drawing).
A tall single section is cut down to the rows around the step's controls. Controls scale to fill
the card; text keeps 3.0's sizes.
-->
<script lang="ts">
  import type { ItemTarget, Panel, PanelSection as Section } from "@chocktochock/bindings";
  import { layoutSection, PX, positionsOf, withinAcross } from "./panel-layout";
  import { em } from "./text-width";
  import PanelSection from "./PanelSection.svelte";

  let {
    panel,
    focus,
    targets,
    guardClosed = false,
  }: {
    /** The panel the sections are on. */
    panel: Panel;
    /** Ids of the sections that hold the step's controls. */
    focus: string[];
    /** The step's targets (from Rust). */
    targets: ItemTarget[];
    /** The step asks for the guard closed. */
    guardClosed?: boolean;
  } = $props();

  /**
   * Every drawing is drawn at one scale, pixels per unit, so every control and every label is the
   * same size in every panel, with text at 3.0's sizes. What does not fit the card is split into
   * crops, one under the other, and a crop still too wide scrolls sideways inside the card.
   */
  const K = 1.8;
  /** Units across a crop shows before the card is measured. */
  const SPAN = 240;
  /** How far beside the step's sections a neighbor may sit and still be drawn. */
  const NEAR = 120;

  let focused = $derived(panel.sections.filter((s) => focus.includes(s.id)));
  // A small group of sections brings its neighbors in the same rows; a tall section stands alone.
  let drawn = $derived.by((): Section[] => {
    if (focused.length === 1 && (focused[0]?.height ?? 0) > 200) return focused;
    const x0 = Math.min(...focused.map((s) => s.x)) - NEAR;
    const x1 = Math.max(...focused.map((s) => s.x + s.width)) + NEAR;
    const y0 = Math.min(...focused.map((s) => s.y));
    const y1 = Math.max(...focused.map((s) => s.y + s.height));
    return panel.sections.filter(
      (s) => focus.includes(s.id) || (s.x >= x0 && s.x + s.width <= x1 && s.y < y1 && s.y + s.height > y0 && s.height <= 200),
    );
  });

  /** A section taller than this is cut down to the controls within REACH of the step's, as 3.0 does. */
  const TALL = 280;
  const REACH = 100;

  type Band = { y0: number; y1: number; x0: number; x1: number };
  const wholeOf = (s: Section): Band => ({ y0: 0, y1: s.height, x0: 0, x1: s.width });

  let width = $state(0);
  /**
   * How many units across the card can show with text at 3.0's size, as 3.0 narrows its crops so
   * no label is drawn small: a wider section is cut to that, and the step's controls spread
   * further apart are drawn in more than one crop, one under the other. 240 until measured.
   */
  let room = $derived(width > 0 ? Math.max(160, width / K - 6) : SPAN);

  /** The rows of a tall section kept around some of its controls. */
  function rowsAround(s: Section, ys: number[]): Partial<Band> {
    if (s.height <= TALL || !ys.length) return {};
    const c = (Math.min(...ys) + Math.max(...ys)) / 2;
    const reach = Math.max(REACH, (Math.max(...ys) - Math.min(...ys)) / 2 + 30);
    const kept = s.items.map((it) => it.y).filter((y) => Math.abs(y - c) <= reach);
    // Room above the kept controls for their shapes and a line of labels over them (a dial's top
    // position); below, the section grows to hold its labels.
    return { y0: Math.max(0, Math.min(...kept) - 34), y1: Math.min(s.height, Math.max(...kept) + 20) };
  }

  /**
   * The crops to draw, each a band for every drawn section. Sections drawn together show whole,
   * and so does a section drawn alone that is at most a tenth wider than the card can show (it
   * scrolls sideways to the step's controls). A wider one is divided across into parts that do not
   * overlap, and the parts that hold the step's controls are drawn one under the other.
   */
  /**
   * Sections drawn together. The step's sections far apart on the panel (the two EFIS panels at
   * either end of the glareshield) are drawn each on its own, one under the other.
   */
  let groups = $derived.by((): Section[][] => {
    if (focused.length < 2) return [drawn];
    const spread = Math.max(...focused.map((s) => s.x + s.width)) - Math.min(...focused.map((s) => s.x));
    return spread > room * 1.5 ? focused.map((s) => [s]) : [drawn];
  });

  let crops = $derived(groups.flatMap((g) => cropsOf(g).map((bands) => ({ group: g, bands }))));

  function cropsOf(group: Section[]): Map<string, Band>[] {
    const only = group.length === 1 ? group[0] : undefined;
    const mine = only ? only.items.filter((_, i) => targets.some((t) => t.section === only.id && t.item === i)) : [];
    if (!only || !mine.length) return [new Map(group.map((s) => [s.id, wholeOf(s)]))];
    const rows = (ys: number[]) => ({ ...wholeOf(only), ...rowsAround(only, ys) });
    if (only.width <= room * 1.1) return [new Map([[only.id, rows(mine.map((it) => it.y))]])];
    const parts = Math.ceil(only.width / room);
    const step = only.width / parts;
    const out: { step: boolean; band: Map<string, Band> }[] = [];
    for (let n = 0; n < parts; n++) {
      const [x0, x1] = [n * step, (n + 1) * step];
      // Every part is drawn, so the whole section is there as on the panel; a part with none of
      // the step's controls keeps the rows the step's controls are in.
      const here = mine.filter((it) => it.x >= x0 && it.x < x1);
      const ys = (here.length ? here : mine).map((it) => it.y);
      out.push({ step: here.length > 0, band: new Map([[only.id, { ...rows(ys), x0, x1 }]]) });
    }
    // The parts with the step's controls come first, so they are in first view.
    return [...out.filter((p) => p.step), ...out.filter((p) => !p.step)].map((p) => p.band);
  }

  /** How a section's labels lay out in a band at `kText` pixels per unit, title at `titleTop`. */
  function layoutFor(s: Section, band: Band, kText: number, titleTop: number) {
    const shown = s.items.flatMap((item, i) =>
      item.y >= band.y0 + 10 && item.y <= band.y1 - 10 && withinAcross(item, band) ? [i] : [],
    );
    const on = (i: number) => targets.find((t) => t.section === s.id && t.item === i);
    return layoutSection(
      s,
      shown,
      (i) => {
        const item = s.items[i] as (typeof s.items)[number];
        return {
          on: !!on(i),
          t: on(i)?.position ?? null,
          rest: !on(i) && item.rest ? positionsOf(item).indexOf(item.rest) : -1,
          lit: on(i)?.lit ?? false,
        };
      },
      kText,
      titleTop,
      // Labels may reach past a crop's edge; the frame fits them, so check them against the section.
      { x0: 0, x1: s.width },
    );
  }

  const topOf = (s: Section, band: Band) => s.y + Math.max(0, band.y0 > 0 ? band.y0 - 18 : 0);

  /**
   * One crop's frames, box and the scale that fits the card. A section drawn alone is framed to
   * what it shows (its controls and labels, with room for its title), so the drawing fills the
   * card instead of the section's empty corners.
   */
  function sizeAt(group: Section[], bands: Map<string, Band>, kText: number, titled = true) {
    const frames = new Map<string, Band | undefined>();
    let box: { x: number; y: number; w: number; h: number };
    const only = group.length === 1 ? group[0] : undefined;
    if (only) {
      const band = bands.get(only.id) ?? wholeOf(only);
      const e = layoutFor(only, band, kText, -1000).extent;
      // Fitted to what is drawn, even where a control at the crop's edge reaches past it.
      // A part of a divided section spans its whole band, so the parts line up one under another.
      const part = band.x0 > 0 || band.x1 < only.width;
      const x0 = part ? Math.min(band.x0, e.x0 - 8) : e.x0 - 8;
      const titleW = titled ? 16 + em(only.title.toUpperCase()) * (PX.title / kText) : 0;
      const x1 = Math.max(part ? Math.max(band.x1, e.x1 + 8) : e.x1 + 8, x0 + titleW);
      // Equal margins on every side; the title, when drawn, sits in the top one.
      const f = { x0, x1, y0: e.y0 - (titled ? 22 : 8), y1: e.y1 + 8 };
      frames.set(only.id, f);
      box = { x: only.x + f.x0 - 2, y: only.y + f.y0 - 2, w: f.x1 - f.x0 + 4, h: f.y1 - f.y0 + 4 };
    } else {
      const band = (s: Section) => bands.get(s.id) ?? wholeOf(s);
      const x = Math.min(...group.map((s) => s.x + band(s).x0)) - 2;
      const y = Math.min(...group.map((s) => topOf(s, band(s)))) - 2;
      const right = Math.max(...group.map((s) => s.x + band(s).x1)) + 2;
      const low = Math.max(
        ...group.map((s) => s.y + Math.max(band(s).y1, layoutFor(s, band(s), kText, topOf(s, band(s)) - s.y).bottom)),
      );
      box = { x, y, w: right - x, h: low + 2 - y };
    }
    return { bands, frames, box, k: K };
  }
  // Only the first part of a divided section carries its title; the rest continue it.
  let views = $derived.by(() => {
    const vs = crops.map((c, n) => {
      // The card's caption names a lone section, so its frame needs no title of its own; sections
      // drawn apart keep theirs, on their first part, so they can be told apart.
      const titled = (groups.length > 1 || c.group.length > 1) && (n === 0 || crops[n - 1]?.group !== c.group);
      return { ...sizeAt(c.group, c.bands, K, titled), group: c.group, titled };
    });
    // The parts of a divided section share one width, so they line up one under another.
    for (const g of new Set(vs.map((v) => v.group))) {
      const parts = vs.filter((v) => v.group === g && g.length === 1);
      if (parts.length < 2) continue;
      const wide = Math.max(...parts.map((v) => v.box.w));
      for (const v of parts) {
        const id = g[0]?.id ?? "";
        const f = v.frames.get(id);
        const grow = wide - v.box.w;
        if (f) v.frames.set(id, { ...f, x0: f.x0 - grow / 2, x1: f.x1 + grow / 2 });
        v.box = { ...v.box, x: v.box.x - grow / 2, w: wide };
      }
    }
    return vs;
  });

  /** A drawing wider than the card starts scrolled to the step's controls. */
  let wrap = $state<HTMLDivElement | null>(null);
  $effect(() => {
    const v = views[0];
    const only = v && v.group.length === 1 ? v.group[0] : undefined;
    if (!wrap || !only || !v || wrap.scrollWidth <= wrap.clientWidth) return;
    const xs = only.items.filter((_, i) => targets.some((t) => t.section === only.id && t.item === i)).map((it) => it.x);
    if (!xs.length) return;
    const mid = only.x + (Math.min(...xs) + Math.max(...xs)) / 2;
    wrap.scrollLeft = (mid - v.box.x) * v.k - wrap.clientWidth / 2;
  });
  const kText = K;

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
</script>

<div class="wrap" bind:clientWidth={width} bind:this={wrap}>
  {#each views as v, n (n)}
    <svg
      class="schematic"
      viewBox="{v.box.x} {v.box.y} {v.box.w} {v.box.h}"
      style:width="{v.box.w * v.k}px"
      role="img"
      aria-label="{title} panel, schematic{views.length > 1 ? `, part ${n + 1} of ${views.length}` : ''}.{described ? ` This step: ${described}.` : ''}"
    >
      {#each v.group as s (s.id)}
        <PanelSection section={s} {targets} {guardClosed} band={v.bands.get(s.id) ?? wholeOf(s)} frame={v.frames.get(s.id)} titled={v.titled} k={kText} />
      {/each}
    </svg>
  {/each}
</div>

<style>
  .wrap {
    width: 100%;
    overflow-x: auto;
  }
  .schematic {
    display: block;
    height: auto;
    margin: 0 auto;
  }
  /* A drawing split in two: the second part starts below a rule, as in 3.0. */
  .schematic + .schematic {
    margin-top: 0.5rem;
    padding-top: 0.5rem;
    border-top: 1px dashed var(--line);
  }
</style>
