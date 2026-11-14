// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import type { Step } from "@chocktochock/bindings";
import type { PackIndex } from "../pack-index";

const literal = (s: string) => s.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");

/**
 * A step name in capitals as a sentence ("FUEL PUMP SWITCHES" as "Fuel pump switches"), keeping
 * glossary terms and codes ("MCP", "N1") and cockpit text (`keep`: the labels of the step's
 * controls, "STAB OUT OF TRIM light") in capitals; mixed case stays.
 */
export function sentence(index: PackIndex, text: string, keep: string[] = []): string {
  if (text !== text.toUpperCase()) return text;
  const kept = new Array<boolean>(text.length).fill(false);
  for (const label of [...keep].sort((a, b) => b.length - a.length)) {
    if (!label) continue;
    for (const m of text.matchAll(new RegExp(`(?<![A-Z0-9/])${literal(label)}(?![A-Z0-9/])`, "g"))) {
      for (let i = m.index; i < m.index + label.length; i++) kept[i] = true;
    }
  }
  let at = 0;
  const words = text.split(/(\s+|[(),])/).map((w) => {
    const from = at;
    at += w.length;
    if (w.trim() && kept.slice(from, at).every(Boolean)) return w;
    return index.glossary(w) !== undefined || /\d|\//.test(w) ? w : w.toLowerCase();
  });
  const out = words.join("");
  return out.charAt(0).toUpperCase() + out.slice(1);
}

/** The labels of the controls a step's targets name ("oh/fuel/AFT 1/1" names AFT 1). */
export function cockpitLabels(step: Step): string[] {
  return (step.targets ?? []).map((t) => t.at.split("/").slice(2, -1).join("/")).filter(Boolean);
}

/** A step's title as the app shows it: a sentence, with its cockpit text in capitals. */
export function stepTitle(index: PackIndex, step: Step): string {
  return sentence(index, step.text, cockpitLabels(step));
}
