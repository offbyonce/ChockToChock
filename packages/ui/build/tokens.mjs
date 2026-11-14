// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Writes the interface's tokens from docs/design/tokens.json, the source of truth (decision
// 0021): theme/tokens.generated.css (sizes, type, radii, borders, gutters and the layout classes,
// as CSS custom properties) and theme/tokens.generated.ts (the dark and light color and panel
// sets, which theme/tokens.ts applies with dimming). Run with `just tokens`; `just
// generated-check` fails if the committed files differ.

import fs from "node:fs";
import path from "node:path";

const root = path.resolve(import.meta.dirname, "../../..");
const source = path.join(root, "docs/design/tokens.json");
const out = path.join(root, "packages/ui/src/theme");
const tokens = JSON.parse(fs.readFileSync(source, "utf8"));

/** Drops "$comment"-style keys. */
const entries = (o) => Object.entries(o).filter(([k]) => !k.startsWith("$"));
/** A pixel value in rem, so it scales with the root size (wide screens and Text size). */
const rem = (px) => `${+(px / 16).toFixed(4)}rem`;

// The generated files' own license header, built from parts so reuse doesn't read it as this
// script's header.
const SPDX = "SPDX" + "-";
const header = [
  `/* ${SPDX}FileCopyrightText: 2026 offbyonce */`,
  `/* ${SPDX}License-Identifier: MIT */`,
  "",
  "/* Generated from docs/design/tokens.json by packages/ui/build/tokens.mjs (just tokens). Do not",
  "   edit by hand. */",
];

const vars = [];
const add = (name, value, note) => vars.push(`  --${name}: ${value};${note ? ` /* ${note} */` : ""}`);
for (const [k, v] of entries(tokens.space)) add(`space-${k}`, rem(v), `${v}`);
for (const [k, v] of entries(tokens.font.family)) add(`font-${k}`, v);
for (const [k, v] of entries(tokens.font.size)) add(`font-size-${k}`, rem(v), `${v}`);
for (const [k, v] of entries(tokens.font.weight)) add(`weight-${k}`, v);
for (const [k, v] of entries(tokens["line-height"])) add(`leading-${k}`, v);
for (const [k, v] of entries(tokens.radius)) add(`radius-${k}`, k === "pill" ? `${v}px` : rem(v), `${v}`);
for (const [k, v] of entries(tokens.border)) add(`border-${k}`, `${v}px`);
for (const [k, v] of entries(tokens.size)) {
  if (typeof v === "object") {
    for (const [k2, v2] of entries(v)) add(`size-${k}-${k2}`, rem(v2), `${v2}`);
  } else if (k === "focus-ring") {
    add(`size-${k}`, `${v}px`);
  } else {
    add(`size-${k}`, rem(v), `${v}`);
  }
}
const { breakpoints: bp, gutter } = tokens.layout;
add("gutter", rem(gutter.phone), `${gutter.phone}, phone`);
add("side-pane", rem(tokens.size["side-pane-tablet"]), "tablet landscape; desktop below");
add("slideout", rem(tokens.size["slideout-tablet"]), "tablet landscape; desktop below");

const colorBlock = (selector, theme) => [
  `${selector} {`,
  ...entries(tokens.color[theme]).map(([k, v]) => `  --color-${k}: ${v};`),
  ...entries(tokens.panel[theme]).map(([k, v]) => `  --panel-${k}: ${v};`),
  "}",
];
const media = (min, body) => [`@media (min-width: ${min}px) {`, "  :root {", ...body.map((l) => `  ${l}`), "  }", "}"];
const css = [
  ...header,
  "",
  "/* Layout classes: phone under 600, tablet portrait 600 to 899, tablet landscape 900 to 1439,",
  "   desktop 1440 and up (scaled by theme/tokens.ts). */",
  ":root {",
  ...vars,
  "}",
  "",
  "/* Colors before theme/tokens.ts applies the chosen theme and dimming: the device's theme. */",
  ...colorBlock(":root", "dark"),
  "@media (prefers-color-scheme: light) {",
  ...colorBlock(":root", "light").map((l) => `  ${l}`),
  "}",
  ...media(bp["phone-max"] + 1, [`  --gutter: ${rem(gutter.tablet)}; /* ${gutter.tablet} */`]),
  ...media(bp["tablet-landscape-max"] + 1, [
    `  --gutter: ${rem(gutter.desktop)}; /* ${gutter.desktop} */`,
    `  --side-pane: ${rem(tokens.size["side-pane"])}; /* ${tokens.size["side-pane"]} */`,
    `  --slideout: ${rem(tokens.size.slideout)}; /* ${tokens.size.slideout} */`,
  ]),
  "",
].join("\n");

const sets = (group) =>
  Object.fromEntries(["dark", "light"].map((theme) => [theme, Object.fromEntries(entries(group[theme]))]));
const ts = [
  `// ${SPDX}FileCopyrightText: 2026 offbyonce`,
  `// ${SPDX}License-Identifier: MIT`,
  "",
  "// Generated from docs/design/tokens.json by packages/ui/build/tokens.mjs (just tokens). Do not",
  "// edit by hand.",
  "",
  `export const COLOR = ${JSON.stringify(sets(tokens.color), null, 2)} as const;`,
  "",
  `export const PANEL = ${JSON.stringify(sets(tokens.panel), null, 2)} as const;`,
  "",
  "/** The widest width of each layout class below desktop, in CSS pixels. */",
  `export const BREAKPOINTS = ${JSON.stringify({ phoneMax: bp["phone-max"], tabletPortraitMax: bp["tablet-portrait-max"], tabletLandscapeMax: bp["tablet-landscape-max"] }, null, 2)} as const;`,
  "",
  "/** The size every desktop layout is drawn at before scaling. */",
  "export const DESIGN_SIZE = { width: 1440, height: 900 } as const;",
  "",
  "/** The smallest rendered sizes inside drawings, in CSS pixels. */",
  `export const DRAWING = { labelMin: ${tokens.font.drawing["label-min"]}, legendMin: ${tokens.font.drawing["light-legend-min"]} } as const;`,
  "",
].join("\n");

fs.writeFileSync(path.join(out, "tokens.generated.css"), css);
fs.writeFileSync(path.join(out, "tokens.generated.ts"), ts);
console.log("tokens written");
