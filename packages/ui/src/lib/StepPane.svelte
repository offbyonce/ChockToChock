<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The side pane for the selected step, as the Checklist board draws it (decision 0021): the step's
place and value, Check, If not, Why with How to do it and Terms, the map figure (on a new panel,
per Settings › Map in steps) or the one-line location, then the controls. Check, If not and Why
are never covered: Where is this? opens leftward, over the list. Approved additions to the board:
a Say line in the Check box, a caution note under If not, and the control card (tap a control).
A step with no control on the map shows the steps coming up below its details.
-->
<script lang="ts">
  import { sentence as toSentence } from "./sentence";
  import type { ItemTarget, MapInSteps, Step, StepState } from "@chocktochock/bindings";
  import type { PackIndex } from "../pack-index";
  import Icon from "./Icon.svelte";
  import Locator from "./Locator.svelte";
  import OverviewMap from "./OverviewMap.svelte";
  import PanelSchematic from "./PanelSchematic.svelte";
  import { panelCaption } from "./panel-name";

  let {
    index,
    step,
    targets,
    number,
    total,
    previous,
    upcoming = [],
    mapMode,
    onMap,
    flow = false,
  }: {
    index: PackIndex;
    step: Step;
    targets: ItemTarget[];
    /** The step's place in its list, as shown: "Step 7 of 10". */
    number: number;
    total: number;
    /** The step before it in the list, to tell a new panel. */
    previous?: Step | undefined;
    upcoming?: { step: Step; state: StepState }[];
    mapMode: MapInSteps;
    /** Hide map and Show map change Settings › Map in steps. */
    onMap: (mode: "hide" | "show") => void;
    /** Flow view's pane (Flow board): the location line and the controls only; Flow shows the
     * step's details itself. */
    flow?: boolean;
  } = $props();

  const sentence = (text: string) => toSentence(index, text);

  let place = $derived(index.place(step));
  let before = $derived(previous ? index.place(previous) : null);
  let newPanel = $derived(!!place && before?.zone.id !== place.zone.id);
  /** Show map in New panel mode on a step whose panel did not change: for this step only. */
  let forced = $state(false);
  /** Desktop (1440 and up) shows the map figure; tablet landscape keeps the location line and
   * the map lives in Where is this? (rules.md §6, TabL board). */
  let desktop = $state(true);
  $effect(() => {
    const query = window.matchMedia?.("(min-width: 1440px)");
    if (!query) return;
    const read = () => {
      desktop = query.matches;
    };
    read();
    query.addEventListener("change", read);
    return () => query.removeEventListener("change", read);
  });
  let wantMap = $derived(
    !flow && desktop && !!place && (mapMode === "always" || (mapMode === "new_panel" && (newPanel || forced))),
  );
  /**
   * A controls drawing that cannot fit its card at 11 px takes the map figure's height: the pane
   * shows the one-line location instead (MapHidden board's line), and Show map brings the figure
   * back for this step on request (the owner, batch 2). Once squeezed, a step stays so.
   */
  let fits = $state<Record<string, boolean | undefined>>({});
  let squeezed = $state(false);
  let asked = $state(false);
  $effect(() => {
    if (wantMap && !asked && Object.values(fits).some((f) => f === false)) squeezed = true;
  });
  let showMap = $derived(wantMap && (!squeezed || asked));

  let how = $derived(step.how ? index.how(step.how) : undefined);
  let explain = $derived(step.explain ? index.explain(step.explain) : undefined);
  let check = $derived(step.check ?? explain?.check ?? null);
  let checkMore = $derived([step.check ? step.check_detail : explain?.check_detail, explain?.success].filter(Boolean) as string[]);
  let ifNot = $derived(step.if_not ?? explain?.if_not ?? null);
  let ifNotMore = $derived([step.if_not ? step.if_not_detail : explain?.if_not_detail, explain?.failure].filter(Boolean) as string[]);
  let why = $derived(step.why ?? explain?.why ?? null);
  let caution = $derived(step.note && step.note_caution ? step.note : null);
  let background = $derived(step.note && !step.note_caution ? step.note : null);
  let terms = $derived((step.terms ?? []).map((t) => ({ term: t, meaning: index.glossary(t) ?? "" })));

  let checkOpen = $state(false);
  let ifNotOpen = $state(false);
  let howOpen = $state(false);
  let termsOpen = $state(false);

  // One drawing per panel, starting with the control's own section; up to three, as 3.0 shows.
  let cards = $derived.by(() => {
    const ids = [...new Set([place?.section?.id, ...targets.map((t) => t.section)].filter((x): x is string => !!x))];
    const byPanel = new Map<string, { panel: NonNullable<ReturnType<PackIndex["panelOf"]>>; focus: string[] }>();
    for (const id of ids) {
      const panel = index.panelOf(id);
      if (!panel) continue;
      const card = byPanel.get(panel.id) ?? { panel, focus: [] };
      card.focus.push(id);
      byPanel.set(panel.id, card);
    }
    return [...byPanel.values()].slice(0, 3);
  });
  /** The step's targets drawn in this card: its legend names dashed and ◆ only when they appear. */
  const inCard = (card: { panel: { sections: { id: string }[] } }) =>
    targets.filter((t) => card.panel.sections.some((s) => s.id === t.section));
  let picked = $state<Record<string, { section: string; item: number } | null>>({});
  /** Each drawing, and whether it has a larger view, for the caption row's link. */
  let drawing = $state<Record<string, { openLarger: () => void } | undefined>>({});
  let enlarge = $state<Record<string, boolean>>({});
  /** The step's own control in the first drawing, for How to do it's link to its card. */
  let own = $derived.by(() => {
    const id = place?.control?.id;
    if (!id) return null;
    for (const t of targets) {
      const item = index.section(t.section)?.items[t.item];
      if (item?.controls?.includes(id)) return t;
    }
    return null;
  });
  const openOwn = () => {
    const card = cards.find((c) => own && c.focus.includes(own.section));
    if (card && own) picked[card.panel.id] = { section: own.section, item: own.item };
  };
  let guardClosed = $derived(/\bGUARDS? CLOSED\b/i.test(step.expect));
  let where = $derived(place ? `${place.zone.area}` : "");
