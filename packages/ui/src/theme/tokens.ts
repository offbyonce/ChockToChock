// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// The theme's colors, dimming and scaling (decision 0021). Colors come from docs/design/tokens.json
// through tokens.generated.ts; sizes are CSS custom properties in tokens.generated.css. Colors and
// the scale are set on the page through the CSSOM, which the content security policy allows (no
// inline <style>). tokens.test.ts checks contrast in both themes at every dimming step.

import { COLOR, DESIGN_SIZE, PANEL } from "./tokens.generated";

export type ColorName = keyof (typeof COLOR)["dark"];
export type PanelName = keyof (typeof PANEL)["dark"];
export type Colors = Record<ColorName, string>;
export type PanelColors = Record<PanelName, string>;

/** Brightness factors for each dimming step, 0 (full) to 4. */
export const DIMMING = [1, 0.8, 0.64, 0.5, 0.4] as const;

/** Backgrounds and outlines of surfaces: they dim with the screen. */
const SURFACES = new Set<string>([
  "bg",
  "surface",
  "surface-2",
  "sheet",
  "field",
  "strip-panel",
  "drop-zone",
  "paper",
  "paper-block",
  "border",
  "border-strong",
  "line",
  "chip",
  "accent-soft",
  "current-row",
  "track",
  "open-bg",
  "open-border",
  "danger-soft",
  "segment-track",
  "segment-border",
  "segment-on",
  "toggle-off",
  "popover-border",
]);
const PANEL_SURFACES = new Set<string>(["bg", "frame", "fill", "fuselage", "seat", "glass", "changed-fill"]);
/** Text on a signal fill, and lit lights, which keep their real color: they do not change. */
const ON_SIGNAL = new Set<string>(["on-accent", "on-done", "on-lit-amber", "lit-amber"]);

/** Text and the background it sits on: at least 4.5:1. */
export const TEXT_PAIRS: [ColorName, ColorName][] = [
  ...(["text", "text-secondary", "text-muted", "text-strong"] as const).flatMap((fg) =>
    (["bg", "surface", "surface-2", "sheet", "field", "current-row"] as const).map(
      (bg) => [fg, bg] as [ColorName, ColorName],
    ),
  ),
  ["accent-text", "accent-soft"],
  ["accent-text", "current-row"],
  ["link", "bg"],
  ["link", "surface"],
  ["open-text", "open-bg"],
  ["open-text", "surface"],
  ["danger-text", "surface"],
  ["danger-text", "danger-soft"],
  ["on-accent", "accent"],
  ["essentials-text", "surface"],
  ["on-lit-amber", "lit-amber"],
];

/** Status icons, rings and outlines a pilot needs to see: at least 3:1. */
export const GRAPHIC_PAIRS: [ColorName, ColorName][] = [
  ...(["accent", "done", "todo-ring", "open-dot", "danger"] as const).flatMap((fg) =>
    (["bg", "surface", "current-row"] as const).map((bg) => [fg, bg] as [ColorName, ColorName]),
  ),
  // The tick drawn on a done step's green circle.
  ["on-done", "done"],
  // The OFP file drop zone's dashed outline (SimBrief and OFP file).
  ["drop-zone-border", "drop-zone"],
  // Every boxed field's outline (batch 4 additions): 3:1 against the field's fill and the
  // surface around it.
  ["field-border", "field"],
  ["field-border", "surface"],
];

/** Inside drawings: labels at 4.5:1, outlines, switch bodies and lens rims at 3:1. */
export const PANEL_TEXT_PAIRS: [PanelName, PanelName][] = [
  ["label", "bg"],
  ["label", "fill"],
  ["text", "bg"],
  ["text", "fill"],
  ["changed-text", "changed-fill"],
];
export const PANEL_GRAPHIC_PAIRS: [PanelName, PanelName][] = [
  ["stroke", "bg"],
  ["resting", "bg"],
  ["changed", "bg"],
  ["changed", "fill"],
];

function channels(hex: string): [number, number, number] {
  const n = Number.parseInt(hex.slice(1), 16);
  return [(n >> 16) & 255, (n >> 8) & 255, n & 255];
}

const toLinear = (c: number) => {
  const s = c / 255;
  return s <= 0.04045 ? s / 12.92 : ((s + 0.055) / 1.055) ** 2.4;
};

