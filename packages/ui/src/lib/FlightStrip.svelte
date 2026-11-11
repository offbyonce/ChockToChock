<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The flight strip at the top of a phase, as in 3.0. Flight setup and SimBrief loading fill it in
batch 2; until then it says no plan is loaded.
-->
<script lang="ts">
  // No props yet; batch 2 passes the flight.
</script>

<div class="strip card">
  <b>Flight</b>
  <span class="sum">Add your clearance: runway, departure, squawk and frequencies</span>
  <button class="edit" type="button" disabled title="Flight setup arrives in batch 2">Edit</button>
</div>

<style>
  .strip {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    min-height: 2.75rem;
    margin-bottom: 0.625rem;
    padding: 0.5rem 1rem;
  }
  b {
    flex: none;
    font-weight: 600;
  }
  .edit {
    position: relative;
    color: var(--accent);
    font-size: 0.875rem;
    font-weight: 500;
  }
  .edit:disabled {
    cursor: default;
  }
  .edit::after {
    position: absolute;
    inset: 50% -0.5rem auto;
    height: 2.75rem;
    content: "";
    transform: translateY(-50%);
  }
  .sum {
    flex: 1;
    min-width: 0;
    color: var(--muted);
    font-size: 0.875rem;
  }
</style>
