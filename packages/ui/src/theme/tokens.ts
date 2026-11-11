// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Color tokens for the dark and light themes, and dimming (Phase 2 brief, step 7). Text contrast
// stays at least 4.5:1 in every theme and at every dimming step; tokens.test.ts checks it.
// Tokens are applied as CSS custom properties through the CSSOM, which the content security
// policy allows (no inline <style>).

export type Palette = {
  // Surfaces.
  bg: string;
  panel: string;
  raised: string;
  border: string;
  accentSurface: string;
  danger: string;
  // Text and icons.
  text: string;
  muted: string;
  accent: string;
  onAccent: string;
  onDanger: string;
  done: string;
  skipped: string;
};

const SURFACES: (keyof Palette)[] = ["bg", "panel", "raised", "border", "accentSurface", "danger"];
/** Text on a colored surface stays as it is, so it stays readable as the surface dims. */
const ON_SURFACE: (keyof Palette)[] = ["onAccent", "onDanger"];

export const DARK: Palette = {
  bg: "#121c2c",
  panel: "#1b2738",
  raised: "#24344b",
  border: "#34465f",
  accentSurface: "#ffb020",
  danger: "#c62828",
  text: "#e8ecf2",
  muted: "#aab6c6",
  accent: "#ffb020",
  onAccent: "#0b111a",
  onDanger: "#ffffff",
  done: "#5fd39b",
  skipped: "#ffb020",
};

export const LIGHT: Palette = {
  bg: "#f4f6f9",
  panel: "#ffffff",
  raised: "#e9eef4",
  border: "#c9d2dd",
  accentSurface: "#8a5200",
  danger: "#b3261e",
  text: "#16202c",
  muted: "#465364",
  accent: "#8a5200",
  onAccent: "#ffffff",
  onDanger: "#ffffff",
  done: "#17693f",
  skipped: "#8a5200",
};

/** Brightness factors for each dimming step, 0 (full) to 4. */
export const DIMMING = [1, 0.8, 0.64, 0.5, 0.4] as const;

/** Text and background pairs that must stay readable. */
export const TEXT_PAIRS: [keyof Palette, keyof Palette][] = [
  ["text", "bg"],
  ["text", "panel"],
  ["text", "raised"],
  ["muted", "bg"],
  ["muted", "panel"],
  ["muted", "raised"],
  ["accent", "bg"],
  ["accent", "panel"],
  ["accent", "raised"],
  ["done", "panel"],
  ["skipped", "panel"],
  ["onAccent", "accentSurface"],
  ["onDanger", "danger"],
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

/** A color with its light scaled down by `factor` (1 leaves it as it is). */
export function dim(hex: string, factor: number): string {
  return `#${channels(hex)
    .map((c) => fromLinear(toLinear(c) * factor))
    .map((c) => c.toString(16).padStart(2, "0"))
    .join("")}`;
}

/**
 * The palette for a theme at a dimming step. Surfaces dim by the step's factor. Text dims less in
 * the dark theme and darkens more in the light theme, so contrast holds as the screen dims; text
 * on a colored surface does not change.
 */
export function palette(dark: boolean, dimming: number): Palette {
  const base = dark ? DARK : LIGHT;
  const factor = DIMMING[Math.min(Math.max(dimming, 0), DIMMING.length - 1)] ?? 1;
  const textFactor = dark ? Math.sqrt(factor) : factor ** 1.5;
  const factorFor = (key: keyof Palette) => {
    if (ON_SURFACE.includes(key)) return 1;
    return SURFACES.includes(key) ? factor : textFactor;
  };
  return Object.fromEntries(
    Object.entries(base).map(([k, v]) => [k, dim(v, factorFor(k as keyof Palette))]),
  ) as Palette;
}

const CSS_NAMES: Record<keyof Palette, string> = {
  bg: "--bg",
  panel: "--panel",
  raised: "--raised",
  border: "--border",
  text: "--text",
  muted: "--muted",
  accentSurface: "--accent-surface",
  accent: "--accent",
  onAccent: "--on-accent",
  danger: "--danger",
  onDanger: "--on-danger",
  done: "--done",
  skipped: "--skipped",
};

/** Sets the theme's tokens on the page. */
export function applyPalette(root: HTMLElement, dark: boolean, dimming: number): void {
  const p = palette(dark, dimming);
  for (const [key, name] of Object.entries(CSS_NAMES)) root.style.setProperty(name, p[key as keyof Palette]);
  root.style.setProperty("color-scheme", dark ? "dark" : "light");
}
