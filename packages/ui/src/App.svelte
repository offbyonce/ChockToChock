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
    type OpenStep,
    type Settings,
  } from "@chocktochock/bindings";
  import { getHost, type SavedItem } from "./host";
  import StepPane from "./lib/StepPane.svelte";
  import Drawer from "./lib/Drawer.svelte";
  import FlightStrip from "./lib/FlightStrip.svelte";
  import FlowView from "./lib/FlowView.svelte";
  import PhaseChips from "./lib/PhaseChips.svelte";
  import TopBar from "./lib/TopBar.svelte";
  import { indexPack, type PackIndex } from "./pack-index";
  import KeptNotice from "./lib/KeptNotice.svelte";
  import OpenStepsCard from "./lib/OpenStepsCard.svelte";
  import OpenStepsPage from "./pages/OpenStepsPage.svelte";
  import PhasePage from "./pages/PhasePage.svelte";
  import ReferencePage from "./pages/ReferencePage.svelte";
  import SettingsPage from "./pages/SettingsPage.svelte";
  import SetupPage from "./pages/SetupPage.svelte";
  import { currentRoute } from "./route.svelte";
  import { applyPalette, applyScale } from "./theme/tokens";

  const host = getHost();
  const route = currentRoute();

  /**
   * The list (the page) and the side pane scroll on their own with no scrollbar, as on the boards;
   * edge fades show where there is more (the owner, batch 2). Wheel, trackpad, touch and keys
   * scroll them as usual.
   */
  let pageMore = $state({ up: false, down: false });
  let paneMore = $state({ up: false, down: false });
  const edges = (top: number, view: number, whole: number) => ({ up: top > 1, down: top + view < whole - 1 });
  $effect(() => {
    const read = () => {
      const d = document.documentElement;
      pageMore = edges(d.scrollTop, d.clientHeight, d.scrollHeight);
    };
    read();
    const watch = new ResizeObserver(read);
    watch.observe(document.body);
    window.addEventListener("scroll", read, { passive: true });
    window.addEventListener("resize", read);
    return () => {
      watch.disconnect();
      window.removeEventListener("scroll", read);
      window.removeEventListener("resize", read);
    };
  });
  function fades(el: HTMLElement) {
    const read = () => (paneMore = edges(el.scrollTop, el.clientHeight, el.scrollHeight));
    const watch = new ResizeObserver(read);
    watch.observe(el);
    for (const child of el.children) watch.observe(child);
    const added = new MutationObserver(() => {
      for (const child of el.children) watch.observe(child);
      read();
    });
    added.observe(el, { childList: true });
    el.addEventListener("scroll", read, { passive: true });
    read();
    return {
      destroy: () => {
        watch.disconnect();
        added.disconnect();
        el.removeEventListener("scroll", read);
      },
    };
  }
  let info = $state<AppInfo | null>(null);
  let packId = $state("");
  let index = $state<PackIndex | null>(null);
  let lists = $state(new Map<string, Checklist>());
  let intros = $state(new Map<string, { heading?: string | null; text: string }[]>());
  let settings = $state<Settings | null>(null);
  let flight = $state<Flight | null>(null);
  let rail = $state<Rail | null>(null);
  let open = $state<OpenStep[]>([]);
  /** Open steps from the phases before the one on screen. */
  let behind = $state<OpenStep[]>([]);
  /** When progress or settings were last saved on this device. */
  let savedAt = $state<number | null>(null);
  /** Saved data the app could not use, kept until the pilot decides (decision 0012). */
  let kept = $state<{ what: "flight" | "settings"; state: "newer" | "unreadable"; text: string }[]>([]);
  let view = $state<PhaseView | null>(null);
  /** In Essentials, the same phase in Full, for "Show N more steps from Full". */
  let fullView = $state<PhaseView | null>(null);
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
      // What is saved on this device, if anything; what cannot be used is kept, never dropped.
      const saved = await host.loadSaved(aircraft.id);
      const usable = <T,>(what: "flight" | "settings", item: SavedItem<T>): T | null => {
        if (!item) return null;
        if (item.loaded.state === "ready") return item.loaded.value;
        kept = [...kept, { what, state: item.loaded.state, text: item.text }];
        return null;
      };
      settings = usable("settings", saved.settings) ?? (await host.defaultSettings());
      const savedFlight = usable("flight", saved.flight);
      flight = savedFlight && savedFlight.pack === aircraft.id ? savedFlight : await host.newFlight(aircraft.id);
      await refresh();
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

  // The desktop scale and the Text and controls size (decision 0021).
  $effect(() => applyScale(document.documentElement, Number(settings?.textSize ?? "100")));

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
    const s = settings;
    if (!s) return;
    host.openSteps(packId, f, s, id).then((b) => {
      if (!stale) behind = b;
    });
    if (s.essentials) {
      host.phaseView(packId, f, { ...s, essentials: false }, id).then((v) => {
        if (!stale) fullView = v;
      });
    } else {
      fullView = null;
    }
    host.phaseView(packId, f, s, id).then(
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
  // Flight setup sits before the first stage, as in 3.0: that stage's tab and chips show, with no
  // chip current.
  let stage = $derived(screen === "setup" ? (rail?.stages[0]?.title ?? null) : (view?.stage ?? null));
  let stagePhases = $derived(
    screen === "phase" || screen === "start" || screen === "setup"
      ? (rail?.stages.find((s) => s.title === stage)?.phases ?? [])
      : [],
  );
  let current = $derived(view?.lists.find((l) => l.current)?.current ?? null);
  /** The step the context panel shows, its row, and the steps after it. */
  let pane = $derived.by(() => {
    if (!index || !view || screen === "settings" || screen === "setup" || screen === "open") return null;
    const id = flow ? (cursor ?? current) : (selected ?? (screen === "reference" ? null : current));
    const rows = view.lists.flatMap((l) => l.steps);
    const at = rows.findIndex((r) => r.id === id);
    const row = rows[at];
    const step = id ? index.step(id) : undefined;
    if (!row || !step) return null;
    const upcoming = rows
      .slice(at + 1, at + 9)
      .map((r) => ({ step: index?.step(r.id), state: r.state }))
      .filter((u): u is { step: NonNullable<typeof u.step>; state: typeof u.state } => !!u.step);
    const list = view.lists.find((l) => l.steps.some((r) => r.id === row.id));
    const previous = at > 0 ? index.step(rows[at - 1]?.id ?? "") : undefined;
    return { step, row, upcoming, total: list?.steps.length ?? rows.length, previous };
  });

  /** Hide map sets Map in steps to On request, keeping the choice for Show map to restore. */
  function changeMap(how: "hide" | "show"): void {
    if (!settings) return;
    const now = settings.mapInSteps;
    const next =
      how === "hide"
        ? { ...settings, mapInSteps: "on_request" as const, mapBeforeHide: now === "on_request" ? settings.mapBeforeHide : now }
        : { ...settings, mapInSteps: settings.mapBeforeHide === "on_request" ? ("new_panel" as const) : settings.mapBeforeHide };
    void changeSettings(next);
  }

  /** The rail and open steps for the flight and settings. */
  async function refresh(): Promise<void> {
    if (!flight || !settings) return;
    rail = await host.rail(packId, flight, settings);
    open = await host.openSteps(packId, flight, settings);
  }

  /** Saves on this device, unless saved data the app cannot use is waiting for the pilot. */
  async function persist(what: "flight" | "settings"): Promise<void> {
    const value = what === "flight" ? flight : settings;
    if (!value || kept.some((k) => k.what === what)) return;
    savedAt = await host.save(packId, what, value);
  }

  async function act(action: Action): Promise<void> {
    if (!flight) return;
    try {
      flight = await host.apply(packId, flight, action);
      await refresh();
      await persist("flight");
    } catch (e) {
      fail(e);
    }
  }

  /** A change the pilot can take back for a short while: the flight before it, and what it did. */
  let undo = $state<{ text: string; before: Flight } | null>(null);
  let undoTimer: ReturnType<typeof setTimeout> | undefined;
  function offerUndo(text: string, before: Flight): void {
    undo = { text, before };
    clearTimeout(undoTimer);
    undoTimer = setTimeout(() => (undo = null), 10000);
  }
  async function takeBack(): Promise<void> {
    if (!undo) return;
    flight = undo.before;
    undo = null;
    clearTimeout(undoTimer);
    try {
      await refresh();
      await persist("flight");
    } catch (e) {
      fail(e);
    }
  }
  async function clearFlight(): Promise<void> {
    if (!flight) return;
    const before = flight;
    await act({ type: "clear" });
    offerUndo("Flight cleared.", before);
  }

  async function changeSettings(next: Settings): Promise<void> {
    settings = next;
    try {
      await refresh();
      await persist("settings");
    } catch (e) {
      fail(e);
    }
  }

  async function startFresh(what: "flight" | "settings"): Promise<void> {
    await host.setAside(packId, what);
    kept = kept.filter((k) => k.what !== what);
    await persist(what);
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
    if (settings) void changeSettings({ ...settings, listView: v });
  };
</script>

<div class="shell" data-app-ready={rail && (view || screen === "settings" || screen === "setup" || screen === "open") ? "" : undefined}>
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
    <PhaseChips phases={stagePhases} current={phaseId} {detail} essentials={settings.essentials} {total} {savedAt} />
    <Drawer
      open={menuOpen}
      {rail}
      {stage}
      {total}
      {detail}
      essentials={settings.essentials}
      openCount={open.length}
      version={info?.version ?? null}
      onClose={() => (menuOpen = false)}
    />
    <div class="app" class:nopane={!pane} class:flow={flow && screen !== "settings" && screen !== "setup"}>
      <main>
        <div class="inner">
          {#each kept as k (k.what)}
            <KeptNotice what={k.what} state={k.state} text={k.text} onFresh={() => startFresh(k.what)} />
          {/each}
          {#if screen === "settings"}
            <SettingsPage {settings} onChange={(s) => void changeSettings(s)} />
          {:else if screen === "setup"}
            <SetupPage
              {flight}
              first={rail.stages[0]?.phases[0] ?? null}
              onSet={(field, value) => act({ type: "setValue", field, value })}
              onOption={(option, value) => act({ type: "setOption", option, value })}
            />
          {:else if screen === "open"}
            <OpenStepsPage
              {index}
              {open}
              onTick={(id) => act({ type: "tick", step: id, at: Date.now() })}
              onUnskip={(id) => act({ type: "unskip", step: id })}
            />
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
              onUnskip={(id) => act({ type: "unskip", step: id })}
              onBlank={(step, index, value) => act({ type: "setBlank", step, index, value })}
              onSelect={(id) => (selected = id)}
              onClear={(list) => act({ type: "clearList", list })}
            />
          {:else if view}
            <FlightStrip
              parts={view.strip}
              note={view.note}
              values={flight.values}
              notes={flight.notes}
              onSet={(field, value) => act({ type: "setValue", field, value })}
              onNotes={(notes) => act({ type: "setNotes", notes })}
              onClear={clearFlight}
            />
            {#if !flow}
              <OpenStepsCard {index} open={behind} onTick={(id) => act({ type: "tick", step: id, at: Date.now() })} />
            {/if}
            {#if flow}
              <FlowView {index} {view} {cursor} {detail} onConfirm={confirm} onSkip={skip} onMove={(id) => (cursor = id)} onBlank={(step, index, value) => act({ type: "setBlank", step, index, value })} />
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
                onUnskip={(id) => act({ type: "unskip", step: id })}
                onBlank={(step, index, value) => act({ type: "setBlank", step, index, value })}
                onSelect={(id) => (selected = id)}
                onClear={(list) => act({ type: "clearList", list })}
                full={!settings.essentials}
                {fullView}
                onOption={(option, value) => act({ type: "setOption", option, value })}
                onValue={(field, value) => act({ type: "setValue", field, value })}
              />
            {/if}
          {:else}
            <p class="msg">That phase is not in this flight.</p>
          {/if}
          {#if undo}
            <div class="toast" role="status">
              <span>{undo.text}</span>
              <button type="button" onclick={() => void takeBack()}>Undo</button>
            </div>
          {/if}
          <p class="legal">
            Simulator training only. Not for real-world use. A study aid for flight simulation, not
            an approved checklist; follow your operator's or training provider's procedures.
          </p>
        </div>
      </main>
      <div class="page-fade up" class:on={pageMore.up} aria-hidden="true"></div>
      <div class="page-fade down" class:on={pageMore.down} aria-hidden="true"></div>
      {#if pane}
        <aside class="pane" class:more-up={paneMore.up} class:more-down={paneMore.down} aria-label="About the step" use:fades>
          {#key pane.step.id}
            <StepPane
              {index}
              step={pane.step}
              targets={pane.row.targets}
              number={pane.row.number}
              total={pane.total}
              previous={pane.previous}
              upcoming={pane.upcoming}
              mapMode={settings?.mapInSteps ?? "new_panel"}
              onMap={changeMap}
              {flow}
            />
          {/key}
        </aside>
      {/if}
    </div>
  {/if}
</div>

<style>
  .app {
    display: grid;
    grid-template-columns: minmax(0, 1fr) var(--side-pane);
    align-items: start;
  }
  .app.nopane {
    grid-template-columns: minmax(0, 1fr);
  }
  main {
    min-width: 0;
    padding: var(--space-6) var(--gutter) var(--space-10);
  }
  /* The content stops at 1040 before scaling; extra width becomes margin (decision 0021). */
  .inner {
    max-width: var(--size-content-max);
    margin: 0 auto;
  }
  /* The Checklist board's side pane. */
  .pane {
    position: sticky;
    top: calc(var(--size-header) + var(--size-phase-row));
    height: calc(100vh - var(--size-header) - var(--size-phase-row));
    padding: var(--space-4) var(--space-6);
    /* It scrolls, but draws no scrollbar, as the Checklist board's pane. */
    overflow-y: auto;
    scrollbar-width: none;
    border-left: var(--border-thin) solid var(--color-line);
    background: var(--color-surface-2);
  }
  .pane::-webkit-scrollbar {
    display: none;
  }
  .pane.more-up {
    mask-image: linear-gradient(180deg, transparent, black var(--space-10));
  }
  .pane.more-down {
    mask-image: linear-gradient(180deg, black calc(100% - var(--space-10)), transparent);
  }
  .pane.more-up.more-down {
    mask-image: linear-gradient(180deg, transparent, black var(--space-10), black calc(100% - var(--space-10)), transparent);
  }
  /* The page draws no scrollbar either (the boards draw none). */
  :global(html) {
    scrollbar-width: none;
  }
  :global(html::-webkit-scrollbar) {
    display: none;
  }
  /* Edge fades over the list where it scrolls on, above and below. */
  .page-fade {
    position: fixed;
    left: 0;
    width: calc(100% - var(--side-pane));
    height: var(--space-10);
    pointer-events: none;
    opacity: 0;
  }
  .app.nopane .page-fade {
    width: 100%;
  }
  .page-fade.on {
    opacity: 1;
  }
  .page-fade.up {
    top: calc(var(--size-header) + var(--size-phase-row));
    background: linear-gradient(180deg, var(--color-bg), transparent);
  }
  .page-fade.down {
    bottom: 0;
    background: linear-gradient(0deg, var(--color-bg), transparent);
  }
  /* 3.0's toast: dark on light (light on dark), centered at the bottom. */
  .toast {
    position: fixed;
    bottom: var(--space-6);
    left: 50%;
    z-index: 70;
    display: flex;
    align-items: center;
    gap: var(--space-3);
    width: max-content;
    max-width: calc(100vw - 2 * var(--gutter));
    box-sizing: border-box;
    padding: var(--space-2) var(--space-4);
    border-radius: var(--radius-control);
    background: var(--color-text);
    box-shadow: none;
    color: var(--color-bg);
    font-size: var(--font-size-secondary);
    transform: translateX(-50%);
  }
  .toast button {
    flex: none;
    margin-left: var(--space-3);
    color: inherit;
    font-weight: var(--weight-semibold);
    text-decoration: underline;
  }
  .msg {
    padding: var(--space-6);
  }
  /* Muted, not red: it is a standing notice, not an error. */
  .legal {
    margin: var(--space-8) 0 0;
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  /* Narrow screens show the context inside the step instead. */
  @media (max-width: 899px) {
    .app,
    .app.flow {
      grid-template-columns: minmax(0, 1fr);
    }
    .pane {
      display: none;
    }
    .page-fade {
      width: 100%;
    }
  }
</style>
