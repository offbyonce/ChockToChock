<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

One step in Checklist view, laid out as in 3.0: tick box, number and name, the value on the right
in a monospaced face, and where it is underneath. Every step looks the same (decision 0013).
States read without color too (step 7): to do is an empty box; current has a bar, a heavier box
and the word "Current"; done is a filled box with a check and the time; skipped is a dashed box
with the words "Skipped, still open". The quick level shows the item and value only.
-->
<script lang="ts">
  import type { Detail, Step, StepRow } from "@chocktochock/bindings";
  import type { PackIndex } from "../pack-index";
  import ContextPane from "./ContextPane.svelte";
  import Icon from "./Icon.svelte";
  import StepDetail from "./StepDetail.svelte";
  import BlankFields from "./BlankFields.svelte";
  import { fillParts } from "./value-parts";

  let {
    index,
    step,
    row,
    doneAt,
    detail,
    selected,
    marked,
    large = false,
    full = true,
    onTick,
    onSkip,
    onUnskip,
    onBlank,
    onSelect,
  }: {
    index: PackIndex;
    step: Step;
    row: StepRow;
    doneAt: number | undefined;
    detail: Detail;
    selected: boolean;
    /** Carries the blue bar: the step the side pane shows. */
    marked: boolean;
    large?: boolean;
    full?: boolean;
    onTick: () => void;
    onSkip: () => void;
    onUnskip: () => void;
    /** What the pilot typed into the step's n-th blank. */
    onBlank: (index: number, value: string) => void;
    onSelect: () => void;
  } = $props();

  let open = $state(false);
  let mapOpen = $state(false);
  /** As 3.0: a value that is long or has blanks to fill goes on its own line, in the text face. */
  let long = $derived(step.expect.includes("___") || step.expect.length > 36);
  /** A value with blanks: the parts with blanks first, the rest of the instruction after. */
  let fill = $derived(fillParts(step.expect));
  /** Other controls the step uses, shown as "+N" after where it is. */
  let more = $derived(Math.max(0, (step.controls?.length ?? 0) - 1));
  let place = $derived(index.place(step));
  let time = $derived(
    doneAt === undefined ? "" : new Date(doneAt).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" }),
  );
  const STATE_WORDS = { to_do: "to do", current: "current", done: "done", skipped: "skipped, still open" };
  let tickLabel = $derived(
    row.state === "done" ? `Clear the tick on ${step.text}` : `Confirm ${step.text}: ${step.expect}`,
  );
</script>

