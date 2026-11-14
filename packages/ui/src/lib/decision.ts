// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Decision steps (decision 0022; Decision.dc.html). A step that `decide`s an option asks it in
// its row; the answer sets the option for the flight and ticks the step. A step whose targets
// depend on that answer links back to it. What the rows show comes from here: the page builds a
// `Decisions` from the flight, and the design gallery builds one from an invented fixture.

import type { Step, UserOption } from "@chocktochock/bindings";

/** How the answer was given, for the done row's line under the name. */
export type DecisionSource = "plan" | "you" | "changed";

/** A decision step's row. */
export type DecisionState = {
  option: UserOption;
  /** The chosen value, or null while unanswered. */
  answer: string | null;
  source: DecisionSource | null;
  /** An answer offered from the flight plan, waiting for the pilot to confirm. */
  suggestion: { value: string; amount: string } | null;
  /** Change is open: the choices show again on the row, in place. */
  changing: boolean;
};

/** A step whose targets depend on a decision. */
export type Dependent = {
  option: UserOption;
  /** The step that asks it. */
  decisionStep: string;
  answer: string | null;
  /** The step's value for the answer, when known; otherwise the step's own value shows. */
  value: string | null;
};

export type Decisions = {
  state(step: Step): DecisionState | null;
  dependent(step: Step): Dependent | null;
  /** Answers (or changes the answer to) `value`. */
  answer(stepId: string, value: string): void;
  /** Opens Change on the decision row. */
  change(stepId: string): void;
  /** Closes Change, keeping the answer. */
  keep(stepId: string): void;
};

/** The option's choices, in order, with their labels. */
export function choices(option: UserOption): { value: string; label: string }[] {
  return option.values.map((value) => ({
    value,
    label: option.choices?.find((c) => c.value === value)?.label ?? value,
  }));
}

export function labelOf(option: UserOption, value: string): string {
  return choices(option).find((c) => c.value === value)?.label ?? value;
}

/** A label without its parenthesis: "1,000 lb (450 kg) or less" → "1,000 lb or less". */
export function shortLabel(label: string): string {
  return label
    .replace(/\s*\([^)]*\)/g, "")
    .replace(/\s+/g, " ")
    .trim();
}

/** "More than 1,000 lb" → "more than 1,000 lb", for the middle of a sentence. */
export function lowerFirst(text: string): string {
  return text.charAt(0).toLowerCase() + text.slice(1);
}

/** The option a step's targets depend on, if any. */
export function dependsOn(step: Step): string | null {
  return (step.targets ?? []).find((t) => t.when)?.when?.option ?? null;
}

export const SOURCE_WORDS: Record<DecisionSource, string> = {
  plan: "Confirmed from your SimBrief plan",
  you: "Answered by you",
  changed: "Changed by you",
};

/** The toast after a changed answer: "Center tank: more than 1,000 lb. Fuel pump switches updated." */
export function changedText(option: UserOption, value: string, updated: string[]): string {
  const said = `${option.title}: ${lowerFirst(shortLabel(labelOf(option, value)))}.`;
  return updated.length ? `${said} ${updated.join(", ")} updated.` : said;
}

/**
 * The keys on a decision step that asks (Decision.dc.html's hint): 1, 2 … answer; Space confirms
 * the plan's suggestion and never ticks the step without an answer. True when the key was used.
 */
export function decisionKey(key: string, step: Step, decisions: Decisions): boolean {
  const d = decisions.state(step);
  if (!d || (d.answer !== null && !d.changing)) return false;
  const list = choices(d.option);
  const n = Number(key);
  const pick = Number.isInteger(n) && n >= 1 ? list[n - 1] : undefined;
  if (pick) {
    decisions.answer(step.id, pick.value);
    return true;
  }
  if (key === " ") {
    if (d.suggestion && d.answer === null && !d.changing) decisions.answer(step.id, d.suggestion.value);
    return true;
  }
  return false;
}
