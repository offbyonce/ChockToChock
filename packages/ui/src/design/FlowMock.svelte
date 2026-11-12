<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Mockup: Flow view on the system. One step large, its value and blanks, the CHECK callout, and the
confirm button within thumb reach: pinned to the bottom on phones, under the step elsewhere.
-->
<script lang="ts">
  import Button from "../ds/Button.svelte";
  import Callout from "../ds/Callout.svelte";
  import Field from "../ds/Field.svelte";

  let values = $state(["152", "163", "5000"]);
</script>

<article class="flow">
  <p class="crumb">Before start · Step 3 of 9 · Glareshield (P7) · Mode control panel</p>
  <h1>MCP</h1>
  <p class="line">Check it, then confirm.</p>
  <div class="fields">
    {#each ["V2", "HEADING", "ALTITUDE"] as label, i (label)}
      <Field variant="inline" id="flow-{i}" {label} before={label} value={values[i] ?? ""} onInput={(v) => (values[i] = v)} />
    {/each}
  </div>
  <p class="rest">ARM A/T, ARM LNAV AND VNAV FOR AN RNAV DEPARTURE</p>
  <Callout caption="Check" title="A/T ARM switch at ARM, green light on" detail="FMA shows ARM" />
  <p class="why"><b>Why</b> · Departure speed, heading and altitude</p>
  <div class="acts">
    <Button variant="primary" icon="check">Confirm, next step</Button>
    <Button icon="skip">Skip</Button>
    <span class="keys">Space: confirm · Left arrow: back · S: skip</span>
  </div>
  <p class="next"><b>Next</b> DOOR LIGHTS <span>ALL OUT</span></p>
</article>

<style>
  .flow {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
    max-width: var(--measure);
  }
  .crumb {
    margin: 0;
    color: var(--muted);
    font-size: var(--text-sm);
  }
  h1 {
    margin: 0;
    font: var(--weight-bold) var(--text-3xl) / var(--leading-tight) var(--sans);
    letter-spacing: var(--tracking-title);
  }
  .line {
    margin: calc(-1 * var(--space-2)) 0 0;
    color: var(--muted);
  }
  .fields {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-3) var(--space-6);
  }
  .rest {
    margin: 0;
    color: var(--accent);
    font: var(--weight-medium) var(--text-lg) / var(--leading-snug) var(--mono);
  }
  .why {
    margin: 0;
    font-size: var(--text-sm);
  }
  .why b,
  .next b {
    font-weight: var(--weight-bold);
  }
  .acts {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--space-3);
  }
  .keys {
    color: var(--muted);
    font-size: var(--text-xs);
  }
  .next {
    margin: 0;
    padding-top: var(--space-4);
    border-top: var(--border-thin) solid var(--line);
    font-size: var(--text-sm);
  }
  .next span {
    color: var(--muted);
    font-family: var(--mono);
  }
  /* Phones: the confirm button sits at the bottom, within thumb reach, over the safe area. */
  @media (max-width: 599px) {
    h1 {
      font-size: var(--text-2xl);
    }
    .acts {
      position: fixed;
      right: 0;
      bottom: 0;
      left: 0;
      z-index: 30;
      display: grid;
      grid-template-columns: 1fr auto;
      padding: var(--space-3) var(--gutter) max(var(--space-3), env(safe-area-inset-bottom));
      border-top: var(--border-thin) solid var(--line);
      background: var(--surface);
    }
    .keys {
      display: none;
    }
    .flow {
      padding-bottom: var(--space-16);
    }
  }
</style>
