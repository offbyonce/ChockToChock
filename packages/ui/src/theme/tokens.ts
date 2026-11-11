// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Color tokens for the dark and light themes, and dimming (Phase 2 brief, step 7). The roles and
// base colors follow the 3.0 design. Text contrast stays at least 4.5:1 in every theme and at
// every dimming step; tokens.test.ts checks it. Tokens are applied as CSS custom properties
// through the CSSOM, which the content security policy allows (no inline <style>).

export type Palette = {
  // Surfaces: dim with the screen.
  bg: string;
  surface: string;
  zone: string;
  line: string;
  accentSoft: string;
  goSoft: string;
  warnSoft: string;
  stopSoft: string;
  // Text, icons and signal surfaces (the primary button, the Emergency button): dim less, so
  // they stay readable.
  ink: string;
  muted: string;
  zoneInk: string;
  accent: string;
  go: string;
  warn: string;
  stop: string;
  accentSurface: string;
  emergency: string;
  // Text on a signal surface: does not change.
  onAccent: string;
  onEmergency: string;
};

const SURFACES: (keyof Palette)[] = [
  "bg",
  "surface",
  "zone",
  "line",
  "accentSoft",
  "goSoft",
  "warnSoft",
  "stopSoft",
];
const ON_SIGNAL: (keyof Palette)[] = ["onAccent", "onEmergency"];

export const DARK: Palette = {
  bg: "#0e1116",
  surface: "#161a21",
  zone: "#1e242d",
  line: "#262c36",
  accentSoft: "#15273d",
  goSoft: "#10291f",
  warnSoft: "#2b2110",
  stopSoft: "#2c1616",
  ink: "#e8ebf0",
  muted: "#97a1b1",
  zoneInk: "#b4bdca",
  accent: "#5aa2f0",
  go: "#3ecf8e",
  warn: "#f0b35e",
  stop: "#ff8a80",
  accentSurface: "#5aa2f0",
  emergency: "#b42318",
  onAccent: "#0b1320",
  onEmergency: "#ffffff",
};

export const LIGHT: Palette = {
  bg: "#f7f8fa",
  surface: "#ffffff",
  zone: "#eef1f4",
  line: "#e2e5ea",
  accentSoft: "#e8f1fb",
  goSoft: "#e3f5ec",
  warnSoft: "#fdf3e3",
  stopSoft: "#fdecec",
  ink: "#12161c",
  muted: "#5d6677",
  zoneInk: "#44505c",
  accent: "#0a66c2",
  go: "#0f7d4d",
  warn: "#9a4b07",
  stop: "#c62828",
  accentSurface: "#0a66c2",
  emergency: "#b42318",
  onAccent: "#ffffff",
  onEmergency: "#ffffff",
};

/** Brightness factors for each dimming step, 0 (full) to 4. */
export const DIMMING = [1, 0.8, 0.64, 0.5, 0.4] as const;

/** Text and background pairs that must stay readable. */
export const TEXT_PAIRS: [keyof Palette, keyof Palette][] = [
  ["ink", "bg"],
  ["ink", "surface"],
  ["ink", "zone"],
  ["muted", "bg"],
  ["muted", "surface"],
  ["muted", "zone"],
  ["zoneInk", "zone"],
  ["accent", "bg"],
  ["accent", "surface"],
  ["accent", "accentSoft"],
  ["go", "surface"],
  ["go", "goSoft"],
  ["warn", "surface"],
  ["warn", "warnSoft"],
  ["stop", "surface"],
  ["stop", "stopSoft"],
  ["onAccent", "accentSurface"],
  ["onEmergency", "emergency"],
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
 * The palette for a theme at a dimming step. Surfaces dim by the step's factor. Text and signal
 * surfaces dim less in the dark theme and darken more in the light theme, so contrast holds as
 * the screen dims; text on a signal surface does not change.
 */
export function palette(dark: boolean, dimming: number): Palette {
  const base = dark ? DARK : LIGHT;
  const factor = DIMMING[Math.min(Math.max(dimming, 0), DIMMING.length - 1)] ?? 1;
  const textFactor = dark ? Math.sqrt(factor) : factor ** 1.5;
  const factorFor = (key: keyof Palette) => {
    if (ON_SIGNAL.includes(key)) return 1;
    return SURFACES.includes(key) ? factor : textFactor;
  };
  return Object.fromEntries(
    Object.entries(base).map(([k, v]) => [k, dim(v, factorFor(k as keyof Palette))]),
  ) as Palette;
}

/** `accentSoft` becomes `--accent-soft`. */
const cssName = (key: string) => `--${key.replace(/[A-Z]/g, (c) => `-${c.toLowerCase()}`)}`;

/** Sets the theme's tokens on the page. */
export function applyPalette(root: HTMLElement, dark: boolean, dimming: number): void {
  const p = palette(dark, dimming);
  for (const [key, value] of Object.entries(p)) root.style.setProperty(cssName(key), value);
  root.style.setProperty("color-scheme", dark ? "dark" : "light");
}
