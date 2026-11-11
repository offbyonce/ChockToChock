<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The app frame, laid out as in 3.0: the top bar with stage tabs, the phase chips, the screen for
the current address, and on wide screens the context panel for the selected step. State comes
from the host (decision 0012): a flight, settings, the rail and the phase on screen, all computed
in Rust. Progress lives in memory until batch 1 saves it on the device.
-->
<script lang="ts">
  import {
    type Action,
    type AppInfo,
    type Checklist,
    type Flight,
    isAppError,
    type ListView,
    type PhaseView,
    type Progress,
    type Rail,
    type Settings,
  } from "@chocktochock/bindings";
  import { getHost } from "./host";
  import ContextPane from "./lib/ContextPane.svelte";
  import Drawer from "./lib/Drawer.svelte";
  import FlightStrip from "./lib/FlightStrip.svelte";
  import FlowView from "./lib/FlowView.svelte";
  import PhaseChips from "./lib/PhaseChips.svelte";
  import TopBar from "./lib/TopBar.svelte";
  import { indexPack, type PackIndex } from "./pack-index";
  import PhasePage from "./pages/PhasePage.svelte";
  import ReferencePage from "./pages/ReferencePage.svelte";
  import SettingsPage from "./pages/SettingsPage.svelte";
  import { currentRoute } from "./route.svelte";
  import { applyPalette } from "./theme/tokens";

  const host = getHost();
  const route = currentRoute();

  let info = $state<AppInfo | null>(null);
  let packId = $state("");
  let index = $state<PackIndex | null>(null);
  let lists = $state(new Map<string, Checklist>());
  let intros = $state(new Map<string, { heading?: string | null; text: string }[]>());
  let settings = $state<Settings | null>(null);
  let flight = $state<Flight | null>(null);
  let rail = $state<Rail | null>(null);
  let view = $state<PhaseView | null>(null);
  let error = $state<string | null>(null);
  let menuOpen = $state(false);
  /** The step shown in the context panel; null means the current step. */
  let selected = $state<string | null>(null);
  /** The step Flow view shows; null follows the current step. */
  let cursor = $state<string | null>(null);

  function fail(e: unknown): void {
    error = isAppError(e) ? `${e.kind}: ${e.message}` : String(e);
  }

  async function start(): Promise<void> {
    try {
      info = await host.appInfo();
      const aircraft = (await host.packs()).find((p) => p.kind === "aircraft");
      if (!aircraft) throw new Error("No aircraft pack was found.");
      const pack = await host.loadPack(aircraft.id);
      packId = aircraft.id;
      index = indexPack(pack);
      lists = new Map(pack.lists.map((l) => [l.id, l]));
      intros = new Map((pack.content.phases?.phases ?? []).map((p) => [p.id, p.intro ?? []]));
      settings = await host.defaultSettings();
      flight = await host.newFlight(aircraft.id);
      rail = await host.rail(aircraft.id, flight);
    } catch (e) {
      fail(e);
    }
  }

  start();

  $effect(() => {
    if (!settings) return;
    const { theme, dimming } = settings;
    const query = window.matchMedia?.("(prefers-color-scheme: dark)");
    const apply = () => {
      const dark = theme === "dark" || (theme === "system" && (query?.matches ?? true));
      applyPalette(document.documentElement, dark, dimming);
    };
    apply();
    query?.addEventListener("change", apply);
    return () => query?.removeEventListener("change", apply);
  });

  let screen = $derived(route.value.screen);
  /** The phase on screen: a rail phase, or the open reference tab. */
  let phaseId = $derived.by(() => {
    const r = route.value;
    if (r.screen === "phase") return r.id;
    if (r.screen === "reference") return r.id ?? rail?.reference[0]?.id ?? null;
    if (r.screen === "start") return rail?.stages[0]?.phases[0]?.id ?? null;
    return null;
  });

  // A new screen starts on its current step.
  $effect(() => {
    void phaseId;
    selected = null;
    cursor = null;
  });

  $effect(() => {
    const f = flight;
    const id = phaseId;
    if (!f || !id || !packId) {
      view = null;
      return;
    }
    let stale = false;
    host.phaseView(packId, f, id).then(
      (v) => {
        if (!stale) view = v;
      },
      (e) => {
        if (stale) return;
        view = null;
        if (!(isAppError(e) && e.kind === "InvalidAction")) fail(e);
      },
    );
    return () => {
      stale = true;
    };
  });

  let listView = $derived<ListView>(settings?.listView ?? "checklist");
  let detail = $derived(settings?.detail ?? "expanded");
  let flow = $derived(screen !== "reference" && listView === "flow");
  let total = $derived<Progress>(
    (rail?.stages ?? [])
      .flatMap((s) => s.phases)
      .map((p) => p.progress)
      .reduce((a, b) => ({ done: a.done + b.done, skipped: a.skipped + b.skipped, toDo: a.toDo + b.toDo, total: a.total + b.total }), {
        done: 0,
        skipped: 0,
        toDo: 0,
        total: 0,
      }),
  );
  let stage = $derived(view?.stage ?? null);
  let stagePhases = $derived(
    screen === "phase" || screen === "start" ? (rail?.stages.find((s) => s.title === stage)?.phases ?? []) : [],
  );
  let current = $derived(view?.lists.find((l) => l.current)?.current ?? null);
  /** The step the context panel shows, its row, and the steps after it. */
  let pane = $derived.by(() => {
    if (!index || !view || screen === "settings") return null;
    const id = flow ? (cursor ?? current) : (selected ?? (screen === "reference" ? null : current));
    const rows = view.lists.flatMap((l) => l.steps);
    const at = rows.findIndex((r) => r.id === id);
    const row = rows[at];
    const step = id ? index.step(id) : undefined;
    if (!row || !step) return null;
    const upcoming = rows
      .slice(at + 1, at + 5)
      .map((r) => ({ step: index?.step(r.id), state: r.state }))
      .filter((u): u is { step: NonNullable<typeof u.step>; state: typeof u.state } => !!u.step);
    return { step, row, upcoming };
  });

  async function act(action: Action): Promise<void> {
    if (!flight) return;
    try {
      flight = await host.apply(packId, flight, action);
      rail = await host.rail(packId, flight);
    } catch (e) {
      fail(e);
    }
  }

  const tick = (id: string, done: boolean) => act(done ? { type: "untick", step: id } : { type: "tick", step: id, at: Date.now() });
  const skip = (id: string) => {
    cursor = null;
    return act({ type: "skip", step: id, at: Date.now() });
  };
  const confirm = (id: string) => {
    cursor = null;
    return act({ type: "tick", step: id, at: Date.now() });
  };
  const changeView = (v: ListView) => {
    if (settings) settings = { ...settings, listView: v };
  };
