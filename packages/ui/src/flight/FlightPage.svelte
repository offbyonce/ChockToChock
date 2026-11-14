<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The Flight pages (FlightSection, Weather, Start and SimBrief boards), replacing Flight setup: the
shared header (Checklist back, the title and route, the SimBrief import time, Descriptions
On / Off), the section list (232) with "N to enter", the section, and "Find it in your OFP" beside
it. Every field of the old Flight setup lives in one section.
-->
<script lang="ts">
  import type { FillField, Flight, SavedPlan, Settings, StartState, UserOption } from "@chocktochock/bindings";
  import Icon from "../lib/Icon.svelte";
  import { FLIGHT_SECTIONS, flightHref, type FlightSection } from "../route.svelte";
  import OfpPanel from "./OfpPanel.svelte";
  import OverviewView from "./OverviewView.svelte";
  import SectionView from "./SectionView.svelte";
  import SimBriefView from "./SimBriefView.svelte";
  import StartView from "./StartView.svelte";
  import WeatherView from "./WeatherView.svelte";
  import { SECTIONS, toEnter, unitOf, type Ctx, type Ofp } from "./sections";

  let {
    section,
    focus,
    flight,
    settings,
    plan,
    options,
    states,
    backHref,
    onSet,
    onNotes,
    onOption,
    onStartState,
    onImport,
    onFile,
    onClear,
    onDescriptions,
  }: {
    section: FlightSection;
    /** A field to focus ("— enter …" links), or the Weather tab. */
    focus: string | null;
    flight: Flight;
    settings: Settings;
    plan: SavedPlan | null;
    options: UserOption[];
    states: StartState[];
    backHref: string;
    onSet: (field: FillField, value: string) => void;
    onNotes: (notes: string) => void;
    onOption: (option: string, value: string) => void;
    onStartState: (state: string | null) => void;
    onImport: (user: string) => Promise<string | null>;
    onFile: (text: string) => Promise<string | null>;
    onClear: () => void;
    onDescriptions: (on: boolean) => void;
  } = $props();

  const NAMES: Record<FlightSection, string> = {
    overview: "Overview",
    route: "Route",
    fuel: "Fuel and weights",
    cruise: "Cruise and levels",
    weather: "Weather",
    clearance: "Clearance",
    takeoff: "Takeoff",
    approach: "Approach",
    start: "Start options",
    simbrief: "SimBrief and OFP file",
  };
  const SUBS: Partial<Record<FlightSection, string>> = {
    overview: "This flight at a glance. Anything still to enter is listed first.",
    start: "How this flight begins. The steps adjust to match.",
    simbrief: "Bring your plan in once. Every page and step that needs a number uses it.",
  };

  let wxPanel = $state<Ofp | null>(null);
  let ctx = $derived<Ctx>({ values: flight.values ?? {}, plan, notes: flight.notes ?? "", unit: unitOf(flight) });
  let def = $derived(SECTIONS.find((s) => s.id === section) ?? null);
  /** The selected row of each section, which the OFP panel follows. */
  let picked = $state<Record<string, string>>({});
  // Each visit to a section starts on the board's selected row.
  let shownSection = $state<string | null>(null);
  $effect(() => {
    if (section !== shownSection) {
      shownSection = section;
      picked = {};
    }
  });
  let selected = $derived(def ? (picked[def.id] ?? def.selected) : "");
  let row = $derived(def?.groups.flatMap((g) => g.rows).find((r) => r.id === selected) ?? null);
  let route = $derived.by(() => {
    const o = flight.values.origin;
    const d = flight.values.destination;
    if (!o && !d) return "No flight entered yet";
    const alt = flight.values.alternate ? ` · alternate ${flight.values.alternate}` : "";
    return `${o || "—"} → ${d || "—"}${alt}`;
  });
  const time = (ms: number) => new Date(ms).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit", hourCycle: "h23" });
  let approach = $derived.by(() => {
    const option = options.find((o) => o.id === "approach");
    return option ? { option, value: flight.options.approach ?? option.default } : null;
  });

  // A "— enter …" link: select the field's row and put the cursor in its box.
  $effect(() => {
    const field = focus;
    if (!def || !field) return;
    const target = def.groups.flatMap((g) => g.rows).find((r) => r.field === field || r.second?.field === field);
    if (!target) return;
    picked = { ...picked, [def.id]: target.id };
    queueMicrotask(() => {
      const el = document.querySelector<HTMLElement>(`[data-row="${target.id}"] [data-field="${field}"]`);
      el?.focus({ preventScroll: true });
      el?.scrollIntoView({ block: "center" });
    });
  });
