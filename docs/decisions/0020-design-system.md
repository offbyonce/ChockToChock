<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0020: A design system, enforced, before more screens

Date: 2026-11-12. Status: withdrawn; replaced by [0021](0021-design-system-and-layouts.md). Changes the bar set in the Phase 2 brief: 3.0 stays the
reference for content and features, but not for layout quality.

## Context
Screens were matched to 3.0 one at a time, and spacing and layout drifted: the audit
([docs/design/audit.md](../design/audit.md)) found 46 spacing values, 31 font sizes, 16 radii, 11
shadows, four segmented controls and five copies of the page title. Fixing screens one by one
does not hold, and 3.0 is not a high enough bar.

## Decision
- **Tokens.** One spacing scale on a 4 px base, a small type scale, radii, borders, shadows, the
  theme colors and a layout grid (columns, gutters, pane widths, breakpoints), in one token file.
  Nothing outside it.
- **Components.** One implementation each for buttons, chips, cards, list rows, fields, segmented
  controls, pane headers and section titles. Screens use only these. Panel drawings keep their
  own templates (one size per control kind, as now), with templates added for display units,
  gauges, readouts, levers and handles.
- **Enforcement.** A test fails on any raw size or color outside the token file, and every
  component has screenshot tests.
- **Layouts per class.** Phone (one column, step details as a bottom sheet, drawings
  collapsible), tablet portrait and landscape, laptop, desktop, and large or ultrawide (a maximum
  content width).
- **Docs.** `docs/design/system.md` holds the tokens, components, layouts and a quality rubric:
  grid alignment, spacing consistency, hierarchy, density, line length, touch targets, contrast.
- **Order.** Mockups of the phase screen, step detail, Flow view and Flight setup on the system,
  at phone, tablet and desktop sizes, go to the maintainer before any refactor. After approval,
  every screen moves onto the system.
- **Qualification.** Every screen passes a screenshot matrix (three phones, three tablets in both
  orientations, three laptop and desktop sizes, 2560 by 1440 and 4K at scale 1 and 2), checks for
  200 % zoom, reflow at 320 px, larger system text, safe areas, the on-screen keyboard, Safari's
  dynamic toolbar, touch targets, orientation change mid-flow and crisp drawings, and a scored
  independent review against the rubric as well as against 3.0. A real-device checklist for an
  iPad and a phone joins the brief, run each release. Tablet and phone support is claimed only
  once this passes.
- Screens of batch 2's SimBrief step wait for the system; its Rust work goes on.

## Consequences
- Every existing screen is refactored once, after the mockups are approved.
- New screens cost less: they are assembled from components on the grid.
- The screenshot comparison with 3.0 continues, but a screen can now differ from 3.0 when the
  rubric says it is better.
