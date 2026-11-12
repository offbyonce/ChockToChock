<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The design system's own page (unlinked): the component gallery and the screen mockups, picked by
the address, `?view=gallery|phase|step|flow|setup&theme=dark|light`. Mockups show the system's
layouts at whatever size the window is.
-->
<script lang="ts">
  import type { FlatPack, ItemTarget, Panel, Step } from "@chocktochock/bindings";
  import { indexPack, type PackIndex } from "../pack-index";
  import AppShell from "../ds/AppShell.svelte";
  import Sheet from "../ds/Sheet.svelte";
  import { applyPalette } from "../theme/tokens";
  import FlowMock from "./FlowMock.svelte";
  import Gallery from "./Gallery.svelte";
  import MockRail from "./MockRail.svelte";
  import MockTop from "./MockTop.svelte";
  import PhaseMock from "./PhaseMock.svelte";
  import SetupMock from "./SetupMock.svelte";
  import StepPane from "./StepPane.svelte";

  let { packUrl }: { packUrl: string } = $props();

  const params = new URLSearchParams(window.location.search);
  const view = params.get("view") ?? "gallery";
  const dark = params.get("theme") !== "light";
  applyPalette(document.documentElement, dark, 0);
  document.documentElement.style.colorScheme = dark ? "dark" : "light";

  let panel = $state<Panel | null>(null);
  let index = $state<PackIndex | null>(null);
  let step = $state<Step | null>(null);
  let targets = $state<ItemTarget[]>([]);
  let narrow = $state(window.innerWidth < 900);
  let sheet = $state(view === "step" && window.innerWidth < 900);

  $effect(() => {
    void fetch(packUrl)
      .then((r) => r.json())
      .then((pack: FlatPack) => {
        index = indexPack(pack);
        step = index.step("ifly-737-max8.before-start.fuel-pump-switches") ?? null;
        const oh = pack.content.panels.find((p) => p.id === "oh") ?? null;
        const fuel = oh?.sections.find((s) => s.id === "fuel");
        panel = oh;
        if (fuel) {
          targets = fuel.items.flatMap((it, item) => {
            if (it.kind !== "toggle_switch") return [];
            const open = it.label.startsWith("CTR");
            return [{ section: "fuel", item, position: open ? null : 1, lit: false, open }];
          });
        }
      });
  });
</script>

<svelte:window onresize={() => (narrow = window.innerWidth < 900)} />

<div data-design-ready={panel ? "" : undefined}>
  {#if view === "gallery"}
    <div class="pad"><Gallery /></div>
  {:else}
    {#snippet paneContent()}<StepPane {panel} {targets} {index} {step} />{/snippet}
    <AppShell pane={view === "phase" || view === "step" ? paneContent : undefined}>
      {#snippet top()}<MockTop view={view === "flow" ? "flow" : "checklist"} title={view === "setup" ? "Flight setup" : "Before start"} />{/snippet}
      {#snippet rail()}<MockRail />{/snippet}
      {#if view === "flow"}
        <FlowMock />
      {:else if view === "setup"}
        <SetupMock />
      {:else}
        <PhaseMock onOpen={() => (sheet = narrow)} />
      {/if}
    </AppShell>
    <Sheet open={sheet} title="Fuel pump switches" onClose={() => (sheet = false)}>
      <StepPane {panel} {targets} {index} {step} collapsible heading={false} details />
    </Sheet>
  {/if}
</div>

<style>
  .pad {
    padding: var(--space-6) var(--gutter);
  }
</style>
