<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0021: The design system and layouts

Date: 2026-11-12. Status: proposed. Replaces the proposed [0020](0020-design-system.md), which is
withdrawn.

## Context
The owner approved an interface design after five rounds of mockups and an independent review.
The handoff is in [docs/design/](../design/README.md). 0020's tokens and breakpoints were drafted
before it and differ from it. 3.0 stays the reference for features, not for layout.

## Decision
- **Tokens.** [`tokens.json`](../design/tokens.json) is the source of truth for every color, font,
  size, space, radius, shadow and panel color, in a dark and a light set. The interface reads them
  as CSS custom properties generated from that file. Switching theme swaps only the color and panel
  sets.
- **Components.** Pages are built from the shared components listed in
  [rules.md](../design/rules.md) §1, one implementation each. Pages hold no raw color, size,
  spacing, radius, font size, shadow or hand-drawn SVG. A lint in `just ci` fails on any of these
  outside the token and component files.
- **Sizes.** Phone under 600 px, tablet portrait 600 to 899, tablet landscape 900 to 1439 (also a
  half-screen window beside the simulator), desktop 1440 and up. On desktop the whole interface
  scales by `max(1, min(width / 1440, height / 900))`, never below 100 %. Wide screens add no
  columns; the checklist column's content stops at 1040 px before scaling. Settings adds Text and
  controls size (100, 115, 130 %) on top.
- **Color meanings.** Blue only for what the current step changes and for the current step or
  selection; amber for open (skipped, open-step counts, If not); red for emergency and destructive
  actions; green for done. Values and labels are neutral. Selected tabs and segments get a 1.5 px
  accent ring as well as weight. Text is at least 4.5:1; drawing outlines, switch bodies and status
  icons at least 3:1. A test checks both themes in CI.
- **Drawings.** One shared renderer draws the overview maps, every panel, the side view and the
  captain-seat view from pack data, at any size, in either theme, with the states changed,
  resting, conditional, resolved, lit and unlit (unlit is an outline and legend, no fill). Panel
  arrangement and proportions come from the pack and never change. Drawings size by width and
  keep their aspect ratio. Labels render at 11 px or more and light legends at 10 px or more at
  the smallest size each drawing is shown. Where a drawing cannot fit that in its space, it opens
  as a full-width sheet; labels are never shrunk and geometry never changes. Switches are drawn
  in their real ON direction (the pack's first position at the top) with OFF and ON labels. The
  existing control templates (keys, toggles, dials) carry over where they are better than the
  boards' stand-ins and gain the states above; templates for displays, gauges, readouts, levers,
  handles and continuous knobs are added.
- **Disclosure.** Detail opens on demand, but Check, If not and Why are never hidden or covered.
  Hiding the map hides only the map.
- **Words.** Capitals only for text that appears in the airplane or its documents; app labels in
  sentence case. One name per action (Reset flight plan; Clear all progress, which also clears
  ticks; both with Undo). The flight strip uses the same labels closed and open. No designer notes
  or sample-data text ships. American English.
- **Release checks.** Every row of [coverage.md](../design/coverage.md) has a reachable home at
  every size, or the release fails. Each batch's independent review checks both themes, phone,
  tablet portrait, tablet landscape, 1440 and 2560 by 1440, the coverage rows it touches, the
  token lint, 44 px targets, contrast, that nothing was removed, and a matched 3.0 compare.
- **Qualification before claiming tablet and phone support** (kept from 0020): a screenshot
  matrix (phones 375×667, 390×844, 430×932; tablets 768×1024, 820×1180, 1024×1366 in both
  orientations; 1280×800, 1440×900, 1920×1080; 2560×1440 and 4K at scale 1 and 2), 200 % zoom,
  reflow at 320 px, larger system text, safe areas, the on-screen keyboard, Safari's dynamic
  toolbar, touch targets, orientation change mid-flow and crisp drawings, plus a real-device
  checklist in the Phase 2 brief.
- **Fonts.** Geist for text and Geist Mono for values, bundled with their license (OFL).

## Consequences
- Every screen moves onto the tokens and components once, in the step 7 batches of the Phase 2
  brief; 0020's token values, components and mockup page are replaced.
- A design change is a change to `tokens.json` or a component, never to a page.
