// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Normalizes British spellings in imported text to American, and records every word changed.

const WORDS = {
  aeroplane: "airplane",
  aeroplanes: "airplanes",
  aluminium: "aluminum",
  amongst: "among",
  analyse: "analyze",
  analysed: "analyzed",
  analyses: "analyzes",
  analysing: "analyzing",
  behaviour: "behavior",
  behaviours: "behaviors",
  cancelled: "canceled",
  cancelling: "canceling",
  centre: "center",
  centred: "centered",
  centres: "centers",
  colour: "color",
  coloured: "colored",
  colours: "colors",
  defence: "defense",
  depressurisation: "depressurization",
  depressurise: "depressurize",
  depressurised: "depressurized",
  favour: "favor",
  favourable: "favorable",
  fibre: "fiber",
  fuelled: "fueled",
  fuelling: "fueling",
  grey: "gray",
  honour: "honor",
  initialise: "initialize",
  initialised: "initialized",
  judgement: "judgment",
  kerb: "curb",
  labelled: "labeled",
  labelling: "labeling",
  levelled: "leveled",
  levelling: "leveling",
  licence: "license",
  licences: "licenses",
  litre: "liter",
  litres: "liters",
  manoeuvre: "maneuver",
  manoeuvres: "maneuvers",
  manoeuvring: "maneuvering",
  maximise: "maximize",
  memorise: "memorize",
  memorised: "memorized",
  metre: "meter",
  metres: "meters",
  minimise: "minimize",
  minimised: "minimized",
  neighbouring: "neighboring",
  normalise: "normalize",
  offence: "offense",
  optimise: "optimize",
  optimised: "optimized",
  organise: "organize",
  organised: "organized",
  pressurisation: "pressurization",
  pressurise: "pressurize",
  pressurised: "pressurized",
  prioritise: "prioritize",
  programme: "program",
  programmes: "programs",
  recognise: "recognize",
  recognised: "recognized",
  signalled: "signaled",
  signalling: "signaling",
  stabilisation: "stabilization",
  stabilise: "stabilize",
  stabilised: "stabilized",
  stabilising: "stabilizing",
  storey: "story",
  synchronise: "synchronize",
  synchronised: "synchronized",
  travelled: "traveled",
  travelling: "traveling",
  tyre: "tire",
  tyres: "tires",
  utilise: "utilize",
  whilst: "while",
};

const WORD = /[A-Za-z]+/g;
const SUSPECT = /(?:isation|ise[sd]?|ising|our|ours|tre|tres|lled|lling)$/;
const NOT_BRITISH =
  /^(?:.*wise|.*controlled|otherwise|precise|concise|noise|raise[sd]?|raising|rise[sd]?|rising|arise[sd]?|arising|cruise[sd]?|cruising|advise[sd]?|advising|revise[sd]?|exercise[sd]?|promise[sd]?|surprise[sd]?|comprise[sd]?|disguise[sd]?|expertise|premise[sd]?|our|ours|your|yours|four|hour|hours|pour|pours|tour|tours|sour|contour|contours|detour|flour|scour|called|calling|filled|filling|pulled|pulling|rolled|rolling|controlled|controlling|selling|spelled|spelling|stalled|stalling|installed|installing|killed|drilled|falling|telling|shelled|knelled|compelled|propelled|expelled|dispelled|polled|polling|tolled|trolled|smelled|swelled|welled|yelled|billed|skilled|spilled|stilled|thrilled|tilled|willed|milled|grilled|chilled|fulled|walled|balled|galled|mulled|nulled|culled|dulled|lulled|hulled|gulled|shrilled|metre|centre)$/;

function matchCase(original, replacement) {
  if (original === original.toUpperCase()) return replacement.toUpperCase();
  if (original[0] === original[0].toUpperCase()) {
    return replacement[0].toUpperCase() + replacement.slice(1);
  }
  return replacement;
}

/** Records spelling changes and words that look British but are not on the list. */
export class SpellingLog {
  changes = new Map();
  suspects = new Map();

  summary() {
    const sorted = (map) => Object.fromEntries([...map].sort(([a], [b]) => a.localeCompare(b)));
    return { changes: sorted(this.changes), suspects: sorted(this.suspects) };
  }
}

/** The American spelling of `text`. */
export function normalizeSpelling(text, log) {
  return text.replace(WORD, (word) => {
    const lower = word.toLowerCase();
    const american = Object.hasOwn(WORDS, lower) ? WORDS[lower] : undefined;
    if (american === undefined) {
      if (log && SUSPECT.test(lower) && !NOT_BRITISH.test(lower)) {
        log.suspects.set(lower, (log.suspects.get(lower) ?? 0) + 1);
      }
      return word;
    }
    const result = matchCase(word, american);
    if (log) {
      const key = `${lower} -> ${american}`;
      log.changes.set(key, (log.changes.get(key) ?? 0) + 1);
    }
    return result;
  });
}

/** Keys whose values are ids or fixed vocabulary, never prose. */
const NOT_PROSE = new Set([
  "basis",
  "color",
  "control",
  "controls",
  "equals",
  "explain",
  "field",
  "fill",
  "from",
  "how",
  "id",
  "kind",
  "lists",
  "one_of",
  "opens",
  "option",
  "part",
  "phase",
  "pick",
  "quick",
  "route_legs",
  "set",
  "status",
  "style",
  "to",
  "values",
  "when_known",
  "zone",
  "default",
  "done",
  "skip",
]);

/** A copy of `value` with every prose string normalized. Object keys are left alone. */
export function normalizeDeep(value, log) {
  if (typeof value === "string") return normalizeSpelling(value, log);
  if (Array.isArray(value)) return value.map((item) => normalizeDeep(item, log));
  if (typeof value === "object" && value !== null) {
    return Object.fromEntries(
      Object.entries(value).map(([key, item]) => [key, NOT_PROSE.has(key) ? item : normalizeDeep(item, log)]),
    );
  }
  return value;
}
