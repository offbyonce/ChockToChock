// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Turns the 3.0 CDU walkthrough into structured pack data. 3.0 builds each page in code, so the
// loader runs that code twice, with no flight data and with marker values. Comparing the two
// shows which parts are live values and what is shown when they are missing.

const STYLES = { g: "data", a: "accent", u: "user" };

/** A 3.0 screen cell as a flat list of `{ text, style }`. */
export function flatten(cell) {
  if (cell === undefined || cell === null || cell === "") return [];
  if (typeof cell === "string") return [{ text: cell, style: null }];
  if (Array.isArray(cell)) return cell.flatMap(flatten);
  for (const [key, style] of Object.entries(STYLES)) {
    if (cell[key] !== undefined && cell[key] !== null) {
      return cell[key] === "" ? [] : [{ text: String(cell[key]), style }];
    }
  }
  throw new Error(`unknown CDU cell: ${JSON.stringify(cell)}`);
}

const escapeRegex = (text) => text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");

function matcher(entry) {
  return new RegExp(`(?<![0-9.])${escapeRegex(entry.value)}(?![0-9])`, "g");
}

/** Non-overlapping occurrences of table values in `text`, longest value first. */
export function findHits(text, table) {
  const hits = [];
  for (const entry of [...table].sort((a, b) => b.value.length - a.value.length)) {
    for (const match of text.matchAll(matcher(entry))) {
      const start = match.index;
      const end = start + entry.value.length;
      if (!hits.some((hit) => start < hit.end && hit.start < end)) hits.push({ start, end, entry });
    }
  }
  return hits.sort((a, b) => a.start - b.start);
}

function compact(object) {
  return Object.fromEntries(
    Object.entries(object).filter(([, value]) => value !== undefined && value !== null && value !== ""),
  );
}

function targetPart(target, extra, perPage) {
  if (target.route_legs) {
    const { empty_style: _unused, ...rest } = extra;
    return compact({ route_legs: target.route_legs, per_page: perPage, ...rest });
  }
  return compact({ ...target, ...extra });
}

function literal(text, style) {
  if (text === "") return [];
  return style ? [{ text, style }] : [text];
}

const samePart = (a, b) => a !== undefined && b !== undefined && a.text === b.text && a.style === b.style;

class PageConverter {
  constructor(table, perPage, gaps, name) {
    this.table = table;
    this.perPage = perPage;
    this.gaps = gaps;
    this.name = name;
  }

  conditionsOf(hits) {
    const fields = hits
      .filter((hit) => !hit.entry.always)
      .map(({ entry }) => (entry.target.route_legs || entry.target.index ? "route" : entry.target.field));
    return [...new Set(fields)];
  }

  /** A part with its live values split out, keeping the part's style. */
  inline(part, hits) {
    if (hits.length === 0) return literal(part.text, part.style);
    if (hits.length === 1) {
      const [{ start, end, entry }] = hits;
      return [
        targetPart(
          entry.target,
          { style: part.style, before: part.text.slice(0, start), after: part.text.slice(end) },
          this.perPage,
        ),
      ];
    }
    const out = [];
    let at = 0;
    for (const { start, end, entry } of hits) {
      out.push(...literal(part.text.slice(at, start), part.style));
      out.push(targetPart(entry.target, { style: part.style }, this.perPage));
      at = end;
    }
    out.push(...literal(part.text.slice(at), part.style));
    return out;
  }

  /** Splits prose around its live values when the text around them also appears when blank. */
  anchored(blank, filled, hits) {
    const anchors = [];
    let at = 0;
    for (const hit of hits) {
      anchors.push(filled.slice(at, hit.start));
      at = hit.end;
    }
    anchors.push(filled.slice(at));
    if (!blank.startsWith(anchors[0]) || !blank.endsWith(anchors[anchors.length - 1])) return null;
    const gaps = [];
    let cursor = anchors[0].length;
    for (let i = 1; i < anchors.length - 1; i++) {
      if (anchors[i] === "") return null;
      const found = blank.indexOf(anchors[i], cursor);
      if (found < 0) return null;
      gaps.push(blank.slice(cursor, found));
      cursor = found + anchors[i].length;
    }
    const lastStart = blank.length - anchors[anchors.length - 1].length;
    if (lastStart < cursor) return null;
    gaps.push(blank.slice(cursor, lastStart));
    const out = [...literal(anchors[0], null)];
    hits.forEach((hit, i) => {
      out.push(targetPart(hit.entry.target, { empty: gaps[i] }, this.perPage));
      out.push(...literal(anchors[i + 1], null));
    });
    return out;
  }

  when(conditions, show, otherwise) {
    return compact({
      when_known: conditions,
      show,
      otherwise: otherwise.length > 0 ? otherwise : undefined,
    });
  }

