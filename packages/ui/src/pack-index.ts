// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Lookups into a loaded pack by id, so screens can show the text Rust's views refer to. No rules
// live here: what shows, what counts and what is current all come from the host.

import type {
  Control,
  ExplainNote,
  FlatPack,
  Overview,
  OverviewArea,
  OverviewRegion,
  Panel,
  PanelSection,
  Step,
  UserOption,
  Zone,
} from "@chocktochock/bindings";

export type PackIndex = {
  name: string;
  step(id: string): Step | undefined;
  /** The step's first control, its zone and the panel section drawn for that zone. */
  place(step: Step): Place | null;
  zones: Zone[];
  overview: Overview | null;
  options: UserOption[];
  /** A shared "how to do it" note's steps, by id. */
  how(id: string): string[] | undefined;
  /** A shared explanation, by id. */
  explain(id: string): ExplainNote | undefined;
  /** A panel section by id. */
  section(id: string): PanelSection | undefined;
  /** The panel a section is on. */
  panelOf(id: string): Panel | undefined;
  /** The overview region a zone is in, and the map areas that region shows. */
  region(zone: Zone): MapRegion | null;
  /** A control by id. */
  control(id: string): Control | undefined;
  /** A glossary term's meaning. */
  glossary(term: string): string | undefined;
  /** A light's meaning (its note), by panel, section and item index. */
  lightMeaning(panel: string, section: string, item: number): string | undefined;
};

export type MapRegion = { region: OverviewRegion | null; areas: OverviewArea[] };

const center = (a: OverviewArea) => [a.x + a.width / 2, a.y + a.height / 2] as const;
const holds = (r: OverviewRegion, [x, y]: readonly [number, number]) =>
  x >= r.x && x <= r.x + r.width && y >= r.y && y <= r.y + r.height;

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
  const panels = new Map<string, Panel>();
  for (const panel of pack.content.panels ?? []) {
    for (const s of panel.sections) {
      sections.set(s.id, s);
      panels.set(s.id, panel);
    }
  }

  const glossary = new Map((pack.content.glossary?.terms ?? []).map((t) => [t.term, t.meaning]));
  const lightNotes = new Map((pack.content.lights?.lights ?? []).map((n) => [n.locator, n.meaning]));

  return {
    name: pack.manifest.name,
    control: (id) => controls.get(id),
    glossary: (term) => glossary.get(term),
    lightMeaning(panel, section, item) {
      const items = sections.get(section)?.items ?? [];
      const it = items[item];
      if (!it) return undefined;
      // The n-th item with that label in the section, counted from 1, as light notes name them.
      const n = items.slice(0, item + 1).filter((o) => o.label === it.label).length;
      return lightNotes.get(`${panel}/${section}/${it.label}/${n}`);
    },
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
    options: pack.content.phases?.options ?? [],
    section: (id) => sections.get(id),
    panelOf: (id) => panels.get(id),
    how: (id) => pack.content.notes?.how?.find((n) => n.id === id)?.steps,
    explain: (id) => pack.content.notes?.explain?.find((n) => n.id === id),
    region(zone) {
      const overview = pack.content.controls?.overview;
      if (!overview) return null;
      const regions = overview.regions ?? [];
      // An area belongs to the first region that holds its center.
      const regionOf = (a: OverviewArea) => regions.find((r) => holds(r, center(a))) ?? null;
      const mine = overview.areas.find((a) => a.zone === zone.id);
      if (!mine) return null;
      const region = regionOf(mine);
      if (region) return { region, areas: overview.areas.filter((a) => regionOf(a) === region) };
      // A pack without regions: the areas of zones in the same named area.
      const sameArea = (a: OverviewArea) => zones.get(a.zone)?.area === zone.area;
      return { region: null, areas: overview.areas.filter(sameArea) };
    },
  };
}
