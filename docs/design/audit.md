<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# Design audit (design system batch, step 1)

Taken on the `phase-2/design-system` branch at the end of batch 2's step 6. Every style value in
the interface's 30 style sources (Svelte components, `app.css`, the web app's pages) was counted
by `tools/design-audit/audit.py`; the full tables, with where each value is used, are in
[audit-tables.md](audit-tables.md). Rerun the tool after the refactor to compare.

## Summary

| What | Distinct values | Uses | A system needs about |
| --- | ---: | ---: | ---: |
| Spacing (padding, margin, gap, offsets) | 46 | 320 | 8 to 10 |
| Font sizes | 31 | 131 | 6 to 7 |
| Font weights | 5 | 107 | 3 |
| Line heights | 9 | 22 | 3 |
| Letter spacing | 7 | 18 | 2 |
| Corner radii | 16 | 56 | 4 |
| Border widths | 4 | 54 | 2 |
| Shadows | 11 | 13 | 3 |
| Colors (all but 15 uses through theme variables) | 32 | 343 | the theme set |
| Fixed sizes (widths, heights) | 49 | 89 | from the grid and the touch target |
| Class names styled separately in two or more files | 43 | | 0 |

## Inconsistencies

**Spacing.** 46 values where a 4 px scale has about 10. Half of them are used once or twice and
sit between scale steps: 0.15, 0.2, 0.3, 0.35, 0.4, 0.4375, 0.45, 0.5625, 0.6, 0.65, 0.7,
0.8125, 0.9 and 0.9375 rem (2.4 to 15 px). Neighbors differ by 1 or 2 px: 0.6 against 0.625 rem,
0.8 against 0.8125 rem, 0.4 against 0.4375 against 0.45 rem. Card padding is 0.875 rem in one
header, 0.8125 rem in another, 1 rem in a third.

**Type.** 31 sizes, many 1 px apart: 0.72, 0.7188, 0.75, 0.78, 0.7812, 0.8, 0.8125, 0.84, 0.8438,
0.86, 0.87 and 0.875 rem all sit between 11.5 and 14 px. Flow view alone uses five sizes plus a
`vw` clamp. Weights: 400, 500, 600, 650 and 700, with 650 only on page titles. Nine line heights
and seven letter spacings, so the small upper-case section titles ("THE CONTROLS", "ROUTE",
menu groups) differ: 0.05, 0.06 and 0.08 em tracking at 0.6875 or 0.75 rem.

**Radii.** 16 values: 0.375, 0.4375, 0.5, 0.5625, 0.6, 0.625, 0.75, 0.875 rem, 7 px, 9 px, 50 %,
99 rem and others. Cards are 0.75 rem, the flight strip pop-out 0.75 rem, the toast 0.6 rem,
segmented controls 0.625 rem in one place and 9 px in another.

**Borders and shadows.** Rules are 1 px, but fields and blanks use 1.5 px and 2 px underlines.
Eleven shadows for four kinds of surface (a raised segment, a pop-out, a sheet, a toast), at
14, 18, 20, 25, 28, 35 and 40 % black.

**Colors.** Good: 328 of 343 uses go through theme variables. Off the theme: three opacities of
black for shadows and scrims (25, 35, 45 %), two `color-mix` tints written inline, and three raw
hex colors on the placeholder site page.

**Components built more than once.**
- Page titles: the same `h1` (1.6875 rem, 650) is restyled in five pages, with two different
  bottom margins (0.5 and 1.125 rem); Flow's title is a sixth, clamp-sized.
- Lead paragraphs: three copies.
- Segmented controls: four implementations (Settings, Flight setup's Start card, phase options,
  the unit choice beside a field), on two different backgrounds (`--zone`, `--bg`), with two
  shadows and two radii.
- Links: three local `.link` styles (flight strip, Flow, open steps), one muted and small, one in
  the accent color, one underlined.
- Buttons: the shared `.btn`, restyled again inside Flow view and step rows.
- Cards: the shared `.card`, restyled in the reference page; the phase option cards are a third
  shape.
- Badges, tags and counters: separate `.tag`, `.badge`, `.count` and `.dot` styles in five files.
- Text fields: 2.875 rem tall in Flight setup, 2.5 rem in the flight strip, underlined blanks in
  steps; three focus and invalid treatments.
- Control heights: 2.25, 2.5, 2.75 and 2.875 rem for controls that should share one touch height.

**Layout.** There is no grid. Pane widths come from a `clamp` in App, the list's measure from each
page, and breakpoints are written per component: 560, 600, 640, 760, 900, 1023, 1024 and 1100 px all appear.
There is no phone layout beyond collapsing the side pane, and no maximum content width for large
screens.

## Controls the drawing rules do not cover

The drawings draw lights and push buttons as one key, toggles and dials from one template each,
and show each step's targets (decisions 0014 to 0018). These controls fall outside those rules
and need their own treatment, now or in batch 3:

- **Kinds with no template (46 items).** Display units and screens (24: the DUs, CDUs, the
  standby display, the IRS display and keypad, audio panel 3, the stabilizer trim wheel), gauges
  (7: fuel temperature, cabin altitude and climb, duct pressure, outflow valve, crew oxygen),
  readouts (6: meter display, FLT ALT, LAND ALT, ADF frequencies, rudder trim), levers (5: landing
  gear, speedbrake, flaps, start levers 1 and 2) and handles (4: parking brake, engine and APU
  fire handles). Each is drawn by its own code; they need templates of their own (one per kind)
  in the system.
- **Selectors with no positions to point at (23).** Continuous knobs and rheostats ("OFF to
  BRIGHT" lighting knobs, the MCP course, speed, heading and altitude knobs, the FLT ALT and LAND
  ALT knobs, the ADF tuners, the DU brightness knobs) and two selectors whose real positions are
  not in the vendor documentation we use (EFIS RANGE and the bank angle selector, shown with one
  stand-in caption each). A step can highlight them but never show a target position.
- **Controls with no panel item (15).** The control column, the manual gear extension access, the
  high altitude landing switch, the flight deck windows, the oxygen masks, the EFB ground power
  page, pushback, briefings, ground equipment, chocks, the exterior inspection and the maintenance
  log. Steps for them show the flight deck map only (and a note when the map cannot show them).
- **Positions left to a condition (39 targets in 21 steps).** Center fuel pumps, ground power and
  APU choices, anti-ice, the MINS selectors, one-engine memory items and others. Drawn with no
  position and "SEE STEP"; the condition itself is only in the step's wording.
- **Lights that may cycle.** The window heat ON lights get no light state (decision 0018).
- **Pack data the drawings ignore (52 items; removed in this batch).** Lights and push buttons that still carry their
  own width or height from the 3.0 import (door lights 48 wide, annunciator keys 42 by 24 and 54
  by 34, gear lights 42 by 0, cargo fire keys 38 to 46 wide). Every key is drawn at the standard
  size, so these values were dead and misleading. `pack check` now rejects a size on anything but a
  display unit or readout, and the values are gone (one more turned up than this audit first
  counted: 52).
