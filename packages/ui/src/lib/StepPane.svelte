<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The side pane for the selected step (round 6, R6PanePre; round 5 Checklist board above it): the
step and its value, Check, If not, a caution note, Why with How to do it and Terms, the location
line, and a label-free preview of each panel the step uses. The location line opens the
slide-out with Where it is expanded; a preview (or Show panel) opens it on that panel. Nothing
else is drawn; the rest of the pane stays empty. Approved additions: a Say line in the Check box
and a caution note under If not. A step with no control shows the steps coming up instead.
-->
<script lang="ts">
  import { sentence as toSentence, stepTitle } from "./sentence";
  import type { ItemTarget, Step, StepState } from "@chocktochock/bindings";
  import type { PackIndex } from "../pack-index";
  import Icon from "./Icon.svelte";
  import PanelPreview from "./PanelPreview.svelte";
  import type { Opening } from "./opening";
  import { panelCaption } from "./panel-name";

  let {
    index,
    step,
    targets,
    number,
    total,
    upcoming = [],
    preview = true,
    onOpen,
    flow = false,
  }: {
    index: PackIndex;
    step: Step;
    targets: ItemTarget[];
    /** The step's place in its list, as shown: "Step 7 of 10". */
    number: number;
    total: number;
    upcoming?: { step: Step; state: StepState }[];
    /** Settings › Panel preview in steps; off, a Show panel button instead. */
    preview?: boolean;
    /** Opens the slide-out. */
    onOpen: (how: Opening) => void;
    /** Flow view's pane (R6Flow): the location line and the previews only; Flow shows the step's
     * details itself. */
    flow?: boolean;
  } = $props();

  const sentence = (text: string) => toSentence(index, text);

  let place = $derived(index.place(step));
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

  // One preview per panel the step's controls are on, its own control's first.
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
    return [...byPanel.values()];
  });
  const caption = (focus: string[]) => panelCaption(focus.map((id) => index.section(id)?.title ?? id));
  /** The step's own control, for How to do it's link to its card. */
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
    if (card && own) onOpen({ entry: "control", panel: card.panel.id, control: { section: own.section, item: own.item } });
  };
  let guardClosed = $derived(/\bGUARDS? CLOSED\b/i.test(step.expect));
  let where = $derived(place ? `${place.zone.area}` : "");
</script>

<div class="pane-step">
  {#if !flow}
  <div class="head">
    <span class="pos">Step {number} of {total} · {stepTitle(index, step)}</span>
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
    <button class="line" class:flow type="button" onclick={() => onOpen({ entry: "where" })}>
      {#if !flow}<span class="pin"><Icon name="pin" /></span>{/if}
      <span class="area">{where} · {place.zone.title}</span>
      <span class="lk">Where is this?</span>
    </button>
    {#if preview}
      {#each cards as card (card.panel.id)}
        <button class="preview" type="button" aria-label="Show the whole {caption(card.focus)}" onclick={() => onOpen({ entry: "panel", panel: card.panel.id })}>
          <PanelPreview panel={card.panel} focus={card.focus} {targets} {guardClosed} label="The whole {caption(card.focus)}" />
          <span class="foot">
            <span class="cap">{caption(card.focus)} · blue: set by this step</span>
            <span class="show">Show panel</span>
            <kbd aria-hidden="true">P</kbd>
          </span>
        </button>
      {/each}
    {:else}
      <button class="show-panel" type="button" onclick={() => onOpen({ entry: "panel" })}>
        <Icon name="panel" /> Show panel <kbd aria-hidden="true">P</kbd>
      </button>
    {/if}
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
  /* The location line (R6PanePre): one 44 px target that opens Where it is. */
  .line {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    min-height: var(--size-touch-min);
    padding: 0 var(--space-3);
    border: var(--border-thin) solid var(--color-border);
    border-radius: var(--radius-control);
    background: var(--color-surface);
    font: inherit;
    font-size: var(--font-size-secondary);
    text-align: left;
  }
  .pin {
    display: inline-flex;
    color: var(--color-text-muted);
  }
  .pin :global(svg) {
    width: var(--size-icon-small);
    height: var(--size-icon-small);
  }
  .line .area {
    flex: 1;
    color: var(--color-text-secondary);
  }
  .line .lk {
    color: var(--color-link);
  }
  /* The panel preview card (R6PanePre): the whole card is the button; nothing below it. */
  .preview {
    flex: none;
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    padding: var(--space-2) var(--space-3);
    border: var(--border-thin) solid var(--panel-frame);
    border-radius: var(--radius-control);
    background: var(--panel-bg);
    font: inherit;
    text-align: left;
  }
  .foot {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    min-height: calc(var(--size-touch-min) - 2 * var(--space-2));
  }
  .foot .cap {
    flex: 1;
    color: var(--panel-label);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-regular);
  }
  .show {
    color: var(--color-link);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
  }
  kbd {
    padding: 0 var(--space-1);
    border: var(--border-thin) solid var(--color-border);
    border-radius: var(--radius-small);
    color: var(--color-text-muted);
    font: inherit;
    font-size: var(--font-size-caption);
  }
  /* Settings › Panel preview in steps Off: the Show panel button (R6Pane). */
  .show-panel {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: var(--space-2);
    min-height: var(--size-touch-min);
    padding: 0 var(--space-4);
    border: var(--border-thin) solid var(--color-border);
    border-radius: var(--radius-control);
    background: var(--color-button-bg);
    box-shadow: var(--color-button-shadow);
    color: var(--color-text);
    font: inherit;
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
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
