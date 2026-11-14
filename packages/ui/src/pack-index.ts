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
  /** The phase a step's list is in, by id. */
  phaseOf(stepId: string): string | undefined;
  /** The step that asks an option (decision 0022), if any. */
  decider(option: string): Step | undefined;
  /** The steps whose targets depend on an option's answer, in pack order. */
  dependents(option: string): Step[];
  /**
   * The steps whose targets name a panel item, each with its phase's title: in flight order (the
   * phases' order, then their lists'), each step once. A step that two lists repeat in the same
   * phase (a flow and its checklist) is listed once, at its first place.
   */
  usedIn(panel: string, section: string, item: number): { step: Step; phase: string }[];
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

  /** The locator a step target uses for an item: "oh/fuel/AFT 1/1", the n-th of its label. */
  const locator = (panel: string, section: string, item: number) => {
    const items = sections.get(section)?.items ?? [];
    const it = items[item];
    if (!it) return null;
    const n = items.slice(0, item + 1).filter((o) => o.label === it.label).length;
    return `${panel}/${section}/${it.label}/${n}`;
  };
  const phaseOfList = new Map<string, string>();
  const phaseIdOfList = new Map<string, string>();
  for (const phase of pack.content.phases?.phases ?? []) {
    for (const list of phase.lists) {
      phaseOfList.set(list, phase.title);
      phaseIdOfList.set(list, phase.id);
    }
  }
  // Lists in flight order: each phase's lists in turn, then any list no phase holds.
  const listsById = new Map(pack.lists.map((l) => [l.id, l]));
  const flightOrder = [
    ...(pack.content.phases?.phases ?? []).flatMap((p) => p.lists.flatMap((id) => listsById.get(id) ?? [])),
    ...pack.lists.filter((l) => !phaseOfList.has(l.id)),
  ].filter((l, i, all) => all.indexOf(l) === i);
  const listOfStep = new Map<string, string>();
  for (const list of pack.lists) for (const step of list.steps) listOfStep.set(step.id, list.id);

  return {
    name: pack.manifest.name,
    phaseOf: (stepId) => phaseIdOfList.get(listOfStep.get(stepId) ?? ""),
    decider: (option) => [...steps.values()].find((s) => s.decide === option),
    dependents: (option) =>
      [...steps.values()].filter((s) => (s.targets ?? []).some((t) => t.when?.option === option)),
    usedIn(panel, section, item) {
      const at = locator(panel, section, item);
      if (!at) return [];
      const seen = new Set<string>();
      const used: { step: Step; phase: string }[] = [];
      for (const list of flightOrder) {
        const phase = phaseOfList.get(list.id) ?? list.title;
        for (const step of list.steps) {
          if (!(step.targets ?? []).some((t) => t.at === at)) continue;
          const key = `${step.text}\u0000${phase}`;
          if (seen.has(key)) continue;
          seen.add(key);
          used.push({ step, phase });
        }
      }
      return used;
    },
    control: (id) => controls.get(id),
    glossary: (term) => glossary.get(term),
    lightMeaning(panel, section, item) {
      // The n-th item with that label in the section, counted from 1, as light notes name them.
      const at = locator(panel, section, item);
      return at ? lightNotes.get(at) : undefined;
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
