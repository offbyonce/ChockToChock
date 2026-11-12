<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The flight strip at the top of a phase, as in 3.0: one line summing up the clearance (built in
Rust, decision 0019) and the last line of the notes. Tapping it opens the Flight pop-out with the
route, clearance and takeoff values, the notes, a way to Flight setup and Clear flight, which asks
first. Tapping one part of the summary opens the pop-out on that part's field.
-->
<script lang="ts">
  import type { FillField, StripPart } from "@chocktochock/bindings";

  let {
    parts,
    note,
    values,
    notes,
    onSet,
    onNotes,
    onClear,
  }: {
    parts: StripPart[];
    note: string;
    values: Partial<Record<FillField, string>>;
    notes: string;
    onSet: (field: FillField, value: string) => void;
    onNotes: (notes: string) => void;
    onClear: () => void;
  } = $props();

  type Field = { field: FillField; label: string; max: number; mode?: "numeric" | "decimal" };
  const GROUPS: [string, Field[]][] = [
    [
      "Route",
      [
        { field: "origin", label: "From", max: 4 },
        { field: "destination", label: "To", max: 4 },
      ],
    ],
    [
      "Clearance",
      [
        { field: "runway", label: "Runway", max: 4 },
        { field: "sid", label: "Departure", max: 8 },
        { field: "transition", label: "Transition", max: 8 },
        { field: "initial_altitude", label: "Initial alt", max: 5, mode: "numeric" },
        { field: "squawk", label: "Squawk", max: 4, mode: "numeric" },
        { field: "ground_frequency", label: "Ground", max: 7, mode: "decimal" },
        { field: "departure_frequency", label: "Departure freq", max: 7, mode: "decimal" },
      ],
    ],
    [
      "Takeoff",
      [
        { field: "qnh", label: "QNH", max: 6, mode: "decimal" },
        { field: "takeoff_flaps", label: "Flaps", max: 2, mode: "numeric" },
        { field: "v1", label: "V1", max: 3, mode: "numeric" },
        { field: "vr", label: "VR", max: 3, mode: "numeric" },
        { field: "v2", label: "V2", max: 3, mode: "numeric" },
        { field: "cg", label: "CG", max: 4, mode: "decimal" },
        { field: "takeoff_trim", label: "Stab trim", max: 5, mode: "decimal" },
      ],
    ],
  ];

  let open = $state(false);
  let asking = $state(false);
  let strip: HTMLElement | undefined = $state();
  let line: HTMLButtonElement | undefined = $state();

  function show(field: FillField | null): void {
    open = true;
    asking = false;
    queueMicrotask(() => {
      const el = (field && strip?.querySelector<HTMLInputElement>(`#sp-${field}`)) || strip?.querySelector<HTMLInputElement>(".fsg input");
      el?.focus({ preventScroll: true });
    });
  }
  function hide(): void {
    if (!open) return;
    open = false;
    asking = false;
    line?.focus({ preventScroll: true });
  }
  function toggle(e: MouseEvent): void {
    if (open) return hide();
    const part = (e.target as HTMLElement).closest<HTMLElement>("[data-field]");
    show((part?.dataset.field as FillField | undefined) ?? null);
  }
  /** Closes on a press outside the strip or on Escape, as 3.0 did. */
  function outside(e: PointerEvent): void {
    if (open && strip && !strip.contains(e.target as Node)) {
      open = false;
      asking = false;
    }
  }
  function key(e: KeyboardEvent): void {
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

<section class="fstrip card" class:open bind:this={strip}>
  <button type="button" class="fsline" aria-expanded={open} aria-controls="spop" bind:this={line} onclick={toggle}>
    <b>Flight</b>
    <span class="fsum">
      {#if parts.length}
        {#each parts as p, i (p.field)}{#if i}<i>·</i>{/if}<span data-field={p.field}>{p.text}</span>{/each}
      {:else}
        <span class="e">Add your clearance: runway, departure, squawk and frequencies</span>
      {/if}
      {#if note}<span class="fsnote">Notes: {note}</span>{/if}
    </span>
    <span class="fsed">Edit</span>
  </button>
  {#if open}
    <div id="spop" role="dialog" aria-label="Flight details">
      <div class="sphd">
        <h2>Flight</h2>
        <button class="link" type="button" onclick={hide}>Close</button>
      </div>
      {#each GROUPS as [title, fields] (title)}
        <div class="spg">{title}</div>
        <div class="fsg">
          {#each fields as f (f.field)}
            <label class="fsf">
              <span>{f.label}</span>
              <input
                id="sp-{f.field}"
                value={values[f.field] ?? ""}
                maxlength={f.max}
                inputmode={f.mode}
                autocomplete="off"
                oninput={(e) => onSet(f.field, e.currentTarget.value)}
                onchange={(e) => trimmed(f.field, e.currentTarget)}
              />
            </label>
          {/each}
        </div>
      {/each}
      <label class="spg" for="sp-notes">Notes</label>
      <textarea
        id="sp-notes"
        rows="3"
        placeholder="Jot down what ATC says: frequency changes, re-clearances, taxi route"
        value={notes}
        oninput={(e) => onNotes(e.currentTarget.value)}
      ></textarea>
      {#if asking}
        <div class="spft ask">
          <span class="spcq" id="spcq"><b>Clear the whole flight?</b> Removes the ticks, flight details, clearance, notes and timers. Settings stay.</span>
          <span class="spfr">
            <button class="btn" type="button" onclick={() => (asking = false)}>Cancel</button>
            <button
              class="btn dangerb"
              type="button"
              aria-describedby="spcq"
              onclick={() => {
                open = false;
                asking = false;
                onClear();
              }}>Clear flight</button
            >
          </span>
        </div>
      {:else}
        <div class="spft">
          <a class="link to-setup" href="#/setup" onclick={() => (open = false)}>All flight details in Flight setup</a>
          <span class="spfr">
            <button class="link spclr" type="button" onclick={() => (asking = true)}>Clear flight</button>
            <button class="btn primary" type="button" onclick={hide}>Done</button>
          </span>
        </div>
      {/if}
    </div>
  {/if}
</section>

<style>
  .fstrip {
    position: relative;
    z-index: 5;
    min-width: 0;
    margin-bottom: 0.625rem;
    overflow: visible;
  }
  .fstrip.open {
    z-index: 30;
  }
  .fsline {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    width: 100%;
    min-width: 0;
    min-height: 2.75rem;
    padding: 0.5rem 1rem;
    border: 0;
    border-radius: inherit;
    background: none;
    color: var(--ink);
    font: inherit;
    text-align: left;
    cursor: pointer;
  }
  .fsline > b {
    flex: none;
    font-weight: 600;
  }
  .fsum {
    flex: 1;
    min-width: 0;
    color: var(--ink);
    font: 500 0.875rem/1.5 var(--mono);
    overflow-wrap: anywhere;
  }
  .fsum i {
    margin: 0 0.4rem;
    color: var(--muted);
    font-style: normal;
  }
  .fsum .e {
    color: var(--muted);
    font: 400 0.875rem var(--sans);
  }
  .fsnote {
    display: block;
    margin-top: 0.15rem;
    overflow: hidden;
    color: var(--muted);
    font: 400 0.8125rem/1.4 var(--sans);
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .fsed {
    flex: none;
    color: var(--accent);
    font-size: 0.875rem;
    font-weight: 500;
  }
  .fsline:hover .fsed {
    text-decoration: underline;
  }
  #spop {
    position: absolute;
    top: calc(100% + 0.375rem);
    left: 0;
    z-index: 40;
    width: min(44rem, 100%);
    box-sizing: border-box;
    padding: 1rem;
    border: 1px solid var(--line);
    border-radius: 0.75rem;
    background: var(--surface);
    box-shadow: 0 12px 32px rgb(0 0 0 / 18%);
  }
  .sphd {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 0.5rem;
  }
  h2 {
    margin: 0;
    font-size: 1rem;
  }
  .spg {
    display: block;
    margin: 0.75rem 0 0.375rem;
    color: var(--muted);
    font: 600 0.6875rem var(--sans);
    letter-spacing: 0.06em;
    text-transform: uppercase;
  }
  .fsg {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 0.625rem 0.75rem;
  }
  .fsf {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    color: var(--muted);
    font-size: 0.8125rem;
  }
  .fsf span {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .fsf input,
  textarea {
    min-width: 0;
    height: 2.5rem;
    box-sizing: border-box;
    padding: 0 0.6rem;
    border: 1px solid var(--line);
    border-radius: 0.5rem;
    background: var(--surface);
    color: var(--ink);
    font: 500 1rem var(--mono);
    text-transform: uppercase;
  }
  textarea {
    width: 100%;
    height: auto;
    padding: 0.5rem 0.6rem;
    font: 400 0.9375rem/1.4 var(--sans);
    text-transform: none;
    resize: vertical;
  }
  .spft {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    margin-top: 0.875rem;
  }
  .spft.ask {
    flex-wrap: wrap;
    align-items: flex-start;
    padding: 0.75rem;
    border-radius: 0.6rem;
    background: color-mix(in srgb, var(--stop) 8%, transparent);
  }
  .spcq {
    flex: 1 1 16rem;
    color: var(--ink);
    font-size: 0.875rem;
  }
  .spfr {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: flex-end;
    gap: 1rem;
  }
  /* 3.0's quiet links: small, grey, darker on hover. */
  .link {
    display: inline-flex;
    align-items: center;
    min-height: 2.75rem;
    padding-inline: 0.5rem;
    color: var(--muted);
    font-size: 0.8125rem;
    text-decoration: none;
  }
  .link:hover {
    color: var(--ink);
  }
  /* As in 3.0, the way to Flight setup sits on the right with Clear flight and Done. */
  .to-setup {
    margin-left: auto;
  }
  .spclr {
    margin: 0;
    color: var(--stop);
  }
  .dangerb {
    border-color: var(--stop);
    color: var(--stop);
  }
  @media (max-width: 760px) {
    .fsum {
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }
    .fsg {
      grid-template-columns: repeat(3, minmax(0, 1fr));
      gap: 0.5rem;
    }
    #spop {
      position: fixed;
      top: auto;
      right: 0;
      bottom: 0;
      left: 0;
      width: auto;
      max-height: 85vh;
      overflow: auto;
      padding-bottom: max(1rem, env(safe-area-inset-bottom));
      border-radius: 1rem 1rem 0 0;
      box-shadow: 0 0 0 100vmax rgb(0 0 0 / 35%);
    }
  }
</style>
