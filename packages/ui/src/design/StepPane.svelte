<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Mockup: a step's details on the system, as the side pane shows them (and the sheet on phones):
the header, the CHECK and IF NOT callouts, where it is, the controls drawing (collapsible on
phones), and what the control does.
-->
<script lang="ts">
  import type { ItemTarget, Panel, Step } from "@chocktochock/bindings";
  import Button from "../ds/Button.svelte";
  import Callout from "../ds/Callout.svelte";
  import PaneHeader from "../ds/PaneHeader.svelte";
  import SectionTitle from "../ds/SectionTitle.svelte";
  import Locator from "../lib/Locator.svelte";
  import OverviewMap from "../lib/OverviewMap.svelte";
  import PanelSchematic from "../lib/PanelSchematic.svelte";
  import { type PackIndex, splitArea } from "../pack-index";
  import { FUEL_STEP } from "./data";

  let {
    panel,
    targets,
    collapsible = false,
    heading = true,
    index = null,
    step = null,
    details = false,
  }: {
    panel: Panel | null;
    targets: ItemTarget[];
    collapsible?: boolean;
    heading?: boolean;
    index?: PackIndex | null;
    step?: Step | null;
    /** Show the step's own CHECK, IF NOT and Why: in the sheet, where the list is hidden; the side
     * pane leaves them to the list, as now. */
    details?: boolean;
  } = $props();
  let place = $derived(index && step ? index.place(step) : null);
  let region = $derived(index && place ? (index.region(place.zone)?.region ?? null) : null);
  /** "Forward overhead · P5", as the side pane names it. */
  let area = $derived(place ? splitArea(place.zone.area) : null);
  let drawing = $state(false);
  let shown = $derived(!collapsible || drawing);
</script>

<div class="sp">
  {#if heading}
    <PaneHeader level={2} crumb={FUEL_STEP.where} title={FUEL_STEP.text} />
  {:else}
    <p class="where">{FUEL_STEP.where}</p>
  {/if}
  <p class="value">{FUEL_STEP.value}</p>
  {#if details}
    <div class="cells">
      <Callout caption="Check" title={FUEL_STEP.check.title} detail={FUEL_STEP.check.detail} />
      <Callout tone="warn" caption="If not" title={FUEL_STEP.ifNot.title} detail={FUEL_STEP.ifNot.detail} />
    </div>
    <section>
      <SectionTitle>Why</SectionTitle>
      <p>{FUEL_STEP.why}</p>
    </section>
  {/if}
  {#if index && place && area}
    <section>
      <div class="ctl">
        <SectionTitle>Where it is</SectionTitle>
        <span class="area"><b>{region?.name ?? area[0]}</b>{#if region?.code ?? area[1]}{" · "}{region?.code ?? area[1]}{/if}</span>
      </div>
      <Locator {index} zone={place.zone} />
      <OverviewMap {index} zone={place.zone} />
    </section>
  {/if}
  <section>
    <div class="ctl">
      <SectionTitle>The controls</SectionTitle>
      {#if collapsible}
        <Button variant="link" compact onclick={() => (drawing = !drawing)}>{drawing ? "Hide" : "Show"} the drawing</Button>
      {/if}
    </div>
    {#if shown && panel}
      {#if region?.top}<p class="edge">↑ {region.top}</p>{/if}
      <PanelSchematic {panel} focus={["fuel"]} {targets} />
      {#if region?.bottom}<p class="edge">↓ {region.bottom}</p>{/if}
    {/if}
  </section>
  <section>
    <SectionTitle>Where</SectionTitle>
    <p>{FUEL_STEP.location}</p>
  </section>
  <section>
    <SectionTitle>What it does</SectionTitle>
    <p>{FUEL_STEP.does}</p>
  </section>
</div>

<style>
  .sp {
    display: flex;
    flex-direction: column;
    gap: var(--space-5);
  }
  .sp :global(.ph) {
    margin-bottom: 0;
  }
  .value {
    margin: calc(-1 * var(--space-3)) 0 0;
    color: var(--accent);
    font: var(--weight-medium) var(--text-sm) / var(--leading-snug) var(--mono);
  }
  .where {
    margin: 0;
    color: var(--muted);
    font-size: var(--text-sm);
  }
  .cells {
    display: grid;
    gap: var(--space-3);
  }
  section p {
    margin: 0;
    font-size: var(--text-sm);
    line-height: var(--leading-normal);
  }
  .ctl {
    display: flex;
    flex-wrap: wrap;
    align-items: baseline;
    justify-content: space-between;
    gap: var(--space-2);
  }
  .area {
    font-size: var(--text-sm);
  }
  .area b {
    font-weight: var(--weight-bold);
  }
  .edge {
    margin: var(--space-1) 0;
    color: var(--muted);
    font: var(--weight-bold) var(--text-xs) / var(--leading-normal) var(--sans);
    letter-spacing: var(--tracking-caps);
    text-align: center;
    text-transform: uppercase;
  }
</style>
