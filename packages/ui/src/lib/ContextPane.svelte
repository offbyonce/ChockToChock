<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The context for one step, as in 3.0's right-hand panel: the control, where it is on the flight
deck map ("Where is this?" shows the whole flight deck), the panel sections it is on with the
step's target position, then where to find it and what it does. A step with no control on the
map shows the steps coming up instead, so the panel is never empty; its details stay in the list
or Flow view, never repeated here. Shown beside
the list on wide screens and inside the step on narrow ones.
-->
<script lang="ts">
  import type { ItemTarget, Step, StepState } from "@chocktochock/bindings";
  import { type PackIndex, splitArea } from "../pack-index";
  import Locator from "./Locator.svelte";
  import OverviewMap from "./OverviewMap.svelte";
  import PanelSchematic from "./PanelSchematic.svelte";
  import { panelCaption } from "./panel-name";

  let {
    index,
    step,
    targets,
    upcoming = [],
    heading = true,
    details = true,
  }: {
    index: PackIndex;
    step: Step;
    /** The step's panel items and target positions (from Rust). */
    targets: ItemTarget[];
    /** The steps after this one, for a step with no control on the map. */
    upcoming?: { step: Step; state: StepState }[];
    heading?: boolean;
    /** For a step with no map control, show the steps coming up. Off inside an opened step. The
     * step's own details are never repeated here: the list or Flow view shows them. */
    details?: boolean;
  } = $props();

  /** "LANDING GEAR PINS" as "Landing gear pins", like control names; mixed case stays. */
  const sentence = (text: string) =>
    text === text.toUpperCase() ? text.charAt(0) + text.slice(1).toLowerCase() : text;


  let place = $derived(index.place(step));
  let region = $derived(place ? (index.region(place.zone)?.region ?? null) : null);
  /**
   * "Forward overhead · P5"; where a region holds several panels ("P9 · CONTROL STAND · P8"), the
   * zone's own panel instead, as 3.0 shows it ("Center console · Control stand").
   */
  let area = $derived.by(() => {
    if (!place) return null;
    const [name, code] = splitArea(place.zone.area);
    if (!region) return [name, code];
    const many = (region.code ?? "").includes("·");
    return [region.name, many ? (code ?? name) : (region.code ?? code)];
  });
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
  const cardTitle = (focus: string[]) => panelCaption(focus.map((id) => index.section(id)?.title ?? id));
  let guardClosed = $derived(/\bGUARDS? CLOSED\b/i.test(step.expect));
  let setTo = $derived(
    targets
      .map((t) => {
        const item = index.section(t.section)?.items[t.item];
        const pos = t.position != null ? item?.positions?.split(",")[t.position]?.trim() : undefined;
        return item && pos ? `${item.label} set to ${pos}` : null;
      })
      .filter(Boolean),
  );
</script>