  part(blank, filled, fallback, table, label) {
    const same = samePart(blank, filled);
    const all = findHits(filled.text, table);
    const hits = same ? all.filter((hit) => hit.entry.always) : all;
    if (hits.length === 0) {
      if (same) return literal(filled.text, filled.style);
      const otherwise = blank ? literal(blank.text, blank.style) : [];
      if (fallback.length === 0) {
        this.gaps.push(`${this.name}: ${label}: live text with no known value; kept the blank form`);
        return otherwise;
      }
      return [this.when(fallback, literal(filled.text, filled.style), otherwise)];
    }
    if (same) return this.inline(filled, hits);
    if (blank && !blank.style && !filled.style) {
      const parts = this.anchored(blank.text, filled.text, hits);
      if (parts) return parts;
    }
    if (hits.length === 1) {
      const [{ start, end, entry }] = hits;
      return [
        targetPart(
          entry.target,
          {
            style: filled.style,
            before: filled.text.slice(0, start),
            after: filled.text.slice(end),
            empty: blank?.text,
            empty_style: blank?.style,
          },
          this.perPage,
        ),
      ];
    }
    const otherwise = blank ? literal(blank.text, blank.style) : [];
    return [this.when(this.conditionsOf(hits), this.inline(filled, hits), otherwise)];
  }

  /** One text or cell, compared between the blank and filled runs. */
  cell(blankCell, filledCell, fallback, label, extraTable = []) {
    const table = [...this.table.values, ...extraTable];
    const blank = flatten(blankCell);
    const filled = flatten(filledCell);
    const joined = filled.map((part) => part.text).join("");
    const whole = this.table.whole.find((entry) => entry.value === joined);
    const blankJoined = blank.map((part) => part.text).join("");
    if (whole && joined !== blankJoined) {
      return [
        targetPart(
          whole.target,
          {
            style: filled[0].style,
            empty: blankJoined,
            empty_style: blank.length === 1 ? blank[0].style : undefined,
          },
          this.perPage,
        ),
      ];
    }
    if (blank.length === filled.length) {
      return filled.flatMap((part, i) => this.part(blank[i], part, fallback, table, label));
    }
    if (blank.length === 0 && filled.length === 1) {
      return this.part(undefined, filled[0], fallback, table, label);
    }
    const hitsByPart = filled.map((part) => findHits(part.text, table));
    const conditions = this.conditionsOf(hitsByPart.flat());
    const known = conditions.length > 0 ? conditions : fallback;
    const otherwise = blank.flatMap((part) => literal(part.text, part.style));
    if (known.length === 0) {
      this.gaps.push(`${this.name}: ${label}: live text with no known value; kept the blank form`);
      return otherwise;
    }
    return [
      this.when(
        known,
        filled.flatMap((part, i) => this.inline(part, hitsByPart[i])),
        otherwise,
      ),
    ];
  }
}

function textHits(cell, table) {
  return flatten(cell).flatMap((part) => findHits(part.text, table));
}

/** Converts the walkthrough. `table` is `{ values, whole, leg1Via, overflow }`. */
export function convertCdu(blankPages, filledPages, table, gaps) {
  if (blankPages.length !== filledPages.length) {
    throw new Error("the blank and filled CDU runs have different page counts");
  }
  const lookup = {
    values: [...table.values, ...table.whole.filter((entry) => entry.value !== "")],
    whole: [...table.whole, { value: table.leg1Via.value, target: table.leg1Via.target }],
  };
  return blankPages.map((blank, pageIndex) => {
    const filled = filledPages[pageIndex];
    const perPage = filled.s.r.length - 1;
    const converter = new PageConverter(lookup, perPage, gaps, `${filled.pg} "${filled.t}"`);
    const screenHits = filled.s.r.flatMap((row) =>
      row.flatMap((cell) => (cell && cell.sep !== undefined ? [] : textHits(cell, lookup.values))),
    );
    const pageConditions = converter.conditionsOf(screenHits);
    const rows = filled.s.r.map((filledRow, r) => {
      const blankRow = blank.s.r[r];
      const separator = filledRow[0] && filledRow[0].sep !== undefined ? filledRow[0].sep : undefined;
      const rowHits = filledRow.flatMap((cell) =>
        cell && cell.sep !== undefined ? [] : textHits(cell, lookup.values),
      );
      const rowConditions = converter.conditionsOf(rowHits);
      const fallback = rowConditions.length > 0 ? rowConditions : pageConditions;
      const convert = (i, name) =>
        filledRow[i] && filledRow[i].sep !== undefined
          ? []
          : converter.cell(blankRow[i], filledRow[i], fallback, `row ${r + 1} ${name}`);
      return nonEmpty({
        separator: separator === undefined ? undefined : separator === 1 ? "" : String(separator),
        left_label: convert(0, "left label"),
        left: convert(1, "left"),
        right_label: convert(2, "right label"),
        right: convert(3, "right"),
      });
    });
    const prose = (name, extra = []) =>
      converter.cell(blank[name], filled[name], pageConditions, name, extra);
    const pageNumber = () => {
      const match = /^(.*\/)(\d+)$/.exec(filled.s.p ?? "");
      if (match && filled.s.p !== blank.s.p) {
        return [...literal(match[1], null), { route_legs: "pages", per_page: perPage }];
      }
      return converter.cell(blank.s.p, filled.s.p, pageConditions, "page number");
    };
    return nonEmpty(
      {
        page: filled.pg,
        key: filled.key,
        title: filled.t,
        action: prose("a"),
        check: prose("c"),
        note: prose("n", [table.overflow]),
        screen: nonEmpty(
          {
            title: converter.cell(blank.s.t, filled.s.t, pageConditions, "screen title"),
            page: pageNumber(),
            rows,
            highlight: filled.s.hl,
            scratchpad: converter.cell(blank.s.sp, filled.s.sp, pageConditions, "scratchpad"),
          },
          ["title", "page", "rows"],
        ),
      },
      ["action", "check", "screen"],
    );
  });
}

