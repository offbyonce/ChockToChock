<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The flight strip (Checklist board, 48 px, closed): the route, Runway, SID, Initial altitude,
Squawk and Departure frequency (a dash when empty), the notes count and a chevron. With nothing
entered yet it reads "Add your flight: import from SimBrief or enter the clearance" (owner,
batch 3). The whole strip, the prompt too, opens the strip panel (StripOpen, StripFreq): it drops
below the strip over a scrim with the clearance fields, Frequencies (ground, tower, departure),
Notes from ATC, and a footer with Import latest from SimBrief, where the values came from, Reset
flight plan (Undo after, in the shared toast) and All flight data, the Flight pages.
-->
<script lang="ts">
  import type { FillField } from "@chocktochock/bindings";
  import Icon from "./Icon.svelte";
  import { rangeMessage } from "../flight/sections";

  let {
    values,
    notes,
    provenance = "",
    onSet,
    onNotes,
    onClear,
    onImport,
  }: {
    values: Partial<Record<FillField, string>>;
    notes: string;
    /** Where the values came from, e.g. "Runway and SID from SimBrief, 10:02 · the rest typed". */
    provenance?: string;
    onSet: (field: FillField, value: string) => void;
    onNotes: (notes: string) => void;
    /** Resets the flight plan; the page offers Undo. */
    onClear: () => void;
    /** Imports the latest SimBrief plan (or goes to SimBrief and OFP file to set one up). */
    onImport: () => void;
  } = $props();

  type Field = { field: FillField; label: string; max: number; mode?: "numeric" | "decimal" };
  const CLEARANCE: Field[] = [
    { field: "runway", label: "Runway", max: 4 },
    { field: "sid", label: "SID", max: 8 },
    { field: "transition", label: "Transition", max: 8 },
    { field: "initial_altitude", label: "Initial altitude", max: 5, mode: "numeric" },
    { field: "squawk", label: "Squawk", max: 4, mode: "numeric" },
  ];
  const FREQUENCIES: Field[] = [
    { field: "ground_frequency", label: "Ground", max: 7, mode: "decimal" },
    { field: "tower_frequency", label: "Tower", max: 7, mode: "decimal" },
    { field: "departure_frequency", label: "Departure frequency", max: 7, mode: "decimal" },
  ];

  /** The closed strip's fields, as the board lists them. */
  const FIELDS: { field: FillField; label: string }[] = [
    { field: "runway", label: "Runway" },
    { field: "sid", label: "SID" },
    { field: "initial_altitude", label: "Initial altitude" },
    { field: "squawk", label: "Squawk" },
    { field: "departure_frequency", label: "Departure frequency" },
  ];
  /** A field as the strip shows it: the SID with its transition, "SUMMA2.SEA" (Checklist board). */
  function shownValue(field: FillField): string {
    const value = values[field] ?? "";
    return field === "sid" && value && values.transition ? `${value}.${values.transition}` : value;
  }
  let filled = $derived(["origin", "destination", ...FIELDS.map((f) => f.field)].some((f) => values[f as FillField]));
  let route = $derived(`${values.origin || "—"} → ${values.destination || "—"}`);
  let noteCount = $derived(notes.split("\n").filter((l) => l.trim()).length);

  let open = $state(false);
  /** Opened with a pointer: the cursor goes to Runway, but its ring shows only once a key is
   * pressed (owner: the ring is for keyboard focus). */
  let quiet = $state(false);
  const message = (field: FillField) => rangeMessage(field, { values, plan: null, notes, unit: "" });
  let freqOpen = $state(false);
  let strip: HTMLElement | undefined = $state();
  let line: HTMLButtonElement | undefined = $state();

  function show(field: FillField | null): void {
    open = true;
    if (field && FREQUENCIES.some((f) => f.field === field)) freqOpen = true;
    queueMicrotask(() => {
      const el = (field && strip?.querySelector<HTMLInputElement>(`#sp-${field}`)) || strip?.querySelector<HTMLInputElement>(".spf input");
      el?.focus({ preventScroll: true });
    });
  }
  function hide(): void {
    if (!open) return;
    open = false;
    line?.focus({ preventScroll: true });
  }
  function toggle(e: MouseEvent): void {
    if (open) return hide();
    // A click has a detail (the click count); Enter or Space on the button has none.
    quiet = e.detail > 0;
    const part = (e.target as HTMLElement).closest<HTMLElement>("[data-field]");
    show((part?.dataset.field as FillField | undefined) ?? null);
  }
  /** Closes on a press outside the strip or on Escape, as 3.0 did. */
  function outside(e: PointerEvent): void {
    if (open && strip && !strip.contains(e.target as Node)) open = false;
  }
  function key(e: KeyboardEvent): void {
    if (open && e.key !== "Escape") quiet = false;
    if (open && e.key === "Escape") {
      e.preventDefault();
      hide();
    }
  }
  const trimmed = (field: FillField, el: HTMLInputElement) => {
    const v = el.value.trim();
    if (v !== el.value) onSet(field, v);
  };
