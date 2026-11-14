<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The slide-out (round 6, ROUND6.md; boards R6Panel, R6Where, R6Control, R6TabL; Locator.dc.html):
the step's panel, labeled at 11 px or more, and where it is. It fills the box its parent gives it
(beside the side pane, over the step list, below the flight strip). In order: a switcher when the
step's controls are on several panels; the header (the step, the panel's name, where it is, Esc
and close); The panel, with a navigator above it when it is wider than the slide-out (owner,
gate reviews); a tapped control's card under it; then Where it is (folded to one line, or
expanded: the plain-language line, the side and seat views and the area map). How it opens depends
on the entry point: the panel (Show panel, a preview, P), Where it is expanded (the location line,
W), or a control's card. A panel taller than the room keeps its height and the body scrolls as one
surface, opening with the step's controls in view; nothing inside scrolls up and down on its own.
-->
<script lang="ts">
  import type { ItemTarget, Step } from "@chocktochock/bindings";
  import { labelOf, lowerFirst, type Decisions } from "./decision";
  import type { PackIndex } from "../pack-index";
  import Icon from "./Icon.svelte";
  import type { Opening } from "./opening";
  import OverviewMap from "./OverviewMap.svelte";
  import PanelNavigator from "./PanelNavigator.svelte";
  import { positionsOf } from "./panel-layout";
  import PanelSchematic from "./PanelSchematic.svelte";
  import { panelCaption } from "./panel-name";
  import { stepTitle } from "./sentence";
  import ViewDrawing from "./ViewDrawing.svelte";
  import WholeDeck from "./WholeDeck.svelte";

  let {
    index,
    step,
    number,
    targets,
    opening,
    panel: chosen = $bindable(),
    onClose,
    onGo,
    decisions,
  }: {
    index: PackIndex;
    step: Step;
    /** The step's number in its list ("Step 7"). */
    number: number;
    targets: ItemTarget[];
    /** How it opened: which section shows first. */
    opening: Opening;
    /** The panel shown, by id, when the step's controls are on several. */
    panel?: string | undefined;
    onClose: () => void;
    /** Goes to a step (the control card's Used in links). */
    onGo: (stepId: string) => void;
    /** For a control a decision set: where its position came from, and Change (decision 0022). */
    decisions?: Decisions | undefined;
  } = $props();
  let dependent = $derived(decisions?.dependent(step) ?? null);

  // svelte-ignore state_referenced_locally
  let whereOpen = $state(opening.entry === "where");
  // svelte-ignore state_referenced_locally
  let picked = $state<{ section: string; item: number } | null>(opening.entry === "control" ? (opening.control ?? null) : null);
  let deck = $state(false);
  // svelte-ignore state_referenced_locally
  if (opening.panel && !chosen) chosen = opening.panel;

  let place = $derived(index.place(step));
  /** One entry per panel the step's controls are on, its own control's first. */
  let panels = $derived.by(() => {
    const ids = [...new Set([place?.section?.id, ...targets.map((t) => t.section)].filter((x): x is string => !!x))];
    const out = new Map<string, { panel: NonNullable<ReturnType<PackIndex["panelOf"]>>; focus: string[] }>();
    for (const id of ids) {
      const p = index.panelOf(id);
      if (!p) continue;
      const entry = out.get(p.id) ?? { panel: p, focus: [] };
      entry.focus.push(id);
      out.set(p.id, entry);
    }
    return [...out.values()];
  });
  let shown = $derived(panels.find((p) => p.panel.id === chosen) ?? panels[0]);
  let caption = $derived(shown ? panelCaption(shown.focus.map((id) => index.section(id)?.title ?? id)) : "");
  let zone = $derived(place?.zone);
  let where = $derived(
    zone ? [zone.area, zone.position ? zone.position.charAt(0).toLowerCase() + zone.position.slice(1) : null].filter(Boolean).join(" · ") : "",
  );
  let region = $derived(zone ? (index.region(zone)?.region ?? null) : null);
  /** The plain-language line, folded: its first sentence. */
  let locate = $derived(region?.locate ? region.locate.split(/(?<=\.)\s/)[0]?.replace(/\.$/, "") : "");
  /** The tapped control's card (round 6, R6Control). */
  let card = $derived.by(() => {
    if (!picked || !shown) return null;
    const { section, item } = picked;
    const it = index.section(section)?.items[item];
    if (!it) return null;
    const control = (it.controls ?? []).map((id) => index.control(id)).find(Boolean);
    const positions = positionsOf(it);
    // A switch reads top to bottom: its first position is up (decision 0021, the real direction).
    const ways = positions.length === 2 ? ["up", "down"] : positions.length === 3 ? ["up", "center", "down"] : [];
    const across = it.kind === "toggle_switch" && ways.length ? positions.map((p, n) => `${p} (${ways[n]})`) : positions;
    const target = targets.find((t) => t.section === section && t.item === item);
    const setTo = target && target.position != null ? positions[target.position] : null;
    return {
      title: control ? `${it.label} · ${control.name.charAt(0).toLowerCase()}${control.name.slice(1)}` : it.label,
      where: [index.section(section)?.title ? panelCaption([index.section(section)?.title ?? ""]) : null, zone?.area].filter(Boolean).join(" · "),
      positions: across.join(" · "),
      inStep: target ? (setTo ? `${setTo}${target.resolved ? " ◆" : ""}` : target.lit ? "Expected on" : target.open ? "Only if the step's condition applies" : "Used by this step") : null,
      does: it.description || control?.function || null,
      lit: it.kind === "light" ? index.lightMeaning(shown.panel.id, section, item) : undefined,
      usedIn: index.usedIn(shown.panel.id, section, item),
      // "OFF ◆ · from your center tank answer, 1,000 lb (450 kg) or less  Change" (R6Control).
      from:
        target?.resolved && dependent?.answer != null
          ? `from your ${lowerFirst(dependent.option.title)} answer, ${labelOf(dependent.option, dependent.answer)}`
          : null,
    };
  });
  /** The decision a resolved control's position came from, for the legend's ◆ entry. */
  let decided = $derived.by(() => {
    if (!targets.some((t) => t.resolved)) return null;
    const option = (step.targets ?? []).find((t) => t.when)?.when?.option;
    return index.options.find((o) => o.id === option)?.title ?? null;
  });

  // The room the labeled panel has: the panel section's size, less the legend, the navigator and
  // the figure's own padding and border.
  let sectionWidth = $state(0);
  let sectionHeight = $state(0);
  let legendHeight = $state(0);
  let navHeight = $state(0);
  let rowHeight = $state(0);
  /** The Where it is row's height (folded, or its heading when expanded): the room leaves it. */
  let foldHeight = $state(0);
  /** The body's own padding, top and bottom. */
  let bodyPad = $state(0);
  let wider = $state(false);
  let scroller = $state<HTMLDivElement | null>(null);
  let figure = $state<HTMLElement | null>(null);
  /** The figure's padding and border on each axis, both sides. */
  const edge = (axis: "x" | "y") => {
    if (!figure) return 0;
    const css = getComputedStyle(figure);
    const n = (k: string) => Number.parseFloat(css.getPropertyValue(k)) || 0;
    return axis === "x"
      ? n("padding-left") + n("padding-right") + n("border-left-width") + n("border-right-width")
      : n("padding-top") + n("padding-bottom") + n("border-top-width") + n("border-bottom-width");
  };
  /** The gap between the navigator, the drawing and the legend (space-1, space-2). */
  let gap = $state(0);
  let roomGap = $state(0);
  let roomEl = $state<HTMLElement | null>(null);
  /** The body's side padding, and the gap inside the panel section. */
  let bodySide = $state(0);
  let panelGap = $state(0);
  $effect(() => {
    if (figure) gap = Number.parseFloat(getComputedStyle(figure).rowGap) || 0;
    if (roomEl) {
      const css = getComputedStyle(roomEl);
      roomGap = Number.parseFloat(css.rowGap) || 0;
      bodyPad = (Number.parseFloat(css.paddingTop) || 0) + (Number.parseFloat(css.paddingBottom) || 0);
      bodySide = (Number.parseFloat(css.paddingLeft) || 0) + (Number.parseFloat(css.paddingRight) || 0);
      const panel = roomEl.querySelector(".panel");
      panelGap = panel ? Number.parseFloat(getComputedStyle(panel).rowGap) || 0 : 0;
    }
  });
  let room = $derived(
    sectionWidth > 0 && sectionHeight > 0
      ? {
          width: sectionWidth - bodySide - edge("x"),
          height: Math.max(
            0,
            sectionHeight - bodyPad - rowHeight - foldHeight - roomGap - panelGap * (wider ? 2 : 1) - (wider ? navHeight : 0) - legendHeight - edge("y") - gap,
          ),
        }
      : undefined,
  );

  /** The body scrolls (a panel taller than the room): fades where there is more, and it opens
   * with the step's controls in view. */
  let more = $state({ up: false, down: false });
  function edges() {
    const el = roomEl;
    if (el) more = { up: el.scrollTop > 1, down: el.scrollTop + el.clientHeight < el.scrollHeight - 1 };
  }
  $effect(() => {
    const el = roomEl;
    void room;
    void shown;
    if (!el) return;
    const frame = requestAnimationFrame(() => {
      // Opened from Where is this?: Where it is in view.
      const whereAt = opening.entry === "where" ? el.querySelector(".where-it-is") : null;
      if (whereAt) {
        el.scrollTop += whereAt.getBoundingClientRect().top - el.getBoundingClientRect().top - Number.parseFloat(getComputedStyle(el).paddingTop || "0");
        edges();
        return;
      }
      // Opened on a control: its card in view, under the ringed control.
      const cardAt = opening.entry === "control" ? el.querySelector(".card") : null;
      if (cardAt) {
        const box = el.getBoundingClientRect();
        const r = cardAt.getBoundingClientRect();
        el.scrollTop += Math.max(0, r.bottom - box.bottom + Number.parseFloat(getComputedStyle(el).paddingBottom || "0"));
        edges();
        return;
      }
      const on = [...el.querySelectorAll("g.item.on")].map((g) => g.getBoundingClientRect());
      if (on.length) {
        const box = el.getBoundingClientRect();
        const top = Math.min(...on.map((r) => r.top)) - box.top + el.scrollTop;
        const bottom = Math.max(...on.map((r) => r.bottom)) - box.top + el.scrollTop;
        el.scrollTop = Math.max(0, (top + bottom) / 2 - el.clientHeight / 2);
      }
      edges();
    });
    return () => cancelAnimationFrame(frame);
  });

  // A control tapped in the slide-out: its card into view.
  let cardEl = $state<HTMLElement | null>(null);
  $effect(() => {
    if (picked && cardEl) cardEl.scrollIntoView?.({ block: "nearest" });
  });

  let head = $state<HTMLElement | null>(null);
  $effect(() => {
    head?.focus();
  });
  function key(e: KeyboardEvent) {
    if (e.key === "Escape") {
      e.preventDefault();
      onClose();
    }
  }
