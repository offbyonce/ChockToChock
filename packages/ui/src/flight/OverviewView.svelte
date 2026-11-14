<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Overview (FSOverview): "Still to enter" lists every value the steps still need, each linking to
its field; "This flight" holds the key values, each linking to its section.
-->
<script lang="ts">
  import { flightHref } from "../route.svelte";
  import { SECTIONS, toEnter, type Ctx } from "./sections";

  let { ctx, descriptions }: { ctx: Ctx; descriptions: boolean } = $props();

  const sectionOf = (id: string) => SECTIONS.find((s) => s.groups.some((g) => g.rows.some((r) => r.id === id)));
  const first = (text: string) => text.split(/(?<=\.)\s/)[0] ?? text;
  const v = (k: keyof Ctx["values"]) => ctx.values[k] ?? "";
  const grouped = (n: number) => Math.round(n).toLocaleString("en-US");

  let missing = $derived(toEnter(ctx.values));
  let planned = $derived.by(() => {
    const t = ctx.plan?.plan.times.enrouteS;
    return t != null ? `${Math.floor(t / 3600)} h ${Math.round((t % 3600) / 60)} min planned.` : "";
  });
  let flightRows = $derived([
    {
      label: "Route",
      value: v("origin") || v("destination") ? `${v("origin") || "—"} → ${v("destination") || "—"}` : "—",
      desc: [v("alternate") ? `Alternate ${v("alternate")}.` : "", planned].filter(Boolean).join(" ") || "Where you're going.",
      to: "route" as const,
      title: "Route",
    },
    {
      label: "Block fuel",
      value: v("fuel") ? `${grouped(Number(v("fuel")))} ${ctx.unit}` : "—",
      desc: "All the fuel on board before engine start.",
      to: "fuel" as const,
      title: "Fuel and weights",
    },
    {
      label: "Cruise level",
      value: v("cruise_level") ? `FL${v("cruise_level")}` : "—",
      desc: v("cost_index") ? `Cost index ${v("cost_index")}.` : "The level you plan to cruise at.",
      to: "cruise" as const,
      title: "Cruise and levels",
    },
    {
      label: "Departure",
      value: [v("runway"), v("sid")].filter(Boolean).join(" · ") || "—",
      desc:
        [v("initial_altitude") ? `Initial altitude ${v("initial_altitude")}.` : "", v("squawk") ? `Squawk ${v("squawk")}.` : ""].filter(Boolean).join(" ") ||
        "The runway and SID ATC cleared.",
      to: "clearance" as const,
      title: "Clearance",
    },
  ]);
  const time = (ms: number) => new Date(ms).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit", hourCycle: "h23" });
</script>

<section class="card" aria-label="Still to enter">
  <div class="ghead">
    <h3>Still to enter</h3>
    <span class="fill"></span>
    <span class="note">{missing.length} {missing.length === 1 ? "value" : "values"}</span>
  </div>
  {#each missing as row (row.id)}
    {@const s = sectionOf(row.id)}
    <div class="orow">
      <span class="label">{row.label}</span>
      <a class="enter" href={s ? flightHref(s.id, row.field) : "#"}>— {(row.placeholder ?? "Enter").replace(/^Enter/, "enter")}</a>
      <span class="about">
        {#if descriptions}<span class="desc">{first(row.desc)}</span>{/if}
        {#if s}<a class="go" href={flightHref(s.id)}>{s.title} ›</a>{/if}
      </span>
    </div>
  {:else}
    <div class="orow none"><span class="desc">Nothing the steps need is missing.</span></div>
  {/each}
</section>
<section class="card" aria-label="This flight">
  <div class="ghead">
    <h3>This flight</h3>
    <span class="fill"></span>
    {#if ctx.plan}<span class="note">From SimBrief, {time(ctx.plan.loadedAt)}</span>{/if}
  </div>
  {#each flightRows as row (row.label)}
    <div class="orow">
      <span class="label">{row.label}</span>
      <span class="ro">{row.value}</span>
      <span class="about">
        {#if descriptions}<span class="desc">{row.desc}</span>{/if}
        <a class="go" href={flightHref(row.to)}>{row.title} ›</a>
      </span>
    </div>
  {/each}
</section>

<style>
  .card {
    flex: none;
    margin: 0;
    max-width: calc(26 * var(--space-10));
    border: var(--border-thin) solid var(--color-border);
    border-radius: var(--radius-panel);
    background: var(--color-surface);
  }
  .ghead {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    min-height: calc(var(--size-touch-min) + var(--space-2));
    padding: 0 calc(var(--space-4) + var(--space-1));
  }
  h3 {
    margin: 0;
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
  }
  .fill {
    flex: 1;
  }
  .note {
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  .orow {
    display: grid;
    grid-template-columns:
      calc(4 * var(--space-10) + var(--space-3))
      calc(3 * var(--space-10) + var(--space-6) + var(--space-1) + var(--border-thick))
      minmax(0, 1fr);
    column-gap: var(--space-4);
    align-items: start;
    padding: var(--space-3) calc(var(--space-4) + var(--space-1));
    border-top: var(--border-thin) solid var(--color-line);
  }
  .orow.none {
    display: block;
  }
  .label {
    padding-top: calc(var(--space-3) - var(--border-thin));
    font-size: var(--font-size-body);
    font-weight: var(--weight-semibold);
  }
  .ro,
  .enter {
    padding-top: calc(var(--space-3) - var(--border-thin));
    font-family: var(--font-mono);
    font-size: var(--font-size-value-mono);
    overflow-wrap: anywhere;
  }
  /* Ink, as the board draws it; the section link under the description is the blue one. */
  .enter {
    color: var(--color-text);
    text-decoration: none;
  }
  .about {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: calc(var(--space-1) + var(--border-thick) / 2);
    padding-top: calc(var(--space-3) - var(--border-thin));
  }
  .desc {
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
    line-height: var(--leading-normal);
  }
  .go {
    color: var(--color-link);
    font-size: var(--font-size-secondary);
    text-decoration: none;
  }
  @media (max-width: 599px) {
    .orow {
      grid-template-columns: minmax(0, 1fr);
      row-gap: var(--space-2);
    }
    .label,
    .ro,
    .enter,
    .about {
      padding-top: 0;
    }
  }
</style>
