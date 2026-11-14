<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT

The decision step's states (Decision.dc.html), from an invented fixture, for review, tests and the
board pairs only; unlinked and never part of the installed app. `?view=decision&theme=dark|light&
state=unanswered|suggested|answered|changing|changed|phone`. Each state is a starting point: the
choices, Confirm, Change, Keep this answer and Undo all work, as in the app.
-->
<script lang="ts">
  import type { Flight } from "@chocktochock/bindings";
  import { changedText, decisionKey, dependsOn, type DecisionSource, type Decisions } from "../lib/decision";
  import ListCard from "../lib/ListCard.svelte";
  import Toast from "../lib/Toast.svelte";
  import { indexPack } from "../pack-index";
  import { applyPalette } from "../theme/tokens";
  import { FUEL_VALUE, OPTION, PACK, STEPS, SUGGESTION, fixtureList } from "./decision-fixture";

  let { state: start = "unanswered", theme = "dark" }: { state?: string; theme?: string } = $props();
  // svelte-ignore state_referenced_locally
  applyPalette(document.documentElement, theme !== "light", 0);

  const index = indexPack(PACK);
  const decider = STEPS.find((s) => s.decide)?.id ?? "";
  const flight: Flight = {
    format: 1,
    pack: "fixture",
    packVersion: "0.0.0",
    options: {},
    startState: null,
    session: { done: {}, skipped: {} },
    values: {},
    blanks: {},
    notes: "",
    timers: {},
  };

  // svelte-ignore state_referenced_locally
  const initial = start;
  let answer = $state<string | null>(initial === "answered" || initial === "changing" ? "low" : initial === "changed" ? "more" : null);
  let source = $state<DecisionSource | null>(initial === "answered" || initial === "changing" ? "plan" : initial === "changed" ? "changed" : null);
  let changing = $state(initial === "changing");
  let suggestion = $state(initial === "suggested" || initial === "phone" ? SUGGESTION : null);
  let undo = $state<{ text: string; restore: () => void } | null>(
    initial === "changed"
      ? {
          text: changedText(OPTION, "more", ["Fuel pump switches"]),
          restore: () => {
            answer = "low";
            source = "plan";
          },
        }
      : null,
  );

  let list = $derived(fixtureList(answer !== null));

  const decisions: Decisions = {
    state: (step) =>
      step.decide === OPTION.id
        ? { option: OPTION, answer, source, suggestion: answer === null ? suggestion : null, changing: changing && answer !== null }
        : null,
    dependent: (step) =>
      dependsOn(step) === OPTION.id ? { option: OPTION, decisionStep: decider, answer, value: FUEL_VALUE[answer ?? "none"] ?? null } : null,
    answer(_id, value) {
      const before = { answer, source };
      if (changing) {
        changing = false;
        if (value === answer) return;
        answer = value;
        source = "changed";
        undo = {
          text: changedText(OPTION, value, ["Fuel pump switches"]),
          restore: () => {
            answer = before.answer;
            source = before.source;
          },
        };
      } else {
        answer = value;
        source = suggestion && value === suggestion.value ? "plan" : "you";
      }
    },
    change() {
      if (answer !== null) changing = true;
    },
    keep() {
      changing = false;
    },
  };

  function onKey(e: KeyboardEvent): void {
    if (e.target instanceof Element && e.target.closest("button, input, a")) return;
    const step = index.step(decider);
    if (step && decisionKey(e.key, step, decisions)) e.preventDefault();
  }
  const none = () => {};
</script>

<svelte:window onkeydown={onKey} />

<div class="frame" class:phone={start === "phone"} data-dec={start}>
  <ListCard
    {index}
    {list}
    source={PACK.lists[0]}
    {flight}
    detail="expanded"
    selected={null}
    onTick={none}
    onSkip={none}
    onUnskip={none}
    onBlank={none}
    onSelect={none}
    onClear={none}
    {decisions}
  />
  {#if undo}
    {@const u = undo}
    <Toast
      text={u.text}
      onUndo={() => {
        u.restore();
        undo = null;
      }}
      onExpire={none}
    />
  {/if}
</div>

<style>
  :global(body) {
    margin: 0;
    background: var(--color-bg);
    color: var(--color-text);
  }
  /* The boards' frame: 1120 × 560 with 32 and 40 padding; on phone, the screen. */
  .frame {
    position: relative;
    box-sizing: border-box;
    width: calc(28 * var(--space-10));
    height: calc(14 * var(--space-10));
    padding: var(--space-8) var(--space-10);
    overflow: hidden;
    background: var(--color-bg);
  }
  .frame.phone {
    width: 100%;
    height: 100vh;
    padding: var(--space-4) var(--gutter);
  }
</style>
