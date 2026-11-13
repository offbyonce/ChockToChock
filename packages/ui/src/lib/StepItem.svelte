<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

One step in the procedure card (Checklist board): its status icon (which ticks), the name with
its E marker, the value in an aligned mono column, and the row menu (Make this the current step,
Mark done, Skip for now, Add to or Remove from Essentials). The whole row selects the step, a 44 px
target. The current step is larger, with where it is, Done, Skip for now and the keyboard hint; a
skipped step has a Skipped pill and Do it now. States read without color too: the icon's shape
and the words. Every step looks the same (decision 0013). Blanks stay inline fields in the value
column until batch 4 (owner). On narrow screens a chevron opens the step's details, until batch 5.
-->
<script lang="ts">
  import type { Detail, Step, StepRow } from "@chocktochock/bindings";
  import type { PackIndex } from "../pack-index";
  import BlankFields from "./BlankFields.svelte";
  import ContextPane from "./ContextPane.svelte";
  import Icon from "./Icon.svelte";
  import { stepTitle } from "./sentence";
  import StepDetail from "./StepDetail.svelte";
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
    onEssentials,
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
    /** Adds the step to Essentials (true) or removes it (false). */
    onEssentials?: ((add: boolean) => void) | undefined;
  } = $props();

  let open = $state(false);
  let mapOpen = $state(false);
  let menu = $state(false);
  /** A value with blanks: the parts with blanks first, the rest of the instruction after. */
  let fill = $derived(fillParts(step.expect));
  let place = $derived(index.place(step));
  let title = $derived(stepTitle(index, step));
  let current = $derived(row.state === "current");
  let time = $derived(
    doneAt === undefined ? "" : new Date(doneAt).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" }),
  );
  const STATE_WORDS = { to_do: "to do", current: "current", done: "done", skipped: "skipped, still open" };
  let tickLabel = $derived(row.state === "done" ? `Clear the tick on ${title}` : `Confirm ${title}: ${step.expect}`);

  function act(fn: () => void): void {
    menu = false;
    fn();
  }
  function outside(e: PointerEvent): void {
    if (menu && !(e.target instanceof Element && e.target.closest(`[data-menu="${row.id}"]`))) menu = false;
  }
  function key(e: KeyboardEvent): void {
    if (menu && e.key === "Escape") menu = false;
  }
</script>

<svelte:document onpointerdown={outside} onkeydown={key} />

