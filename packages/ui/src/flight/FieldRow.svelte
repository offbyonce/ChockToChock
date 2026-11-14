<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

One Flight field row (FlightSection boards): the label, the value, then the description with the line
that says where the value goes under it. An editable value is always a visible 44 px box; a value still needed
is an empty box with an amber border and a placeholder; a choice is a field-styled button that
opens a menu in place; read-only values and rows that belong to another section are plain mono
text, with a "Section ›" link under the description. Selecting the row (a click or focus) points
the "Find it in your OFP" panel at it. Descriptions Off keeps the label, the value and the location line.
-->
<script lang="ts">
  import type { FillField } from "@chocktochock/bindings";
  import { flightHref, phaseHref } from "../route.svelte";
  import Icon from "../lib/Icon.svelte";
  import { type Ctx, type Row, rangeMessage } from "./sections";

  let {
    row,
    ctx,
    selected,
    descriptions,
    sectionTitle,
    onSelect,
    onSet,
    onNotes,
  }: {
    row: Row;
    ctx: Ctx;
    selected: boolean;
    descriptions: boolean;
    /** The title of the section a cross-section row links to. */
    sectionTitle: (id: NonNullable<Row["link"]>) => string;
    onSelect: () => void;
    onSet: (field: FillField, value: string) => void;
    onNotes?: (notes: string) => void;
  } = $props();

  let menu = $state(false);
  const grouped = (n: number) => Math.round(n).toLocaleString("en-US");

  /** A value as the box shows it: weights in full with the unit, headings with °, levels with FL. */
  function shown(field: FillField, weight?: "full" | "thousands"): string {
    const raw = ctx.values[field] ?? "";
    if (!raw) return "";
    if (weight) {
      const n = Number(raw) * (weight === "thousands" ? 1000 : 1);
      return Number.isFinite(n) ? `${grouped(n)} ${ctx.unit}` : raw;
    }
    return row.show && field === row.field ? row.show(raw) : raw;
  }
  /** What the pilot typed, back into the flight's form. */
  function stored(text: string, weight?: "full" | "thousands", numeric = false): string {
    const t = text.trim();
    if (!t) return "";
    if (weight) {
      const n = Number(t.replace(/[^0-9.]/g, ""));
      if (!Number.isFinite(n) || t.replace(/[^0-9.]/g, "") === "") return "";
      return weight === "thousands" ? String(Number((n / 1000).toFixed(1))) : String(Math.round(n));
    }
    if (numeric || row.show) return t.replace(/^FL/i, "").replace(/[°%,+\s]/g, "");
    return t.toUpperCase();
  }
  let readOnly = $derived(!!row.read || !!row.link);
  /** 3.0's range for the field, as a short message under the box when the value is outside it. */
  let message = $derived(row.field && !row.read && !row.link ? rangeMessage(row.field, ctx) : null);
  let secondMessage = $derived(row.second ? rangeMessage(row.second.field, ctx) : null);
  let readValue = $derived.by(() => {
    if (row.read) return row.read(ctx);
    if (row.field) return shown(row.field) || "—";
    return "—";
  });
  let empty = $derived(!!row.field && !readOnly && !ctx.values[row.field]);
  let chipLink = $derived(row.link ? flightHref(row.link) : row.phase ? phaseHref(row.phase) : null);
  let chipText = $derived(row.link ? `${sectionTitle(row.link)} ›` : (row.chip ?? ""));

  function pick(value: string): void {
    menu = false;
    if (row.field) onSet(row.field, value);
  }
  function outside(e: PointerEvent): void {
    if (menu && !(e.target instanceof Element && e.target.closest(`[data-pick="${row.id}"]`))) menu = false;
  }
</script>