<div class="context">
  {#if heading}
    <h2>{place?.control?.name ?? sentence(step.text)}</h2>
    {#if place}
      <p class="path">{place.zone.area} · {place.zone.title}</p>
    {/if}
  {/if}

  {#if place && area}
    <div class="where">
      <span class="label">Where it is</span>
      <span><b>{area[0]}</b>{#if area[1]}<span class="dot">{" · "}</span><span class="code">{area[1]}</span>{/if}</span>
      <Locator {index} zone={place.zone} />
    </div>
    <OverviewMap {index} zone={place.zone} />

    {#if cards.length}
      <h3 class="label">The controls</h3>
      {#each cards as card (card.panel.id)}
        <figure class="card">
          <figcaption>{cardTitle(card.focus)}</figcaption>
          {#if region?.top}<p class="edge">↑ {region.top}</p>{/if}
          <div class="pad"><PanelSchematic panel={card.panel} focus={card.focus} {targets} {guardClosed} /></div>
          {#if region?.bottom}<p class="edge">↓ {region.bottom}</p>{/if}
        </figure>
      {/each}
      <p class="tiny">
        {#if setTo.length}Blue: set by this step ({setTo.join("; ")}). Not the simulator's live state.{/if}
        {#if targets.some((t) => t.open)}Dashed: only if the step's condition applies.{/if}
        Lights are drawn unlit unless the step expects them on. Schematic, not to scale.
      </p>
    {/if}
    {#if !targets.length && place.control}
      <!-- A control no panel draws, such as the control column: say so, rather than show nothing. -->
      <p class="tiny">{place.control.name} isn't drawn among the controls.</p>
    {/if}

    {#if place.control}
      <div>
        <p class="lbl">Where</p>
        <p>{place.control.location}</p>
      </div>
      <div>
        <p class="lbl">What it does</p>
        <p>{place.control.function}</p>
      </div>
    {/if}
  {:else if details}
    <p class="tiny">This step has no control on the flight deck map.</p>
    {#if upcoming.length}
      <h3 class="label">Coming up</h3>
      <ol class="next">
        {#each upcoming as u (u.step.id)}
          <li class={u.state} class:long={u.step.expect.length > 28}>
            <span class="nm">{u.step.text}</span>
            <span class="mono">{u.step.expect}</span>
          </li>
        {/each}
      </ol>
    {/if}
  {/if}
</div>

<style>
  /* Sizes and weights follow 3.0's context panel. */
  .context {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }
  h2 {
    margin: 0;
    font-size: var(--font-size-body);
    font-weight: var(--weight-semibold);
    line-height: 1.25;
  }
  .path {
    margin: calc(-1 * var(--space-2)) 0 var(--space-1);
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
  }
  .label {
    margin: 0;
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-bold);
    text-transform: uppercase;
  }
  .where {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--space-1) var(--space-2);
    font-size: var(--font-size-secondary);
  }
  .where b {
    font-weight: var(--weight-semibold);
  }
  .dot {
    color: var(--color-text-muted);
    white-space: pre;
  }
  .code {
    color: var(--color-text-muted);
    font: var(--weight-regular) var(--font-size-caption) var(--font-mono);
  }
  h3.label {
    margin-top: var(--space-3);
    padding-top: var(--space-6);
    border-top: var(--border-thin) solid var(--color-border);
  }
  figure {
    margin: 0;
  }
  /* The drawing sits on the panel background, in both themes (decision 0021). */
  figure.card {
    border-color: var(--panel-frame);
    background: var(--panel-bg);
  }
  figcaption {
    padding: var(--space-3) var(--space-3);
    border-bottom: var(--border-thin) solid var(--color-border);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
  }
  /* The drawing takes the card's width; its own frame gives it a margin. */
  .pad {
    padding: var(--space-1);
  }
  /* The region's orientation captions around the panel, as 3.0 shows them. */
  .edge {
    padding-top: var(--space-2);
    color: var(--panel-label);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-semibold);
    text-align: center;
  }
  .edge + .pad {
    padding-top: var(--space-1);
  }
  figure > .edge:last-child {
    padding: 0 0 var(--space-2);
  }
  .tiny {
    margin: 0;
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  .lbl {
    margin: var(--space-1) 0 var(--space-1);
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-semibold);
  }
  p {
    margin: 0;
    font-size: var(--font-size-secondary);
  }
  .next {
    display: grid;
    gap: 0;
    margin: 0;
    padding: 0;
    list-style: none;
  }
  .next li {
    display: grid;
    grid-template-columns: auto minmax(0, 1fr);
    gap: var(--space-1) var(--space-4);
    padding: var(--space-2) 0;
    border-top: var(--border-thin) solid var(--color-border);
    font-size: var(--font-size-secondary);
  }
  .next li:first-child {
    border-top: 0;
  }
  .nm {
    font-weight: var(--weight-medium);
  }
  /* In the text face, as long values are in the list: the monospaced face makes "1,000" look like
     "1, 000" and its parentheses look like brackets. */
  .next li.long {
    grid-template-columns: minmax(0, 1fr);
  }
  .next li.long .mono {
    text-align: left;
  }
  .next .mono {
    color: var(--color-text-muted);
    font: var(--weight-regular) var(--font-size-secondary) / var(--leading-snug) var(--font-sans);
    text-align: right;
  }
  .next li.done {
    color: var(--color-text-muted);
  }
</style>