</script>

<div class="slideout" role="dialog" aria-labelledby="so-title" tabindex="-1" onkeydown={key}>
  {#if panels.length > 1}
    <div class="switcher" role="tablist" aria-label="The step's panels">
      {#each panels as p (p.panel.id)}
        {@const name = panelCaption(p.focus.map((id) => index.section(id)?.title ?? id))}
        <button type="button" role="tab" aria-selected={p === shown} class:on={p === shown} onclick={() => (chosen = p.panel.id)}>{name}</button>
      {/each}
    </div>
  {/if}
  <header class="head">
    <div class="titles">
      <span class="step">Step {number} · {stepTitle(index, step)}</span>
      <h2 id="so-title" tabindex="-1" bind:this={head}>{caption}</h2>
      {#if where}<span class="where">{where}</span>{/if}
    </div>
    <kbd class="esc" aria-hidden="true">Esc</kbd>
    <button class="close" type="button" aria-label="Close" onclick={onClose}><Icon name="x" /></button>
  </header>
  <div class="body" class:more-up={more.up} class:more-down={more.down} bind:clientWidth={sectionWidth} bind:clientHeight={sectionHeight} bind:this={roomEl} onscroll={edges}>
    <section class="panel" aria-labelledby="so-panel">
      <div class="row" bind:clientHeight={rowHeight}>
        <h3 id="so-panel">The panel</h3>
        <button class="deck-toggle" type="button" aria-expanded={deck} onclick={() => (deck = !deck)}>Whole flight deck <Icon name="chevron-down" /></button>
      </div>
      {#if deck && zone}<WholeDeck {index} {zone} />{/if}
      {#if wider && scroller && shown}
        <div class="nav" bind:clientHeight={navHeight}>
          <PanelNavigator panel={shown.panel} focus={shown.focus} {targets} {scroller} title={caption} />
        </div>
      {/if}
      {#if shown}
        <figure class="figure" bind:this={figure}>
          <PanelSchematic panel={shown.panel} focus={shown.focus} {targets} {index} large ownCard={false} {room} bind:picked bind:wider bind:scroller />
          <figcaption bind:clientHeight={legendHeight}>
            Blue: set by this step{#if targets.some((t) => t.open)}{" "}· dashed: only if the step's condition applies{/if}{#if decided}{" "}· ◆ set by your {decided.toLowerCase()} answer{/if} · lights drawn unlit · tap a control for details
          </figcaption>
        </figure>
      {/if}
    </section>
    {#if card}
      <section class="card" aria-labelledby="so-card" bind:this={cardEl}>
        <div class="card-head">
          <div class="titles">
            <h3 id="so-card">{card.title}</h3>
            <span class="sub">{card.where}</span>
          </div>
          <button class="close small" type="button" aria-label="Close control details" onclick={() => (picked = null)}><Icon name="x" /></button>
        </div>
        <dl>
          {#if card.positions}<dt>Positions</dt><dd class="mono">{card.positions}</dd>{/if}
          {#if card.inStep}
            <dt>In this step</dt>
            {#if card.from && dependent}
              {@const ask = dependent.decisionStep}
              <dd><span class="mono">{card.inStep}</span> · {card.from} <button class="lk inline" type="button" onclick={() => decisions?.change(ask)}>Change</button></dd>
            {:else}
              <dd class="mono">{card.inStep}</dd>
            {/if}
          {/if}
          {#if card.lit}<dt>When lit</dt><dd>{card.lit}</dd>{/if}
          {#if card.does}<dt>What it does</dt><dd>{card.does}</dd>{/if}
          {#if card.usedIn.length}
            <dt>Used in</dt>
            <dd class="links">
              {#each card.usedIn as u (u.step.id)}
                <button class="lk" type="button" onclick={() => onGo(u.step.id)}>{stepTitle(index, u.step)} · {u.phase}</button>
              {/each}
            </dd>
          {/if}
        </dl>
      </section>
    {/if}
    {#if whereOpen}
      <section class="where-it-is" aria-labelledby="so-where">
        <button id="so-where" class="where-head" type="button" aria-expanded="true" bind:clientHeight={foldHeight} onclick={() => (whereOpen = false)}>
          <span>Where it is</span>
          <span class="up"><Icon name="chevron-down" /></span>
        </button>
        {#if region?.locate}<p class="locate">{region.locate}</p>{/if}
        <div class="views">
          {#if index.overview?.views?.side}
            <ViewDrawing frame={index.overview.views.side} region={region?.name ?? null} kind="side" title="From the side: how high, how far" />
          {/if}
          {#if index.overview?.views?.seat}
            <ViewDrawing frame={index.overview.views.seat} region={region?.name ?? null} kind="seat" title="From the captain seat: which side" />
          {/if}
          {#if zone}
            <figure class="area-map">
              <figcaption>On the {(region?.name ?? zone.area).toLowerCase()}</figcaption>
              <OverviewMap {index} {zone} compact />
            </figure>
          {/if}
        </div>
        <div class="key" aria-hidden="true">
          <span><i class="sw here"></i>This area</span>
          <span><i class="sw other"></i>Other panels</span>
        </div>
      </section>
    {:else}
      <button class="fold" type="button" aria-expanded="false" bind:clientHeight={foldHeight} onclick={() => (whereOpen = true)}>
        <strong>Where it is</strong>
        <span>{locate}</span>
        <Icon name="chevron-down" />
      </button>
    {/if}
  </div>
</div>

<style>
  /* Locator.dc.html: the pop-over tokens; square except the top left corner. */
  .slideout {
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    overflow: hidden;
    border: var(--border-thin) solid var(--color-popover-border);
    border-radius: var(--radius-popover) 0 0 0;
    background: var(--color-popover-surface);
    box-shadow: var(--color-overlay-shadow);
    color: var(--color-text);
  }
  .switcher {
    display: flex;
    gap: var(--space-2);
    padding: var(--space-2) var(--space-6) 0;
  }
  .switcher button {
    min-height: var(--size-touch-min);
    padding: 0 var(--space-3);
    border: var(--border-thin) solid var(--color-border);
    border-radius: var(--radius-pill);
    background: none;
    color: var(--color-text-secondary);
    font: inherit;
    font-size: var(--font-size-secondary);
  }
  .switcher button.on {
    border-color: var(--color-accent);
    color: var(--color-accent-text);
  }
  .head {
    flex: none;
    display: flex;
    align-items: flex-start;
    gap: var(--space-2);
    padding: var(--space-4) var(--space-4) var(--space-3) var(--space-6);
    border-bottom: var(--border-thin) solid var(--color-line);
  }
  .titles {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: var(--border-thick);
  }
  .step {
    color: var(--color-link);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-semibold);
  }
  h2 {
    margin: 0;
    font-size: var(--font-size-panel-title);
    font-weight: var(--weight-bold);
    outline: none;
  }
  .where {
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
  }
  .esc {
    align-self: center;
    padding: 0 var(--space-1);
    border: var(--border-thin) solid var(--color-popover-border);
    border-radius: var(--radius-small);
    color: var(--color-text-muted);
    font: inherit;
    font-size: var(--font-size-caption);
  }
  .close {
    display: flex;
    align-items: center;
    justify-content: center;
    width: var(--size-touch-min);
    height: var(--size-touch-min);
    margin-top: calc(-1 * var(--space-2));
    border: 0;
    background: none;
    color: var(--color-text-secondary);
  }
  .body {
    flex: 1;
    min-height: 0;
    overflow-y: auto;
    scrollbar-width: none;
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    padding: var(--space-4) var(--space-6);
  }
  .body::-webkit-scrollbar {
    display: none;
  }
  .body.more-up {
    mask-image: linear-gradient(180deg, transparent, black var(--space-10));
  }
  .body.more-down {
    mask-image: linear-gradient(180deg, black calc(100% - var(--space-10)), transparent);
  }
  .body.more-up.more-down {
    mask-image: linear-gradient(180deg, transparent, black var(--space-10), black calc(100% - var(--space-10)), transparent);
  }
  .panel {
    flex: none;
    min-height: 0;
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }
  .row {
    display: flex;
    align-items: center;
    min-height: var(--size-touch-min);
  }
  h3 {
    margin: 0;
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
  }
  .figure {
    margin: 0;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    padding: var(--space-2) var(--space-3);
    border: var(--border-thin) solid var(--panel-frame);
    border-radius: var(--radius-control);
    background: var(--panel-bg);
  }
  figcaption {
    color: var(--panel-label);
    font-size: var(--font-size-caption);
    line-height: var(--leading-snug);
  }
  .fold {
    flex: none;
    display: flex;
    align-items: center;
    gap: var(--space-3);
    min-height: var(--size-touch-min);
    padding: 0 var(--space-4);
    border: var(--border-thin) solid var(--color-popover-border);
    border-radius: var(--radius-control);
    background: var(--color-surface-2);
    color: var(--color-text-secondary);
    font: inherit;
    font-size: var(--font-size-secondary);
    text-align: left;
  }
  .fold strong {
    color: var(--color-text);
    font-weight: var(--weight-semibold);
  }
  .fold span {
    flex: 1;
    min-width: 0;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }
  .row h3 {
    flex: 1;
  }
  .deck-toggle {
    display: inline-flex;
    align-items: center;
    gap: var(--space-1);
    min-height: var(--size-touch-min);
    border: 0;
    background: none;
    color: var(--color-link);
    font: inherit;
    font-size: var(--font-size-secondary);
  }
  .deck-toggle[aria-expanded="true"] :global(.icon) {
    transform: rotate(180deg);
  }
  /* The control card (R6Control). */
  .card {
    flex: none;
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    padding: var(--space-3) var(--space-2) var(--space-3) var(--space-4);
    border: var(--border-thin) solid var(--color-popover-border);
    border-radius: var(--radius-control);
    background: var(--color-surface-2);
  }
  .card-head {
    display: flex;
    align-items: flex-start;
    gap: var(--space-2);
  }
  .card h3 {
    margin: 0;
    font-size: var(--font-size-body);
    font-weight: var(--weight-semibold);
  }
  .sub {
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
  }
  .close.small {
    margin-top: calc(-1 * var(--space-2));
  }
  dl {
    display: grid;
    grid-template-columns: max-content minmax(0, 1fr);
    gap: var(--space-1) var(--space-4);
    margin: 0;
    font-size: var(--font-size-secondary);
    line-height: var(--leading-snug);
  }
  dt {
    color: var(--color-text-muted);
  }
  dd {
    margin: 0;
  }
  .mono {
    font-family: var(--font-mono);
    font-size: var(--font-size-value-mono);
  }
  .links {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-1) var(--space-2);
  }
  .lk.inline {
    display: inline;
    min-height: 0;
    padding: 0;
    font: inherit;
  }
  .lk {
    display: inline-flex;
    align-items: center;
    min-height: var(--size-touch-min);
    border: 0;
    background: none;
    color: var(--color-link);
    font: inherit;
    text-align: left;
  }
  /* Where it is, expanded (R6Where). */
  .where-it-is {
    flex: none;
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
  }
  .where-head {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    min-height: var(--size-touch-min);
    margin: 0 calc(-1 * var(--space-2));
    padding: 0 var(--space-2);
    border: 0;
    border-radius: var(--radius-small);
    background: none;
    color: var(--color-text);
    font: inherit;
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
    text-align: left;
  }
  .where-head span:first-child {
    flex: 1;
  }
  .up {
    display: inline-flex;
    color: var(--color-text-secondary);
    transform: rotate(180deg);
  }
  .locate {
    margin: 0;
    font-size: var(--font-size-body);
    line-height: var(--leading-normal);
  }
  .views {
    display: grid;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    gap: var(--space-3);
  }
  .area-map {
    margin: 0;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
    padding: var(--space-2) var(--space-3);
    border: var(--border-thin) solid var(--panel-frame);
    border-radius: var(--radius-control);
    background: var(--panel-bg);
  }
  .area-map figcaption {
    font-weight: var(--weight-semibold);
  }
  .key {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--space-1) var(--space-6);
    color: var(--color-text-secondary);
    font-size: var(--font-size-caption);
  }
  .key span {
    display: flex;
    align-items: center;
    gap: var(--space-2);
  }
  .sw {
    display: inline-block;
    width: var(--space-4);
    height: calc(var(--space-2) + var(--border-thick));
    border-radius: var(--border-thick);
  }
  .sw.here {
    border: var(--border-thick) solid var(--panel-changed);
    background: var(--panel-changed-fill);
  }
  .sw.other {
    border: var(--border-thin) solid var(--panel-stroke);
    background: var(--panel-fill);
  }
</style>
