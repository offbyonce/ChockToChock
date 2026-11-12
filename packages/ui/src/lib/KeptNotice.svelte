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
    gap: 0.75rem 1rem;
    margin-bottom: 0.875rem;
    padding: 0.75rem 1rem;
    border: 1px solid var(--warn);
    border-radius: 0.75rem;
    background: var(--warn-soft);
  }
  p {
    flex: 1 1 24rem;
    margin: 0;
    font-size: 0.875rem;
  }
  b {
    font-weight: 600;
  }
  .acts {
    display: flex;
    gap: 0.5rem;
  }
</style>
