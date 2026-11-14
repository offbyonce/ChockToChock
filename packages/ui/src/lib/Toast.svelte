<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The Undo toast (Decision.dc.html, DecChanged), shared by every change that offers Undo: Clear
ticks, Reset flight plan, a changed decision. It announces itself (role="status"), stays at least
8 seconds and pauses while hovered or focused, and its Undo is an ordinary button, reachable by
keyboard. It never covers the current step's buttons: when it would, it moves above them, or
below them when there is no room above. On phone it sits above the bottom area.
-->
<script lang="ts">
  let {
    text,
    onUndo,
    onExpire,
    seconds = 10,
    avoid = ".it.current .actions",
  }: {
    text: string;
    onUndo: () => void;
    /** The toast's time ran out. */
    onExpire: () => void;
    /** At least 8. */
    seconds?: number;
    /** What the toast must never cover. */
    avoid?: string;
  } = $props();

  let box = $state<HTMLElement | null>(null);
  let hovered = $state(false);
  let focused = $state(false);
  /** Where the toast's foot sits, from the window's foot; null keeps the stylesheet's. */
  let lift = $state<number | null>(null);

  // The time left, which stands still while the pilot points at or focuses the toast.
  // svelte-ignore state_referenced_locally
  let left = Math.max(8, seconds) * 1000;
  $effect(() => {
    if (hovered || focused) return;
    const start = Date.now();
    const timer = setTimeout(onExpire, left);
    return () => {
      clearTimeout(timer);
      left = Math.max(0, left - (Date.now() - start));
    };
  });

  /** Moves the toast off the current step's buttons when the two would meet. */
  function place(): void {
    if (!box) return;
    lift = null;
    const target = document.querySelector(avoid);
    if (!target) return;
    const t = target.getBoundingClientRect();
    const own = box.getBoundingClientRect();
    if (own.bottom <= t.top || own.top >= t.bottom || own.right <= t.left || own.left >= t.right) return;
    const gap = parseFloat(getComputedStyle(box).getPropertyValue("--toast-gap")) || 8;
    const above = window.innerHeight - t.top + gap;
    lift = t.top - own.height - gap >= 0 ? above : window.innerHeight - t.bottom - own.height - gap;
  }
  $effect(() => {
    void text;
    place();
    const again = () => place();
    window.addEventListener("scroll", again, true);
    window.addEventListener("resize", again);
    return () => {
      window.removeEventListener("scroll", again, true);
      window.removeEventListener("resize", again);
    };
  });
</script>

<div
  bind:this={box}
  class="toast"
  role="status"
  style:bottom={lift === null ? null : `${lift}px`}
  onpointerenter={() => (hovered = true)}
  onpointerleave={() => (hovered = false)}
  onfocusin={() => (focused = true)}
  onfocusout={() => (focused = false)}
>
  <span>{text}</span>
  <button type="button" onclick={onUndo}>Undo</button>
</div>

<style>
  .toast {
    --toast-gap: var(--space-2);
    position: fixed;
    bottom: calc(var(--space-6) + env(safe-area-inset-bottom, 0px));
    left: 50%;
    z-index: 70;
    display: flex;
    align-items: center;
    gap: var(--space-4);
    width: max-content;
    max-width: calc(100vw - 2 * var(--gutter));
    min-height: calc(var(--size-touch-min) + var(--space-2));
    box-sizing: border-box;
    padding: 0 var(--space-2) 0 calc(var(--space-4) + var(--border-thick));
    border-radius: var(--radius-panel);
    background: var(--color-text);
    box-shadow: var(--color-toast-shadow);
    color: var(--color-toast-text);
    font-size: var(--font-size-secondary);
    transform: translateX(-50%);
  }
  button {
    flex: none;
    min-height: var(--size-touch-min);
    padding: 0 calc(var(--space-3) + var(--space-1) / 2);
    border: 0;
    background: transparent;
    color: var(--color-toast-link);
    font: inherit;
    font-size: var(--font-size-secondary);
    font-weight: var(--weight-bold);
  }
</style>
