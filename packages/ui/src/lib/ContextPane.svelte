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

  /** A panel section's title as a panel's name, as in 3.0: "Electrical meters panel". */
  const panelName = (title: string) => (/panel$/i.test(title) ? title : `${title} panel`);

  let place = $derived(index.place(step));
  let region = $derived(place ? (index.region(place.zone)?.region ?? null) : null);
  let area = $derived(place ? (region ? [region.name, region.code ?? null] : splitArea(place.zone.area)) : null);
  // Up to three sections, as 3.0 shows, starting with the control's own.
  let sections = $derived.by(() => {
    const ids = [...new Set([place?.section?.id, ...targets.map((t) => t.section)].filter((x): x is string => !!x))];
    return ids
      .map((id) => index.section(id))
      .filter((s): s is NonNullable<typeof s> => !!s)
      .slice(0, 3);
  });
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
    <h2>{place?.control?.name ?? step.text}</h2>
    {#if place}
      <p class="path">{place.zone.area} · {place.zone.title}</p>
    {/if}
  {/if}

  {#if place && area}
    <div class="where">
      <span class="label">Where it is</span>
      <b>{area[0]}</b>{#if area[1]}<span class="code"> · {area[1]}</span>{/if}
      <Locator {index} zone={place.zone} />
    </div>
    <OverviewMap {index} zone={place.zone} />

    {#if sections.length}
      <h3 class="label">The controls</h3>
      {#each sections as section (section.id)}
        <figure class="card">
          <figcaption>{panelName(section.title)}</figcaption>
          {#if region?.top}<p class="edge">↑ {region.top}</p>{/if}
          <div class="pad"><PanelSchematic {section} {targets} {guardClosed} /></div>
          {#if region?.bottom}<p class="edge">↓ {region.bottom}</p>{/if}
        </figure>
      {/each}
      <p class="tiny">
        {#if setTo.length}Highlighted: what this step asks for ({setTo.join("; ")}). Not the simulator's live state.{/if}
        Schematic, not to scale.
      </p>
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
          <li class={u.state}>
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
    gap: 0.625rem;
  }
  h2 {
    margin: 0;
    font-size: 1.0625rem;
    font-weight: 600;
    line-height: 1.25;
    letter-spacing: -0.01em;
  }
  .path {
    margin: -0.5rem 0 0.25rem;
    color: var(--muted);
    font-size: 0.8125rem;
  }
  .label {
    margin: 0;
    color: var(--muted);
    font-size: 0.6875rem;
    font-weight: 700;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }
  .where {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.25rem 0.5rem;
    font-size: 0.8125rem;
  }
  .where b {
    font-weight: 600;
  }
  .code {
    color: var(--muted);
    font: 0.75rem var(--mono);
  }
  h3.label {
    margin-top: 0.75rem;
    padding-top: 1.25rem;
    border-top: 1px solid var(--line);
  }
  figure {
    margin: 0;
  }
  figcaption {
    padding: 0.625rem 0.75rem;
    border-bottom: 1px solid var(--line);
    font-size: 0.8125rem;
    font-weight: 600;
  }
  .pad {
    padding: 0.5rem 0.75rem;
  }
  /* The region's orientation captions around the panel, as 3.0 shows them. */
  .edge {
    padding-top: 0.5rem;
    color: var(--muted);
    font-size: 0.6875rem;
    font-weight: 600;
    letter-spacing: 0.05em;
    text-align: center;
  }
  .edge + .pad {
    padding-top: 0.25rem;
  }
  figure > .edge:last-child {
    padding: 0 0 0.5rem;
  }
  .tiny {
    margin: 0;
    color: var(--muted);
    font-size: 0.75rem;
  }
  .lbl {
    margin: 0.25rem 0 0.125rem;
    color: var(--muted);
    font-size: 0.75rem;
    font-weight: 600;
  }
  p {
    margin: 0;
    font-size: 0.875rem;
  }
  .next {
    display: grid;
    gap: 0;
    margin: 0;
    padding: 0;
    list-style: none;
  }
  .next li {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    gap: 0.125rem 1rem;
    padding: 0.5rem 0;
    border-top: 1px solid var(--line);
    font-size: 0.875rem;
  }
  .next li:first-child {
    border-top: 0;
  }
  .nm {
    font-weight: 500;
  }
  .next .mono {
    color: var(--muted);
    font-size: 0.8125rem;
  }
  .next li.done {
    color: var(--muted);
  }
</style>
