<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Weather (Weather.dc.html, Winds.dc.html): airport tabs, the METAR as printed with the selected
row's groups highlighted, the decoded rows as buttons (value, groups as printed, description,
where the value goes, ?), the Altimeter as an editable field on the Departure and Destination tabs
(filled from the METAR, overwritable, inHg / hPa), and the TAF as printed under its disclosure.
En route winds come from the plan's navigation log. Decoding is Rust's code table; a group it does
not cover shows as printed, "Not decoded". Descriptions are the boards' stand-ins (batch 7).
-->
<script lang="ts">
  import type { Decoded, DecodedRow, Element, FillField, Flight, SavedPlan } from "@chocktochock/bindings";
  import { getHost } from "../host";
  import Icon from "../lib/Icon.svelte";
  import { flightHref } from "../route.svelte";
  import { rangeMessage, type Ofp, type Part } from "./sections";

  let {
    flight,
    plan,
    descriptions,
    tab: wanted,
    onSet,
    panel = $bindable(null),
  }: {
    flight: Flight;
    plan: SavedPlan | null;
    descriptions: boolean;
    /** The tab to open: departure, destination, alternate or winds (or a field in it). */
    tab: string | null;
    onSet: (field: FillField, value: string) => void;
    /** The OFP panel for the selected row, which the page shows beside the section. */
    panel?: Ofp | null;
  } = $props();

  const host = getHost();
  let vals = $derived<Partial<Record<FillField, string>>>(flight.values ?? {});
  type Tab = "departure" | "destination" | "alternate" | "winds";
  // svelte-ignore state_referenced_locally
  let tab = $state<Tab>(wanted === "destination_qnh" || wanted === "destination" ? "destination" : wanted === "alternate" ? "alternate" : wanted === "winds" ? "winds" : "departure");
  $effect(() => {
    if (wanted === "destination_qnh" || wanted === "destination") tab = "destination";
    else if (wanted === "qnh" || wanted === "departure") tab = "departure";
    // A step's "— enter QNH" link: the cursor goes to the Altimeter box.
    if (wanted === "qnh" || wanted === "destination_qnh") setTimeout(() => document.getElementById("wx-alt")?.focus(), 0);
  });

  let TABS = $derived<{ id: Tab; label: string }[]>([
    { id: "departure", label: `Departure ${vals.origin ?? ""}`.trim() },
    { id: "destination", label: `Destination ${vals.destination ?? ""}`.trim() },
    { id: "alternate", label: `Alternate ${vals.alternate ?? ""}`.trim() },
    { id: "winds", label: "En route winds" },
  ]);
  let airport = $derived(
    tab === "departure" ? plan?.plan.origin : tab === "destination" ? plan?.plan.destination : tab === "alternate" ? plan?.plan.alternates[0] : undefined,
  );
  let icao = $derived(
    tab === "departure" ? vals.origin : tab === "destination" ? vals.destination : tab === "alternate" ? vals.alternate : "",
  );
  let metar = $derived(airport?.metar ?? null);
  let taf = $derived(airport?.taf ?? null);

  let decoded = $state<Decoded | null>(null);
  $effect(() => {
    const text = metar;
    decoded = null;
    if (text) void host.decodeMetar(text).then((d) => (decoded = d));
  });

  /** The rows shown: the weather itself, not the report's kind, station or time. */
  const HIDDEN: Element[] = ["kind", "station", "time"];
  let rows = $derived((decoded?.rows ?? []).map((r, i) => ({ r, i })).filter(({ r }) => !HIDDEN.includes(r.element)));
  let picked = $state<number | null>(null);
  let selected = $derived(picked !== null && rows.some((x) => x.i === picked) ? picked : (rows.find((x) => x.r.element === "wind")?.i ?? rows[0]?.i ?? null));
  let altRow = $derived(rows.find((x) => x.r.element === "altimeter") ?? null);
  let field = $derived<FillField | null>(tab === "departure" ? "qnh" : tab === "destination" ? "destination_qnh" : null);
  let unit = $derived(vals.qnh_unit ?? (altRow?.r.raw[0]?.startsWith("Q") ? "hPa" : "inHg"));
  let more = $state<number | null>(null);
  let altMessage = $derived(field ? rangeMessage(field, { values: vals, plan, notes: "", unit: "" }) : null);
  let tafOpen = $state(false);

  const LABELS: Partial<Record<Element, string>> = {
    auto: "Report",
    validity: "Valid",
    wind: "Wind",
    wind_variation: "Wind",
    visibility: "Visibility",
    runway_visual_range: "Runway visual range",
    weather: "Weather",
    no_weather: "Weather",
    clouds: "Clouds",
    cavok: "CAVOK",
    temperature: "Temperature",
    altimeter: "Altimeter",
    temperature_extreme: "Temperatures",
    no_change: "Trend",
    change: "Change",
    remarks: "Remarks",
    not_decoded: "Not decoded",
  };
  const DESC: Partial<Record<Element, string>> = {
    wind: "Where the wind blows from, in degrees true, then its speed in knots. It decides the runway in use and your crosswind on takeoff.",
    visibility: "How far you can see along the ground. 10 SM is the most a US report shows.",
    clouds: "Sky cover and the base above the airport, in hundreds of feet. Broken or overcast is the first layer that counts as a ceiling.",
    temperature: "Feeds takeoff performance and the engine anti-ice decision. When the two get close, fog and low cloud become likely.",
    altimeter: "The pressure setting that makes the altimeters read height above sea level here. Filled from the METAR; type a newer one from the ATIS. Used by the altimeter step.",
    runway_visual_range: "How far along the runway you can see its lights, for each runway reported.",
    weather: "What is falling or hanging in the air: rain, snow, mist, fog and the like, with how heavy it is.",
    cavok: "Ceiling and visibility OK: good visibility, no cloud that matters and no significant weather.",
    auto: "The report came from an automatic station, with no observer.",
    no_change: "The forecaster expects no significant change in the next two hours.",
    change: "A change the report expects, and when.",
    remarks: "Extra notes the station adds. Some are decoded; the rest are shown as printed.",
    not_decoded: "This group isn't in the app's code table, so it's shown as printed, not guessed.",
  };
  /** Where a decoded value goes in the cockpit, as the Weather board words it. */
  const LOC: Partial<Record<Element, string>> = { wind: "Enter on the CDU › TAKEOFF REF", temperature: "Enter on the CDU › TAKEOFF REF" };
  /**
   * Where a row's code is defined, in plain words (the code table's verified section, owner
   * 2026-11-14): "FM 15 · 15.5" is regulation 15.5 of the WMO's METAR code; "US · 3.1.5.6" is that
   * section of the FAA's aviation weather guidance. A row with no verified section says nothing.
   */
  function definedIn(source: string): string {
    const [code, section] = source.split(" · ");
    if (!section) return "";
    const part = section.startsWith("note") ? `${section} to the code` : `regulation ${section}`;
    if (code === "FM 15") return `Defined in the WMO's METAR code (FM 15), ${part}.`;
    if (code === "FM 51") return `Defined in the WMO's TAF code (FM 51), ${part}.`;
    if (code === "US") return `US practice, defined in the FAA's aviation weather guidance, section ${section}.`;
    return "";
  }
  const ORDINAL = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

  /** The METAR as printed: each group, highlighted when the selected row reads it. */
  let printed = $derived((decoded?.groups ?? []).map(([text, row]) => ({ text, hi: row === selected })));

  function setAltimeter(text: string): void {
    if (!field) return;
    onSet(field, text.trim().replace(/[^0-9.]/g, ""));
  }
  /** The other unit: both airports' settings follow (1 inHg is 33.8639 hPa). */
  function setUnit(next: "inHg" | "hPa"): void {
    if (next === unit) return;
    onSet("qnh_unit", next);
    for (const f of ["qnh", "destination_qnh"] as const) {
      const n = Number(vals[f]);
      if (!vals[f] || !Number.isFinite(n)) continue;
      onSet(f, next === "hPa" ? String(Math.round(n * 33.8639)) : (n / 33.8639).toFixed(2));
    }
  }

  let tafLines = $derived(
    (taf ?? "")
      .replace(/\s+/g, " ")
      .trim()
      .split(/ (?=(?:FM\d{6}|TEMPO|BECMG|PROB\d0)\b)/)
      .reduce<string[]>((out, part) => {
        // PROB30 TEMPO stays on one line.
        if (/^TEMPO/.test(part) && /PROB\d0$/.test(out.at(-1) ?? "")) out[out.length - 1] += ` ${part}`;
        else out.push(part);
        return out;
      }, []),
  );
  const time = (t: [number, number, number] | null | undefined) => (t ? `Observed ${String(t[1]).padStart(2, "0")}:${String(t[2]).padStart(2, "0")} UTC` : "");
  const loaded = (ms: number) => new Date(ms).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit", hourCycle: "h23" });

  // The OFP panel follows the selected row.
  $effect(() => {
    panel = ofp;
  });
  let ofp = $derived.by<Ofp | null>(() => {
    if (tab === "winds") return windsOfp;
    const row = rows.find((x) => x.i === selected)?.r;
    if (!row || !decoded) return null;
    const label = LABELS[row.element] ?? "This group";
    const first = decoded.groups.findIndex(([, r]) => r === selected);
    // The weather section prints the report on two lines (Weather.dc.html's close-up).
    const groups = decoded.groups;
    const half = Math.ceil(groups.length / 2);
    const parts: Part[] = groups.flatMap(([text, r], n): Part[] => [
      { text, hi: r === selected },
      ...(n === groups.length - 1 ? [] : [n === half - 1 ? { text: "", br: true } : { text: " " }]),
    ]);
    return {
      section: 5,
      title: `${label} is in the weather section`,
      body: "The OFP is the document SimBrief prints for the flight. Its sections come in the same order every time.",
      cap: "Close-up, weather section",
      close: () => parts,
      note: first >= 0 ? `${label} is the ${ORDINAL[first] ?? `${first + 1}th`} group${first === 2 ? ", after the airport and the time" : ""}.` : "",
    };
  });

  // En route winds: the plan's navigation log.
  type WindRow = { fix: string; level: string; wind: string; temp: string; mark: string };
  const level = (ft: number | undefined) => (ft == null ? "—" : ft >= 18000 ? `FL${Math.round(ft / 100)}` : `${Math.round(ft).toLocaleString("en-US")}`);
  const wind = (d: number | undefined, s: number | undefined) => (d == null || s == null ? "—" : `${String(d).padStart(3, "0")}/${String(s).padStart(3, "0")}`);
  const temp = (t: number | undefined) => (t == null ? "—" : t > 0 ? `+${t}` : `${t}`);
  let fixes = $derived(plan?.plan.fixes ?? []);
  let surface = $state<{ dep: string; dest: string }>({ dep: "—", dest: "—" });
  $effect(() => {
    const dep = plan?.plan.origin?.metar;
    const dest = plan?.plan.destination?.metar;
    const read = async (m: string | null | undefined) => {
      if (!m) return { w: "—", t: "—" };
      const d = await host.decodeMetar(m);
      const wg = d.groups.find(([, r]) => d.rows[r]?.element === "wind")?.[0] ?? "";
      const tg = d.groups.find(([, r]) => d.rows[r]?.element === "temperature")?.[0] ?? "";
      const w = /^(\d{3}|VRB)(\d{2,3})/.exec(wg);
      const t = /^(M?\d{2})\//.exec(tg)?.[1]?.replace("M", "-");
      return { w: w ? `${w[1]}/${w[2]?.padStart(2, "0")}` : "—", t: t ? temp(Number(t)) : "—" };
    };
    void Promise.all([read(dep), read(dest)]).then(([a, b]) => (surface = { dep: `${a.w}|${a.t}`, dest: `${b.w}|${b.t}` }));
  });
  let windRows = $derived.by<WindRow[]>(() => {
    const out: WindRow[] = [];
    const [dw, dt] = surface.dep.split("|");
    out.push({ fix: vals.origin || "—", level: "—", wind: dw ?? "—", temp: dt ?? "—", mark: "Departure surface wind" });
    for (const f of fixes) {
      if (f.windDir == null && f.oat == null) continue;
      const id = (f.ident ?? "").toUpperCase();
      out.push({
        fix: id || "—",
        level: level(f.altitudeFt),
        wind: wind(f.windDir, f.windSpeed),
        temp: temp(f.oat),
        mark: id === "TOC" ? "Top of climb" : id === "TOD" ? "Top of descent" : "",
      });
    }
    const [aw, at] = surface.dest.split("|");
    out.push({ fix: vals.destination || "—", level: "—", wind: aw ?? "—", temp: at ?? "—", mark: "Destination surface wind" });
    return out;
  });
  /** Three levels on the way down: the top of descent, the middle, and the lowest at or above 10,000 ft. */
  let descent = $derived.by(() => {
    const down = fixes.filter((f) => (f.stage ?? "").toUpperCase().startsWith("D") && f.altitudeFt != null && f.windDir != null);
    if (!down.length) return "—";
    const sorted = [...down].sort((a, b) => (b.altitudeFt ?? 0) - (a.altitudeFt ?? 0));
    const top = sorted[0];
    const low = [...sorted].reverse().find((f) => (f.altitudeFt ?? 0) >= 10000) ?? sorted.at(-1);
    const midAlt = ((top?.altitudeFt ?? 0) + (low?.altitudeFt ?? 0)) / 2;
    const mid = sorted.reduce<(typeof sorted)[number] | undefined>(
      (best, f) => (!best || Math.abs((f.altitudeFt ?? 0) - midAlt) < Math.abs((best.altitudeFt ?? 0) - midAlt) ? f : best),
      undefined,
    );
    return [top, mid, low]
      .filter((f, n, all) => f && all.indexOf(f) === n)
      .map((f) => `FL${Math.round((f?.altitudeFt ?? 0) / 100)} ${wind(f?.windDir, f?.windSpeed)}`)
      .join(" · ");
  });
  let windsOfp = $derived<Ofp>({
    section: 3,
    title: "Winds are in the navigation log",
    body: "The OFP is the document SimBrief prints for the flight. Its sections come in the same order every time.",
    cap: "Close-up, one navigation log line",
    close: () => {
      const cruise = fixes.filter((f) => f.windDir != null && (f.stage ?? "").toUpperCase() === "CRZ").slice(0, 2);
      if (!cruise.length) return [{ text: "—" }];
      return cruise.flatMap((f, n) => {
        const t = f.oat == null ? "—" : f.oat < 0 ? `M${-f.oat}` : `${f.oat}`;
        return [
          { text: `${(f.ident ?? "").toUpperCase()} ${level(f.altitudeFt)} ` },
          { text: wind(f.windDir, f.windSpeed), hi: n === 0 },
          { text: ` ${t}`, br: n === 0 },
        ];
      });
    },
    note: "Each fix shows its level, then the wind (direction/speed), then the temperature. M means minus.",
  });
