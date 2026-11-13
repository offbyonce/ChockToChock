<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Flow view, as in 3.0: one step at a time in large type, with the buttons at the bottom where a
thumb reaches them on a tablet. The current-item box (step 7) puts the item on the left and the
value on the right. The pilot confirms each step; the app never knows a switch position
(decision 0006). At the end of the phase come the completion calls.
-->
<script lang="ts">
  import BlankFields from "./BlankFields.svelte";
  import { fillParts } from "./value-parts";
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
    onBlank,
  }: {
    index: PackIndex;
    view: PhaseView;
    /** The step shown; null follows the current step. */
    cursor: string | null;
    detail: Detail;
    onConfirm: (id: string) => void;
    onSkip: (id: string) => void;
    onMove: (id: string) => void;
    /** What the pilot typed into a step's blank. */
    onBlank: (id: string, index: number, value: string) => void;
  } = $props();

  let entries = $derived(view.lists.flatMap((list) => list.steps.map((row, i) => ({ list, row, i }))));
  let at = $derived.by(() => {
    const id = cursor ?? view.lists.find((l) => l.current)?.current ?? null;
    return id ? entries.findIndex((e) => e.row.id === id) : -1;
  });
  let entry = $derived(at >= 0 ? entries[at] : undefined);
  let step = $derived<Step | undefined>(entry ? index.step(entry.row.id) : undefined);
  let fill = $derived(step ? fillParts(step.expect) : null);
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
      {#if fill}
        <div class="value fill">
          <BlankFields name={step.text} blanks={fill.blanks} rest={fill.rest} values={entry.row.values} onBlank={(n, v) => onBlank(step.id, n, v)} />
        </div>
      {:else}
        <p class="value">{step.expect}</p>
        {#if entry.row.hint}<p class="vh">{entry.row.hint}</p>{/if}
      {/if}
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
      <p class="nx"><b>Next</b> <span class="nn">{next.text}</span> {#if !next.expect.includes("___")}<span class="mono">{next.expect}</span>{/if}</p>
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
    min-height: calc(100vh - var(--size-header) - var(--size-phase-row) - var(--size-flight-strip) - 2 * var(--space-10));
  }
  .crumb {
    margin: 0;
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
  }
  /* The name and the value share a line when both fit; otherwise the value goes under the name,
     as in 3.0, so the name is never squeezed onto more lines. */
  .box {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-2) var(--space-8);
    margin: var(--space-3) 0 var(--space-2);
    padding: var(--space-6) var(--space-6);
    border: var(--border-thin) solid var(--color-border);
    border-left: var(--border-bar) solid var(--color-accent);
    border-radius: var(--radius-panel);
    background: var(--color-surface);
  }
  .box.done {
    border-left-color: var(--color-done);
  }
  .box.skipped {
    border-left-style: dashed;
    border-left-color: var(--color-open-text);
  }
  /* The headline is sized to fit its column: its longest word on one line and the whole name in
     about three, so it never breaks inside a word or leaves one word hanging. */
  .box {
    container-type: inline-size;
  }
  h1 {
    margin: 0;
    font-size: clamp(
      var(--font-size-section-title),
      min(calc(55cqi / (var(--longest) * 0.66)), calc(160cqi / (var(--len) * 0.66))),
      var(--font-size-flow-title)
    );
    font-weight: 650;
    line-height: 1.06;
    text-wrap: balance;
    max-width: 100%;
  }
  .value {
    margin: 0;
    color: var(--color-accent);
    font: var(--weight-medium) var(--font-size-flow-value) / var(--leading-tight) var(--font-mono);
  }
  .value.fill {
    color: var(--color-text);
    font-size: var(--font-size-body);
  }
  .vh {
    margin: var(--space-1) 0 0;
    color: var(--color-accent);
    font-size: var(--font-size-secondary);
  }
  .state {
    flex-basis: 100%;
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--space-2) var(--space-3);
    margin: 0;
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
  }
  .ns {
    padding: var(--space-1) var(--space-2);
    border-radius: var(--radius-pill);
    background: var(--color-surface-2);
    color: var(--color-text-secondary);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-bold);
  }
  .more {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    margin-top: var(--space-2);
  }
  .more p {
    margin: 0;
  }
  .link {
    align-self: flex-start;
    min-height:  var(--size-touch-min);
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
  }
  details summary {
    display: inline-flex;
    align-items: center;
    gap: var(--space-2);
    min-height:  var(--size-touch-min);
    color: var(--color-accent);
    font-weight: var(--weight-bold);
    cursor: pointer;
  }
  @media (min-width: 900px) {
    .inline-context {
      display: none;
    }
  }
  .bar {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--space-3);
    margin-top: auto;
    padding-top: var(--space-4);
  }
  .bar + .bar {
    margin-top: 0;
  }
  .bar .btn {
    min-height: var(--size-flow-button);
    padding-inline: var(--space-6);
  }
  .bar .big {
    padding-inline: var(--space-10);
    font-size: var(--font-size-panel-title);
  }
  .keys {
    margin-left: auto;
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
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
    gap: var(--space-1) var(--space-3);
    margin: var(--space-4) 0 0;
    padding-top: var(--space-3);
    border-top: var(--border-thin) solid var(--color-border);
  }
  .nx b {
    color: var(--color-text-muted);
    font-weight: var(--weight-semibold);
  }
  .nn {
    font-weight: var(--weight-medium);
  }
  .nx .mono {
    color: var(--color-text-muted);
    font-size: var(--font-size-secondary);
  }
  .end {
    margin: var(--space-2) 0 var(--space-4);
  }
  .calls {
    display: grid;
    gap: var(--space-2);
    margin: 0;
    padding: 0;
    list-style: none;
  }
  .calls li {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    padding: var(--space-4) var(--space-4);
    border-radius: var(--radius-control);
    background: var(--color-open-bg);
    color: var(--color-open-text);
    font-weight: var(--weight-bold);
  }
  .calls li.done {
    background: var(--color-surface-2);
    color: var(--color-done);
    text-transform: uppercase;
  }
</style>
