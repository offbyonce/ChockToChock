<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Round 6's dense-panel gate (ROUND6.md), for review and screenshots only; unlinked and never part
of the installed app. For the three densest sections and the fuel panel: the label-free preview at
the 1440 side pane, the tablet landscape pane and the phone sheet widths, in the round 6 preview
card, beside the full labeled panel at its 11 px size. Each takes a real step's targets from the
pack (its `at` locators read here, for review only). `?view=gate&theme=dark|light`.
-->
<script lang="ts">
  import type { FlatPack, ItemTarget, Panel } from "@chocktochock/bindings";
  import { MIN_SCALE, positionsOf } from "../lib/panel-layout";
  import PanelPreview from "../lib/PanelPreview.svelte";
  import PanelSchematic from "../lib/PanelSchematic.svelte";
  import Slideout from "../lib/Slideout.svelte";
  import { indexPack } from "../pack-index";
  import { panelCaption } from "../lib/panel-name";
  import { applyPalette } from "../theme/tokens";

  let { packUrl }: { packUrl: string } = $props();
  const params = new URLSearchParams(location.search);
  applyPalette(document.documentElement, params.get("theme") !== "light", 0);

  let pack = $state<FlatPack | null>(null);
  $effect(() => {
    fetch(packUrl)
      .then((r) => r.json())
      .then((p: FlatPack) => {
        pack = p;
      });
  });

  /** The sections under review and the step whose targets each shows. */
  const GATE = [
    { panel: "mn", section: "cl", step: "ifly-737-max8.preflight.stab-out-of-trim-light", why: "captain lights" },
    { panel: "gl", section: "mcp", step: "ifly-737-max8.before-start.mcp", why: "the MCP" },
    { panel: "oh", section: "heat", step: "ifly-737-max8.before-taxi.probe-heat", why: "the densest overhead section" },
    { panel: "oh", section: "fuel", step: "ifly-737-max8.before-start.fuel-pump-switches", why: "the fuel panel" },
  ];
  /** Drawing widths: the preview card's inside at each size (the boards' card, 10/12/8 padding). */
  const SIZES = [
    { name: "1440 side pane", width: 446 },
    { name: "Tablet landscape pane", width: 366 },
    { name: "Phone sheet", width: 358 },
  ];

  type Raw = { at: string; set?: string | null; open?: boolean | null; light?: string | null };
  /** A step's targets as the app draws them, from "panel/section/LABEL/n" locators. */
  function targetsOf(p: Panel, step: { targets?: Raw[] | null }): ItemTarget[] {
    const out: ItemTarget[] = [];
    for (const t of step.targets ?? []) {
      const parts = t.at.split("/");
      const [panelId, sectionId] = parts;
      const n = Number(parts.at(-1));
      const label = parts.slice(2, -1).join("/");
      if (panelId !== p.id) continue;
      const section = p.sections.find((s) => s.id === sectionId);
      if (!section) continue;
      const matches = section.items.map((it, i) => ({ it, i })).filter(({ it }) => it.label === label);
      const hit = matches[n - 1];
      if (!hit) continue;
      const position = t.set ? positionsOf(hit.it).indexOf(t.set) : -1;
      out.push({
        section: section.id,
        item: hit.i,
        position: position >= 0 ? position : null,
        lit: t.light === "on",
        open: !!t.open,
        resolved: false,
      });
    }
    return out;
  }

  let index = $derived(pack ? indexPack(pack) : null);
  /** The slide-out at the boards' sizes: below the flight strip to the window's foot. */
  const ROOMS = [
    { name: "1440", width: 840, height: 732 },
    { name: "Tablet landscape", width: 700, height: 656 },
  ];
  let rows = $derived.by(() => {
    if (!pack) return [];
    const p = pack;
    const steps = p.lists.flatMap((l) => l.steps);
    return GATE.map((g) => {
      const panel = p.content.panels.find((x) => x.id === g.panel) as Panel;
      const section = panel.sections.find((s) => s.id === g.section);
      const step = steps.find((s) => s.id === g.step);
      return { ...g, panel, section, step, targets: step ? targetsOf(panel, step as { targets?: Raw[] }) : [] };
    });
  });
</script>

<main class="gate">
  <h1>Round 6 dense-panel gate: label-free previews beside the full panel</h1>
  {#if index}
    {#each rows.filter((r) => r.section?.id === "cl" || r.section?.id === "fuel") as r (r.section?.id)}
      {#if r.step}
        {#each ROOMS as room (room.name)}
          <section class="row" data-slideout="{r.section?.id}-{room.name === '1440' ? 'desktop' : 'tablet'}">
            <h2>Slide-out, {room.name} ({room.width} × {room.height}) · {r.step.text}</h2>
            <div class="slot" style:width="{room.width}px" style:height="{room.height}px">
              <Slideout {index} step={r.step} number={(r.step.number ?? 1)} targets={r.targets} onClose={() => {}} />
            </div>
          </section>
        {/each}
      {/if}
    {/each}
  {/if}
  {#each rows as r (r.section?.id)}
    {#if r.section}
      <section class="row" data-gate={r.section.id}>
        <h2>{panelCaption([r.section.title])} ({r.why}) · {r.step?.text}: {r.step?.expect}</h2>
        <div class="line">
          {#each SIZES as size (size.name)}
            <figure class="size">
              <figcaption>{size.name}, {size.width} px</figcaption>
              <div class="card" style:width="{size.width + 26}px">
                <PanelPreview panel={r.panel} focus={[r.section.id]} targets={r.targets} label="The whole {panelCaption([r.section.title])}" />
                <span class="cap">{panelCaption([r.section.title])} · blue: set by this step</span>
              </div>
            </figure>
          {/each}
        </div>
        <figure class="full">
          <figcaption>Full labeled panel, 11 px labels ({Math.round(r.section.width * MIN_SCALE)} px wide)</figcaption>
          <div class="fullwrap" style:width="{Math.round(r.section.width * MIN_SCALE) + 40}px">
            <PanelSchematic panel={r.panel} focus={[r.section.id]} targets={r.targets} large />
          </div>
        </figure>
      </section>
    {/if}
  {/each}
</main>

<style>
  .gate {
    padding: var(--space-6);
    background: var(--color-bg);
    color: var(--color-text);
    font-family: var(--font-sans);
  }
  h1 {
    margin: 0 0 var(--space-4);
    font-size: var(--font-size-section-title);
  }
  h2 {
    margin: 0 0 var(--space-3);
    font-size: var(--font-size-body);
  }
  .row {
    margin-bottom: var(--space-10);
  }
  .line {
    display: flex;
    flex-wrap: wrap;
    align-items: flex-start;
    gap: var(--space-6);
  }
  figure {
    margin: 0;
  }
  figcaption {
    margin-bottom: var(--space-2);
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  /* The round 6 preview card (round6/boards/Checklist.dc.html, r6 preview). */
  .card {
    box-sizing: border-box;
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    padding: var(--space-2) var(--space-3);
    border: var(--border-thin) solid var(--panel-frame);
    border-radius: var(--radius-control);
    background: var(--panel-bg);
  }
  .cap {
    color: var(--panel-label);
    font-size: var(--font-size-caption);
  }
  .full {
    margin-top: var(--space-4);
  }
  .slot {
    position: relative;
  }
  .fullwrap {
    max-width: none;
  }
</style>