<li class="it {row.state}" class:sel={selected} class:marked class:large>
  <button class="tick" type="button" aria-label={tickLabel} aria-pressed={row.state === "done"} onclick={onTick}>
    <span class="box">
      {#if row.state === "done"}<Icon name="check" />{:else if row.state === "skipped"}<Icon name="skip" />{/if}
    </span>
  </button>
  <div class="b">
    <!-- Selecting a step shows it in the context panel; the tick box is the only thing that ticks. -->
    <button class="row" type="button" onclick={onSelect} aria-current={selected ? "true" : undefined}>
      <span class="name">
        <small>{row.number}</small>{step.text}
        {#if full && row.essential && !large}<span class="ebadge" title="In Essentials" aria-hidden="true">E</span><span class="sr-only">, in Essentials</span>{/if}
        <span class="sr-only">, {STATE_WORDS[row.state]}</span>
        {#if row.state === "current"}<span class="tag cur" aria-hidden="true">Current</span>{/if}
        {#if row.state === "skipped"}<span class="tag skip" aria-hidden="true">Skipped, still open</span>{/if}
        {#if row.state === "done"}<span class="tag done" aria-hidden="true">Done {time}</span>{/if}
        {#if !row.counts}<span class="tag ns" aria-hidden="true">Not modeled in the simulator</span>{/if}
      </span>
      {#if step.expect && !fill}
        <span class="val" class:long>{step.expect}{#if row.hint}<small class="vh">{row.hint}</small>{/if}</span>
      {/if}
    </button>
    {#if step.expect && fill}
      <!-- As 3.0: each blank is a field with the words before it, the parts to fill in on one line
           in the monospaced face, and the rest of the instruction on its own muted line. A blank
           shows the flight value (decision 0019) until the pilot types over it. -->
      <BlankFields name={step.text} blanks={fill.blanks} rest={fill.rest} values={row.values} {onBlank} />
    {/if}
    {#if detail === "expanded"}
      {#if place}
        <div class="loc">{place.zone.area} · {place.zone.title}{#if more > 0}{" "}<span class="more">+{more}</span>{/if}</div>
      {/if}
      {#if step.look_for}<div class="look"><b>Look for:</b> {step.look_for}</div>{/if}
      <button
        class="chev"
        type="button"
        aria-expanded={open}
        aria-label="{open ? 'Hide' : 'Show'} details for {step.text}"
        onclick={() => (open = !open)}
      >
        <Icon name="chevron-down" />
      </button>
      {#if open}
        <!-- As 3.0: the details, where it is and the map on narrow screens (the side panel has
             them on wide ones), then Skip for now as a quiet button at the bottom. -->
        <div class="dtl">
          <div class="inline-detail"><StepDetail {index} {step} /></div>
          {#if place?.control}
            <p class="narrow where"><b>Where</b> · {place.control.location}</p>
          {/if}
          <!-- A step with no Why says what its control does, in the step on every width, as 3.0 did. -->
          {#if !step.why && place?.control?.function}
            <p class="where"><b>What it does</b> · {place.control.function}</p>
          {/if}
          {#if place}
            <button class="btn quiet narrow" type="button" aria-expanded={mapOpen} onclick={() => (mapOpen = !mapOpen)}>
              <Icon name="map-pin" />{mapOpen ? "Hide map" : "Show on map"}
            </button>
            {#if mapOpen}
              <div class="narrow inmap"><ContextPane {index} {step} targets={row.targets} heading={false} details={false} /></div>
            {/if}
          {/if}
          {#if row.state === "skipped"}
            <p class="dskip">
              <button class="btn quiet" type="button" onclick={onUnskip}>Unskip</button>
              <span>Skipped. It stays in Open steps until you tick it.</span>
            </p>
          {:else if row.state !== "done"}
            <p class="dskip">
              <button class="btn quiet" type="button" onclick={onSkip}><Icon name="skip" /> Skip for now</button>
              <span>Find it later in Open steps.</span>
            </p>
          {/if}
        </div>
      {/if}
    {/if}
  </div>
</li>

<style>
  /* Beside a side pane (tablet landscape and up) the pane shows Check, If not and Why, so the
     row does not repeat them (Checklist board). */
  @media (min-width: 900px) {
    :global(.app:not(.nopane):not(.flow)) .inline-detail {
      display: none;
    }
  }
  /* Sizes and weights follow 3.0's step rows. */
  .it {
    position: relative;
    display: grid;
    grid-template-columns: var(--size-touch-min) minmax(0, 1fr);
    border-top: var(--border-thin) solid var(--color-border);
  }
  .it:first-child {
    border-top: 0;
  }
  .it.marked {
    box-shadow: inset var(--border-bar) 0 0 var(--color-accent);
  }
  .it.sel {
    background: var(--color-accent-soft);
  }
  .tick {
    display: grid;
    place-items: start center;
    min-height: var(--size-touch-min);
    padding-top: var(--space-3);
  }
  .box {
    display: grid;
    place-items: center;
    width: var(--size-status-icon);
    height: var(--size-status-icon);
    border: var(--size-focus-ring) solid var(--color-text-muted);
    border-radius: var(--radius-small);
    background: var(--color-surface);
    font-size: var(--font-size-caption);
  }
  .current .box {
    border: var(--border-thick) solid var(--color-accent);
  }
  .done .box {
    border-color: var(--color-done);
    background: var(--color-done);
    color: var(--color-surface);
  }
  .skipped .box {
    border: var(--border-thick) dashed var(--color-open-text);
    color: var(--color-open-text);
  }
  .b {
    position: relative;
    min-width: 0;
    padding: var(--space-3) var(--space-10) var(--space-3) 0;
  }
  /* The whole row is the button, full width; it keeps its text height so rows stay dense. The
     step row component in batch 3 makes the full row (with its location line) the target. */
  .row {
    min-height: 0;
    display: flex;
    flex-wrap: wrap;
    align-items: baseline;
    justify-content: space-between;
    gap: var(--space-1) var(--space-4);
    width: 100%;
    text-align: left;
  }
  .name {
    min-width: 0;
    font-weight: var(--weight-medium);
  }
  .name small {
    margin-right: var(--space-2);
    color: var(--color-text-muted);
    font: var(--weight-regular) var(--font-size-caption) var(--font-mono);
    vertical-align: middle;
  }
  .val {
    min-width: 0;
    max-width: 62%;
    margin-left: auto;
    font: var(--weight-regular) var(--font-size-value-mono) / var(--leading-normal) var(--font-mono);
    text-align: right;
  }
  .val.long {
    flex: 1 1 100%;
    max-width: none;
    margin: var(--space-1) 0 0;
    font: var(--weight-regular) var(--font-size-secondary) / var(--leading-normal) var(--font-sans);
    text-align: left;
  }
  /* A hint from flight values under the step's value ("GND 121.700"), as 3.0 showed it. */
  .vh {
    display: block;
    color: var(--color-accent);
    font-size: var(--font-size-secondary);
  }
  .more {
    color: var(--color-text-muted);
  }
  .look {
    margin-top: var(--space-1);
    font-size: var(--font-size-secondary);
  }
  .look b {
    font-weight: var(--weight-semibold);
  }
  .done .name,
  .done .val {
    color: var(--color-text-muted);
  }
  .loc {
    margin-top: var(--space-1);
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  /* 3.0's boxed E after a step in Essentials. */
  .ebadge {
    display: inline-grid;
    place-items: center;
    width: var(--size-icon-small);
    height: var(--size-icon-small);
    margin-left: var(--space-2);
    border: var(--border-thin) solid var(--color-accent);
    border-radius: var(--radius-small);
    color: var(--color-accent);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-semibold);
    vertical-align: middle;
  }
  .tag {
    display: inline-block;
    margin-left: var(--space-2);
    padding: 0 var(--space-2);
    border-radius: var(--radius-pill);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-semibold);
    line-height: var(--leading-tight);
    vertical-align: middle;
  }
  .tag.cur {
    background: var(--color-accent);
    color: var(--color-on-accent);
  }
  .tag.skip {
    border: var(--border-thin) dashed var(--color-open-text);
    color: var(--color-open-text);
    line-height: var(--leading-tight);
  }
  .tag.done {
    padding: 0;
    color: var(--color-done);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-semibold);
  }
  .tag.ns {
    background: var(--color-surface-2);
    color: var(--color-text-secondary);
  }
  .chev {
    position: absolute;
    top: 0;
    right: 0;
    display: grid;
    place-items: center;
    width:  var(--size-touch-min);
    height:  var(--size-touch-min);
    border-radius: var(--radius-control);
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  .chev :global(.icon) {
    transition: transform 0.12s;
  }
  .chev[aria-expanded="true"] :global(.icon) {
    transform: rotate(180deg);
  }
  .dtl {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    margin: var(--space-2) 0 var(--space-1);
    padding: var(--space-1) 0 var(--space-1);
    font-size: var(--font-size-secondary);
    line-height: 1.45;
  }
  .where {
    margin: 0;
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
  }
  .where b {
    color: var(--color-text);
    font-weight: var(--weight-semibold);
  }
  .btn.quiet {
    align-self: flex-start;
    min-height:  var(--size-touch-min);
    font-weight: var(--weight-medium);
  }
  .btn.quiet :global(.icon) {
    color: var(--color-text-muted);
  }
  .inmap {
    padding-top: var(--space-1);
  }
  .dskip {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--space-2) var(--space-3);
    margin: var(--space-1) 0 0;
    padding-top: var(--space-3);
    border-top: var(--border-thin) solid var(--color-border);
  }
  .dskip span {
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
  }
  /* Wide screens show where it is and the map in the panel beside the list. */
  @media (min-width: 900px) {
    .narrow {
      display: none !important;
    }
  }
  /* Non-normal and reference lists use larger type (step 7). */
  .large {
    font-size: var(--font-size-body);
  }
  .large .val {
    font-size: var(--font-size-body);
  }
  @media (max-width: 599px) {
    .val {
      width: 100%;
      max-width: 100%;
      margin-left: 0;
      text-align: left;
    }
  }
</style>
