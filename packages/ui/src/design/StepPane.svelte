<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Mockup: a step's details on the system, as the side pane shows them (and the sheet on phones):
the header, the CHECK and IF NOT callouts, where it is, the controls drawing (collapsible on
phones), and what the control does.
-->
<script lang="ts">
  import type { ItemTarget, Panel } from "@chocktochock/bindings";
  import Button from "../ds/Button.svelte";
  import Callout from "../ds/Callout.svelte";
  import PaneHeader from "../ds/PaneHeader.svelte";
  import SectionTitle from "../ds/SectionTitle.svelte";
  import PanelSchematic from "../lib/PanelSchematic.svelte";
  import { FUEL_STEP } from "./data";

  let {
    panel,
    targets,
    collapsible = false,
    heading = true,
  }: { panel: Panel | null; targets: ItemTarget[]; collapsible?: boolean; heading?: boolean } = $props();
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
  <div class="cells">
    <Callout caption="Check" title={FUEL_STEP.check.title} detail={FUEL_STEP.check.detail} />
    <Callout tone="warn" caption="If not" title={FUEL_STEP.ifNot.title} detail={FUEL_STEP.ifNot.detail} />
  </div>
  <section>
    <SectionTitle>Why</SectionTitle>
    <p>{FUEL_STEP.why}</p>
  </section>
  <section>
    <div class="ctl">
      <SectionTitle>The controls</SectionTitle>
      {#if collapsible}
        <Button variant="link" compact onclick={() => (drawing = !drawing)}>{drawing ? "Hide" : "Show"} the drawing</Button>
      {/if}
    </div>
    {#if shown && panel}
      <PanelSchematic {panel} focus={["fuel"]} {targets} />
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
    align-items: baseline;
    justify-content: space-between;
  }
</style>
