<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

Saved data the app could not use: made by a newer version, or unreadable (decision 0012). It is
kept as it is, never dropped: the pilot can save a copy, or start fresh, which moves it aside on
this device instead of deleting it. Nothing is saved over it until then.
-->
<script lang="ts">
  let {
    what,
    state,
    text,
    onFresh,
  }: { what: "flight" | "settings"; state: "newer" | "unreadable"; text: string; onFresh: () => void } = $props();

  let name = $derived(what === "flight" ? "progress" : "settings");
  let href = $derived(URL.createObjectURL(new Blob([text], { type: "application/json" })));
  $effect(() => {
    const url = href;
    return () => URL.revokeObjectURL(url);
  });
</script>

<div class="kept" role="alert">
  <p>
    <b>Your saved {name} {state === "newer" ? "is from a newer version of the app" : "could not be read"}.</b>
    It is kept on this device as it was. Save a copy, or start fresh; starting fresh sets it aside
    rather than deleting it.
  </p>
  <div class="acts">
    <a class="btn" {href} download="chocktochock-{what}.json">Save a copy</a>
    <button class="btn primary" type="button" onclick={onFresh}>Start fresh</button>
  </div>
</div>

<style>
  .kept {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--space-3) var(--space-4);
    margin-bottom: var(--space-4);
    padding: var(--space-3) var(--space-4);
    border: var(--border-thin) solid var(--color-open-text);
    border-radius: var(--radius-panel);
    background: var(--color-open-bg);
  }
  p {
    flex: 1 1 auto;
    margin: 0;
    font-size: var(--font-size-secondary);
  }
  b {
    font-weight: var(--weight-semibold);
  }
  .acts {
    display: flex;
    gap: var(--space-2);
  }
</style>