</script>

<div class="wx">
  <div class="top">
    <div class="tabs" role="tablist" aria-label="Airport">
      {#each TABS as t (t.id)}
        <button
          type="button"
          role="tab"
          aria-selected={tab === t.id}
          onclick={() => {
            tab = t.id;
            picked = null;
            more = null;
          }}>{t.label}</button
        >
      {/each}
    </div>
    {#if tab === "winds"}
      {#if plan}<span class="obs">Forecast from the plan, {loaded(plan.loadedAt)}</span>{/if}
    {:else if decoded}
      <span class="obs">{time(decoded.time)}</span>
    {/if}
  </div>

  {#if tab === "winds"}
    <div class="wt">
      <h2>Winds along your route</h2>
      <span class="sub">Forecast winds and temperatures at your planned levels. The FMC can update them in flight.</span>
    </div>
    {#if plan}
      <section class="card">
        <div class="ghead"><h3>Where these go</h3></div>
        <div class="wrow">
          <span class="label">Cruise wind</span>
          <span class="mono">{vals.cruise_wind || "—"}</span>
          <span class="desc">{descriptions ? "The average at cruise level, for the CDU’s performance page. Also in Cruise and levels." : ""}</span>
          <span class="loc">Enter on the CDU › PERF INIT</span>
        </div>
        <div class="wrow">
          <span class="label">Descent forecast</span>
          <span class="mono">{descent}</span>
          <span class="desc">{descriptions ? "Winds at three levels on the way down, so the descent path is planned with them." : ""}</span>
          <span class="loc">Enter on the CDU › DES FORECAST</span>
        </div>
      </section>
      <section class="card" aria-label="Wind at each fix">
        <div class="ghead"><h3>Wind at each fix, from the navigation log</h3></div>
        <div class="trow head"><span>FIX</span><span>LEVEL</span><span>WIND</span><span>TEMP °C</span><span></span></div>
        {#each windRows as w, n (n)}
          <div class="trow"><span>{w.fix}</span><span>{w.level}</span><span>{w.wind}</span><span>{w.temp}</span><span class="mark">{w.mark}</span></div>
        {/each}
      </section>
    {:else}
      <p class="empty">
        Import your SimBrief plan to see the weather, or enter the departure and destination.
        <a href={flightHref("simbrief")}>SimBrief and OFP file</a> · <a href={flightHref("route")}>Route</a>
      </p>
    {/if}
  {:else}
    {#if decoded}
      <section class="card metar">
        <span class="mt">METAR, the current report as printed</span>
        <div class="groups">
          {#each printed as g, n (n)}<span class:hi={g.hi}>{g.text}</span>{/each}
        </div>
      </section>
    {:else}
      <p class="empty">
        Import your SimBrief plan to see the weather, or enter the departure and destination.
        <a href={flightHref("simbrief")}>SimBrief and OFP file</a> · <a href={flightHref("route")}>Route</a>
      </p>
    {/if}
    {#if decoded || field}
      <section class="card rows">
        {#each rows as { r, i } (i)}
          {#if r.element === "altimeter" && field}
            {@render altimeterRow(r, i)}
          {:else}
            <button class="xrow" type="button" aria-pressed={selected === i} onclick={() => (picked = i)}>
              <span class="label">{LABELS[r.element] ?? r.element}</span>
              <span class="val">
                {#if r.lines.length}<span class="mono">{#each r.lines as l, n (n)}{l}{#if n < r.lines.length - 1}<br />{/if}{/each}</span>{:else}<span class="mono">Not decoded</span>{/if}
                <span class="raw">{r.raw.join(" ")}</span>
              </span>
              <span class="desc">{descriptions || more === i ? (DESC[r.element] ?? "") : ""}{#if more === i && r.source}<span class="src"> {definedIn(r.source)}</span>{/if}</span>
              <span class="loc">{LOC[r.element] ?? ""}</span>
              <span
                class="q"
                role="button"
                tabindex="0"
                aria-label="More about {(LABELS[r.element] ?? r.element).toLowerCase()}"
                aria-expanded={more === i}
                onclick={(e) => {
                  e.stopPropagation();
                  more = more === i ? null : i;
                }}
                onkeydown={(e) => {
                  if (e.key === "Enter" || e.key === " ") {
                    e.preventDefault();
                    e.stopPropagation();
                    more = more === i ? null : i;
                  }
                }}>?</span
              >
            </button>
          {/if}
        {/each}
        {#if field && !altRow}
          {@render altimeterRow(null, -1)}
        {/if}
      </section>
    {/if}
    {#if taf}
      <button class="taf" type="button" aria-expanded={tafOpen} onclick={() => (tafOpen = !tafOpen)}>TAF: the forecast for {icao || airport?.icao || ""} {tafOpen ? "⌃" : "⌄"}</button>
      {#if tafOpen}<pre class="tafp">{tafLines.join("\n")}</pre>{/if}
    {/if}
  {/if}
</div>

{#snippet altimeterRow(r: DecodedRow | null, i: number)}
  <div class="xrow alt" class:sel={selected === i && i >= 0}>
    <label class="label" for="wx-alt">Altimeter</label>
    <span class="val">
      <input id="wx-alt" class="box" data-field={field} class:need={!vals[field ?? "qnh"]} value={vals[field ?? "qnh"] ?? ""} placeholder="Enter" inputmode="decimal" aria-invalid={field && altMessage ? "true" : undefined} aria-describedby={field && altMessage ? "wx-alt-err" : undefined} onchange={(e) => setAltimeter(e.currentTarget.value)} />
      {#if r}<span class="raw">From METAR {r.raw.join(" ")}</span>{/if}
      {#if field && altMessage}<span class="err" id="wx-alt-err" role="alert"><Icon name="field-alert" />{altMessage}</span>{/if}
    </span>
    <span class="about">
      {#if descriptions || more === i}<span class="desc">{DESC.altimeter}{#if more === i && r?.source}<span class="src"> {definedIn(r.source)}</span>{/if}</span>{/if}
      <span class="seg" role="group" aria-label="Altimeter unit">
        {#each ["inHg", "hPa"] as u (u)}
          <button type="button" aria-pressed={unit === u} onclick={() => setUnit(u as "inHg" | "hPa")}>{u}</button>
        {/each}
      </span>
    </span>
    <span class="loc">Set on the EFIS › BARO</span>
    <button class="q" type="button" aria-label="More about the altimeter setting" aria-expanded={more === i} onclick={() => (more = more === i ? null : i)}>?</button>
  </div>
{/snippet}


<style>
  .wx {
    display: contents;
  }
  .top {
    display: flex;
    align-items: center;
    justify-content: space-between;
  }
  .tabs {
    display: flex;
    padding: var(--border-thick);
    border: var(--border-thin) solid var(--color-segment-border);
    border-radius: var(--radius-segment);
    background: var(--color-segment-track);
  }
  .tabs button,
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
  .tabs button[aria-selected="true"],
  .seg button[aria-pressed="true"] {
    background: var(--color-segment-on);
    outline: var(--size-focus-ring) solid var(--color-accent);
    outline-offset: calc(-1 * var(--size-focus-ring));
    box-shadow: var(--color-segment-shadow);
    color: var(--color-text-strong);
    font-weight: var(--weight-semibold);
  }
  .seg {
    display: inline-flex;
    padding: var(--border-thick);
    border: var(--border-thin) solid var(--color-segment-border);
    border-radius: var(--radius-control);
    background: var(--color-segment-track);
  }
  .seg button {
    height: calc(var(--size-touch-min) - var(--space-1));
    min-width: var(--size-touch-min);
    padding: 0 calc(var(--space-2) + var(--border-thick));
    border-radius: calc(var(--radius-small) + var(--border-thick));
  }
  .obs {
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  .card {
    flex: none;
    margin: 0;
    max-width: calc(26 * var(--space-10));
    overflow: hidden;
    border: var(--border-thin) solid var(--color-border);
    border-radius: var(--radius-panel);
    background: var(--color-surface);
  }
  .metar {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    padding: var(--space-4) calc(var(--space-4) + var(--space-1));
  }
  .mt {
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
  }
  .groups {
    display: flex;
    flex-wrap: wrap;
    gap: calc(var(--space-1) + var(--border-thick));
    font-family: var(--font-mono);
    font-size: var(--font-size-panel-title);
  }
  .groups span {
    padding: var(--border-thick) calc(var(--space-1) + var(--border-thick));
    border-radius: var(--radius-small);
    color: var(--color-text-secondary);
  }
  .groups .hi {
    background: var(--color-accent-soft);
    box-shadow: inset 0 0 0 calc(var(--border-thick) * 3 / 4) var(--color-accent);
    color: var(--color-text-strong);
  }
  .rows {
    display: flex;
    flex-direction: column;
  }
  .xrow {
    flex: none;
    display: grid;
    /* Weather.dc.html: 136, 160, the description, 44; the location line under the description. */
    grid-template-columns:
      calc(3 * var(--space-10) + var(--space-4))
      calc(4 * var(--space-10))
      minmax(0, 1fr)
      var(--size-touch-min);
    grid-template-areas: "l v d q" ". . c .";
    column-gap: var(--space-4);
    row-gap: var(--space-1);
    align-items: start;
    padding: var(--space-3) var(--space-1) var(--space-3) calc(var(--space-4) + var(--space-1));
    border: 0;
    border-bottom: var(--border-thin) solid var(--color-line);
    background: transparent;
    color: inherit;
    font: inherit;
    text-align: left;
  }
  .xrow:last-child {
    border-bottom: 0;
  }
  .xrow[aria-pressed="true"],
  .xrow.sel {
    background: var(--color-current-row);
    box-shadow: inset var(--border-bar) 0 0 var(--color-accent);
  }
  .label {
    font-size: var(--font-size-body);
    font-weight: var(--weight-semibold);
  }
  .alt .label {
    padding-top: calc(var(--space-2) + var(--border-thick));
  }
  .val {
    display: flex;
    flex-direction: column;
    gap: var(--border-thick);
  }
  .alt .val {
    gap: var(--space-1);
  }
  .mono {
    font-family: var(--font-mono);
    font-size: var(--font-size-value-mono);
  }
  .raw {
    color: var(--color-text-muted);
    font-family: var(--font-mono);
    font-size: var(--font-size-caption);
  }
  .desc {
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
    line-height: var(--leading-normal);
  }
  .src {
    color: var(--color-text-muted);
  }
  .about {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: var(--space-2);
  }
  .xrow > .label,
  .wrow > .label {
    grid-area: l;
  }
  .xrow > .val,
  .wrow > .mono {
    grid-area: v;
  }
  .xrow > .desc,
  .xrow > .about,
  .wrow > .desc {
    grid-area: d;
  }
  .loc {
    grid-area: c;
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
    line-height: var(--leading-normal);
  }
  .xrow > .q {
    grid-area: q;
  }
  .q {
    display: flex;
    align-items: center;
    justify-content: center;
    width: var(--size-touch-min);
    height: var(--size-touch-min);
    border: 0;
    background: transparent;
    color: var(--color-text-muted);
    font: inherit;
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
  }
  .box {
    width: 100%;
    height: var(--size-touch-min);
    box-sizing: border-box;
    padding: 0 calc(var(--space-2) + var(--border-thick));
    border: var(--border-thin) solid var(--color-field-border);
    border-radius: calc(var(--radius-small) + var(--border-thick));
    background: var(--color-field);
    color: var(--color-text);
    font-family: var(--font-mono);
    font-size: var(--font-size-value-mono);
  }
  .box.need {
    border-color: var(--color-open-text);
  }
  .box[aria-invalid="true"] {
    border-color: var(--color-danger-text);
  }
  /* FSInvalid: the alert icon and "Enter X to Y", 12 px, in the red text color. */
  .err {
    display: flex;
    align-items: flex-start;
    gap: calc(var(--space-1) + var(--border-thick));
    color: var(--color-danger-text);
    font-size: var(--font-size-caption);
    line-height: var(--leading-snug);
  }
  .err :global(.icon) {
    flex: none;
    width: calc(var(--size-icon-small) - var(--border-thick));
    height: calc(var(--size-icon-small) - var(--border-thick));
    margin-top: var(--border-thin);
  }
  .taf {
    align-self: flex-start;
    min-height: var(--size-touch-min);
    padding: 0;
    border: 0;
    background: none;
    color: var(--color-link);
    font: inherit;
    font-size: var(--font-size-secondary);
  }
  .tafp,
  .metar,
  .empty,
  .wt,
  .top,
  .taf {
    flex: none;
  }
  .tafp {
    max-width: calc(26 * var(--space-10));
    margin: 0;
    padding: var(--space-4) calc(var(--space-4) + var(--space-1));
    border: var(--border-thin) solid var(--color-border);
    border-radius: var(--radius-panel);
    background: var(--color-surface);
    color: var(--color-text);
    font-family: var(--font-mono);
    font-size: var(--font-size-secondary);
    line-height: var(--leading-normal);
    white-space: pre-wrap;
  }
  .empty {
    max-width: calc(26 * var(--space-10));
    margin: 0;
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
    line-height: var(--leading-normal);
  }
  .empty a {
    color: var(--color-link);
  }
  .wt {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }
  .wt h2 {
    margin: 0;
    font-size: var(--font-size-panel-title);
    font-weight: var(--weight-bold);
  }
  .sub {
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
  }
  .ghead {
    display: flex;
    align-items: center;
    min-height: var(--size-touch-min);
    padding: 0 calc(var(--space-4) + var(--space-1));
  }
  h3 {
    margin: 0;
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
  }
  .wrow {
    display: grid;
    grid-template-columns: calc(5 * var(--space-10)) calc(4 * var(--space-10)) minmax(0, 1fr);
    grid-template-areas: "l v d" ". . c";
    column-gap: var(--space-4);
    row-gap: var(--space-1);
    align-items: start;
    padding: var(--space-3) calc(var(--space-4) + var(--space-1));
    border-top: var(--border-thin) solid var(--color-line);
  }
  .trow {
    display: grid;
    grid-template-columns: repeat(2, calc(3 * var(--space-10))) calc(3 * var(--space-10) + var(--space-4) + var(--space-1)) calc(3 * var(--space-10)) minmax(0, 1fr);
    column-gap: var(--space-4);
    align-items: center;
    /* The board's 44 px rows plus their 1 px line. */
    min-height: calc(var(--size-touch-min) + var(--border-thin));
    padding: 0 calc(var(--space-4) + var(--space-1));
    border-bottom: var(--border-thin) solid var(--color-line);
    font-family: var(--font-mono);
    font-size: var(--font-size-value-mono);
  }
  .trow:last-child {
    border-bottom: 0;
  }
  .trow.head {
    color: var(--color-text-muted);
    font-family: inherit;
    font-size: var(--font-size-caption);
    font-weight: var(--weight-semibold);
  }
  /* Tablet landscape (WxTabL, WindsTabL): "Observed" under the tabs; label (120), value (200) and
     ? on the first line, the description and location line under them at full width. */
  @media (min-width: 900px) and (max-width: 1439px) {
    .top {
      flex-direction: column;
      align-items: flex-start;
      gap: var(--space-2);
    }
    .xrow {
      grid-template-columns: calc(3 * var(--space-10)) calc(5 * var(--space-10)) minmax(0, 1fr) var(--size-touch-min);
      grid-template-areas: "l v . q" "d d d d" "c c c c";
      column-gap: var(--space-3);
      row-gap: var(--space-2);
      padding: var(--space-3) var(--space-1) var(--space-3) var(--space-4);
    }
    .wrow {
      grid-template-columns: calc(4 * var(--space-10)) minmax(0, 1fr);
      grid-template-areas: "l v" "d d" "c c";
      column-gap: var(--space-3);
      row-gap: var(--space-2);
      padding: var(--space-3) var(--space-4);
    }
    .trow {
      grid-template-columns: repeat(2, calc(2 * var(--space-10) + var(--space-2))) calc(2 * var(--space-10) + var(--space-6)) calc(2 * var(--space-10) + var(--space-2)) minmax(0, 1fr);
    }
  }
  @media (max-width: 899px) {
    .xrow,
    .wrow {
      grid-template-columns: minmax(0, 1fr) var(--size-touch-min);
      grid-template-areas: "l q" "v v" "d d" "c c";
      row-gap: var(--space-2);
    }
    .trow {
      grid-template-columns: repeat(4, minmax(0, 1fr));
    }
    .trow .mark,
    .trow.head span:last-child {
      grid-column: 1 / -1;
    }
    .tabs {
      flex-wrap: wrap;
    }
  }
</style>
