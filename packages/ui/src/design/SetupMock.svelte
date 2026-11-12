<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Mockup: Flight setup on the system. The Start choice, the SimBrief card (username remembered,
the loaded plan with its route and age, a file as the second way), and the flight values in
groups on one form grid, the groups collapsing on phones.
-->
<script lang="ts">
  import Button from "../ds/Button.svelte";
  import Card from "../ds/Card.svelte";
  import Field from "../ds/Field.svelte";
  import PaneHeader from "../ds/PaneHeader.svelte";
  import Segmented from "../ds/Segmented.svelte";

  let start = $state("cold");
  let qnhUnit = $state("inHg");
  let fuelUnit = $state("LB");
  const f = (label: string, value: string, help: string, example = "") => ({ label, value, help, example });
  const GROUPS = [
    ["Flight", [f("Flight number", "TSTX45", "Up to 7 characters"), f("From", "KSEA", "4-letter ICAO code"), f("To", "KSFO", "4-letter ICAO code"), f("Cruise level", "350", "FL100 to FL410")]],
    [
      "Departure clearance",
      [
        f("Runway", "16L", "01 to 36, with L, C or R"),
        f("Departure (SID)", "SUMMA2", "As cleared, or blank for radar vectors"),
        f("Transition", "SEA", "The SID transition, if any"),
        f("Initial altitude", "5000", "Feet, in 100 ft steps"),
        f("Squawk", "", "4 digits, 0 to 7", "4521"),
        f("Ground frequency", "", "The frequency to call for taxi", "121.700"),
        f("Departure frequency", "", "Contact after takeoff", "125.600"),
        f("Runway heading", "163", "001 to 360 degrees"),
      ],
    ],
  ] as const;
</script>

<PaneHeader title="Flight setup" lead="Enter your numbers once. The checklist fills them in wherever a step needs them." />

<div class="stack">
  <Card title="Start">
    <div class="row">
      <p>Turnaround skips Power up, for a next leg with the airplane already powered.</p>
      <Segmented
        label="Start"
        choices={[
          { value: "cold", label: "Cold and dark" },
          { value: "turn", label: "Turnaround" },
        ]}
        value={start}
        onChange={(v) => (start = v)}
      />
    </div>
  </Card>

  <Card title="SimBrief plan">
    <div class="sb">
      <div class="plan">
        <b>KSEA → KSFO · TSTX45</b>
        <span>SUMMA2 SUMMA J1 BTG … · made 2 hours ago · kept for offline use</span>
      </div>
      <div class="acts">
        <Button variant="primary" icon="arrow-right">Refresh plan</Button>
        <Button variant="quiet" compact>Use a saved plan file</Button>
        <Button variant="quiet" compact>Forget username</Button>
      </div>
    </div>
  </Card>

  {#each GROUPS as [title, fields] (title)}
    <Card {title}>
      <div class="grid">
        {#each fields as fl, i (fl.label)}
          <Field id="m-{title}-{i}" label={fl.label} value={fl.value} help={fl.help} example={fl.example} onInput={() => {}} />
        {/each}
        {#if title === "Departure clearance"}
          <Field
            id="m-qnh"
            label="QNH"
            value=""
            example={qnhUnit === "hPa" ? "1013" : "29.92"}
            help={qnhUnit === "hPa" ? "948 to 1050 hPa" : "28.00 to 31.00 inHg"}
            onInput={() => {}}
            unit={{ values: ["inHg", "hPa"], value: qnhUnit, onChange: (v) => (qnhUnit = v) }}
          />
          <Field
            id="m-fuel"
            label="Fuel"
            value="19658"
            help="Departing fuel"
            onInput={() => {}}
            unit={{ values: ["LB", "KG"], value: fuelUnit, onChange: (v) => (fuelUnit = v) }}
          />
        {/if}
      </div>
    </Card>
  {/each}
</div>

<nav class="way"><Button variant="primary" iconEnd="arrow-right" href="#start">Start at APU start</Button></nav>

<style>
  .stack {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }
  .row {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-3) var(--space-6);
  }
  .row p {
    flex: 1 1 var(--card-min);
    margin: 0;
    color: var(--muted);
    font-size: var(--text-sm);
  }
  .sb {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
  }
  .plan {
    display: flex;
    flex-direction: column;
    gap: var(--space-1);
  }
  .plan b {
    font: var(--weight-bold) var(--text-md) var(--mono);
  }
  .plan span {
    color: var(--muted);
    font-size: var(--text-sm);
  }
  .acts {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--space-2);
  }
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(var(--field-min), 1fr));
    gap: var(--space-5) var(--space-4);
  }
  .way {
    display: flex;
    justify-content: flex-end;
    margin-top: var(--space-6);
  }
</style>
