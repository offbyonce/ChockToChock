<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Mockup: the phase screen on the system. The header, the flight strip, one card per list with its
rows, an opened step with its CHECK and IF NOT callouts, and the way on.
-->
<script lang="ts">
  import Button from "../ds/Button.svelte";
  import Callout from "../ds/Callout.svelte";
  import Card from "../ds/Card.svelte";
  import Chip from "../ds/Chip.svelte";
  import ListRow from "../ds/ListRow.svelte";
  import PaneHeader from "../ds/PaneHeader.svelte";
  import TickBox from "../ds/TickBox.svelte";
  import { PROCEDURE } from "./data";

  let { onOpen }: { onOpen?: (n: number) => void } = $props();
</script>

<PaneHeader crumb="Departure · Phase 2 of 8" title="Before start" />

<div class="strip">
  <Card>
    <div class="fs">
      <b>Flight</b>
      <span class="sum">
        {#each ["KSEA → KSFO", "RWY 16L", "SUMMA2.SEA", "5000 FT", "SQK 4521"] as part, i (part)}{#if i}<i aria-hidden="true">·</i><wbr />{/if}<span class="part">{part}</span>{/each}
      </span>
      <Button variant="link" compact>Edit</Button>
    </div>
  </Card>
</div>

<div class="open">
  <Card>
    <div class="fs">
      <span class="odot" aria-hidden="true"></span>
      <b>Open steps</b>
      <span class="muted">2 from earlier pages · APU start 2</span>
      <Button variant="link" compact>Show</Button>
    </div>
  </Card>
</div>

<Card kind="Procedure" title="Before start procedure" flush>
  {#snippet actions()}
    <span class="count">1 / 6</span>
    <Button variant="quiet" compact>Clear</Button>
  {/snippet}
  <ol class="rows">
    {#each PROCEDURE as s (s.n)}
      <ListRow
        number={s.n}
        title={s.text}
        value={s.value}
        valueBelow={s.value.length > 32}
        meta={s.where}
        marked={s.state === "current"}
        selected={s.open}
        muted={s.state === "done"}
        onSelect={() => onOpen?.(s.n)}
      >
        {#snippet leading()}<TickBox state={s.state} label="Confirm {s.text}: {s.value}" />{/snippet}
        {#snippet chips()}
          {#if s.essential}<Chip title="In Essentials">E</Chip>{/if}
          {#if s.state === "current"}<Chip tone="accent" solid>Current</Chip>{/if}
          {#if s.state === "done"}<Chip tone="go">Done 10:11</Chip>{/if}
          {#if s.state === "skipped"}<Chip tone="warn" dashed>Skipped, still open</Chip>{/if}
        {/snippet}
        {#snippet trailing()}<Button variant="quiet" compact icon="chevron-down" label="Show details for {s.text}" />{/snippet}
        {#snippet body()}
          {#if s.open && s.check}
            <div class="cells">
              <Callout caption="Check" title={s.check.title} detail={s.check.detail} />
              {#if s.ifNot}<Callout tone="warn" caption="If not" title={s.ifNot.title} detail={s.ifNot.detail} />{/if}
            </div>
            {#if s.why}<p class="why"><b>Why</b> · {s.why}</p>{/if}
            <Button variant="quiet" compact icon="skip">Skip for now</Button>
          {/if}
        {/snippet}
      </ListRow>
    {/each}
  </ol>
</Card>

<nav class="way" aria-label="Phases either side">
  <Button href="#apu" icon="arrow-left">APU start</Button>
  <Button href="#pushback" variant="primary" iconEnd="arrow-right">Pushback</Button>
</nav>

<style>
  .strip,
  .open {
    margin-bottom: var(--space-4);
  }
  .odot {
    width: var(--space-2);
    height: var(--space-2);
    border-radius: var(--radius-full);
    background: var(--warn);
  }
  .muted {
    flex: 1;
    color: var(--muted);
    font-size: var(--text-sm);
  }
  .fs {
    display: flex;
    flex-wrap: wrap;
    align-items: baseline;
    gap: var(--space-1) var(--space-3);
    margin: calc(-1 * var(--space-2)) 0;
  }
  .sum {
    flex: 1;
    min-width: 0;
    font: var(--weight-medium) var(--text-sm) / var(--leading-snug) var(--mono);
  }
  .sum i {
    margin: 0 var(--space-2);
    color: var(--muted);
    font-style: normal;
  }
  /* A part never breaks; the line wraps between parts, so no line starts with a separator. */
  .part {
    white-space: nowrap;
  }
  .count {
    color: var(--muted);
    font: var(--weight-regular) var(--text-xs) var(--mono);
  }
  .rows {
    margin: 0;
    padding: 0;
  }
  .cells {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(var(--card-min), 1fr));
    gap: var(--space-3);
    margin-bottom: var(--space-3);
  }
  .why {
    margin: 0 0 var(--space-2);
    font-size: var(--text-sm);
    line-height: var(--leading-snug);
  }
  .why b {
    font-weight: var(--weight-bold);
  }
  .way {
    display: flex;
    justify-content: space-between;
    gap: var(--space-3);
    margin-top: var(--space-6);
  }
</style>
