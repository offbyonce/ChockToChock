<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# Chock to Chock design system

Decision [0020](../decisions/0020-design-system.md). The bar is the rubric below, not parity with
3.0. Every screen is built from these tokens, components and layouts; `design-tokens.test.ts`
fails on anything else. The audit that led here is in [audit.md](audit.md).

The tokens live in `packages/ui/src/theme/tokens.css` (sizes) and `theme/tokens.ts` (colors, set
for the theme and dimming). The components live in `packages/ui/src/ds/`. The design page
(`apps/web/design/`, unlinked) shows every component and the screen mockups:
`design/?view=gallery|phase|step|flow|setup&theme=dark|light`.

## Tokens

### Spacing: one scale, 4 px base

| Token | rem | px | Used for |
| --- | ---: | ---: | --- |
| `--space-1` | 0.25 | 4 | gaps inside a control, chip padding |
| `--space-2` | 0.5 | 8 | gaps between related items, a label and its field |
| `--space-3` | 0.75 | 12 | row padding, gaps between buttons |
| `--space-4` | 1 | 16 | card padding, the phone gutter, gaps between fields |
| `--space-5` | 1.25 | 20 | gaps between sections in a pane |
| `--space-6` | 1.5 | 24 | the tablet gutter, space under a page title |
| `--space-8` | 2 | 32 | the laptop gutter, the gap between screen and pane |
| `--space-10`, `-12`, `-16` | 2.5, 3, 4 | 40, 48, 64 | large separations, the end of a page |

Nothing sits between steps. A component that needs a size off the scale gets a named layout
token (below), not a raw value.

### Type

Geist for text, B612 Mono for values (Phase 2 brief), each bundled with its license.

| Token | rem | px | Used for |
| --- | ---: | ---: | --- |
| `--text-xs` | 0.75 | 12 | section titles (upper case), chips, counts, help lines |
| `--text-sm` | 0.875 | 14 | secondary text, meta lines, links, buttons, values in rows |
| `--text-md` | 1 | 16 | body, step names, field values |
| `--text-lg` | 1.125 | 18 | non-normal step names, the Flow value |
| `--text-xl` | 1.375 | 22 | pane titles |
| `--text-2xl` | 1.75 | 28 | page titles |
| `--text-3xl` | 2.25 | 36 | the Flow step name |

Weights: 400, 500, 600 (`--weight-regular`, `-medium`, `-bold`). Line heights: 1.2 for titles,
1.35 for rows and short text, 1.5 for paragraphs. Tracking: -0.015 em on titles, 0.06 em on
upper-case section titles; nothing else.

### Shape and depth

- Radii: 6 px (chips inside controls, keys), 10 px (buttons, fields, segmented controls), 14 px
  (cards, pop-outs, sheets), full (pills, dots).
- Borders: 1 px rules; 2 px for focus, the step bar, tick boxes and underlined blanks.
- Shadows: raised (a chosen segment), overlay (pop-outs, sheets), toast; one shadow color and one
  scrim.

### Controls

Every button, field and segment is at least 44 px tall (`--control-h`); on fine pointers,
controls inside dense rows may use 36 px (`--control-h-compact`). Icons are 20 px, or 16 px in a
button.

### Colors

The theme set in `tokens.ts`: surfaces (`--bg`, `--surface`, `--zone`, `--line` and the soft
tints) dim with the screen; text and signals (`--ink`, `--muted`, `--accent`, `--go`, `--warn`,
`--stop`, `--emergency`) dim less, so text stays at 4.5:1 or better at every dimming step
(`tokens.test.ts`). Color never carries meaning alone: states also change shape and words.

## Layout

### Classes and breakpoints

| Class | Width | Columns | Gutter | Screen | Pane |
| --- | --- | --- | --- | --- | --- |
| Phone | under 600 | 1 | 16 | full width | step details open as a bottom sheet; drawings collapsible |
| Tablet portrait | 600 to 899 | 1 | 24 | full width, up to 832 | as phone, the sheet centered |
| Tablet landscape | 900 to 1199 | 2 | 24 | up to 832 | beside the screen, 360 |
| Laptop | 1200 to 1439 | 2 | 32 | up to 832 | 400 |
| Desktop | 1440 to 1919 | 2 | 32 | up to 832 | 440 |
| Large and ultrawide | 1920 and up | 2 | 32 | up to 832 | 480; content centered, at most 1680 wide |

Media queries use only 600, 900, 1200, 1440 and 1920 px (and one pixel under each for max-width).
The top bar and phase rail stick; the pane sticks under them and scrolls on its own. Text runs at
most 72 characters wide (`--measure`). The app respects the safe areas (notches, home indicator)
on every edge.

