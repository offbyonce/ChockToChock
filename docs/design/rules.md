# Design rules

## 1. Tokens and components, never per-page values

- Every color, font size, weight, spacing value, radius, shadow and size comes from `tokens.json`, exposed as CSS custom properties (for example `--color-accent`, `--space-4`). Theme switching swaps the color and panel sets; nothing else changes.
- Pages are assembled from shared components. At minimum: header, stage tabs, phase track, flight strip (closed and open), step row (to do, current, done, skipped, decision), row menu, Check box, If not box, Why row with How to do it and Terms, location line, map figure, controls figure, Where is this? pop-over and sheet, open-steps list, segmented control, toggle switch, button set (primary, secondary, skip, danger), field, field row with description and destination chip, section list, drawer, bottom sheet, bottom action bar, timer chip.
- **Panel drawings are rendered from pack data by one component.** The forward overhead map, every panel, the side view and the captain-seat view are drawn by shared renderers that take the pack's geometry plus a state (changed, resting, conditional, lit, unlit). No screen contains its own hand-drawn panel. The same renderer serves the step, the Where is this? pop-over, the Flight deck page and the phone sheet, at any size.
- A pull request that adds a raw hex value, pixel value or hand-drawn SVG to a page fails review.

## 2. Sizes and scaling

| Width | Layout |
|---|---|
| under 600 | Phone |
| 600 to 899 | Tablet portrait |
| 900 to 1439 | Tablet landscape (also a half-screen window beside the sim) |
| 1440 and up | Desktop, scaled |

- Scale = `max(1, min(width / 1440, height / 900))`. Never below 100%. 2560 x 1440 gives 160%; 5K at 1x gives 320%.
- Wide screens never add columns. The checklist column's content stops at 1040 px before scaling; extra width becomes margin.
- Settings has a Text and controls size choice (100, 115, 130%) applied on top.

## 3. Type and spacing

- Type sizes: 12, 14, 16, 20, 24, 32, plus 15 for mono values, 48 and 32 in Flow only. One page-title size per layout.
- Spacing steps: 4, 8, 12, 16, 24, 32, 40. Rows 12 top and bottom. Panels 16 to 24. Gutters 32 desktop, 20 tablet, 16 phone.
- Metrics: header 60, phase row 60, flight strip 48, side pane 520 at 1440, Flight section list 232.
- Every control and link is at least 44 x 44, including row menus, ? buttons, tabs, segments and inline links.

## 4. Color meaning

- **Blue** marks only what the current step changes, and the current step or selection. Values and labels are neutral text.
- **Amber** means open: skipped steps, open-step counts, If not.
- **Red** means emergency or a destructive action (Reset flight plan, Clear all progress).
- **Green** means done.
- **Unlit lights are an outline and legend only, with no fill.** Lit lights take their real color.
- Selected tabs and segments get a 1.5 px accent ring as well as weight, never just a pale fill.
- Text at least 4.5:1. Drawing outlines, switch bodies and status icons at least 3:1. Check both themes in CI.

## 5. Drawings

- Drawings size by width and keep their aspect ratio. Never fix the height.
- Labels render at 11 px or more and light legends at 10 px or more at the smallest size each drawing is used. Adjust label size inside the drawing, never the geometry.
- Switches are drawn in their real ON direction (down is ON for most of these switches) with OFF and ON labels beside each.
- Changed controls: blue fill and outline. Resting controls: neutral outline. A control whose setting depends on a condition the app does not know yet: dashed blue outline, with the legend "dashed: only if the step's condition applies". When the condition is known (for example from the center tank step), draw the resolved position instead.
- The step map is unlabeled except the focused panel, with direction captions (aft and forward, captain and F/O side).
- In light mode drawings use the light panel palette. No dark boxes on light pages.

## 6. Step pane and disclosure

- Order in the side pane: step and value, Check, If not, Why with How to do it and Terms, location line or map, controls drawing. Check, If not and Why are never hidden or covered, by the map, a pop-over or a sheet.
- Hiding the map hides only the map. The location line stays, with Where is this? and Show map.
- Desktop: the full map shows when the panel changes (setting: Always, New panel, On request, Off). Tablet and phone: the location line always shows, and the map lives in the Where is this? pop-over or sheet.
- If not states the condition and the action. ? explains why.
- Terms in a step get a "Terms ⌄" line that expands in place, and tapping a term opens its glossary entry.
- Tapping a light in a drawing opens its meaning.

## 7. Words

- Capitals only for text that appears in the airplane or its documents: placards, light legends, settings values, CDU pages, METAR groups. App labels use sentence case: Check, If not, The controls, Find it in your OFP.
- One name per action. **Reset flight plan** clears the plan, clearance and notes. **Clear all progress** (Settings) also clears ticks. Both offer Undo.
- Flight strip labels are the same closed and open: Runway, SID, Transition, Initial altitude, Squawk, Departure frequency, Notes.
- No designer notes or "sample data" text in the shipped interface.
- American English.
