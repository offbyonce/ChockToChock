<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The app frame: top bar, phase rail and the screen for the current address. State comes from the
host (decision 0012): a flight, settings and the rail, all computed in Rust.
-->
<script lang="ts">
  import { type AppInfo, type Flight, isAppError, type Rail, type Settings } from "@chocktochock/bindings";
  import { getHost } from "./host";
  import PhaseRail from "./lib/PhaseRail.svelte";
  import TopBar from "./lib/TopBar.svelte";
  import PhasePage from "./pages/PhasePage.svelte";
  import ReferencePage from "./pages/ReferencePage.svelte";
  import SettingsPage from "./pages/SettingsPage.svelte";
  import { currentRoute } from "./route.svelte";
  import { applyPalette } from "./theme/tokens";

  const host = getHost();
  const route = currentRoute();

  let info = $state<AppInfo | null>(null);
  let settings = $state<Settings | null>(null);
  let flight = $state<Flight | null>(null);
  let rail = $state<Rail | null>(null);
  let error = $state<string | null>(null);
  let menuOpen = $state(false);

  async function start(): Promise<void> {
    try {
      info = await host.appInfo();
      const aircraft = (await host.packs()).find((p) => p.kind === "aircraft");
      if (!aircraft) throw new Error("No aircraft pack was found.");
      await host.loadPack(aircraft.id);
      settings = await host.defaultSettings();
      flight = await host.newFlight(aircraft.id);
      rail = await host.rail(aircraft.id, flight);
    } catch (e) {
      error = isAppError(e) ? `${e.kind}: ${e.message}` : String(e);
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

  let firstPhase = $derived(rail?.stages[0]?.phases[0]?.id ?? null);
  let phaseId = $derived.by(() => {
    const r = route.value;
    if (r.screen === "phase") return r.id;
    return r.screen === "start" ? firstPhase : null;
  });
  let located = $derived.by(() => {
    if (!rail || !phaseId) return null;
    for (const stage of rail.stages) {
      const phase = stage.phases.find((p) => p.id === phaseId);
      if (phase) return { phase, stage: stage.title };
    }
    const phase = rail.reference.find((p) => p.id === phaseId);
    return phase ? { phase, stage: "Emergency" } : null;
  });
</script>

<div class="app" data-app-ready={rail ? "" : undefined}>
  <TopBar version={info?.version ?? null} {menuOpen} onMenu={() => (menuOpen = !menuOpen)} />
  {#if error}
    <p class="error" role="alert">{error}</p>
  {:else if !rail}
    <p class="loading">Loading…</p>
  {:else}
    <div class="body">
      <aside id="phase-rail" class:open={menuOpen}>
        <PhaseRail {rail} current={phaseId} onNavigate={() => (menuOpen = false)} />
      </aside>
      {#if menuOpen}
        <button class="scrim" type="button" aria-label="Close phases" onclick={() => (menuOpen = false)}></button>
      {/if}
      <main>
        {#if route.value.screen === "reference"}
          <ReferencePage phases={rail.reference} />
        {:else if route.value.screen === "settings"}
          <SettingsPage />
        {:else if located}
          <PhasePage phase={located.phase} stage={located.stage} />
        {:else}
          <p class="loading">That phase is not in this flight.</p>
        {/if}
      </main>
    </div>
  {/if}
</div>

<style>
  .app {
    display: flex;
    flex-direction: column;
    min-height: 100vh;
  }
  .body {
    position: relative;
    display: flex;
    flex: 1;
  }
  /* The rail keeps its own scroll under the sticky top bar (64px). */
  aside {
    position: sticky;
    top: 64px;
    width: 300px;
    height: calc(100vh - 64px);
    flex: none;
    overflow-y: auto;
    border-right: 1px solid var(--border);
    background: var(--panel);
  }
  main {
    flex: 1;
    min-width: 0;
  }
  .scrim {
    display: none;
  }
  .loading,
  .error {
    padding: 1.5rem;
  }
  .error {
    color: var(--accent);
  }
  @media (max-width: 1023px) {
    aside {
      position: fixed;
      z-index: 2;
      top: 64px;
      bottom: 0;
      left: 0;
      height: auto;
      transform: translateX(-100%);
      transition: transform 0.15s ease-out;
    }
    aside.open {
      transform: none;
      box-shadow: 0 0 24px rgb(0 0 0 / 45%);
    }
    .scrim {
      position: fixed;
      z-index: 1;
      inset: 64px 0 0;
      display: block;
      border: 0;
      background: rgb(0 0 0 / 40%);
    }
  }
  @media (prefers-reduced-motion: reduce) {
    aside {
      transition: none;
    }
  }
</style>