<li class="it {row.state}" class:sel={selected} class:marked class:large class:current>
  <button class="tick" type="button" aria-label={tickLabel} aria-pressed={row.state === "done"} onclick={onTick}>
    <span class="status {row.state}" aria-hidden="true">
      {#if row.state === "done"}<Icon name="check" />{:else if row.state === "skipped"}<i class="dash"></i>{:else if current}<i class="pip"></i>{/if}
    </span>
  </button>
  <!-- The whole row selects the step (a stretched target over the row); the controls in it sit
       above that target. -->
  <div class="namecol">
    <button class="row" type="button" onclick={onSelect} aria-current={selected ? "true" : undefined}>
      <span class="name">
        <span class="nm">{title}</span>
        {#if full && row.essential && !large}<span class="ebadge" title="Essentials step" aria-hidden="true">E</span><span class="sr-only">, in Essentials</span>{/if}
        <span class="sr-only">, {STATE_WORDS[row.state]}</span>
        {#if row.state === "skipped"}<span class="pill" aria-hidden="true">Skipped</span>{/if}
        {#if row.state === "done" && time}<span class="stamp" aria-hidden="true">Done {time}</span>{/if}
        {#if !row.counts}<span class="ns">Not modeled in the simulator</span>{/if}
      </span>
      {#if current && place}<span class="where-line">{place.zone.area} · {place.zone.title}</span>{/if}
      {#if detail === "expanded" && step.look_for}<span class="look"><b>Look for:</b> {step.look_for}</span>{/if}
    </button>
    {#if row.state === "skipped"}<button class="do-now" type="button" onclick={onSelect}>Do it now</button>{/if}
  </div>
  <div class="value">
    {#if step.expect && fill}
      <BlankFields name={step.text} blanks={fill.blanks} rest={fill.rest} values={row.values} {onBlank} />
    {:else if step.expect}
      <span class="val">{step.expect}</span>
    {/if}
    {#if row.hint}<small class="hint">{row.hint}</small>{/if}
  </div>
  <div class="end" data-menu={row.id}>
    <button class="more" class:on={menu} type="button" aria-label="More actions for {title}" aria-expanded={menu} aria-haspopup="menu" onclick={() => (menu = !menu)}>···</button>
    {#if menu}
      <div class="menu" role="menu" aria-label="Actions for {title}">
        <button role="menuitem" type="button" onclick={() => act(onSelect)}>Make this the current step</button>
        {#if row.state !== "done"}<button role="menuitem" type="button" onclick={() => act(onTick)}>Mark done</button>{/if}
        {#if row.state === "skipped"}
          <button role="menuitem" type="button" onclick={() => act(onUnskip)}>Unskip</button>
        {:else if row.state !== "done"}
          <button role="menuitem" class="skip" type="button" onclick={() => act(onSkip)}>Skip for now</button>
        {/if}
        {#if onEssentials && !large}
          <hr />
          <button role="menuitem" type="button" onclick={() => act(() => onEssentials?.(!row.essential))}>{row.essential ? "Remove from Essentials" : "Add to Essentials"}</button>
        {/if}
      </div>
    {/if}
  </div>
  {#if current}
    <div class="actions">
      <button class="done" type="button" onclick={onTick}>Done</button>
      <button class="skipb" type="button" onclick={onSkip}>Skip for now</button>
      <span class="fill"></span>
      <span class="keys">Space done · ← back · S skip</span>
    </div>
  {/if}
  {#if detail === "expanded"}
    <button class="chev narrow" type="button" aria-expanded={open} aria-label="{open ? 'Hide' : 'Show'} details for {title}" onclick={() => (open = !open)}>
      <Icon name="chevron-down" />
    </button>
    {#if open}
      <!-- Narrow screens (no side pane): the details, where it is and the map in the step, until
           batch 5's sheets. -->
      <div class="dtl narrow">
        <StepDetail {index} {step} />
        {#if place?.control}<p class="where"><b>Where</b> · {place.control.location}</p>{/if}
        {#if !step.why && place?.control?.function}<p class="where"><b>What it does</b> · {place.control.function}</p>{/if}
        {#if place}
          <button class="btn quiet" type="button" aria-expanded={mapOpen} onclick={() => (mapOpen = !mapOpen)}>
            <Icon name="map-pin" />{mapOpen ? "Hide map" : "Show on map"}
          </button>
          {#if mapOpen}<ContextPane {index} {step} targets={row.targets} heading={false} details={false} />{/if}
        {/if}
      </div>
    {/if}
  {/if}
</li>

<style>
  /* The board's row: status 40, name, value 34%, menu 44. */
  .it {
    position: relative;
    display: grid;
    grid-template-columns: calc(var(--size-touch-min) - var(--space-1)) minmax(0, 1fr) 34% var(--size-touch-min);
    column-gap: var(--space-3);
    align-items: center;
    padding: calc(var(--space-1) + var(--border-thick) / 2) var(--space-1) calc(var(--space-1) + var(--border-thick) / 2) var(--space-4);
    border-bottom: var(--border-thin) solid var(--color-line);
  }
  .it:last-child {
    border-bottom: 0;
  }
  .it.current {
    row-gap: var(--space-3);
    align-items: start;
    padding: calc(var(--space-3) + var(--border-thick)) var(--space-1) calc(var(--space-3) + var(--border-thick)) var(--space-3);
    background: var(--color-current-row);
    box-shadow: inset var(--border-bar) 0 0 var(--color-accent);
  }
  .it.marked:not(.current) {
    box-shadow: inset var(--border-bar) 0 0 var(--color-accent);
  }
  .it.sel:not(.current) {
    background: var(--color-accent-soft);
  }
  .tick {
    display: grid;
    place-items: center;
    width: var(--size-touch-min);
    height: var(--size-touch-min);
    margin-left: calc(-1 * var(--border-thick));
    border: 0;
    background: none;
  }
  .status {
    display: grid;
    place-items: center;
    box-sizing: border-box;
    width: var(--size-status-icon);
    height: var(--size-status-icon);
    border: var(--border-thick) solid var(--color-todo-ring);
    border-radius: 50%;
  }
  .status.current {
    border-color: var(--color-accent);
  }
  .pip {
    width: calc(var(--size-status-icon) / 2);
    height: calc(var(--size-status-icon) / 2);
    border-radius: 50%;
    background: var(--color-accent);
  }
  .status.done {
    border-color: var(--color-done);
    background: var(--color-done);
    color: var(--color-on-done);
  }
  .status.done :global(svg) {
    width: var(--size-icon-small);
    height: var(--size-icon-small);
  }
  .status.skipped {
    border-style: dashed;
    border-color: var(--color-open-dot);
  }
  .dash {
    width: calc(var(--size-status-icon) / 2);
    height: var(--border-thick);
    background: var(--color-open-dot);
  }
  .namecol {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 0 var(--space-2);
    min-width: 0;
  }
  /* The row's own target reaches over the whole row; the buttons in it stay on top. */
  .row::after {
    position: absolute;
    inset: 0;
    content: "";
  }
  .tick,
  .do-now,
  .end,
  .actions,
  .value :global(input),
  .chev,
  .dtl {
    position: relative;
    z-index: 1;
  }
  .row {
    display: flex;
    flex-direction: column;
    justify-content: center;
    gap: var(--border-thick);
    min-width: 0;
    min-height: var(--size-touch-min);
    padding: 0;
    border: 0;
    background: none;
    color: var(--color-text);
    font: inherit;
    text-align: left;
  }
  .current .row {
    gap: var(--space-1);
    padding-top: var(--space-2);
  }
  .row {
    flex: 0 1 auto;
  }
  .name {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--space-2) calc(var(--space-2) + var(--border-thick));
  }
  .nm {
    font-size: var(--font-size-body);
    font-weight: var(--weight-medium);
  }
  .done .nm {
    color: var(--color-text-secondary);
  }
  .current .nm {
    color: var(--color-text-strong);
    font-size: var(--font-size-panel-title);
    font-weight: var(--weight-bold);
  }
  .ebadge {
    padding: 0 var(--space-1);
    border: var(--border-thin) solid var(--color-essentials-border);
    border-radius: var(--radius-small);
    color: var(--color-essentials-text);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-bold);
    line-height: var(--space-4);
  }
  .pill {
    padding: var(--border-thick) var(--space-2);
    border: var(--border-thin) solid var(--color-open-border);
    border-radius: var(--radius-pill);
    background: var(--color-open-bg);
    color: var(--color-open-text);
    font-size: var(--font-size-caption);
    font-weight: var(--weight-semibold);
  }
  .stamp,
  .ns {
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  .where-line,
  .look {
    color: var(--color-text-secondary);
    font-size: var(--font-size-secondary);
  }
  .look b {
    color: var(--color-text);
  }
  .value {
    display: flex;
    flex-direction: column;
    gap: var(--border-thick);
    min-width: 0;
  }
  .current .value {
    padding-top: var(--space-2);
  }
  .val {
    font-family: var(--font-mono);
    font-size: var(--font-size-value-mono);
    line-height: var(--leading-snug);
  }
  .current .val {
    color: var(--color-text-strong);
    font-weight: var(--weight-medium);
  }
  .done .val {
    color: var(--color-text-secondary);
  }
  .hint {
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  .end {
    position: relative;
  }
  .end:has(.menu) {
    z-index: 4;
  }
  .more {
    width: var(--size-touch-min);
    height: var(--size-touch-min);
    border: 0;
    border-radius: var(--radius-small);
    background: none;
    color: var(--color-text-muted);
    font: inherit;
    font-size: var(--font-size-panel-title);
    line-height: 1;
  }
  .current .more {
    color: var(--color-text-secondary);
  }
  .more.on {
    background: var(--color-chip);
    color: var(--color-text-strong);
  }
  /* The row menu (Checklist board): pop-over tokens. */
  .menu {
    position: absolute;
    top: calc(var(--size-touch-min) + var(--space-2));
    right: 0;
    z-index: 3;
    display: flex;
    flex-direction: column;
    width: calc(10 * var(--space-6) + var(--space-2));
    padding: calc(var(--space-1) + var(--border-thick) / 2);
    border: var(--border-thin) solid var(--color-border);
    border-radius: var(--radius-control);
    background: var(--color-surface);
    box-shadow: var(--color-overlay-shadow);
  }
  .menu button {
    height: var(--size-touch-min);
    padding: 0 var(--space-3);
    border: 0;
    border-radius: var(--radius-small);
    background: transparent;
    color: var(--color-text);
    font: inherit;
    font-size: var(--font-size-secondary);
    text-align: left;
  }
  .menu button:hover {
    background: var(--color-surface-2);
  }
  .menu .skip {
    color: var(--color-open-text);
  }
  .menu hr {
    height: var(--border-thin);
    margin: var(--space-1) var(--space-2);
    border: 0;
    background: var(--color-line);
  }
  .do-now {
    min-height: var(--size-touch-min);
    padding: 0 var(--space-1);
    border: 0;
    background: none;
    color: var(--color-link);
    font: inherit;
    font-size: var(--font-size-secondary);
  }
  .actions {
    grid-column: 2 / 5;
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }
  .done {
    height: var(--size-touch-min);
    padding: 0 var(--space-6);
    border: 0;
    border-radius: var(--radius-control);
    background: var(--color-accent);
    color: var(--color-on-accent);
    font: inherit;
    font-size: var(--font-size-body);
    font-weight: var(--weight-bold);
  }
  .skipb {
    height: var(--size-touch-min);
    padding: 0 var(--space-4);
    border: var(--border-thin) solid var(--color-open-border);
    border-radius: var(--radius-control);
    background: var(--color-skip-bg);
    color: var(--color-open-text);
    font: inherit;
    font-size: var(--font-size-body);
    font-weight: var(--weight-medium);
  }
  .fill {
    flex: 1;
  }
  .keys {
    color: var(--color-text-muted);
    font-size: var(--font-size-caption);
  }
  .chev {
    grid-column: 4;
    width: var(--size-touch-min);
    height: var(--size-touch-min);
    border: 0;
    background: none;
    color: var(--color-text-muted);
  }
  .chev[aria-expanded="true"] :global(.icon) {
    transform: rotate(180deg);
  }
  .dtl {
    grid-column: 2 / 5;
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    padding: var(--space-1) 0 var(--space-2);
    font-size: var(--font-size-secondary);
    line-height: var(--leading-snug);
  }
  .where {
    margin: 0;
    color: var(--color-text-muted);
  }
  .where b {
    color: var(--color-text);
    font-weight: var(--weight-semibold);
  }
  .btn.quiet {
    align-self: flex-start;
    min-height: var(--size-touch-min);
  }
  /* Wide screens show the details in the side pane. */
  @media (min-width: 900px) {
    .narrow {
      display: none !important;
    }
  }
  /* Non-normal and reference lists use larger type (step 7). */
  .large .nm,
  .large .val {
    font-size: var(--font-size-body);
  }
  @media (max-width: 599px) {
    .it {
      grid-template-columns: var(--size-touch-min) minmax(0, 1fr) var(--size-touch-min);
    }
    .value {
      grid-column: 2;
    }
    .actions,
    .dtl {
      grid-column: 2 / 4;
    }
    .keys {
      display: none;
    }
  }
</style>
