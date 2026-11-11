// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Lookups into a loaded pack by id, so screens can show the text Rust's views refer to. No rules
// live here: what shows, what counts and what is current all come from the host.

import type { Control, FlatPack, Overview, PanelSection, Step, Zone } from "@chocktochock/bindings";

export type PackIndex = {
  name: string;
  step(id: string): Step | undefined;
  /** The step's first control, its zone and the panel section drawn for that zone. */
  place(step: Step): Place | null;
  zones: Zone[];
  overview: Overview | null;
};

export type Place = {
  control: Control | null;
  zone: Zone;
  section: PanelSection | null;
};

/** "Forward overhead (P5)" → ["Forward overhead", "P5"]. */
export function splitArea(area: string): [string, string | null] {
  const match = /^(.*?)\s*\(([^)]+)\)\s*$/.exec(area);
  return match ? [match[1] ?? area, match[2] ?? null] : [area, null];
}

export function indexPack(pack: FlatPack): PackIndex {
  const steps = new Map<string, Step>();
  for (const list of pack.lists) for (const step of list.steps) steps.set(step.id, step);
  const controls = new Map((pack.content.controls?.controls ?? []).map((c) => [c.id, c]));
  const zones = new Map((pack.content.controls?.zones ?? []).map((z) => [z.id, z]));
  const sections = new Map<string, PanelSection>();
  for (const panel of pack.content.panels ?? []) for (const s of panel.sections) sections.set(s.id, s);

  return {
    name: pack.manifest.name,
    step: (id) => steps.get(id),
    place(step) {
      for (const id of step.controls ?? []) {
        const control = controls.get(id);
        const zone = control?.zone ? zones.get(control.zone) : undefined;
        if (control && zone) return { control, zone, section: sections.get(zone.id) ?? null };
      }
      return null;
    },
    zones: [...zones.values()],
    overview: pack.content.controls?.overview ?? null,
  };
}
