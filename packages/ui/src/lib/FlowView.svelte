<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Flow view, as in 3.0: one step at a time in large type, with the buttons at the bottom where a
thumb reaches them on a tablet. The current-item box (step 7) puts the item on the left and the
value on the right. The pilot confirms each step; the app never knows a switch position
(decision 0006). At the end of the phase come the completion calls.
-->
<script lang="ts">
  import type { Detail, PhaseView, Step } from "@chocktochock/bindings";
  import type { PackIndex } from "../pack-index";
  import { phaseHref } from "../route.svelte";
  import ContextPane from "./ContextPane.svelte";
  import Icon from "./Icon.svelte";
  import StepDetail from "./StepDetail.svelte";

  let {
    index,
    view,
    cursor,
    detail,
    onConfirm,
    onSkip,
    onMove,
  }: {
    index: PackIndex;
    view: PhaseView;
    /** The step shown; null follows the current step. */
    cursor: string | null;
    detail: Detail;
    onConfirm: (id: string) => void;
    onSkip: (id: string) => void;
    onMove: (id: string) => void;
  } = $props();

  let entries = $derived(view.lists.flatMap((list) => list.steps.map((row, i) => ({ list, row, i }))));
  let at = $derived.by(() => {
    const id = cursor ?? view.lists.find((l) => l.current)?.current ?? null;
    return id ? entries.findIndex((e) => e.row.id === id) : -1;
  });
  let entry = $derived(at >= 0 ? entries[at] : undefined);
  let step = $derived<Step | undefined>(entry ? index.step(entry.row.id) : undefined);
  let place = $derived(step ? index.place(step) : null);
  /** Whether the step sets a control (its value names a position), or only checks something. */
  let sets = $derived(entry?.row.targets.some((t) => t.position != null) ?? false);
  let next = $derived.by(() => {
    const e = entries.slice(at + 1).find((x) => x.row.state !== "done");
    return e ? index.step(e.row.id) : undefined;
  });
  let showDetail = $state(true);

  /** 3.0's keys: Space confirms, Left arrow goes back, S skips. Ignored while typing or on a control. */
  function onKey(e: KeyboardEvent): void {
    if (!entry || !step || e.altKey || e.ctrlKey || e.metaKey) return;
    const t = e.target;
    if (t instanceof Element && t.closest("input, textarea, select, button, a, summary, [contenteditable]")) return;
    if (e.key === " ") {
      e.preventDefault();
      onConfirm(step.id);
    } else if (e.key === "ArrowLeft") {
      const prev = entries[at - 1];
      if (prev) onMove(prev.row.id);
    } else if (e.key === "s" || e.key === "S") {
      if (entry.row.state !== "skipped") onSkip(step.id);
    }
  }
</script>

<svelte:window onkeydown={onKey} />