</script>

<div class="flight">
  <header class="fhead">
    <a class="back" href={backHref}><Icon name="chevron-left" />Checklist</a>
    <div class="ttl">
      <h1>Flight</h1>
      <span class="route">{route}</span>
    </div>
    <span class="fill"></span>
    {#if plan}<span class="from">From SimBrief, {time(plan.loadedAt)}</span>{/if}
    <div class="desc" role="group" aria-label="Field descriptions">
      Descriptions
      <span class="seg">
        <button type="button" aria-pressed={settings.descriptions} onclick={() => onDescriptions(true)}>On</button>
        <button type="button" aria-pressed={!settings.descriptions} onclick={() => onDescriptions(false)}>Off</button>
      </span>
    </div>
  </header>
  <div class="cols">
    <nav aria-label="Flight sections">
      {#each FLIGHT_SECTIONS as id (id)}
        {#if id === "start"}<span class="rule" aria-hidden="true"></span>{/if}
        {@const count = toEnter(flight.values, id).length}
        <a href={flightHref(id)} aria-current={section === id ? "page" : undefined} class:on={section === id}>
          {NAMES[id]}
          {#if count}<span class="count">{count} to enter</span>{/if}
        </a>
      {/each}
    </nav>
    <main class="fmain" class:wide={section === "start" || section === "simbrief"}>
      {#if section !== "weather"}
        <div class="title">
          <h2>{def?.title ?? NAMES[section]}</h2>
          {#if def?.sub ?? SUBS[section]}<span class="sub">{def?.sub ?? SUBS[section]}</span>{/if}
        </div>
      {/if}
      {#if section === "overview"}
        <OverviewView {ctx} descriptions={settings.descriptions} />
      {:else if section === "weather"}
        <WeatherView {flight} {plan} descriptions={settings.descriptions} tab={focus} {onSet} bind:panel={wxPanel} />
      {:else if section === "start"}
        <StartView {flight} {options} {states} {onOption} {onStartState} />
      {:else if section === "simbrief"}
        <SimBriefView user={settings.simbriefUser ?? null} {plan} {onImport} {onFile} {onClear} />
      {:else if def}
        <SectionView section={def} {ctx} {selected} descriptions={settings.descriptions} {approach} onSelect={(id) => (picked = { ...picked, [def.id]: id })} {onSet} {onOption} {onNotes} />
      {/if}
    </main>
    {#if section === "overview"}
      <OfpPanel ofp={null} {ctx} layout />
    {:else if section === "weather"}
      <OfpPanel ofp={wxPanel} {ctx} />
    {:else if def && row}
      <OfpPanel ofp={row.ofp} {ctx} />
    {/if}
  </div>
</div>

<style>
  .flight {
    display: flex;
    flex-direction: column;
    height: 100vh;
    height: 100dvh;
    background: var(--color-bg);
    color: var(--color-text);
  }
  .fhead {
    display: flex;
    flex: none;
    align-items: center;
    gap: var(--space-4);
    height: var(--size-header);
    padding: 0 var(--space-4);
    border-bottom: var(--border-thin) solid var(--color-line);
  }
  .back {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    height: var(--size-touch-min);
    padding: 0 var(--space-3);
    border: var(--border-thin) solid var(--color-border);
    border-radius: var(--radius-control);
    background: var(--color-button-bg);
    box-shadow: var(--color-button-shadow);
    color: var(--color-text);
    font-size: var(--font-size-body);
    text-decoration: none;
  }
  .back :global(.icon) {
    width: var(--size-icon-small);
    height: var(--size-icon-small);
  }
  .ttl {
    display: flex;
    flex-direction: column;
    line-height: var(--leading-tight);
  }
  h1 {
    margin: 0;
    font-size: var(--font-size-panel-title);
    font-weight: var(--weight-bold);
  }
  .route {
    color: var(--color-text-muted);
    font-family: var(--font-mono);
    font-size: var(--font-size-caption);
  }
  .fill {
    flex: 1;
  }
  .from {
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  .desc {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
  }
  .seg {
    display: flex;
    padding: var(--border-thick);
    border: var(--border-thin) solid var(--color-segment-border);
    border-radius: var(--radius-segment);
    background: var(--color-segment-track);
  }
  .seg button {
    height: var(--size-touch-min);
    padding: 0 calc(var(--space-3) + var(--space-1) / 2);
    border: 0;
    border-radius: var(--radius-control);
    background: transparent;
    color: var(--color-text-muted);
    font: inherit;
    font-size: var(--font-size-secondary);
  }
  .seg button[aria-pressed="true"] {
    background: var(--color-segment-on);
    box-shadow:
      inset 0 0 0 calc(var(--border-thick) * 3 / 4) var(--color-accent),
      var(--color-segment-shadow);
    color: var(--color-text-strong);
    font-weight: var(--weight-semibold);
  }
  .cols {
    display: flex;
    flex: 1;
    min-height: 0;
  }
  nav {
    display: flex;
    flex: none;
    flex-direction: column;
    gap: var(--border-thick);
    box-sizing: border-box;
    width: calc(5 * var(--space-10) + var(--space-8));
    padding: var(--space-4) var(--space-3);
    border-right: var(--border-thin) solid var(--color-line);
    overflow-y: auto;
  }
  nav a {
    display: flex;
    flex: none;
    align-items: center;
    justify-content: space-between;
    height: var(--size-touch-min);
    padding: 0 var(--space-3);
    border-radius: calc(var(--radius-small) + var(--border-thick));
    color: var(--color-text-secondary);
    font-size: var(--font-size-body);
    text-decoration: none;
  }
  nav a.on {
    background: var(--color-accent-soft);
    box-shadow: inset 0 0 0 calc(var(--border-thick) * 3 / 4) var(--color-accent);
    color: var(--color-text-strong);
    font-weight: var(--weight-semibold);
  }
  .count {
    color: var(--color-open-text);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-regular);
  }
  nav a.on .count {
    font-weight: var(--weight-semibold);
  }
  .rule {
    flex: none;
    height: var(--border-thin);
    margin: var(--space-2) var(--space-1);
    background: var(--color-line);
  }
  /* Each card keeps its height; the column scrolls (no hidden or squeezed rows). Scrollbars are
     hidden, as on the checklist (batch 2). */
  .fmain > :global(*) {
    flex: none;
  }
  .fmain,
  nav {
    scrollbar-width: none;
  }
  .fmain {
    display: flex;
    flex: 1;
    flex-direction: column;
    gap: var(--space-4);
    min-width: 0;
    padding: var(--space-6) calc(var(--space-6) + var(--space-1));
    overflow-y: auto;
  }
  .fmain.wide {
    padding: var(--space-6) var(--space-8);
  }
  .title {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }
  h2 {
    margin: 0;
    font-size: var(--font-size-section-title);
    font-weight: var(--weight-bold);
  }
  .sub {
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
  }
  /* Tablet portrait and phone keep every section reachable until batch 5's layouts: the
     section list becomes a row, and the OFP panel follows the section. */
  @media (max-width: 899px) {
    .cols {
      flex-direction: column;
      overflow-y: auto;
    }
    nav {
      flex-direction: row;
      width: 100%;
      padding: var(--space-2) var(--space-3);
      border-right: 0;
      border-bottom: var(--border-thin) solid var(--color-line);
      overflow-x: auto;
      overflow-y: visible;
    }
    nav a {
      gap: var(--space-2);
      white-space: nowrap;
    }
    .rule {
      width: var(--border-thin);
      height: auto;
      margin: var(--space-1) var(--space-2);
    }
    .fmain {
      flex: none;
      overflow: visible;
      padding: var(--space-4) var(--gutter);
    }
    .cols > :global(.ofp) {
      width: 100%;
      border-left: 0;
      border-top: var(--border-thin) solid var(--color-line);
      overflow: visible;
    }
    .fhead {
      flex-wrap: wrap;
      height: auto;
      padding-block: var(--space-2);
    }
  }
</style>
