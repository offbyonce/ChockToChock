<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Every component of the system in every variant, one section each. The design shot test captures
each section (data-component) in both themes and compares it with its saved baseline.
-->
<script lang="ts">
  import Button from "../ds/Button.svelte";
  import Callout from "../ds/Callout.svelte";
  import Card from "../ds/Card.svelte";
  import Chip from "../ds/Chip.svelte";
  import Field from "../ds/Field.svelte";
  import ListRow from "../ds/ListRow.svelte";
  import PaneHeader from "../ds/PaneHeader.svelte";
  import SectionTitle from "../ds/SectionTitle.svelte";
  import Segmented from "../ds/Segmented.svelte";
  import TickBox from "../ds/TickBox.svelte";

  let seg = $state("checklist");
  let cards = $state("gpu");
  let unit = $state("inHg");
</script>

<div class="gallery">
  <section data-component="button">
    <SectionTitle>Button</SectionTitle>
    <div class="row">
      <Button variant="primary" iconEnd="arrow-right">Primary</Button>
      <Button>Secondary</Button>
      <Button variant="quiet" icon="skip">Quiet</Button>
      <Button variant="danger">Danger</Button>
      <Button variant="emergency" icon="alert-triangle">Emergency</Button>
      <Button variant="link">Link</Button>
      <Button compact>Compact</Button>
      <Button pressed>Pressed</Button>
      <Button disabled>Disabled</Button>
    </div>
  </section>

  <section data-component="chip">
    <SectionTitle>Chip</SectionTitle>
    <div class="row">
      <Chip>Full · Detailed</Chip>
      <Chip title="In Essentials">E</Chip>
      <Chip tone="accent" solid>Current</Chip>
      <Chip tone="go">Done 10:11</Chip>
      <Chip tone="warn" dashed>Skipped, still open</Chip>
      <Chip solid>Not modeled in the simulator</Chip>
      <Chip tone="stop">Missed</Chip>
    </div>
  </section>

  <section data-component="segmented">
    <SectionTitle>Segmented</SectionTitle>
    <div class="col">
      <Segmented
        label="View"
        choices={[
          { value: "checklist", label: "Checklist" },
          { value: "flow", label: "Flow" },
        ]}
        value={seg}
        onChange={(v) => (seg = v)}
      />
      <Segmented
        layout="cards"
        label="Power source"
        choices={[
          { value: "gpu", label: "Ground power at the gate", description: "Preflight on ground power. Start the APU before the Before start procedure." },
          { value: "apu", label: "No ground power", description: "Start the APU now, straight after power up." },
        ]}
        value={cards}
        onChange={(v) => (cards = v)}
      />
    </div>
  </section>

  <section data-component="field">
    <SectionTitle>Field</SectionTitle>
    <div class="fields">
      <Field id="g-rwy" label="Runway" value="16L" help="01 to 36, with L, C or R" onInput={() => {}} />
      <Field id="g-sq" label="Squawk" value="" example="4521" help="4 digits, 0 to 7" onInput={() => {}} />
      <Field id="g-qnh" label="QNH" value="29.92" help="28.00 to 31.00 inHg" onInput={() => {}} unit={{ values: ["inHg", "hPa"], value: unit, onChange: (v) => (unit = v) }} />
      <Field id="g-flaps" label="Takeoff flaps" kind="select" options={["1", "5", "10"]} value="5" help="From your takeoff data" onInput={() => {}} />
    </div>
    <div class="row">
      <Field variant="inline" id="g-v2" label="V2" before="V2" value="152" onInput={() => {}} />
      <Field variant="inline" id="g-fl" label="FLAPS" before="FLAPS" after="INDICATED" value="5" onInput={() => {}} />
    </div>
  </section>

  <section data-component="card">
    <SectionTitle>Card and list row</SectionTitle>
    <Card kind="Checklist" title="Before takeoff checklist" flush>
      {#snippet actions()}<Button variant="quiet" compact>Clear</Button>{/snippet}
      <ol class="rows">
        <ListRow number={1} title="FLAPS" value="5, GREEN LIGHT" meta="Control stand · Flap lever" muted>
          {#snippet leading()}<TickBox state="done" label="Confirm FLAPS" />{/snippet}
          {#snippet chips()}<Chip tone="go">Done 10:11</Chip>{/snippet}
        </ListRow>
        <ListRow number={2} title="TAKEOFF DATA" value="SET, RUNWAY 16L" meta="Forward electronic panel (P9) · CDUs" marked>
          {#snippet leading()}<TickBox state="current" label="Confirm TAKEOFF DATA" />{/snippet}
          {#snippet chips()}<Chip tone="accent" solid>Current</Chip>{/snippet}
        </ListRow>
        <ListRow number={3} title="STABILIZER TRIM" value="5.25 UNITS">
          {#snippet leading()}<TickBox state="skipped" label="Confirm STABILIZER TRIM" />{/snippet}
          {#snippet chips()}<Chip tone="warn" dashed>Skipped, still open</Chip>{/snippet}
        </ListRow>
      </ol>
    </Card>
  </section>

  <section data-component="callout">
    <SectionTitle>Callout</SectionTitle>
    <div class="fields">
      <Callout caption="Check" title="All door lights out" />
      <Callout tone="warn" caption="If not" title="Light on" detail="That door is not closed" />
    </div>
  </section>

  <section data-component="header">
    <SectionTitle>Pane header and section title</SectionTitle>
    <PaneHeader crumb="Departure · Phase 2 of 8" title="Before start" lead="A lead line under a page title, kept within the measure." />
    <PaneHeader level={2} crumb="Forward overhead (P5) · Fuel panel" title="Fuel pump switches" />
    <SectionTitle>The controls</SectionTitle>
  </section>
</div>

<style>
  .gallery {
    display: flex;
    flex-direction: column;
    gap: var(--space-8);
    max-width: var(--main-max);
  }
  section {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    padding: var(--space-4);
    border-radius: var(--radius-lg);
    background: var(--bg);
  }
  .row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--space-3);
  }
  .col {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }
  .fields {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(var(--field-min), 1fr));
    gap: var(--space-4);
  }
  .rows {
    margin: 0;
    padding: 0;
  }
</style>