<div class="flow">
  {#if entry && step}
    <p class="crumb">{view.title} · Step {at + 1} of {entries.length}</p>
    {#if place && detail === "expanded"}<p class="crumb">{place.zone.area} · {place.zone.title}</p>{/if}

    <section class="box {entry.row.state}" aria-labelledby="flow-step">
      <h1
        id="flow-step"
        style:--len={step.text.length}
        style:--longest={Math.max(...step.text.split(/\s+/).map((w) => w.length))}
      >
        {step.text}
      </h1>
      <p class="value">{step.expect}</p>
      <p class="state">
        {#if entry.row.state === "done"}<Icon name="check" /> Done. Confirm again to move on.
        {:else if entry.row.state === "skipped"}<Icon name="skip" /> Skipped, still open
        {:else if sets}Set it, check it, then confirm.
        {:else}Check it, then confirm.{/if}
        {#if !entry.row.counts}<span class="ns">Not modeled in the simulator</span>{/if}
      </p>
    </section>

    {#if detail === "expanded"}
      {#if showDetail}
        <div class="more">
          <StepDetail {index} {step} large />
          {#if place}
            <details class="inline-context">
              <summary><Icon name="map-pin" /> Where is this?</summary>
              <ContextPane {index} {step} targets={entry.row.targets} heading={false} details={false} />
            </details>
          {/if}
        </div>
      {/if}
      <button class="link" type="button" onclick={() => (showDetail = !showDetail)}>
        {showDetail ? "Hide details" : "Show details"}
      </button>
    {/if}

    <div class="bar">
      <button class="btn primary big" type="button" onclick={() => onConfirm(step.id)}>Confirmed, next step</button>
      <button
        class="btn"
        type="button"
        disabled={at <= 0}
        onclick={() => {
          const prev = entries[at - 1];
          if (prev) onMove(prev.row.id);
        }}
      >
        Back
      </button>
      {#if entry.row.state !== "skipped"}
        <button class="btn skip" type="button" onclick={() => onSkip(step.id)}>Skip</button>
      {/if}
      <span class="keys">Space: confirm · Left arrow: back · S: skip</span>
    </div>
    {#if next}
      <p class="nx"><b>Next</b> <span class="nn">{next.text}</span> <span class="mono">{next.expect}</span></p>
    {/if}
  {:else}
    <p class="crumb">{view.title}</p>
    <h1 class="end">{view.complete ? `${view.title} complete` : "Steps still open"}</h1>
    <ul class="calls">
      {#each view.lists as list (list.id)}
        <li class:done={list.complete}>
          {#if list.complete}
            <Icon name="check" /> {list.title} complete
          {:else}
            <Icon name="alert-triangle" /> {list.title}: {list.progress.skipped} skipped, still open
          {/if}
        </li>
      {/each}
    </ul>
    {#if !view.complete}
      {@const first = entries.find((e) => e.row.state === "skipped")}
      {#if first}
        <div class="bar">
          <button class="btn primary big" type="button" onclick={() => onMove(first.row.id)}>Go to the first skipped step</button>
        </div>
      {/if}
    {/if}
    {#if view.next}
      <div class="bar">
        <a class="btn {view.complete ? 'primary big' : ''}" href={phaseHref(view.next.id)}>
          {view.next.title} <Icon name="arrow-right" />
        </a>
      </div>
    {/if}
  {/if}
</div>

<style>
  .flow {
    display: flex;
    flex-direction: column;
    /* The screen under the bars, less the flight strip and padding, so Next stays in view. */
    min-height: calc(100vh - var(--top-h) - var(--rail-h) - 8.5rem);
  }
  .crumb {
    margin: 0;
    color: var(--muted);
    font-size: 0.8125rem;
  }
  /* The name and the value share a line when both fit; otherwise the value goes under the name,
     as in 3.0, so the name is never squeezed onto more lines. */
  .box {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem 2rem;
    margin: 0.75rem 0 0.5rem;
    padding: 1.25rem 1.5rem;
    border: 1px solid var(--line);
    border-left: 0.375rem solid var(--accent);
    border-radius: 0.75rem;
    background: var(--surface);
  }
  .box.done {
    border-left-color: var(--go);
  }
  .box.skipped {
    border-left-style: dashed;
    border-left-color: var(--warn);
  }
  /* The headline is sized to fit its column: its longest word on one line and the whole name in
     about three, so it never breaks inside a word or leaves one word hanging. */
  .box {
    container-type: inline-size;
  }
  h1 {
    margin: 0;
    font-size: clamp(
      1.6rem,
      min(calc(55cqi / (var(--longest) * 0.66)), calc(160cqi / (var(--len) * 0.66))),
      3.4rem
    );
    font-weight: 650;
    line-height: 1.06;
    letter-spacing: -0.02em;
    text-wrap: balance;
    max-width: 100%;
  }
  .value {
    margin: 0;
    color: var(--accent);
    font: clamp(1.15rem, 1.9vw, 1.7rem) / 1.25 var(--mono);
  }
  .state {
    flex-basis: 100%;
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.375rem 0.75rem;
    margin: 0;
    color: var(--muted);
    font-size: 0.875rem;
  }
  .ns {
    padding: 0.0625rem 0.5rem;
    border-radius: 99rem;
    background: var(--zone);
    color: var(--zone-ink);
    font-size: 0.75rem;
    font-weight: 700;
  }
  .more {
    display: flex;
    flex-direction: column;
    gap: 0.625rem;
    margin-top: 0.5rem;
  }
  .more p {
    margin: 0;
  }
  .link {
    align-self: flex-start;
    min-height: 2.75rem;
    color: var(--muted);
    font-size: 0.8125rem;
  }
  details summary {
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    min-height: 2.75rem;
    color: var(--accent);
    font-weight: 700;
    cursor: pointer;
  }
  @media (min-width: 1024px) {
    .inline-context {
      display: none;
    }
  }
  .bar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 0.625rem;
    margin-top: auto;
    padding-top: 1rem;
  }
  .bar + .bar {
    margin-top: 0;
  }
  .bar .btn {
    min-height: 3.5rem;
    padding-inline: 1.5rem;
  }
  .bar .big {
    padding-inline: 2.25rem;
    font-size: 1.15rem;
  }
  .keys {
    margin-left: auto;
    color: var(--muted);
    font-size: 0.75rem;
  }
  @media (pointer: coarse) {
    .keys {
      display: none;
    }
  }
  .btn:disabled {
    opacity: 0.45;
  }
  .nx {
    display: flex;
    flex-wrap: wrap;
    align-items: baseline;
    gap: 0.25rem 0.75rem;
    margin: 1rem 0 0;
    padding-top: 0.75rem;
    border-top: 1px solid var(--line);
  }
  .nx b {
    color: var(--muted);
    font-weight: 600;
  }
  .nn {
    font-weight: 500;
  }
  .nx .mono {
    color: var(--muted);
    font-size: 0.9rem;
  }
  .end {
    margin: 0.5rem 0 1rem;
  }
  .calls {
    display: grid;
    gap: 0.5rem;
    margin: 0;
    padding: 0;
    list-style: none;
  }
  .calls li {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.875rem 1rem;
    border-radius: 0.625rem;
    background: var(--warn-soft);
    color: var(--warn);
    font-weight: 700;
  }
  .calls li.done {
    background: var(--go-soft);
    color: var(--go);
    text-transform: uppercase;
  }
</style>