</script>

<div class="shell" data-app-ready={rail && (view || screen === "settings") ? "" : undefined}>
  <TopBar
    aircraft={index?.name ?? ""}
    {rail}
    {stage}
    view={settings && view && (screen === "phase" || screen === "start") ? listView : null}
    {menuOpen}
    onMenu={() => (menuOpen = !menuOpen)}
    onView={changeView}
  />
  {#if error}
    <p class="msg" role="alert">{error}</p>
  {:else if !rail || !settings || !flight || !index}
    <p class="msg">Loading…</p>
  {:else}
    <PhaseChips phases={stagePhases} current={phaseId} {detail} {total} />
    <Drawer
      open={menuOpen}
      {rail}
      {stage}
      {total}
      {detail}
      version={info?.version ?? null}
      onClose={() => (menuOpen = false)}
    />
    <div class="app" class:nopane={!pane} class:flow={flow && screen !== "settings"}>
      <main>
        <div class="inner">
          {#if screen === "settings"}
            <SettingsPage {settings} onChange={(s) => (settings = s)} />
          {:else if screen === "reference"}
            <ReferencePage
              {index}
              phases={rail.reference}
              {view}
              intro={view ? (intros.get(view.id) ?? []) : []}
              {lists}
              {flight}
              {detail}
              {selected}
              onTick={tick}
              onSkip={skip}
              onSelect={(id) => (selected = id)}
            />
          {:else if view}
            <FlightStrip />
            {#if flow}
              <FlowView {index} {view} {cursor} {detail} onConfirm={confirm} onSkip={skip} onMove={(id) => (cursor = id)} />
            {:else}
              <PhasePage
                {index}
                {view}
                {lists}
                {flight}
                {detail}
                {selected}
                onTick={tick}
                onSkip={skip}
                onSelect={(id) => (selected = id)}
                onOption={(option, value) => act({ type: "setOption", option, value })}
              />
            {/if}
          {:else}
            <p class="msg">That phase is not in this flight.</p>
          {/if}
          <p class="legal">
            Simulator training only. Not for real-world use. A study aid for flight simulation, not
            an approved checklist; follow your operator's or training provider's procedures.
          </p>
        </div>
      </main>
      {#if pane}
        <aside class="pane" aria-label="About the step">
          {#key pane.step.id}
            <ContextPane {index} step={pane.step} targets={pane.row.targets} upcoming={pane.upcoming} />
          {/key}
        </aside>
      {/if}
    </div>
  {/if}
</div>

<style>
  .app {
    display: grid;
    grid-template-columns: minmax(0, 1fr) clamp(24rem, 33vw, 40rem);
    align-items: start;
  }
  .app.nopane {
    grid-template-columns: minmax(0, 1fr);
  }
  main {
    min-width: 0;
    padding: 1.5rem clamp(1rem, 3vw, 2.5rem) 3rem;
  }
  .inner {
    max-width: 54rem;
    margin: 0 auto;
  }
  .nopane .inner {
    max-width: 70rem;
  }
  .flow .inner {
    max-width: 60rem;
  }
  .pane {
    position: sticky;
    top: calc(var(--top-h) + var(--rail-h));
    height: calc(100vh - var(--top-h) - var(--rail-h));
    padding: 1.375rem 1.25rem;
    overflow-y: auto;
    border-left: 1px solid var(--line);
    background: var(--surface);
  }
  .msg {
    padding: 1.5rem;
  }
  .legal {
    margin: 1.75rem 0 0;
    color: var(--stop);
    font-size: 0.7812rem;
  }
  /* Narrow screens show the context inside the step instead. */
  @media (max-width: 1023px) {
    .app {
      grid-template-columns: minmax(0, 1fr);
    }
    .pane {
      display: none;
    }
  }
</style>
