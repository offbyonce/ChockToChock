// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// The design system's enforcement (decision 0021): no raw size, color, font size, radius or
// shadow outside the token files, media queries only at the layout breakpoints, and no
// hand-drawn SVG outside the shared drawing renderer.

import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

/** Every stylesheet and component under src, by its path from src. */
const SOURCES: Record<string, string> = Object.fromEntries(
  Object.entries(
    import.meta.glob<string>("../**/*.{svelte,css}", { query: "?raw", import: "default", eager: true }),
  ).map(
    // Paths come relative to this folder: "../lib/x.svelte", or "./tokens.css" beside it.
    ([path, text]) => [path.startsWith("./") ? `theme/${path.slice(2)}` : path.slice(3), text],
  ),
);

/** The panel drawings: they take their sizes from their own templates in SVG units
 * (panel-layout.ts), and are the only files that may draw SVG. Batch 2 moves them into the
 * shared renderer and onto the panel colors. */
const DRAWINGS = new Set([
  "lib/PanelSection.svelte",
  "lib/PanelSchematic.svelte",
  "lib/Locator.svelte",
  "lib/OverviewMap.svelte",
]);

/** The generated token file, and the drawings. */
const EXEMPT = new Set(["theme/tokens.generated.css", ...DRAWINGS]);

/** The first width of each layout class above phone (docs/design/tokens.json). */
export const BREAKPOINTS = [600, 900, 1440];

/** The CSS of a file: a stylesheet, or a component's <style> blocks, without comments. */
function css(path: string): string {
  // A raw CSS import is empty under Vitest, so stylesheets are read from disk.
  const text = path.endsWith(".css")
    ? readFileSync(`${import.meta.dirname}/../${path}`, "utf8")
    : (SOURCES[path] ?? "");
  const styles = path.endsWith(".svelte")
    ? [...text.matchAll(/<style[^>]*>([\s\S]*?)<\/style>/g)].map((m) => m[1]).join("\n")
    : text;
  return styles.replace(/\/\*[\s\S]*?\*\//g, "");
}

/** Raw sizes and colors in some CSS, and media queries off the breakpoints. */
export function violations(source: string): string[] {
  const out: string[] = [];
  for (const m of source.matchAll(/\((min|max)-width:\s*(\d+)px\s*\)/g)) {
    const w = Number(m[2]);
    // min-width sits on a breakpoint; max-width one pixel under it, so the two never overlap.
    const ok = m[1] === "min" ? BREAKPOINTS.includes(w) : BREAKPOINTS.includes(w + 1);
    if (!ok) out.push(`breakpoint ${m[1]}-width ${w}px`);
  }
  const body = source.replace(/@media[^{]*\{/g, "{");
  for (const m of body.matchAll(/([a-z-]+)\s*:\s*([^;{}]+);/g)) {
    const [, prop = "", value = ""] = m;
    if (prop.startsWith("--")) continue;
    for (const raw of value.matchAll(/(?<![\w-])-?\d*\.?\d+(px|rem|em)\b/g)) {
      if (Number.parseFloat(raw[0]) !== 0) out.push(`${prop}: ${raw[0]}`);
    }
    for (const raw of value.matchAll(/#[0-9a-f]{3,8}\b|\b(?:rgb|rgba|hsl|hsla)\(/gi)) {
      out.push(`${prop}: ${raw[0]}`);
    }
    if (/^font(-weight)?$/.test(prop)) {
      for (const raw of value.matchAll(/(?<![\w.-])(?:[1-9]00|bold|bolder|lighter)(?![\w.-])/g)) {
        out.push(`${prop}: ${raw[0]}`);
      }
    }
  }
  return out;
}

describe("design tokens", () => {
  const all = Object.keys(SOURCES);

  it("finds raw sizes, raw colors and off-scale breakpoints", () => {
    expect(violations("a { padding: 0.6rem; color: #fff; }")).toEqual(["padding: 0.6rem", "color: #fff"]);
    expect(violations("a { font-weight: 600; font: 500 var(--font-size-body) var(--font-sans); }")).toEqual([
      "font-weight: 600",
      "font: 500",
    ]);
    expect(violations("@media (max-width: 760px) { a { gap: var(--space-2); } }")).toEqual([
      "breakpoint max-width 760px",
    ]);
    // A max-width on the breakpoint itself overlaps the min-width query at that width.
    expect(violations("@media (max-width: 600px) { a { gap: var(--space-2); } }")).toEqual([
      "breakpoint max-width 600px",
    ]);
    expect(violations("@media (max-width: 899px) { a { margin: 0; gap: var(--space-2); } }")).toEqual([]);
  });

  it("keeps every size and color in the token files", () => {
    const found = all
      .filter((f) => !EXEMPT.has(f))
      .flatMap((f) => violations(css(f)).map((v) => `${f}: ${v}`));
    expect(found).toEqual([]);
  });

  it("uses only tokens that exist", () => {
    const tokens = css("theme/tokens.generated.css");
    const defined = new Set([...tokens.matchAll(/(--[a-z0-9-]+):/g)].map((m) => m[1]));
    // Set at runtime by tokens.ts.
    for (const name of ["--scale", "--text-scale"]) defined.add(name);
    const found = all.flatMap((f) => {
      const text = f.endsWith(".css") ? css(f) : (SOURCES[f] ?? "");
      const local = new Set([...text.matchAll(/(--[a-z0-9-]+)(?:\s*:|=)/g)].map((m) => m[1]));
      return [...text.matchAll(/var\((--[a-z0-9-]+)/g)]
        .map((m) => m[1] ?? "")
        .filter((name) => !defined.has(name) && !local.has(name))
        .map((name) => `${f}: ${name}`);
    });
    expect([...new Set(found)]).toEqual([]);
  });

  it("draws SVG only in the drawing renderer", () => {
    const found = all.filter(
      (f) => f.endsWith(".svelte") && !DRAWINGS.has(f) && /<svg[\s>]/.test(SOURCES[f] ?? ""),
    );
    expect(found).toEqual([]);
  });

  it("puts no raw sizes or colors in style attributes", () => {
    const found = all
      .filter((f) => f.endsWith(".svelte") && !DRAWINGS.has(f))
      .flatMap((f) =>
        [...(SOURCES[f] ?? "").matchAll(/\sstyle="([^"]*)"/g)].flatMap((m) =>
          violations(`a { ${m[1]}; }`).map((v) => `${f}: ${v}`),
        ),
      );
    expect(found).toEqual([]);
  });
});