</script>

<svelte:document onpointerdown={outside} onkeydown={key} />

<section class="fstrip" class:open bind:this={strip} aria-label="Flight strip">
  <button
    type="button"
    class="fsline"
    aria-expanded={open}
    aria-controls="spop"
    aria-label={open
      ? "Close the flight strip"
      : filled
        ? `Flight strip: ${route}. Open clearance, notes and flight data`
        : "Add your flight: import from SimBrief or enter the clearance"}
    bind:this={line}
    onclick={toggle}
  >
    {#if open}
      <span class="route">{route}</span>
      <span class="what">Clearance and notes</span>
      <span class="fill"></span>
      <span class="up"><Icon name="chevron-down" /></span>
    {:else}
      {#if filled}
        <span class="route">{route}</span>
        {#each FIELDS as f (f.field)}
          {@const shown = shownValue(f.field)}
          <span class="part" data-field={f.field}><span class="lab">{f.label}</span><span class="val" class:none={!shown}>{shown || "—"}</span></span>
        {/each}
        {#if noteCount}<span class="notes">Notes {noteCount}</span>{/if}
      {:else}
        <span class="prompt">Add your flight: import from SimBrief or enter the clearance</span>
      {/if}
      <span class="fill"></span>
      <Icon name="chevron-down" />
    {/if}
  </button>
  {#if open}
    <div class="scrim" aria-hidden="true"></div>
    <div id="spop" class="panel" role="region" aria-label="Clearance and notes">
      <div class="body">
        <div class="clearance">
          <h2>Clearance</h2>
          <div class="spf">
            {#each CLEARANCE as f (f.field)}
              <label>
                {f.label}
                <input
                  id="sp-{f.field}"
                  class:quiet
                  value={values[f.field] ?? ""}
                  maxlength={f.max}
                  inputmode={f.mode}
                  autocomplete="off"
                  aria-invalid={message(f.field) ? "true" : undefined}
                  aria-describedby={message(f.field) ? `sp-err-${f.field}` : undefined}
                  oninput={(e) => onSet(f.field, e.currentTarget.value)}
                  onchange={(e) => trimmed(f.field, e.currentTarget)}
                />
                {#if message(f.field)}<span class="err" id="sp-err-{f.field}" role="alert"><Icon name="field-alert" />{message(f.field)}</span>{/if}
              </label>
            {/each}
          </div>
          <button class="freq" type="button" aria-expanded={freqOpen} aria-controls="sp-freq" onclick={() => (freqOpen = !freqOpen)}>
            Frequencies: ground, tower, departure {freqOpen ? "⌃" : "⌄"}
          </button>
          {#if freqOpen}
            <div class="spf" id="sp-freq">
              {#each FREQUENCIES as f (f.field)}
                <label>
                  {f.label}
                  <input
                    id="sp-{f.field}"
                    value={values[f.field] ?? ""}
                    maxlength={f.max}
                    inputmode={f.mode}
                    placeholder="Enter"
                    autocomplete="off"
                    aria-invalid={message(f.field) ? "true" : undefined}
                  aria-describedby={message(f.field) ? `sp-err-${f.field}` : undefined}
                    oninput={(e) => onSet(f.field, e.currentTarget.value)}
                    onchange={(e) => trimmed(f.field, e.currentTarget)}
                  />
                  {#if message(f.field)}<span class="err" id="sp-err-{f.field}" role="alert"><Icon name="field-alert" />{message(f.field)}</span>{/if}
                </label>
              {/each}
            </div>
          {/if}
        </div>
        <label class="notesbox">
          Notes from ATC
          <textarea id="sp-notes" value={notes} placeholder="Jot down what ATC says: frequency changes, re-clearances, taxi route" oninput={(e) => onNotes(e.currentTarget.value)}></textarea>
        </label>
      </div>
      <div class="foot">
        <button class="btn import" type="button" onclick={onImport}>Import latest from SimBrief</button>
        {#if provenance}<span class="from">{provenance}</span>{/if}
        <span class="fill"></span>
        <button
          class="reset"
          type="button"
          onclick={() => {
            open = false;
            onClear();
          }}>Reset flight plan</button
        >
        <a class="all" href="#/flight/overview" onclick={() => (open = false)}>All flight data <Icon name="chevron-right" /></a>
      </div>
    </div>
  {/if}
</section>

<style>
  /* The closed strip: the page's width, under the phase track. */
  .fstrip {
    position: sticky;
    top: calc(var(--size-header) + var(--size-phase-row));
    z-index: 18;
    display: flex;
    align-items: center;
    height: var(--size-flight-strip);
    padding: 0 var(--space-4);
    border-bottom: var(--border-thin) solid var(--color-line);
    background: var(--color-surface-2);
  }
  .fstrip.open {
    background: var(--color-strip-panel);
  }
  .fsline {
    flex: 1;
    display: flex;
    align-items: center;
    gap: var(--space-6);
    min-width: 0;
    height: var(--size-touch-min);
    padding: 0 var(--space-3);
    overflow: hidden;
    border: 0;
    border-radius: var(--radius-control);
    background: none;
    color: var(--color-text);
    font: inherit;
    text-align: left;
    white-space: nowrap;
    cursor: pointer;
  }
  .open .fsline {
    gap: var(--space-3);
  }
  .route {
    font-family: var(--font-mono);
    font-size: var(--font-size-value-mono);
    font-weight: var(--weight-semibold);
  }
  .what {
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
  }
  .up {
    display: inline-flex;
    transform: rotate(180deg);
    color: var(--color-text-secondary);
  }
  .part {
    display: flex;
    align-items: baseline;
    gap: calc(var(--space-1) + var(--border-thick) / 2);
  }
  .lab {
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  .val {
    font-family: var(--font-mono);
    font-size: var(--font-size-value-mono);
  }
  .val.none {
    color: var(--color-text-muted);
  }
  .notes {
    padding: calc(var(--border-thick) + var(--border-thin)) calc(var(--space-2) + var(--border-thick));
    border-radius: var(--radius-pill);
    background: var(--color-chip);
    color: var(--color-text-secondary);
    font-size: var(--font-size-caption);
  }
  .prompt {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
  }
  .fill {
    flex: 1;
  }
  .fsline :global(.icon) {
    color: var(--color-text-muted);
  }
  /* The panel drops below the strip over the page, which a scrim dims (StripOpen). */
  .scrim {
    position: fixed;
    inset: calc(var(--size-header) + var(--size-phase-row) + var(--size-flight-strip)) 0 0;
    z-index: -1;
    background: var(--color-scrim);
  }
  .panel {
    position: absolute;
    top: 100%;
    left: 0;
    right: 0;
    border-top: var(--border-thin) solid var(--color-line);
    border-bottom: var(--border-thin) solid var(--color-border);
    background: var(--color-strip-panel);
    box-shadow: var(--color-overlay-shadow);
  }
  .body {
    display: grid;
    grid-template-columns: minmax(0, 2fr) minmax(0, 1fr);
    gap: var(--space-8);
    padding: var(--space-4) var(--space-8);
  }
  .clearance {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }
  h2 {
    margin: 0;
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
  }
  .spf {
    display: grid;
    grid-template-columns: repeat(5, minmax(0, 1fr));
    gap: var(--space-3);
  }
  .spf label {
    display: flex;
    flex-direction: column;
    gap: calc(var(--space-1) + var(--border-thick) / 2);
    min-width: 0;
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  input,
  textarea {
    min-width: 0;
    height: var(--size-touch-min);
    box-sizing: border-box;
    padding: 0 var(--space-3);
    border: var(--border-thin) solid var(--color-field-border);
    border-radius: calc(var(--radius-small) + var(--border-thick));
    background: var(--color-field);
    color: var(--color-text-strong);
    font-family: var(--font-mono);
    font-size: var(--font-size-value-mono);
  }
  input::placeholder,
  textarea::placeholder {
    color: var(--color-text-muted);
  }
  .quiet:focus-visible {
    outline: none;
  }
  input[aria-invalid="true"] {
    border-color: var(--color-danger-text);
  }
  /* FSInvalid: the alert icon and "Enter X to Y", 12 px, in the red text color. */
  .err {
    display: flex;
    align-items: flex-start;
    gap: calc(var(--space-1) + var(--border-thick));
    color: var(--color-danger-text);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-regular);
    line-height: var(--leading-snug);
  }
  .err :global(.icon) {
    flex: none;
    width: calc(var(--size-icon-small) - var(--border-thick));
    height: calc(var(--size-icon-small) - var(--border-thick));
    margin-top: var(--border-thin);
  }
  .freq {
    align-self: flex-start;
    min-height: var(--size-touch-min);
    padding: 0;
    border: 0;
    background: none;
    color: var(--color-link);
    font: inherit;
    font-size: var(--font-size-secondary);
  }
  .notesbox {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-semibold);
  }
  textarea {
    flex: 1;
    min-height: calc(var(--space-10) * 2.5);
    height: auto;
    padding: calc(var(--space-2) + var(--border-thick)) var(--space-3);
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-regular);
    line-height: var(--leading-normal);
    resize: none;
  }
  .foot {
    display: flex;
    align-items: center;
    gap: var(--space-3);
    height: calc(var(--size-touch-min) + var(--space-4));
    padding: 0 var(--space-8);
    border-top: var(--border-thin) solid var(--color-line);
  }
  .import {
    padding: 0 calc(var(--space-3) + var(--space-1) / 2);
    border-color: var(--color-border);
    color: var(--color-text);
  }
  .from {
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  .reset {
    height: var(--size-touch-min);
    padding: 0 var(--space-3);
    border: 0;
    background: transparent;
    color: var(--color-danger-text);
    font: inherit;
    font-size: var(--font-size-secondary);
  }
  .all {
    display: flex;
    align-items: center;
    gap: calc(var(--space-1) + var(--border-thick) / 2);
    height: var(--size-touch-min);
    padding: 0 var(--space-1);
    color: var(--color-link);
    font-size: var(--font-size-body);
    font-weight: var(--weight-semibold);
    text-decoration: none;
  }
  /* Tablet portrait and phone (batch 5 brings their own boards): the notes go under the
     clearance, the fields wrap three or two to a row, and the footer wraps. */
  @media (max-width: 899px) {
    .body {
      grid-template-columns: minmax(0, 1fr);
      gap: var(--space-4);
      padding: var(--space-4) var(--gutter);
    }
    .spf {
      grid-template-columns: repeat(3, minmax(0, 1fr));
    }
    .foot {
      flex-wrap: wrap;
      height: auto;
      padding: var(--space-2) var(--gutter);
    }
    .foot .fill {
      display: none;
    }
    .from {
      flex-basis: 100%;
      order: 1;
    }
    .panel {
      max-height: calc(100dvh - var(--size-header) - var(--size-phase-row) - var(--size-flight-strip));
      overflow-y: auto;
    }
  }
  @media (max-width: 599px) {
    .spf {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }
</style>
