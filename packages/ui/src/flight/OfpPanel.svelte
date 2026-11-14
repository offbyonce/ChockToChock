<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

"Find it in your OFP" (Weather and FlightSection boards, 400 px): where the OFP prints the
selected field. The OFP's sections in order with a schematic page, the one that holds the field
highlighted; a close-up of the printed line with the value marked; "Not set from the OFP" for
values that come from ATC, the chart or the FMC; and on Overview, how an OFP is laid out. Open my
OFP on SimBrief is a plain link: it opens SimBrief and sends nothing from this app.
-->
<script lang="ts">
  import Icon from "../lib/Icon.svelte";
  import { OFP_SECTIONS, type Ctx, type Ofp } from "./sections";

  let {
    ofp,
    ctx,
    layout = false,
  }: {
    ofp: Ofp | null;
    ctx: Ctx;
    /** Overview: every section with what it holds and where the app uses it. */
    layout?: boolean;
  } = $props();

  /** The schematic page's blocks, one per OFP section, as the boards draw them. */
  const BLOCKS = ["b16", "b28", "b14", "b64", "b14", "b22", "b18"];
  const LAYOUT = [
    { name: "Header and times", what: "Flight number, airports, route, cruise level, cost index, times", where: "Route · Cruise and levels" },
    { name: "Fuel plan", what: "Trip, reserves, taxi and block fuel", where: "Fuel and weights" },
    { name: "Weights", what: "Zero fuel weight, payload, takeoff and landing weights", where: "Fuel and weights · Takeoff" },
    { name: "Navigation log", what: "Each fix with its level, wind and temperature", where: "Weather › En route winds" },
    { name: "Alternate", what: "The alternate airport and the fuel to get there", where: "Route · Fuel and weights" },
    { name: "Weather", what: "METAR and TAF for each airport", where: "Weather" },
    { name: "NOTAMs", what: "Notices for the airports and airspace", where: "Read before you fly" },
  ];
</script>

