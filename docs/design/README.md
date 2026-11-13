# Interface design handoff, round 5

This folder is the agreed interface design for Chock to Chock. The owner reviewed and approved it over five rounds of mockups plus an independent design review. Build the interface from it.

## Hard rules

1. **Everything is a design token or a shared component.** No color, size, spacing, radius, font size, shadow or drawing is written on an individual page. Pages use tokens from `tokens.json` and shared components. Panel drawings come from pack data through one renderer, never hand-drawn per screen.
2. **No feature may be removed.** Every 3.0 feature in `coverage.md` must have a reachable home at every size: phone, tablet portrait, tablet landscape, desktop and wide (up to 5K).
3. **Panel layouts cannot change.** Flight deck maps and panel drawings keep the arrangement and proportions from the pack. You may change how they render (color, contrast, label size, sizing), never where things are.
4. **Progressive disclosure is valued.** Reveal detail on demand (tap to open, ?, pop-overs, expandable rows) rather than crowding the screen, but never hide anything a pilot needs to complete the current step.
5. **Dark and light mode are both required** on every screen, including the panel drawings.

## Read in this order

1. `rules.md`: what the tokens mean and the rules every screen follows.
2. `tokens.json`: the token values. This is the source of truth.
3. `screens.md`: every screen, what is on it, its states, and which board shows it.
4. `coverage.md`: where every 3.0 feature lives at every size. Use it as a release check.
5. `content-to-validate.md`: wording and data in the mockups that must be sourced and validated before it ships, plus the panel icon work.
6. `boards/`: the mockup source, one file per board. It is plain HTML with inline styles and `{{t.x}}` / `{{p.x}}` holes that map to `tokens.json`. Use it for layout, hierarchy and behavior. Take values from `tokens.json`, not from the HTML.
7. `review/design-review-r4.md`: the independent review that drove round 5. Every finding (F1 to F22) is addressed in these boards.

## How the boards are built

- Light and dark boards come from the same source with a `theme` prop. Files ending in `Light` are wrappers.
- `Views.dc.html` holds Flow, Emergency, timer, CDU walkthrough, briefing, exterior inspection, approach, Flight log and Settings (`view` prop).
- `Tablet.dc.html` holds both tablet orientations and their overlays (`orient`, `overlay` props). `Mobile.dc.html` holds every phone screen (`screen` prop).
- `Checklist.dc.html` takes `mapHidden`, `w` and `h` props. `Wide.dc.html` renders it at 1600 x 900 scaled 1.6.
- Sample values (KSEA to KSFO, the METAR, fuel numbers, times) are illustration only.

## In this repository

The handoff was moved here unchanged on 2026-11-12. Decisions
[0021](../decisions/0021-design-system-and-layouts.md) (the design system and layouts) and
[0022](../decisions/0022-conditional-controls-and-decision-steps.md) (conditional controls and decision
steps) record it; the proposed 0020 is withdrawn. [audit.md](audit.md) and
[audit-tables.md](audit-tables.md) are the earlier audit that started this work, kept for history.
The boards load a `support.js` canvas runtime that is not part of the handoff, so they are read as
source, not opened in a browser.

### Owner's addendum: control drawings

The switch, button, knob, selector, guard and light drawings in `boards/` are schematic stand-ins.
They define position, states, the ON direction, the OFF and ON labels and the color rules, not the
drawing style. The app's existing control drawings stay wherever they are better, and move into
the shared renderer so they meet `rules.md`: tokens only, both themes, unlit lights outline only,
labels at 11 px or more and light legends at 10 px or more, the real ON direction, and the
changed, resting, conditional and resolved states. Where ours are better, ours win; where a board
shows a state or rule ours lack, it is added. The reviewer brief for batch 2 (the shared drawing
renderer) includes a side-by-side for each control type: the current drawing, the board, and the
result.

### Owner's answers on the handoff

- **Center tank:** the pilot answers until the pack has a sourced wing-tank capacity. After that
  the app may suggest an answer from the SimBrief plan, and the pilot confirms it.
- **Drawings that cannot fit 11 px labels** in their space open as a full-width sheet. Labels never
  go below the rule and the geometry never changes.
- **Open my OFP on SimBrief** is a plain link to SimBrief that sends no user data.
- **Qualification:** the wider list in decision 0021 gates any claim of tablet and phone support.

### Token additions

`tokens.json` is the delivered file plus additions the owner approved on 2026-11-12: border
widths, line heights and eight sizes (icons, the tablet side pane, the drawer, the open-step dot,
status icons, the progress bar and the Flow button). Each names its source board in a `$sources`
entry. Approved the same day for batch 2: `radius.popover`, `color.popover-border` and
`color.overlay-shadow` (WherePop's values, for every pop-over, sheet and drawer) and
`size.map-figure` (280, the side pane's map figure on the Checklist and WherePop boards). New tokens are proposed to the owner with their source first, never added on their own.
