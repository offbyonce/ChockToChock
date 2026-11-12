// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// The design system's enforcement (decision 0020): no raw size or color outside the token files,
// and media queries only at the system's breakpoints. Screens not yet moved onto the system are
// listed in LEGACY; the refactor removes them one by one, and a listed file that is already clean
// fails too, so the list only shrinks.

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

/** The token files themselves, and the panel drawings, which take their sizes from their own
 * templates in SVG units (panel-layout.ts: KEY, TOGGLE, DIAL). */
const EXEMPT = new Set([
  "theme/tokens.css",
  "lib/PanelSection.svelte",
  "lib/PanelSchematic.svelte",
  "lib/Locator.svelte",
  "lib/OverviewMap.svelte",
]);

/** Screens and parts still to move onto the system (step 8 of the design system batch). */
const LEGACY = new Set([
  "App.svelte",
  "app.css",
  "lib/ArrivalData.svelte",
  "lib/BlankFields.svelte",
  "lib/ContextPane.svelte",
  "lib/Drawer.svelte",
  "lib/FieldInput.svelte",
  "lib/FlightStrip.svelte",
  "lib/FlowView.svelte",
  "lib/Icon.svelte",
  "lib/KeptNotice.svelte",
  "lib/ListCard.svelte",
  "lib/OpenStepsCard.svelte",
  "lib/OptionChoices.svelte",
  "lib/PhaseChips.svelte",
  "lib/StepDetail.svelte",
  "lib/StepItem.svelte",
  "lib/TopBar.svelte",
  "pages/OpenStepsPage.svelte",
  "pages/PhasePage.svelte",
  "pages/ReferencePage.svelte",
  "pages/SettingsPage.svelte",
  "pages/SetupPage.svelte",
]);

export const BREAKPOINTS = [600, 900, 1200, 1440, 1920];

/** The CSS of a file: a stylesheet, or a component's <style> blocks, without comments. */
function css(path: string): string {
  // A raw CSS import is empty under Vitest, so stylesheets are read from disk.
  const text = path.endsWith(".css")
    ? readFileSync(new URL(`../${path}`, import.meta.url), "utf8")
    : (SOURCES[path] ?? "");
  const styles = path.endsWith(".svelte")
    ? [...text.matchAll(/<style[^>]*>([\s\S]*?)<\/style>/g)].map((m) => m[1]).join("\n")
    : text;
  return styles.replace(/\/\*[\s\S]*?\*\//g, "");
}

/** Raw sizes and colors in some CSS, and media queries off the breakpoints. */
export function violations(source: string): string[] {
  const out: string[] = [];
  for (const m of source.matchAll(/@media[^{]*\(\s*(?:min|max)-width:\s*(\d+)px\s*\)/g)) {
    const w = Number(m[1]);
    // A max-width query sits one pixel under a breakpoint.
    if (!BREAKPOINTS.includes(w) && !BREAKPOINTS.includes(w + 1)) out.push(`breakpoint ${w}px`);
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
  }
  return out;
}

describe("design tokens", () => {
  const all = Object.keys(SOURCES);

  it("finds raw sizes, raw colors and off-scale breakpoints", () => {
    expect(violations("a { padding: 0.6rem; color: #fff; }")).toEqual(["padding: 0.6rem", "color: #fff"]);
    expect(violations("@media (max-width: 760px) { a { gap: var(--space-2); } }")).toEqual([
      "breakpoint 760px",
    ]);
    expect(violations("@media (max-width: 899px) { a { margin: 0; gap: var(--space-2); } }")).toEqual([]);
  });

  it("keeps every size and color in the token files", () => {
    const found = all
      .filter((f) => !EXEMPT.has(f) && !LEGACY.has(f))
      .flatMap((f) => violations(css(f)).map((v) => `${f}: ${v}`));
    expect(found).toEqual([]);
  });

  it("lists only legacy files that still need moving", () => {
    for (const f of LEGACY) {
      expect(all, `${f} no longer exists`).toContain(f);
      expect(violations(css(f)).length, `${f} is clean: take it off the legacy list`).toBeGreaterThan(0);
    }
  });
});
