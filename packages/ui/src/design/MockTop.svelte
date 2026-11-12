<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Mockup: the top bar on the system. Phones get the menu, the phase name and Emergency; tablets add
the view switch; from tablet landscape the stage tabs and Find show.
-->
<script lang="ts">
  import Button from "../ds/Button.svelte";
  import Segmented from "../ds/Segmented.svelte";
  import { STAGES } from "./data";

  let { view = "checklist", title = "Before start" }: { view?: "checklist" | "flow"; title?: string } = $props();
</script>

<Button variant="quiet" icon="menu" label="Menu" />
<div class="brand">
  <b>Chock to Chock</b>
  <span>iFly 737 MAX 8</span>
</div>
<b class="phase">{title}</b>
<nav class="stages" aria-label="Stages">
  {#each STAGES as s (s)}
    <a href="#{s}" aria-current={s === "Departure" ? "page" : undefined}>{s}</a>
  {/each}
</nav>
<span class="grow"></span>
<span class="find"><Button variant="quiet" icon="search">Find</Button></span>
<span class="view">
  <Segmented
    layout="inline"
    label="View"
    choices={[
      { value: "checklist", label: "Checklist" },
      { value: "flow", label: "Flow" },
    ]}
    value={view}
    onChange={() => {}}
  />
</span>
<Button variant="emergency" icon="alert-triangle">Emergency</Button>

<style>
  .brand {
    display: none;
    flex-direction: column;
    line-height: var(--leading-tight);
  }
  .brand b {
    font-size: var(--text-sm);
    font-weight: var(--weight-bold);
  }
  .brand span {
    color: var(--muted);
    font-size: var(--text-xs);
  }
  .phase {
    min-width: 0;
    overflow: hidden;
    font-size: var(--text-md);
    font-weight: var(--weight-bold);
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .stages {
    display: none;
    gap: var(--space-1);
  }
  .stages a {
    display: inline-flex;
    align-items: center;
    min-height: var(--control-h);
    padding: 0 var(--space-3);
    border-radius: var(--radius-md);
    color: var(--muted);
    font-size: var(--text-sm);
    font-weight: var(--weight-medium);
    text-decoration: none;
  }
  .stages a[aria-current="page"] {
    background: var(--accent-soft);
    color: var(--accent);
  }
  .grow {
    flex: 1;
  }
  .find,
  .view {
    display: none;
  }
  @media (min-width: 600px) {
    .view {
      display: block;
    }
  }
  @media (min-width: 900px) {
    .brand {
      display: flex;
    }
    .phase {
      display: none;
    }
    .stages,
    .find {
      display: flex;
    }
  }
</style>
