<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The drawing renderer's gallery (decision 0021), for review and screenshots only; unlinked and
never part of the installed app. Every control kind in every state it can be drawn in, from
invented one-control panels made here (not pack content), and the pack's side and seat views for
each region. `?theme=dark|light` picks the theme.
-->
<script lang="ts">
  import type { FlatPack, ItemTarget, Panel, PanelItem } from "@chocktochock/bindings";
  import PanelSchematic from "../lib/PanelSchematic.svelte";
  import ViewDrawing from "../lib/ViewDrawing.svelte";
  import { applyPalette } from "../theme/tokens";

  let { packUrl }: { packUrl: string } = $props();

  const params = new URLSearchParams(location.search);
  const dark = params.get("theme") !== "light";
  applyPalette(document.documentElement, dark, 0);

  let pack = $state<FlatPack | null>(null);
  $effect(() => {
    fetch(packUrl)
      .then((r) => r.json())
      .then((p: FlatPack) => {
        pack = p;
      });
  });

  type State = { name: string; target: Omit<ItemTarget, "section" | "item"> | null; guardClosed?: boolean };
  const set = (position: number | null, extra: Partial<ItemTarget> = {}) => ({
    position,
    lit: false,
    open: false,
    resolved: false,
    ...extra,
  });
  const RESTING: State = { name: "Resting", target: null };
  const CHANGED = (p: number | null): State => ({ name: "Changed", target: set(p) });
  const RESOLVED = (p: number): State => ({ name: "Resolved", target: set(p, { resolved: true }) });
  const CONDITIONAL: State = { name: "Conditional", target: set(null, { open: true }) };

  /** Invented one-control panels: a label, positions and a resting position where it has one. */
  const kinds: { title: string; item: Partial<PanelItem>; states: State[] }[] = [
    {
      title: "Toggle switch",
      item: { kind: "toggle_switch", label: "PUMP", positions: "OFF, ON", rest: "OFF" },
      states: [RESTING, CHANGED(1), RESOLVED(0), CONDITIONAL],
    },
    {
      title: "Guarded switch",
      item: { kind: "toggle_switch", label: "STBY PWR", positions: "BAT, OFF, AUTO", rest: "AUTO", guarded: true },
      states: [RESTING, { name: "Changed, guard closed", target: set(2), guardClosed: true }, CHANGED(0)],
    },
    {
      title: "Selector",
      item: { kind: "selector", label: "MODE", positions: "OFF, ALIGN, NAV, ATT", rest: "OFF" },
      states: [RESTING, CHANGED(2), RESOLVED(1), CONDITIONAL],
    },
    {
      title: "Push button",
      item: { kind: "push_button", label: "TEST" },
      states: [RESTING, CHANGED(null), CONDITIONAL],
    },
    {
      title: "Light",
      item: { kind: "light", label: "LOW PRESSURE", color: "amber" },
      states: [
        { name: "Unlit", target: null },
        { name: "Unlit, the step's", target: set(null) },
        { name: "Lit", target: set(null, { lit: true }) },
      ],
    },
    {
      title: "Red and green lights",
      item: { kind: "light", label: "FIRE WARN", color: "red" },
      states: [
        { name: "Unlit", target: null },
        { name: "Lit", target: set(null, { lit: true }) },
      ],
    },
    {
      title: "Lever",
      item: { kind: "lever", label: "FLAP", positions: "UP, 1, 5, 15, 30", rest: "UP" },
      states: [RESTING, CHANGED(2), CONDITIONAL],
    },
    {
      title: "Handle",
      item: { kind: "handle", label: "BRAKE", positions: "RELEASED, SET", rest: "RELEASED" },
      states: [RESTING, CHANGED(1)],
    },
    {
      title: "Gauge and screen",
      item: { kind: "gauge", label: "TEMP" },
      states: [RESTING, CHANGED(null)],
    },
  ];

  const panelOf = (item: Partial<PanelItem>): Panel =>
    ({
      id: "gallery",
      title: "Gallery",
      width: 200,
      height: 110,
      sections: [
        {
          id: "s",
          title: "Invented",
          x: 0,
          y: 0,
          width: 180,
          height: 100,
          items: [{ x: 70, y: 46, controls: ["c"], description: "", ...item } as PanelItem],
        },
      ],
    }) as Panel;

  let regions = $derived(pack?.content.controls?.overview?.regions ?? []);
  let views = $derived(pack?.content.controls?.overview?.views ?? null);
</script>

<main class="gallery" data-gallery-ready={pack ? "" : undefined}>
  <h1>Drawing renderer: every control and state ({dark ? "dark" : "light"})</h1>
  <p class="note">Invented one-control panels for review, not pack content.</p>
  {#each kinds as kind (kind.title)}
    <section class="kind" data-kind={kind.title}>
      <h2>{kind.title}</h2>
      <div class="row">
        {#each kind.states as state (state.name)}
          <figure class="cell" data-cell="{kind.title}: {state.name}">
            <div class="draw">
              <PanelSchematic
                panel={panelOf(kind.item)}
                focus={["s"]}
                targets={state.target ? [{ section: "s", item: 0, ...state.target }] : []}
                guardClosed={state.guardClosed ?? false}
              />
            </div>
            <figcaption>{state.name}</figcaption>
          </figure>
        {/each}
      </div>
    </section>
  {/each}

  {#if views}
    <section class="kind" data-kind="Where is this?">
      <h2>Where is this? side and seat views (from the pack)</h2>
      {#each regions as region (region.name)}
        {#if views.side?.places?.some((p) => p.region === region.name) || views.seat?.places?.some((p) => p.region === region.name)}
          <div class="row views" data-region={region.name}>
            {#if views.side}<ViewDrawing frame={views.side} region={region.name} kind="side" title="{region.name}: from the side" />{/if}
            {#if views.seat}<ViewDrawing frame={views.seat} region={region.name} kind="seat" title="{region.name}: from the captain seat" />{/if}
          </div>
        {/if}
      {/each}
    </section>
  {/if}
</main>

<style>
  .gallery {
    max-width: var(--size-content-max);
    margin: 0 auto;
    padding: var(--space-6) var(--gutter) var(--space-10);
    color: var(--color-text);
  }
  h1 {
    margin: 0 0 var(--space-2);
    font-size: var(--font-size-section-title);
  }
  h2 {
    margin: var(--space-6) 0 var(--space-3);
    font-size: var(--font-size-panel-title);
  }
  .note {
    margin: 0;
    color: var(--color-text-muted);
  }
  .row {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: var(--space-3);
  }
  .row.views {
    grid-template-columns: repeat(2, minmax(0, 1fr));
    margin-bottom: var(--space-3);
  }
  .cell {
    margin: 0;
    overflow: hidden;
    border: var(--border-thin) solid var(--panel-frame);
    border-radius: var(--radius-panel);
    background: var(--panel-bg);
  }
  .draw {
    padding: var(--space-2);
  }
  figcaption {
    padding: var(--space-2) var(--space-3);
    border-top: var(--border-thin) solid var(--color-border);
    background: var(--color-surface);
    font-size: var(--font-size-secondary);
  }
</style>