<aside class="ofp" aria-label="Find it in your OFP">
  <div class="head">
    <span class="kicker">Find it in your OFP</span>
    {#if layout}
      <span class="title">How an OFP is laid out</span>
      <span class="body">SimBrief prints the same sections in the same order every time. Here's what each one holds and where the app uses it.</span>
    {:else if ofp}
      <span class="title">{ofp.title}</span>
      <span class="body">{ofp.body}</span>
    {/if}
  </div>
  {#if layout}
    <ol class="map">
      {#each LAYOUT as m, n (m.name)}
        <li>
          <span class="n">{n + 1}</span>
          <span class="mt">
            <span class="mn">{m.name}</span>
            <span class="mw">{m.what}</span>
            <span class="mu">{m.where}</span>
          </span>
        </li>
      {/each}
    </ol>
  {:else if ofp}
    {#if ofp.notin}
      <div class="notin">
        <Icon name="notice" />
        <span>Not set from the OFP. The OFP only shows what was planned.</span>
      </div>
    {/if}
    {#if ofp.section !== null}
      <div class="outline">
        <figure class="page" role="img" aria-label="Schematic of an OFP page with this section highlighted">
          {#each BLOCKS as b, n (n)}<span class="block {b}" class:hi={n === ofp.section}></span>{/each}
        </figure>
        <ol class="items">
          {#each OFP_SECTIONS as name, n (name)}<li class:hi={n === ofp.section}>{name}</li>{/each}
        </ol>
      </div>
    {/if}
    <figure class="close">
      <span class="cap">{ofp.cap}</span>
      <span class="printed">{#each ofp.close(ctx) as part, n (n)}{#if part.hi}<span class="mark">{part.text}</span>{:else}<span>{part.text}</span>{/if}{#if part.br}<br />{/if}{/each}</span>
      <span class="note">{ofp.note}</span>
    </figure>
  {/if}
  <span class="order">Section order depends on the OFP layout chosen in SimBrief.</span>
  <a class="open" href="https://dispatch.simbrief.com/briefing/latest" target="_blank" rel="noopener noreferrer">Open my OFP on SimBrief ↗</a>
</aside>

<style>
  .ofp {
    display: flex;
    flex-direction: column;
    gap: calc(var(--space-3) + var(--border-thick));
    box-sizing: border-box;
    width: calc(10 * var(--space-10));
    flex: none;
    padding: calc(var(--space-4) + var(--space-1)) var(--space-6);
    border-left: var(--border-thin) solid var(--color-line);
    background: var(--color-surface-2);
    overflow-y: auto;
    scrollbar-width: none;
  }
  .ofp > :global(*) {
    flex: none;
  }
  .head {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }
  .kicker {
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
  }
  .title {
    font-size: var(--font-size-panel-title);
    font-weight: var(--weight-bold);
  }
  .body,
  .notin span {
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
    line-height: var(--leading-normal);
  }
  .notin {
    display: flex;
    align-items: flex-start;
    gap: calc(var(--space-2) + var(--border-thick));
    padding: calc(var(--space-2) + var(--border-thick)) var(--space-3);
    border: var(--border-thin) solid var(--color-border);
    border-radius: calc(var(--radius-small) + var(--border-thick));
    background: var(--color-surface);
  }
  .notin :global(.icon) {
    flex: none;
    width: var(--space-4);
    height: var(--space-4);
    margin-top: var(--border-thick);
    color: var(--color-text-secondary);
  }
  .outline {
    display: grid;
    grid-template-columns: calc(3 * var(--space-10) + var(--space-3)) minmax(0, 1fr);
    gap: calc(var(--space-3) + var(--border-thick));
    align-items: start;
  }
  .page {
    display: flex;
    flex-direction: column;
    gap: calc(var(--space-1) + var(--border-thick) / 2);
    margin: 0;
    padding: calc(var(--space-2) + var(--border-thick));
    border: var(--border-thin) solid var(--color-border);
    border-radius: calc(var(--radius-small));
    background: var(--color-paper);
  }
  .block {
    border-radius: var(--border-thick);
    background: var(--color-paper-block);
  }
  .block.hi {
    background: var(--color-paper-highlight);
    box-shadow: 0 0 0 calc(var(--border-thick) * 3 / 4) var(--color-accent);
  }
  .b14 {
    height: calc(var(--space-3) + var(--border-thick));
  }
  .b16 {
    height: var(--space-4);
  }
  .b18 {
    height: calc(var(--space-4) + var(--border-thick));
  }
  .b22 {
    height: calc(var(--space-6) - var(--border-thick));
  }
  .b28 {
    height: calc(var(--space-6) + var(--space-1));
  }
  .b64 {
    height: calc(2 * var(--space-8));
  }
  .items {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    margin: 0;
    padding: 0 0 0 calc(var(--space-4) + var(--border-thick));
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
  }
  .items .hi {
    color: var(--color-text-strong);
    font-weight: var(--weight-bold);
  }
  .close {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    margin: 0;
    padding: var(--space-3) calc(var(--space-3) + var(--border-thick));
    border: var(--border-thin) solid var(--color-border);
    border-radius: calc(var(--radius-small) + var(--border-thick));
    background: var(--color-paper);
  }
  .cap,
  .note {
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  .cap {
    font-weight: var(--weight-semibold);
  }
  .printed {
    color: var(--color-paper-text);
    font-family: var(--font-mono);
    font-size: var(--font-size-secondary);
    line-height: calc(var(--leading-normal) + 0.1);
    white-space: pre-wrap;
  }
  .mark {
    padding: 0 var(--border-thick);
    border-radius: var(--border-thick);
    background: var(--color-paper-highlight);
    box-shadow: 0 0 0 calc(var(--border-thick) * 3 / 4) var(--color-accent);
    color: var(--color-text-strong);
  }
  .map {
    margin: 0;
    padding: 0;
    overflow: hidden;
    list-style: none;
    border: var(--border-thin) solid var(--color-border);
    border-radius: calc(var(--radius-small) + var(--border-thick));
    background: var(--color-paper);
  }
  .map li {
    display: grid;
    grid-template-columns: calc(var(--space-6) - var(--border-thick)) minmax(0, 1fr);
    column-gap: calc(var(--space-2) + var(--border-thick));
    padding: calc(var(--space-2) + var(--border-thin)) var(--space-3);
    border-bottom: var(--border-thin) solid var(--color-line);
  }
  .n {
    padding-top: var(--border-thick);
    color: var(--color-text-muted);
    font-family: var(--font-mono);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-semibold);
  }
  .mt {
    display: flex;
    flex-direction: column;
    gap: var(--border-thin);
  }
  .mn {
    color: var(--color-paper-text);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
  }
  .mw {
    color: var(--color-text-secondary);
    font-size: var(--font-size-caption);
    line-height: var(--leading-snug);
  }
  .mu {
    color: var(--color-link);
    font-size: var(--font-size-caption);
  }
  .order {
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
    line-height: var(--leading-normal);
  }
  .open {
    display: inline-flex;
    align-items: center;
    min-height: var(--size-touch-min);
    color: var(--color-link);
    font-size: var(--font-size-secondary);
    text-decoration: none;
  }
</style>