<svelte:document onpointerdown={outside} />

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_static_element_interactions -->
<div class="frow" class:sel={selected} data-row={row.id} onclick={onSelect} onfocusin={onSelect}>
  <span class="label" id="lab-{row.id}">{row.label}</span>
  <span class="value">
    {#if row.notes}
      <input class="box" aria-labelledby="lab-{row.id}" value={ctx.notes} placeholder="Enter" autocomplete="off" onchange={(e) => onNotes?.(e.currentTarget.value)} />
    {:else if readOnly}
      <span class="ro">{readValue}</span>
    {:else if row.choices && row.field}
      {@const field = row.field}
      <span class="pickwrap" data-pick={row.id}>
        <button class="box pick" class:need={empty} type="button" aria-haspopup="listbox" aria-expanded={menu} aria-labelledby="lab-{row.id}" data-field={field} onclick={() => (menu = !menu)}>
          <span>{ctx.values[field] || row.placeholder || "Choose"}</span>
          <Icon name="chevron-down" />
        </button>
        {#if menu}
          <ul class="menu" role="listbox" aria-labelledby="lab-{row.id}">
            {#each row.choices as c (c)}
              <li role="option" aria-selected={ctx.values[field] === c}>
                <button type="button" onclick={() => pick(c)}>{c}</button>
              </li>
            {/each}
          </ul>
        {/if}
      </span>
    {:else if row.field}
      {@const field = row.field}
      <span class="boxes" class:two={!!row.second}>
        <input
          class="box"
          class:need={empty}
          aria-labelledby={row.firstName ? undefined : `lab-${row.id}`}
          aria-label={row.firstName ? `${row.label}: ${row.firstName}` : undefined}
          aria-invalid={message ? "true" : undefined}
          aria-describedby={message ? `err-${row.id}` : undefined}
          data-field={field}
          value={shown(field, row.weight)}
          placeholder={row.placeholder ?? (empty ? "Enter" : "")}
          maxlength={row.weight || row.show ? undefined : row.max}
          inputmode={row.mode === "text" ? "text" : row.mode}
          autocomplete="off"
          onchange={(e) => onSet(field, stored(e.currentTarget.value, row.weight, row.mode === "numeric" || row.mode === "decimal"))}
        />
        {#if row.second}
          {@const two = row.second}
          <input
            class="box"
            aria-label="{row.label}: {two.name}"
            aria-invalid={secondMessage ? "true" : undefined}
            aria-describedby={secondMessage ? `err-${row.id}` : undefined}
            data-field={two.field}
            value={shown(two.field, two.weight)}
            placeholder={two.placeholder}
            inputmode={two.mode}
            autocomplete="off"
            onchange={(e) => onSet(two.field, stored(e.currentTarget.value, two.weight, true))}
          />
        {/if}
      </span>
      {#if message || secondMessage}<span class="err" id="err-{row.id}" role="alert"><Icon name="field-alert" />{message ?? secondMessage}</span>{/if}
    {/if}
  </span>
  <span class="about">
    {#if descriptions}<span class="desc">{row.desc}</span>{/if}
    {#if row.loc}<span class="loc">{row.loc}</span>{/if}
    {#if row.seg}
      {@const seg = row.seg}
      <span class="seg" role="group" aria-label={seg.label}>
        {#each seg.values as s (s)}
          {@const on = (ctx.values[seg.field] || seg.values[0]) === s}
          <button type="button" aria-pressed={on} onclick={() => onSet(seg.field, s)}>{s}</button>
        {/each}
      </span>
    {/if}
    {#if chipLink}<a class="go" href={chipLink}>{chipText}</a>{/if}
  </span>
</div>

<style>
  .frow {
    display: grid;
    /* The board's 172 and 150 px columns. */
    grid-template-columns:
      calc(4 * var(--space-10) + var(--space-3))
      calc(3 * var(--space-10) + var(--space-6) + var(--space-1) + var(--border-thick))
      minmax(0, 1fr);
    column-gap: var(--space-4);
    align-items: start;
    padding: var(--space-3) calc(var(--space-4) + var(--space-1));
    border-top: var(--border-thin) solid var(--color-line);
  }
  .frow.sel {
    background: var(--color-accent-soft);
    box-shadow: inset calc(var(--border-thick) + var(--border-thin)) 0 0 var(--color-accent);
  }
  .label {
    padding-top: calc(var(--space-3) - var(--border-thin));
    font-size: var(--font-size-body);
    font-weight: var(--weight-semibold);
  }
  .ro {
    display: block;
    padding-top: calc(var(--space-3) - var(--border-thin));
    font-family: var(--font-mono);
    font-size: var(--font-size-value-mono);
    overflow-wrap: anywhere;
  }
  .boxes {
    display: grid;
    gap: var(--space-2);
  }
  .boxes.two {
    grid-template-columns: 1fr 1fr;
  }
  .box {
    width: 100%;
    min-width: 0;
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
  .value {
    display: flex;
    flex-direction: column;
    gap: calc(var(--space-1) + var(--border-thick));
    min-width: 0;
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
  .box::placeholder {
    color: var(--color-text-muted);
  }
  .pickwrap {
    position: relative;
    display: block;
  }
  .pick {
    display: flex;
    align-items: center;
    justify-content: space-between;
    text-align: left;
  }
  .pick :global(.icon) {
    width: var(--size-icon-small);
    height: var(--size-icon-small);
  }
  .menu {
    position: absolute;
    top: calc(100% + var(--space-1));
    left: 0;
    z-index: 5;
    min-width: 100%;
    margin: 0;
    padding: calc(var(--space-1) + var(--border-thick));
    list-style: none;
    border: var(--border-thin) solid var(--color-popover-border);
    border-radius: var(--radius-control);
    background: var(--color-popover-surface);
    box-shadow: var(--color-overlay-shadow);
  }
  .menu button {
    width: 100%;
    height: var(--size-touch-min);
    padding: 0 var(--space-3);
    border: 0;
    border-radius: var(--radius-small);
    background: transparent;
    color: var(--color-text);
    font-family: var(--font-mono);
    font-size: var(--font-size-value-mono);
    text-align: left;
  }
  .menu [aria-selected="true"] button,
  .menu button:hover {
    background: var(--color-surface-2);
  }
  .about {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: calc(var(--space-1) + var(--border-thick));
    padding-top: calc(var(--space-3) - var(--border-thin));
  }
  .desc {
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
    line-height: var(--leading-normal);
  }
  .loc {
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
    line-height: var(--leading-normal);
  }
  .go {
    color: var(--color-link);
    font-size: var(--font-size-secondary);
    text-decoration: none;
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
    border: 0;
    border-radius: calc(var(--radius-small) + var(--border-thick));
    background: transparent;
    color: var(--color-text-muted);
    font: inherit;
    font-size: var(--font-size-secondary);
  }
  .seg button[aria-pressed="true"] {
    background: var(--color-segment-on);
    outline: var(--size-focus-ring) solid var(--color-accent);
    outline-offset: calc(-1 * var(--size-focus-ring));
    box-shadow: var(--color-segment-shadow);
    color: var(--color-text-strong);
    font-weight: var(--weight-semibold);
  }
  /* Tablet landscape (FSTakeoffTabL): label (140) and field (200) on the first line, the
     description, location line and link under them at full width. */
  @media (min-width: 900px) and (max-width: 1439px) {
    .frow {
      grid-template-columns: calc(3 * var(--space-10) + var(--space-6) - var(--space-1)) calc(5 * var(--space-10)) minmax(0, 1fr);
      column-gap: var(--space-3);
      row-gap: var(--space-1);
      padding: var(--space-3) var(--space-4);
    }
    .about {
      grid-column: 1 / -1;
      padding-top: var(--space-1);
    }
  }
  /* Phone: the label, then the value, then the description and chip, each the row's width
     (batch 5 brings the phone boards). */
  @media (max-width: 599px) {
    .frow {
      grid-template-columns: minmax(0, 1fr);
      row-gap: var(--space-2);
    }
    .label,
    .ro,
    .about {
      padding-top: 0;
    }
  }
</style>