### The frame

`AppShell` holds the top bar, the phase rail, the screen and the pane. The top bar shows, by
class: the menu and the phase name on phones; the view switch from tablet; the stage tabs, the
brand and Find from tablet landscape. Emergency is always there. The rail scrolls sideways and
shows the detail level and progress from tablet landscape.

### Flow on phones

The confirm and skip buttons sit at the bottom of the screen, within thumb reach, above the safe
area; the step scrolls above them.

## Components

One implementation each, in `packages/ui/src/ds/`. Screens use only these.

| Component | Variants | Notes |
| --- | --- | --- |
| `Button` | primary, secondary, quiet, danger, emergency, link; compact; icon before or after; pressed | one primary action per screen |
| `Chip` | neutral, accent, go, warn, stop; solid, dashed | status beside a name: Current (solid), Done (outline), Skipped (dashed) |
| `Card` | with header (kind tag, title, actions) or without; flush | lists inside run edge to edge |
| `ListRow` | leading, chips, value (right or below), meta, trailing, body; marked, selected, muted, large | the step row and every other list |
| `TickBox` | to do, current, done, skipped | a full touch target; states by shape |
| `Field` | boxed (label, input or select, unit choice, help); inline (step blanks) | values kept as typed, trimmed on leaving |
| `Segmented` | bar, inline (beside a field), cards (choices with descriptions) | radiogroup semantics |
| `Callout` | accent (CHECK), warn (IF NOT) | caption, title, detail |
| `PaneHeader` | page (level 1), pane (level 2); crumb, lead, actions | the only page and pane titles |
| `SectionTitle` | one | the only upper-case section caption |
| `Sheet` | bottom sheet on phones, dialog or anchored pop-out wider | safe areas, Escape, scrim |
| `AppShell` | with or without pane | the frame, per layout class |

The panel drawings keep their own templates in SVG units (`panel-layout.ts`), one size per kind:
`KEY` (lights and push buttons), `TOGGLE`, `DIAL`.

### Drawing templates still to build

Planned, one template per kind, in the same units, so these controls stop being drawn case by
case (audit, "Controls the drawing rules do not cover"):

- **Display** for display units and screens (DUs, CDUs, the standby display, the IRS display and
  keypad): a framed screen with its name inside, sized from the pack's `width` and `height` (the
  only kinds that keep a size).
- **Gauge** for round gauges (fuel temperature, cabin altitude and climb, duct pressure, outflow
  valve, crew oxygen): one dial face and needle, no positions.
- **Readout** for digital windows (FLT ALT, LAND ALT, ADF, rudder trim, the meter display): a
  window with its name under it.
- **Lever** for the landing gear, speedbrake, flap and start levers: a slot with labeled detents
  and the knob at the target.
- **Handle** for the parking brake and the fire handles: the handle at rest or pulled.
- **Continuous knob** for rheostats and the MCP knobs: a knob with an arc showing the direction
  to turn (increase or decrease), never a labeled position, since there is none.

Open: the 15 controls with no panel item (the control column, manual gear extension, the high
altitude landing switch, oxygen masks, windows, the EFB ground power page, pushback, briefings,
ground equipment, chocks, the exterior inspection, the maintenance log) stay map-only for now.

## Quality rubric

The independent reviewer scores each screen at each layout class from 1 (poor) to 5 (excellent)
on each line; a screen ships at 4 or better on every line, and nothing scores below 3.0's
equivalent.

| Line | 5 means |
| --- | --- |
| Grid alignment | every edge lines up with the column grid or a neighbor; no element is off by a few pixels |
| Spacing consistency | only scale steps; the same relationship always gets the same space |
| Hierarchy | one clear title, one primary action, and the eye finds the current step first |
| Density | as much as fits without crowding; nothing is spread thin on large screens |
| Line length | text within 72 characters; values never wrap mid-token |
| Touch targets | every control at least 44 px, with 8 px between neighbors on touch screens |
| Contrast | text at 4.5:1 or better, large text and controls at 3:1, in both themes and every dimming step |

## Checks

- `design-tokens.test.ts` (in `just ci`): no raw size or color, and no other breakpoint, outside
  the token files; legacy screens listed until the refactor moves them.
- `just design-components`: each component in both themes against its saved baseline
  (`apps/web/build/design-baselines`), in headless Firefox; `--update` after an intended change.
- `just design-shots <dir>`: the mockups at phone, tablet portrait and landscape, and desktop, in
  both themes.
- Qualification per screen (decision 0020): the screenshot matrix, zoom, reflow, larger text,
  safe areas, the keyboard, Safari's toolbar, orientation change and high-density drawings; then
  the real-device checklist in the Phase 2 brief.