/** Drops empty optional values, keeping the required keys in `keep`. */
function nonEmpty(object, keep = []) {
  return Object.fromEntries(
    Object.entries(object).filter(
      ([key, value]) =>
        keep.includes(key) ||
        !(value === undefined || value === null || (Array.isArray(value) && value.length === 0)),
    ),
  );
}

function targetKey(target) {
  if (target.route_legs) return `count:${target.route_legs}`;
  return [target.field, target.index, target.part].filter((v) => v !== undefined).join(":");
}

/** The field values for a filled run (`filled: true`) or a blank run. */
function renderState(table, filled, legCount) {
  const values = new Map();
  const known = new Set();
  for (const entry of [...table.values, ...table.whole, table.leg1Via]) {
    if (filled || entry.always) {
      values.set(targetKey(entry.target), entry.value);
      known.add(entry.target.field);
    }
  }
  if (filled) known.add("route");
  return {
    values,
    known,
    counts: filled
      ? { pages: legCount.pages, overflow: Number(table.overflow.value) }
      : { pages: 1, overflow: 0 },
  };
}

function styled(text, style) {
  return text === "" ? [] : [{ text, style: style ?? null }];
}

function renderInline(part, state) {
  if (typeof part === "string") return styled(part, null);
  if (part.text !== undefined) return styled(part.text, part.style);
  if (part.route_legs) {
    const count = state.counts[part.route_legs];
    if (count === 0) return styled(part.empty ?? "", null);
    return styled(`${part.before ?? ""}${count}${part.after ?? ""}`, part.style);
  }
  const value = state.values.get(targetKey(part));
  if (value === undefined) return styled(part.empty ?? "", part.empty_style);
  return styled(`${part.before ?? ""}${value}${part.after ?? ""}`, part.style);
}

function renderText(parts, state) {
  return (parts ?? []).flatMap((part) => {
    if (part.when_known) {
      const shown = part.when_known.every((field) => state.known.has(field));
      return (shown ? part.show : (part.otherwise ?? [])).flatMap((p) => renderInline(p, state));
    }
    return renderInline(part, state);
  });
}

/** One entry per character, so differently split but identical text compares equal. Spaces have no visible style. */
function perCharacter(parts) {
  return parts
    .flatMap((part) => [...part.text].map((c) => (c === " " ? c : `${c}|${part.style}`)))
    .join("\n");
}

/**
 * Renders converted pages and compares every text with what 3.0 produced, for both runs.
 * Returns a description of each difference.
 */
export function verifyCdu(pages, blankPages, filledPages, table, legCount) {
  const problems = [];
  for (const [run, originals, filled] of [
    ["blank", blankPages, false],
    ["filled", filledPages, true],
  ]) {
    const state = renderState(table, filled, legCount);
    pages.forEach((page, p) => {
      const original = originals[p];
      const compare = (name, parts, cell) => {
        if (perCharacter(renderText(parts, state)) !== perCharacter(flatten(cell))) {
          problems.push(`${run} run, ${page.page} "${page.title}", ${name} differs`);
        }
      };
      compare("action", page.action, original.a);
      compare("check", page.check, original.c);
      compare("note", page.note, original.n);
      compare("screen title", page.screen.title, original.s.t);
      compare("page number", page.screen.page, original.s.p);
      compare("scratchpad", page.screen.scratchpad, original.s.sp);
      page.screen.rows.forEach((row, r) => {
        const cells = original.s.r[r];
        if (!(cells[0] && cells[0].sep !== undefined)) {
          compare(`row ${r + 1} left label`, row.left_label, cells[0]);
          compare(`row ${r + 1} right label`, row.right_label, cells[2]);
        }
        compare(`row ${r + 1} left`, row.left, cells[1]);
        compare(`row ${r + 1} right`, row.right, cells[3]);
      });
    });
  }
  return problems;
}

/** Marker values that must never appear in converted text. */
export function leakedMarkers(pages, table) {
  const markers = table.values
    .filter((entry) => !entry.always && /[A-Z]{2}|\.\d|\d{4}|°/.test(entry.value))
    .map((entry) => entry.value);
  const leaks = new Set();
  const visit = (value) => {
    if (typeof value === "string") {
      for (const marker of markers) if (value.includes(marker)) leaks.add(marker);
    } else if (typeof value === "object" && value !== null) {
      Object.values(value).forEach(visit);
    }
  };
  visit(pages);
  return [...leaks];
}
