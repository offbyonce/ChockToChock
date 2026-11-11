<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The context for one step, as in 3.0's right-hand panel: where the control is on the flight deck
map, and the panel section it is on as a schematic. Shown beside the list on wide screens and
inside the step on narrow ones.
-->
<script lang="ts">
  import type { Step } from "@chocktochock/bindings";
  import { type PackIndex, splitArea } from "../pack-index";
  import OverviewMap from "./OverviewMap.svelte";
  import PanelSchematic from "./PanelSchematic.svelte";

  let { index, step, heading = true }: { index: PackIndex; step: Step; heading?: boolean } = $props();

  /** A panel section's title as a panel's name, as in 3.0: "Electrical meters panel". */
  const panelName = (title: string) => (/panel$/i.test(title) ? title : `${title} panel`);

  let place = $derived(index.place(step));
  let area = $derived(place ? splitArea(place.zone.area) : null);
  let items = $derived(
    place?.section?.items.filter((i) => (i.controls ?? []).some((c) => (step.controls ?? []).includes(c))) ?? [],
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
    </div>
    {#if index.overview}
      <OverviewMap overview={index.overview} zones={index.zones} zone={place.zone} />
    {/if}

    {#if place.section}
      <h3 class="label">The controls</h3>
      <figure class="card">
        <figcaption>{panelName(place.section.title)}</figcaption>
        <PanelSchematic section={place.section} controls={step.controls ?? []} />
      </figure>
      <p class="tiny">Schematic, not to scale. Switch positions are not shown; set each control as the step says.</p>
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
    {#each items as item (item.label)}
      <p class="item"><b>{item.label}</b>{#if item.positions}<span class="mono"> {item.positions}</span>{/if}. {item.description}</p>
    {/each}
  {:else}
    {#if step.why}
      <div>
        <p class="lbl">Why</p>
        <p>{step.why}</p>
      </div>
    {/if}
    <p class="tiny">This step has no control on the flight deck map.</p>
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
    align-items: baseline;
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
  figure :global(.schematic) {
    padding: 0.75rem;
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
  .item .mono {
    color: var(--muted);
    font-size: 0.8125rem;
  }
</style>