</script>

<div class="pane-step">
  {#if !flow}
  <div class="head">
    <span class="pos">Step {number} of {total} · {sentence(step.text)}</span>
    <span class="value">{step.expect}</span>
  </div>

  {#if check}
    <div class="box check">
      <div class="txt">
        <span class="cap">Check</span>
        <span class="main">{check}</span>
        {#if step.callout}<span class="say">Say: {step.callout}</span>{/if}
        {#if checkOpen}{#each checkMore as line, n (n)}<span class="more">{line}</span>{/each}{/if}
      </div>
      {#if checkMore.length}
        <button class="q" type="button" aria-label="Why this check" aria-expanded={checkOpen} onclick={() => (checkOpen = !checkOpen)}>?</button>
      {/if}
    </div>
  {/if}
  {#if ifNot}
    <div class="box ifnot">
      <div class="txt">
        <span class="cap">If not</span>
        <span class="main">{ifNot}</span>
        {#if ifNotOpen}{#each ifNotMore as line, n (n)}<span class="more">{line}</span>{/each}{/if}
      </div>
      {#if ifNotMore.length}
        <button class="q" type="button" aria-label="More about this case" aria-expanded={ifNotOpen} onclick={() => (ifNotOpen = !ifNotOpen)}>?</button>
      {/if}
    </div>
  {/if}
  {#if caution}<p class="caution"><b>Caution</b> {caution}</p>{/if}

  {#if why || how?.length || explain || background || terms.length || (own && place?.control)}
    <div class="why">
      {#if why}<span><strong>Why</strong> {why}</span>{/if}
      {#if how?.length || explain || background || (own && place?.control)}
        <button class="lk" type="button" aria-expanded={howOpen} onclick={() => (howOpen = !howOpen)}>How to do it ⌄</button>
      {/if}
      {#if terms.length}
        <button class="lk" type="button" aria-expanded={termsOpen} onclick={() => (termsOpen = !termsOpen)}>Terms: {terms.map((t) => t.term).join(", ")} ⌄</button>
      {/if}
    </div>
    {#if howOpen}
      <div class="open-block">
        {#if how?.length}<ol>{#each how as line, n (n)}<li>{line}</li>{/each}</ol>{/if}
        {#if explain}<p><b>What it is</b> {explain.what}</p>{/if}
        {#if background}<p><b>Note</b> {background}</p>{/if}
        {#if own && place?.control}<button class="lk" type="button" onclick={openOwn}>What the {sentence(place.control.name).toLowerCase()} does</button>{/if}
      </div>
    {/if}
    {#if termsOpen}
      <dl class="open-block terms">
        {#each terms as t (t.term)}<div><dt>{t.term}</dt><dd>{t.meaning}</dd></div>{/each}
      </dl>
    {/if}
  {/if}
  {/if}

  {#if place}
    {#if showMap}
      <figure class="map">
        <div class="mapdraw"><OverviewMap {index} zone={place.zone} compact /></div>
        <div class="info">
          <span class="area">{where}</span>
          {#if place.zone.position ?? place.control?.location}<span class="at">{place.zone.position ?? place.control?.location}</span>{/if}
          <Locator {index} zone={place.zone} link />
          <button class="hide" type="button" onclick={() => onMap("hide")}>Hide map</button>
        </div>
      </figure>
    {:else}
      <div class="line">
        <span>{where} · {place.zone.title}</span>
        <Locator {index} zone={place.zone} link />
        {#if flow || !desktop}
          <!-- Flow and tablet landscape keep to the location line. -->
        {:else if squeezed && wantMap}
          <button class="lk" type="button" onclick={() => (asked = true)}>Show map</button>
        {:else if mapMode === "on_request"}
          <button class="lk" type="button" onclick={() => onMap("show")}>Show map</button>
        {:else if mapMode === "new_panel"}
          <button class="lk" type="button" onclick={() => (forced = true)}>Show map</button>
        {/if}
      </div>
    {/if}

    {#each cards as card (card.panel.id)}
      <figure class="controls">
        <div class="cap-row">
          <span class="cap">The controls · {panelCaption(card.focus.map((id) => index.section(id)?.title ?? id))}</span>
          {#if enlarge[card.panel.id]}
            <button class="lk larger" type="button" onclick={() => drawing[card.panel.id]?.openLarger()}>See the whole panel larger</button>
          {/if}
        </div>
        <PanelSchematic
          bind:this={drawing[card.panel.id]}
          panel={card.panel}
          focus={card.focus}
          {targets}
          {guardClosed}
          {index}
          around
          ownButton={false}
          named={`${check ?? ""} ${ifNot ?? ""}`}
          bind:picked={picked[card.panel.id]}
          bind:enlargeable={enlarge[card.panel.id]}
          bind:fits={fits[card.panel.id]}
        />
        <figcaption>
          Blue: set by this step · lights drawn unlit{#if !flow}{" "}· tap a light for its meaning{/if}{#if inCard(card).some((t) => t.open)}{" "}· dashed: only if the step's condition applies{/if}{#if inCard(card).some((t) => t.resolved)}{" "}· ◆ set by a decision step{/if}
        </figcaption>
      </figure>
    {/each}
    {#if !targets.length && place.control}
      <p class="note">{place.control.name} isn't drawn among the controls.</p>
    {/if}
  {:else if upcoming.length}
    <h3 class="soon">Coming up</h3>
    <ol class="next">
      {#each upcoming as u (u.step.id)}
        <li class={u.state}><span class="nm">{u.step.text}</span><span class="mono">{u.step.expect}</span></li>
      {/each}
    </ol>
  {/if}
</div>

<style>
  /* The Checklist board's side pane. */
  /* Fills the side pane, so the controls take the height left below the rest (the Checklist
     and Flow boards). */
  .pane-step {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    min-height: 100%;
  }
  .head {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }
  .pos {
    color: var(--color-link);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
  }
  .value {
    color: var(--color-text-strong);
    font: var(--weight-regular) var(--font-size-value-mono) / var(--leading-snug) var(--font-mono);
  }
  .box {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-1) var(--space-1) var(--space-1) var(--space-4);
    border: var(--border-thin) solid var(--color-border);
    border-radius: var(--radius-control);
    background: var(--color-surface);
  }
  .box.ifnot {
    border-color: var(--color-open-border);
    background: var(--color-open-bg);
  }
  .txt {
    display: flex;
    flex: 1;
    flex-direction: column;
    gap: var(--space-1);
    padding: var(--space-1) 0;
  }
  .cap {
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-semibold);
  }
  .ifnot .cap,
  .ifnot .q {
    color: var(--color-open-text);
  }
  .main {
    font-size: var(--font-size-body);
    font-weight: var(--weight-semibold);
  }
  .say {
    color: var(--color-text-strong);
    font: var(--weight-medium) var(--font-size-value-mono) var(--font-mono);
  }
  .more {
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
  }
  .q {
    flex: none;
    border-radius: var(--radius-pill);
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
  }
  .caution {
    margin: 0;
    color: var(--color-text);
    font-size: var(--font-size-secondary);
  }
  .caution b {
    color: var(--color-open-text);
    font-weight: var(--weight-semibold);
  }
  .why {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    column-gap: var(--space-4);
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
  }
  .why strong {
    color: var(--color-text-strong);
    font-weight: var(--weight-semibold);
  }
  .lk {
    display: inline-flex;
    align-items: center;
    min-width: 0;
    color: var(--color-link);
    font-size: var(--font-size-secondary);
  }
  .open-block {
    margin: 0;
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
  }
  .open-block p,
  .open-block ol {
    margin: 0 0 var(--space-2);
  }
  .terms div {
    display: flex;
    gap: var(--space-2);
  }
  .terms dt {
    color: var(--color-text-strong);
    font-weight: var(--weight-semibold);
  }
  .terms dd {
    margin: 0;
  }
  .map,
  .controls {
    margin: 0;
    padding: var(--space-2) var(--space-3);
    border: var(--border-thin) solid var(--panel-frame);
    border-radius: var(--radius-control);
    background: var(--panel-bg);
  }
  .map {
    display: grid;
    grid-template-columns: var(--size-map-figure) minmax(0, 1fr);
    align-items: center;
    gap: var(--space-3);
  }
  .info {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: var(--border-thick);
  }
  .area {
    color: var(--panel-text);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
  }
  .at {
    color: var(--panel-label);
    font-size: var(--font-size-caption);
    line-height: var(--leading-snug);
  }
  .hide {
    display: inline-flex;
    align-items: center;
    min-width: 0;
    color: var(--panel-label);
    font-size: var(--font-size-secondary);
  }
  .line {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    padding: 0 var(--space-1) 0 var(--space-4);
    border: var(--border-thin) solid var(--color-border);
    border-radius: var(--radius-control);
    background: var(--color-surface);
  }
  /* The board's links on this line: 8 px each side. */
  .line .lk,
  .line :global(.where-link) {
    padding: 0 var(--space-2);
  }
  .line span {
    flex: 1;
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
  }
  /* The card takes the pane's remaining height; its caption, drawing and legend sit together in
     its middle (the Flow board). A drawing taller than the room scrolls inside it, never less than
     room for a row of controls, the legend still under it. */
  .controls {
    display: flex;
    flex: 1 1 0;
    flex-direction: column;
    justify-content: center;
    gap: var(--space-1);
    min-height: 0;
  }
  .controls :global(.wrap) {
    flex: 0 1 auto;
    min-height: calc(3 * var(--size-touch-min));
  }
  .controls .cap {
    color: var(--panel-label);
  }
  /* The card's caption, with "See the whole panel larger" at its right (the owner, batch 2). */
  .cap-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-3);
  }
  .cap-row .larger {
    font-size: var(--font-size-caption);
  }
  figcaption {
    color: var(--panel-label);
    font-size: var(--font-size-caption);
  }
  .note {
    margin: 0;
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  .soon {
    margin: var(--space-2) 0 0;
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-semibold);
  }
  .next {
    margin: 0;
    padding: 0;
    list-style: none;
  }
  .next li {
    display: flex;
    justify-content: space-between;
    gap: var(--space-3);
    padding: var(--space-2) 0;
    border-bottom: var(--border-thin) solid var(--color-line);
    font-size: var(--font-size-secondary);
  }
  .next .mono {
    color: var(--color-text-secondary);
    font-family: var(--font-mono);
  }
  .next li.done {
    color: var(--color-text-muted);
  }
</style>
