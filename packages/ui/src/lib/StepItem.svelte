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

  let {
    index,
    step,
    row,
    doneAt,
    detail,
    selected,
    large = false,
    onTick,
    onSkip,
    onSelect,
  }: {
    index: PackIndex;
    step: Step;
    row: StepRow;
    doneAt: number | undefined;
    detail: Detail;
    selected: boolean;
    large?: boolean;
    onTick: () => void;
    onSkip: () => void;
    onSelect: () => void;
  } = $props();

  let open = $state(false);
  let mapOpen = $state(false);
  let place = $derived(index.place(step));
  let time = $derived(
    doneAt === undefined ? "" : new Date(doneAt).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" }),
  );
  const STATE_WORDS = { to_do: "to do", current: "current", done: "done", skipped: "skipped, still open" };
  let tickLabel = $derived(
    row.state === "done" ? `Clear the tick on ${step.text}` : `Confirm ${step.text}: ${step.expect}`,
  );
</script>

<li class="it {row.state}" class:sel={selected} class:large>
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
        <span class="sr-only">, {STATE_WORDS[row.state]}</span>
        {#if row.state === "current"}<span class="tag cur" aria-hidden="true">Current</span>{/if}
        {#if row.state === "skipped"}<span class="tag skip" aria-hidden="true">Skipped, still open</span>{/if}
        {#if row.state === "done"}<span class="tag done" aria-hidden="true">Done {time}</span>{/if}
        {#if !row.counts}<span class="tag ns" aria-hidden="true">Not modeled in the simulator</span>{/if}
      </span>
      {#if step.expect}<span class="val">{step.expect}</span>{/if}
    </button>
    {#if detail === "expanded"}
      {#if place}<div class="loc">{place.zone.area} · {place.zone.title}</div>{/if}
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
          <StepDetail {index} {step} />
          {#if place?.control}
            <p class="narrow where"><b>Where</b> · {place.control.location}</p>
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
            <p class="dskip"><span>Skipped. It stays open until you tick it.</span></p>
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
  /* Sizes and weights follow 3.0's step rows. */
  .it {
    position: relative;
    display: grid;
    grid-template-columns: 3rem minmax(0, 1fr);
    border-top: 1px solid var(--line);
  }
  .it:first-child {
    border-top: 0;
  }
  .it.current {
    box-shadow: inset 0.1875rem 0 0 var(--accent);
  }
  .it.sel {
    background: var(--accent-soft);
  }
  .tick {
    display: grid;
    place-items: start center;
    min-height: 3.25rem;
    padding-top: 0.8125rem;
  }
  .box {
    display: grid;
    place-items: center;
    width: 1.375rem;
    height: 1.375rem;
    border: 1.5px solid var(--muted);
    border-radius: 0.4375rem;
    background: var(--surface);
    font-size: 0.8rem;
  }
  .current .box {
    border: 2px solid var(--accent);
  }
  .done .box {
    border-color: var(--go);
    background: var(--go);
    color: var(--surface);
  }
  .skipped .box {
    border: 2px dashed var(--warn);
    color: var(--warn);
  }
  .b {
    position: relative;
    min-width: 0;
    padding: 0.625rem 2.75rem 0.625rem 0;
  }
  .row {
    display: flex;
    flex-wrap: wrap;
    justify-content: space-between;
    gap: 0.125rem 1rem;
    width: 100%;
    text-align: left;
  }
  .name {
    min-width: 0;
    font-weight: 500;
  }
  .name small {
    margin-right: 0.4375rem;
    color: var(--muted);
    font: 400 0.75rem var(--mono);
  }
  .val {
    min-width: 0;
    max-width: 62%;
    margin-left: auto;
    font: 0.8438rem/1.5 var(--mono);
    text-align: right;
  }
  .done .name,
  .done .val {
    color: var(--muted);
  }
  .loc {
    margin-top: 1px;
    color: var(--muted);
    font-size: 0.7812rem;
  }
  .tag {
    display: inline-block;
    margin-left: 0.5rem;
    padding: 0 0.4375rem;
    border-radius: 99rem;
    font-size: 0.6875rem;
    font-weight: 600;
    line-height: 1.25rem;
    vertical-align: 0.0625rem;
  }
  .tag.cur {
    background: var(--accent-surface);
    color: var(--on-accent);
  }
  .tag.skip {
    border: 1px dashed var(--warn);
    color: var(--warn);
    line-height: 1.125rem;
  }
  .tag.done {
    padding: 0;
    color: var(--go);
    font: 400 0.6875rem var(--mono);
  }
  .tag.ns {
    background: var(--zone);
    color: var(--zone-ink);
  }
  .chev {
    position: absolute;
    top: 0;
    right: 0;
    display: grid;
    place-items: center;
    width: 2.75rem;
    height: 2.75rem;
    border-radius: 0.5rem;
    color: var(--muted);
    font-size: 0.8rem;
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
    gap: 0.625rem;
    margin: 0.5rem 0 0.2rem;
    padding: 0.25rem 0 0.25rem;
    font-size: 0.87rem;
    line-height: 1.45;
  }
  .where {
    margin: 0;
    color: var(--muted);
    font-size: 0.875rem;
  }
  .where b {
    color: var(--ink);
    font-weight: 600;
  }
  .btn.quiet {
    align-self: flex-start;
    min-height: 2.75rem;
    font-weight: 500;
  }
  .btn.quiet :global(.icon) {
    color: var(--muted);
  }
  .inmap {
    padding-top: 0.25rem;
  }
  .dskip {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.5rem 0.75rem;
    margin: 0.25rem 0 0;
    padding-top: 0.625rem;
    border-top: 1px solid var(--line);
  }
  .dskip span {
    color: var(--muted);
    font-size: 0.8125rem;
  }
  /* Wide screens show where it is and the map in the panel beside the list. */
  @media (min-width: 1024px) {
    .narrow {
      display: none !important;
    }
  }
  /* Non-normal and reference lists use larger type (step 7). */
  .large {
    font-size: 1.0625rem;
  }
  .large .val {
    font-size: 0.95rem;
  }
  @media (max-width: 760px) {
    .val {
      width: 100%;
      max-width: 100%;
      margin-left: 0;
      text-align: left;
    }
  }
</style>