const fromLinear = (l: number) => {
  const s = l <= 0.0031308 ? l * 12.92 : 1.055 * l ** (1 / 2.4) - 0.055;
  return Math.round(Math.min(1, Math.max(0, s)) * 255);
};

const isHex = (value: string) => /^#[0-9a-f]{6}$/i.test(value);

/** Relative luminance (WCAG 2). */
export function luminance(hex: string): number {
  const [r, g, b] = channels(hex).map(toLinear) as [number, number, number];
  return 0.2126 * r + 0.7152 * g + 0.0722 * b;
}

/** Contrast ratio (WCAG 2), from 1 to 21. */
export function contrast(a: string, b: string): number {
  const [hi, lo] = [luminance(a), luminance(b)].sort((x, y) => y - x) as [number, number];
  return (hi + 0.05) / (lo + 0.05);
}

/** A color with its light scaled down by `factor` (1 leaves it as it is). Only plain hex colors
 * dim; "transparent", shadows and scrims stay as they are. */
export function dim(value: string, factor: number): string {
  if (!isHex(value) || factor === 1) return value;
  return `#${channels(value)
    .map((c) => fromLinear(toLinear(c) * factor))
    .map((c) => c.toString(16).padStart(2, "0"))
    .join("")}`;
}

/** The brightness factor for surfaces and for text at a dimming step. Text dims less in the dark
 * theme and darkens more in the light theme, so contrast holds as the screen dims. */
function factors(dark: boolean, dimming: number): { surface: number; text: number } {
  const surface = DIMMING[Math.min(Math.max(dimming, 0), DIMMING.length - 1)] ?? 1;
  return { surface, text: dark ? surface ** 0.4 : surface ** 1.5 };
}

function dimSet<K extends string>(
  base: Record<K, string>,
  surfaces: Set<string>,
  dark: boolean,
  dimming: number,
): Record<K, string> {
  const f = factors(dark, dimming);
  return Object.fromEntries(
    Object.entries<string>(base).map(([k, v]) => {
      // Dark text on a signal fill (dark theme) dims with the fill; white text (light) stays.
      const fixed = ON_SIGNAL.has(k) && (k === "lit-amber" || !dark);
      const factor = fixed ? 1 : surfaces.has(k) ? f.surface : f.text;
      return [k, dim(v, factor)];
    }),
  ) as Record<K, string>;
}

/** The interface colors for a theme at a dimming step. */
export function colors(dark: boolean, dimming: number): Colors {
  return dimSet<ColorName>(COLOR[dark ? "dark" : "light"], SURFACES, dark, dimming);
}

/** The drawing colors for a theme at a dimming step. */
export function panelColors(dark: boolean, dimming: number): PanelColors {
  return dimSet<PanelName>(PANEL[dark ? "dark" : "light"], PANEL_SURFACES, dark, dimming);
}

/** Sets the theme's colors on the page: `--color-<name>` and `--panel-<name>`. */
export function applyPalette(root: HTMLElement, dark: boolean, dimming: number): void {
  for (const [key, value] of Object.entries(colors(dark, dimming)))
    root.style.setProperty(`--color-${key}`, value);
  for (const [key, value] of Object.entries(panelColors(dark, dimming)))
    root.style.setProperty(`--panel-${key}`, value);
  root.style.setProperty("color-scheme", dark ? "dark" : "light");
}

/** The desktop scale for a window: max(1, min(width / 1440, height / 900)). Below desktop widths
 * it is always 1, since width / 1440 is under 1. */
export function scaleFor(width: number, height: number): number {
  return Math.max(1, Math.min(width / DESIGN_SIZE.width, height / DESIGN_SIZE.height));
}

/**
 * Sets the root size from the window's scale and the Text and controls size (a percentage), so
 * every size in rem follows. Breakpoints are in CSS pixels and do not move. Returns a function
 * that stops following the window.
 */
export function applyScale(root: HTMLElement, textPercent: number, win: Window = window): () => void {
  const update = () => {
    root.style.setProperty("--scale", String(scaleFor(win.innerWidth, win.innerHeight)));
    root.style.setProperty("--text-scale", String(textPercent / 100));
  };
  update();
  win.addEventListener("resize", update);
  return () => win.removeEventListener("resize", update);
}
