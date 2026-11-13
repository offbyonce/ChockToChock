# Chock to Chock: design review, rounds 3 and 4

Reviewer: independent product design review, from source (boards not rendered).
Scope: `design-canvas-3/project/*.dc.html` (R3) and `design-canvas-4/project/*.dc.html` (R4), measured against the 3.0 README and the 3.0 vs current screenshots.

How measurements were taken: SVG label sizes are the viewBox font size multiplied by the rendered scale, which is min(rendered width / viewBox width, rendered height / viewBox height) for the default `preserveAspectRatio`. Contrast ratios use the WCAG 2.x relative-luminance formula. Text needs 4.5:1. Graphics a pilot needs to read (panel outlines, switch bodies, lens rims, status icons) need 3:1 (WCAG 1.4.11).

---

## 1. Feature coverage table

Key
- **MISSING**: the board for that size exists and is where the feature should live, but the feature isn't on it and there's no other drawn path to it.
- **NO BOARD**: the page or overlay that would hold the feature hasn't been drawn at that size. That's a coverage gap, not a removal.
- **Wide** = `Wide.dc.html`, which renders R3 `Main` only, at 1600x900 scaled 1.6. Every other page at Wide is NO BOARD. By the scale rule it inherits the desktop page, so those cells read "inherits desktop".
- Tablet menu drawers were never drawn. The hamburger exists on both tablet boards.

| # | 3.0 feature | Desktop 1440 | Wide (to 5K) | Tablet landscape 1180 | Tablet portrait 820 | Phone 390 |
|---|---|---|---|---|---|---|
| 1 | Five stages, phases, 300+ steps | Stage tabs in the header plus phase tabs with progress (R3 Main, R4 Checklist). The phase's challenge-response **Checklist** section (3.0 "Before start checklist") isn't visible on any board, only the Procedure | Same, scaled | Stage drop-down ("Departure ▾") plus phase tabs. "Takeoff" is clipped, and the visible text "Scrolls sideways" is a designer note | Stage drop-down plus phase tabs, clipped after "Taxi" with no scroll cue | Header stage and phase picker ("Departure / Before start ▾") |
| 2 | Checklist or Flow | Header segmented control. The Flow view itself isn't drawn anywhere | Header | Header | Header | Menu (PhoneMenu) |
| 3 | Full or Essentials | "Full · Detailed" chip (Main, R4 Checklist) and the Menu toggles | Chip | **MISSING** (no chip, drawer not drawn) | **MISSING** (no chip, drawer not drawn) | PhoneMenu toggle |
| 4 | Make Essentials your own (add or remove steps) | **MISSING**. The row "···" menu has only Make current, Mark done and Skip | **MISSING** | **MISSING** ("···" present, its contents follow the desktop spec) | **MISSING** | **MISSING** |
| 5 | Essentials marker (E) on steps | R3 Main: every row. R4 Checklist: **current row only** (a regression) | Every row (R3 Main) | **MISSING** (no E on any row) | **MISSING** | Current row only |
| 6 | Detailed or Simple | Chip and Menu toggles | Chip | **MISSING** | **MISSING** | PhoneMenu toggle |
| 7 | Step map and panel views in steps | Aside: step map plus "The controls" (Main, R4 Checklist light and dark) | Aside, scaled | Aside | Inline in the current step, two figures side by side | "Where · Controls" row opens PhoneWhere. Where tab drawn, Controls tab not drawn |
| 8 | Flight deck reference page (overview map, every panel) | R4 FlightDeck, light and dark. Menu › Reference › Flight deck (R4 Menu) | NO BOARD (inherits desktop) | NO BOARD | NO BOARD | **MISSING**: PhoneMenu (R3) has no Flight deck link |
| 9 | Light meanings | R4 FlightDeck aside only. The lens drawn in a step can't be tapped for its meaning | NO BOARD | NO BOARD | NO BOARD | **MISSING** (no route to Flight deck on phone) |
| 10 | Where is this? locator | WherePopover: side view plus captain-seat view | Link in the aside | Link only. Pop-over not drawn | Link only. Pop-over not drawn | PhoneWhere: side view plus overhead map. **The captain-seat view isn't there** (owner decision) |
| 11 | Map on/off | "Hide map" in the aside, "Show map" in MapHidden. Settings not drawn | "Hide map" | **MISSING** (no Hide map, Settings not drawn) | **MISSING** | Map is always behind the "Where · Controls" row (acceptable) |
| 12 | Check, If not, Why | Aside (Main, MapHidden, R4 Checklist) | Aside | Aside | Inline | Inline |
| 13 | How to do it | Aside link (Main, R4 Checklist). **MISSING in the MapHidden state** | Aside link | **MISSING** | **MISSING** | **MISSING** |
| 14 | Terms a step uses (3.0 showed N1, CDU definitions in details) | **MISSING** | **MISSING** | **MISSING** | **MISSING** | **MISSING** |
| 15 | Exterior inspection (18 stops) | R4 StartOptions on/off toggle. The stage and its 18-stop diagram aren't drawn | NO BOARD | NO BOARD | NO BOARD | PhoneMenu link (R3, contradicts the R4 toggle model) |
| 16 | Ground power or APU | R4 StartOptions "Power source". The inline choice at Power up isn't drawn | NO BOARD | NO BOARD | NO BOARD | NO BOARD (strip › All flight data exists, page not drawn) |
| 17 | iFly panel states (7) | R4 StartOptions select (options not listed) | NO BOARD | NO BOARD | NO BOARD | NO BOARD |
| 18 | Turnaround start | R4 StartOptions toggle | NO BOARD | NO BOARD | NO BOARD | NO BOARD |
| 19 | Flight strip: route and clearance | Main strip, StripOpen | Strip | Strip (open state not drawn) | Strip (open state not drawn) | Strip, PhoneStrip |
| 20 | Flight strip notes | StripOpen "Notes from ATC", "Notes 1" chip | Chip | Chip | Chip | PhoneStrip textarea |
| 21 | Step values from flight data (squawk on the transponder step) | R3 Main "— set takeoff trim" link. R4 Flight "Used by the altimeter step". The R4 Checklist board has no linked value | Same as Main | "— set trim" link | "— set trim" link | Not shown on the board |
| 22 | Clear flight, with Undo | StripOpen "Clear flight". R4 Menu "Reset flight plan" (two names, see F9) | NO BOARD | NO BOARD | NO BOARD | PhoneStrip "Clear flight". PhoneMenu has no Reset flight plan |
| 23 | SimBrief import (username, Import latest) | Flight page header "Import latest" (R3 FlightSetup, R4 Main). Not in StripOpen, which is where 3.0 also offered it. Username entry not drawn | NO BOARD | NO BOARD | NO BOARD | NO BOARD (not in PhoneStrip either) |
| 24 | Offline: drop a saved OFP file | NO BOARD ("SimBrief" nav item, section not drawn) | NO BOARD | NO BOARD | NO BOARD | NO BOARD |
| 25 | CDU step grouped by page, and the CDU walkthrough | NO BOARD (R3 FlightSetup CDU chips are related) | NO BOARD | NO BOARD | NO BOARD | NO BOARD |
| 26 | Step timers | NO BOARD | NO BOARD | NO BOARD | NO BOARD | NO BOARD |
| 27 | Emergency reference | Header red button (page not drawn) | Header | Header | Header | Header (40 px tall) |
| 28 | Briefings in the flow | NO BOARD | NO BOARD | NO BOARD | NO BOARD | NO BOARD |
| 29 | Practice | Menu (R3, R4) | NO BOARD (inherits desktop) | NO BOARD (drawer) | NO BOARD (drawer) | PhoneMenu |
| 30 | How procedures work | Menu | NO BOARD | NO BOARD | NO BOARD | PhoneMenu |
| 31 | Glossary | Menu | NO BOARD | NO BOARD | NO BOARD | PhoneMenu |
| 32 | Light and dark mode | R4 Checklist, Flight weather and FlightDeck in both. StartOptions dark only. R4 Menu light only. R3 StripOpen, Where and FlightSetup dark only | Dark only | Dark only | Dark only | Dark only |
| 33 | Flight log (save copy, CSV, JSON, load, print, log and start new) | Menu link, page not drawn | NO BOARD | NO BOARD | NO BOARD | PhoneMenu link, page not drawn |
| 34 | Open steps: one drop-down listing open steps on **earlier pages** | **MISSING**: only "1 open, go to it" for the current phase, plus an amber dot on that phase tab | **MISSING** | **MISSING** ("1 open", current phase) | **MISSING** | **MISSING** ("1 open", current phase) |
| 35 | Skip a step | "Skip for now" on the current step, the row menu, "Do it now" on skipped rows | Same | Current-step button | Current-step button | Bottom bar button |
| 36 | Find (Ctrl K) | Header | Header | **MISSING** | **MISSING** | Header icon |
| 37 | Keyboard: Space, Left arrow, S | "Space done · S skip" hint. The Left arrow (back) is absent from the hint | Same | n/a (touch) | n/a | n/a |
| 38 | Settings (map, theme, Clear, backup) | Menu link, page not drawn | NO BOARD | NO BOARD | NO BOARD | PhoneMenu link |
| 39 | Take the tour | Menu | NO BOARD | NO BOARD | NO BOARD | PhoneMenu |
| 40 | References page | Menu | NO BOARD | NO BOARD | NO BOARD | PhoneMenu |
| 41 | Flight setup (all flight data) | Strip › All flight data › R3 FlightSetup, R4 Flight | NO BOARD | NO BOARD | NO BOARD | PhoneStrip "All flight data" (page not drawn) |
| 42 | Approach type and arrival data at Descent | NO BOARD | NO BOARD | NO BOARD | NO BOARD | NO BOARD |
| 43 | Progress saves (Saved indicator, overall count) | "41/365 · Saved 10:14" | Same | "Saved 10:14", no overall count | Neither | Phase count only |

**MISSING cells: 31.** Rows 4 (5), 14 (5) and 34 (5) account for 15. Then row 13 (4), rows 3, 5, 6, 11 and 36 (2 each, both tablets), and rows 8 and 9 (1 each, phone).

**NO BOARD gaps worth calling out:** there is nothing at any size for timers, the CDU walkthrough, briefings, Flow view, Emergency content, the Flight log page, Settings, exterior inspection content, the offline OFP drop, or Descent approach data. No tablet board exists for the strip open state, the Where pop-over, the menu or any Flight page. No phone board exists for the Flight page or the Flight deck page. No light-mode board exists below 1440.

---

## 2. Findings, most severe first

### F1. Panel drawing labels render at 6 to 7 px. The controls are less legible than 3.0's.
**Boards:** R3 Main, MapHidden, TabletLandscape, TabletPortrait, WherePopover, PhoneWhere. R4 Checklist (light and dark), R4 FlightDeck.

Each SVG has a fixed `height`, so it letterboxes and shrinks the viewBox, and every label shrinks with it.

| Board, figure | viewBox | Rendered box | Scale | Label size, viewBox → rendered |
|---|---|---|---|---|
| Main or R4 Checklist, step map | 420x182 | ~476x118 | 0.65 | 9 → **5.8 px** (direction captions), 11 → 7.1 px ("FUEL") |
| Main or R4 Checklist, controls | 440x186 | ~476x136 | 0.73 | 8 → **5.9 px** (LOW PRESSURE), 9 → 6.6 px (OFF/ON), 11 → 8.0 px (AFT 1 · ON) |
| TabletLandscape, step map | 420x182 | ~388x104 | 0.57 | 11 → **6.3 px** |
| TabletLandscape, controls | 440x186 | ~388x150 | 0.81 | 8 → 6.4 px, 9 → 7.3 px |
| TabletPortrait, controls | 440x186 | ~394x160 | 0.86 | 8 → 6.9 px |
| WherePopover, side and seat views | 400x210, 400x270 | ~278x176 | 0.70, 0.65 | 11 → **7.6 px, 7.2 px** |
| R4 FlightDeck, panel detail | 440x186 | ~394x190 | 0.90 | 8 → 7.2 px, 9 → 8.0 px |

The switch bodies also end up about 10x20 px on desktop. The OFF/ON labels, which are the point of the owner's switch-direction decision, are at 6.6 px. The 3.0 screenshot gives its single panel view about 230 px of height. R3 and R4 give the controls 136 px.

**Why it matters:** the pilot reads this glance-wise, on a second monitor, with eyes going back and forth to the sim. Below about 11 px, OFF and ON stop being reliable to tell apart, and a misread switch direction defeats the feature.

**Fix (rendering only, no arrangement change):**
1. Size each SVG by width and let the height follow the aspect ratio (`height:auto`, or compute the height from the viewBox). Never fix the height.
2. On desktop and tablet, give "The controls" figure `flex:1` so it takes the free height in the aside. R4 Checklist has about 100 px spare (budget below), which lifts the controls to roughly 230 px.
3. Raise label font sizes inside the viewBox so they render at 11 px or more for OFF/ON and panel names, and 10 px or more for lens legends, at the smallest size each board uses. For the fuel panel at about 456 px wide, that's viewBox sizes of about 11 for OFF/ON and 10 for LOW PRESSURE. Panel and lens geometry stay unchanged.
4. Pop-over and phone figures stack vertically (side view above seat view) so each gets the full width (about 600 px desktop, 358 px phone) instead of 278 px. Each figure keeps its own drawing as is.

Aside budget for R4 Checklist (728 px available below the strip): kicker 16, map figure about 166, controls figure about 186, Check 60, If not 60, Why 41, gaps 60, padding 36, which totals about 625 px. In R3 Main the step heading block (about 123 px) pushes the total to about 766 px, which **overflows 728 px, so the Why row is clipped** at 1440x900 under `overflow:hidden`. R4 fixed that by deleting the heading. Keep R4's structure.

### F2. Panel outlines, unlit lenses and resting switches fall below 3:1 in dark mode. In light mode the panels are faint.
**Boards:** every panel drawing, R3 and R4. Colors come from the R4 `P` object.

| Element | Dark | Ratio | Light | Ratio |
|---|---|---|---|---|
| Panel stroke vs figure bg | #3a4351 on #161b23 | **1.73** | #aeb7c2 on #ffffff | **2.03** |
| Panel fill vs figure bg | #1b212b on #161b23 | 1.07 | #dde2e8 on #ffffff | 1.30 |
| Unlit lens rim vs figure bg | #15181e on #161b23 | **1.03** | #9aa3ae on #ffffff | 2.55 |
| Unlit lens fill vs figure bg | #1c2028 on #161b23 | ~1.1 | #cdd3db on #ffffff | 1.51 |
| Resting switch body (FlightDeck) | #4a5361 on #161b23 | **2.22** | #8a93a0 on #ffffff | 3.11 |
| Highlight fill vs panel fill | #163056 vs #1b212b | ok, the stroke carries it | #d6e4fd vs #dde2e8 | **1.02** (only the 2.5 px #2563eb stroke at 5.17 carries it) |
| Windshield label (R3 Where) | #5a6372 on #1d2a3a | **2.40** (text) | n/a | |

In dark mode the "real lens in neutral grey" isn't a visible lens at all. Only its legend text shows, which makes it read like a floating label. The overhead map, the pilot's orientation device, is drawn in outlines at 1.7:1.

**Fix (colors only):**
- Dark: panel stroke `#6b7482` (3.66). Lens rim stroke `#6b7482` with lens fill `#2a303a`. Raise the lens legend to `#a3abb8` (5.73 on #2a303a; the current #8b94a3 drops to 4.34 there). Resting switch body `#6b7482`. Windshield label `#8b94a3` (4.75).
- Light: panel stroke `#8a93a0` (3.11). Keep the lens colors. The highlight is carried by the accent stroke, so make it 3 px at every size and keep the fill as a secondary cue only.
- Re-run both themes through a contrast check in CI against these tokens.

### F3. The "Open steps" drop-down from 3.0 is gone. Only a current-phase link remains.
**Boards:** R3 Main, R4 Checklist, both tablets, Phone.

3.0 had "One drop-down at the top of the page lists every step left open on earlier pages." Every board shows only "1 open, go to it" for the phase on screen, plus an amber dot on that phase's tab. A step skipped in Preflight is invisible from Departure, because the stage tabs carry no dot.

**Why it matters:** skipped steps are a core safety affordance of 3.0. Hunting stage by stage is exactly what 3.0 removed.

**Fix:** make the count control in the procedure header a disclosure, "2 open steps ⌄". It opens a pop-over listing every open step from earlier pages, grouped by stage and phase, each with "Go to step". Put an amber dot on stage tabs too, and on the phone stage picker. On tablet and phone, the "1 open" link opens the same list as a sheet. This adds nothing permanent to the viewport.

### F4. "Make Essentials your own" and the per-row E marker regressed.
**Boards:** R3 Main and R4 Checklist row menus. R4 Checklist rows. Both tablets.

The row "···" menu offers Make current, Mark done and Skip for now. Adding or removing a step from Essentials has no home at any size. R4 Checklist shows E only on the current row, while R3 Main showed it on every row. The tablets show none at all.

**Fix:** add "Add to Essentials" or "Remove from Essentials" to the row menu at every size. Restore E on every row that's an Essentials step, in neutral #a9b4c4 with a #3a4351 border (dark), as R3 Main had it. Put it on tablet rows between the name and the value.

### F5. The tablets drop Find, the step mode, Hide map and How to do it. Their menus and overlays aren't drawn.
**Boards:** TabletLandscape, TabletPortrait.

- No Find button. Ctrl K doesn't exist on a touch tablet, so Find is unreachable.
- No "Full · Detailed" chip, and the drawer isn't drawn.
- No "Hide map". The map-on/off feature has no home.
- No "How to do it".
- No E markers.
- The phase tabs clip with `overflow:hidden` and give no scroll cue. Portrait loses Before takeoff and Takeoff entirely from view. "Scrolls sideways" is a designer note rendered as UI text.

**Fix:**
- Add a 44x44 Find icon button between the stage picker and Checklist/Flow, as on Phone.
- Put the "Full · Detailed" chip at the right end of the phase row, replacing "Scrolls sideways · Saved 10:14". Move "Saved" into the menu header.
- Add "Hide map" beside "Where is this?" in the map figure header, and "How to do it ⌄" on the Why row, exactly as on desktop.
- Give the phase tabs a trailing fade plus a chevron button, and auto-scroll the current phase into view.
- Draw the tablet menu drawer, the strip open state and the Where pop-over.

### F6. The "Where is this?" pop-over covers Check and If not, and the phone version drops the captain-seat view.
**Boards:** WherePopover, PhoneWhere.

On desktop the pop-over sits at left 776, top 352, 640 px wide and about 360 px tall. That lands it over the aside's Check and If not boxes (about y 600 to 740). The owner's rule is that Check, If not and Why are never hidden.

PhoneWhere's "Where" tab has the side view plus the overhead map, but no captain-seat view, which the owner made part of "Where is this?". The full-height sheet (top 70) also hides the step's Check line.

**Fix:**
- Desktop: anchor the pop-over to the left of the link and open it leftward over the step list (x about 236 to 876), so the aside stays readable. Stack the two figures vertically (F1).
- Phone: add the captain-seat view to the Where tab, between the side view and the overhead map. Give the sheet a medium detent (about 60 percent height) with the step's Check line pinned at the sheet foot, so the sheet never fully hides it.

### F7. "Terms a step uses" has no home at any size.
**Boards:** all checklist boards.

3.0's details listed the terms in the step (in the screenshot, "N1: Fan speed… CDU: Control display unit…"). None of the boards show them or a route to them. The "?" buttons belong to Check and If not specifically.

**Fix:** underline terms in step names, Check and If not (dotted underline, neutral text color). Tapping one opens a small glossary pop-over: term, one-line definition, "Open in Glossary". Under Why, add a disclosure line, "Terms: N1, CDU ⌄", that expands the list in place. This is progressive disclosure, so it costs about 20 px.

### F8. MapHidden loses "How to do it".
**Board:** MapHidden.

The Why row reads only "Why  Feeds fuel…", while Main has "How to do it ⌄" on the same row. Hiding the map must hide only the map (owner decision).

**Fix:** share one Why-row component between the two states, including the "How to do it ⌄" link.

### F9. Clearing has two names, and the menu's first item is destructive.
**Boards:** StripOpen and PhoneStrip ("Clear flight"). R4 Menu ("Reset flight plan: Clears the plan, clearance and notes").

The pilot can't tell whether these do the same thing, or whether either clears checklist progress (3.0's "Clear flight" did: "Start fresh"). R4 puts Reset first in the menu, as a full-width red-text button 56 px tall, so it's the most prominent thing in the drawer. The phone menu doesn't have it.

**Fix:**
- Decide the scopes and name them once. If both clear only plan data, use "Reset flight plan" in both places. If the strip action also clears ticks, call it "Start a new flight" and say so in its subtext.
- Keep Reset at the menu root, as the owner decided, but move it to the last group, behind a divider. Keep the Undo toast.
- Add it to PhoneMenu.

### F10. Flight strip labels change name between closed and open, and they're tracked caps.
**Boards:** Main, R4 Checklist, the tablets (closed strip). StripOpen (open).

The closed strip says RWY, SID, INIT, SQK, DEP. The open strip says Runway, Departure, Initial altitude, Squawk, Departure frequency. "SID" becomes "Departure", and "DEP" means departure *frequency*, so the departure procedure appears under two names and "Departure" means two things. The tablets also drop INIT and DEP silently.

**Fix:** one vocabulary across both states. Closed: "Rwy 16L · SID SUMMA2.SEA · Init 5000 · Sqk 4521 · Dep freq —". Open: Runway, SID, Initial altitude, Squawk, Departure frequency. Set the closed labels in sentence case at 12 px, #8b94a3 dark (6.1:1) or #5b6472 light (5.6:1), with no tracking.

The empty "—" placeholder is #6b7482 on #121720 (**3.80:1**, below 4.5). Use #8b94a3 (5.87).

### F11. The SimBrief import was removed from the strip, and the offline OFP drop has no board.
**Boards:** StripOpen, PhoneStrip. R4 Main ("SimBrief" nav item).

3.0: "Enter your SimBrief username in the Flight strip or Flight setup and press Import latest flight". The open strip has neither the username nor the import. The offline "drop a saved OFP file" path appears nowhere.

**Fix:** add "Import from SimBrief" to the open strip's footer row, left of "Clear flight". It reads "Import latest" once a username is saved, and opens a username field otherwise. Draw the SimBrief section of the Flight page with the username, Import latest, and a drop zone reading "Drop a saved OFP file", plus a file picker button so touch users can use it.

### F12. Conditional switches aren't resolved, though the app knows the answer.
**Boards:** Main, R4 Checklist, tablets, Phone.

R3 FlightSetup shows Center tank: 0 lb. The step still reads "ALL ON, CENTER PUMPS ON IF MORE THAN 1,000 LB (ABOUT 450 KG) IN THE CENTER TANK", a three-line mono caps value. The center switches are drawn as empty dashed capsules with no lever, so no direction is shown.

**Why it matters:** this is where the new app can clearly beat 3.0. The data is already there.

**Fix (keeps the dashed outline and the legend, per the owner):**
- When the field is known, draw the lever in the resolved position (OFF, up) inside the dashed outline.
- Show the value as two lines: "ALL ON" first, then "Center pumps OFF · center tank 0 lb". The second line is a link to the Center tank field, per the "values link to their field" decision.
- When the field is blank, keep today's wording and an empty dashed capsule.

### F13. "If not" states a condition but hides the action behind "?".
**Boards:** every Check and If not pair.

"IF NOT: Both center lights on" doesn't say what to do. The remedy is behind "?". If not is never hidden, so its visible line has to carry the action.

**Fix:** write the visible line as condition plus action ("Center lights on: check center tank fuel, then set center pumps OFF"). Keep "?" for the explanation. Apply the same pattern to Check.

### F14. Wide: the scale formula needs a floor and a content measure.
**Board:** Wide (only the R3 Main board exists).

- `scale = min(w/1440, h/900)` with no lower clamp shrinks the UI below 1440x900. A 1280x720 laptop gets scale 0.8, so 13 px meta text becomes 10.4 px and the 5.8 px SVG labels become 4.6 px. The README's typical "beside the simulator" window is narrower still.
- At 3440x1440 ultrawide, scale is 1.6 and the layout is 2150 CSS px wide. The main column becomes about 1610 px, and the R4 Flight description column runs about 150 characters per line.

**Fix (no new columns):**
- Clamp the scale with `max(1, …)`.
- Define the breakpoints below 1440: under about 1180 use the tablet landscape layout, under about 900 tablet portrait, under 600 phone. A half-screen 960x1080 sim window then gets a real layout instead of a shrunken one.
- Cap the main column's inner content at about 1040 CSS px (before scaling), left aligned. The extra width goes to the margin, not to line length.

### F15. Touch targets under 44 px.

| Element | Board | Size |
|---|---|---|
| Row "···" | R3 Main (all rows), R4 Checklist (TERR and current rows) | 44x32 |
| "?" | R3 FlightSetup and R4 Flight rows | 44x32 |
| Checklist/Flow, Descriptions On/Off | desktop | 36 tall |
| Segmented buttons | tablets, Menu, airport tabs, Power source | 38 |
| Segmented buttons | PhoneMenu, PhoneWhere | 40 |
| "Full · Detailed" chip | desktop | 36 |
| Emergency | Phone | 40 |
| "Where is this?", "Hide map" | desktop | about 28 to 32 |
| "Do it now" | skipped row | about 16, no padding |
| "Used by Fuel pump switches" | R3 FlightSetup | about 25 |
| "Used by the altimeter step" (inline) | R4 Flight | about 21 |
| "TAF … Show", "Open my OFP", Flight deck step links | R4 | about 33 to 34 |
| Smallest overhead panels | R4 FlightDeck map | 18 to 23 px tall at 1.42 scale |

**Fix:**
- Make every button 44 px tall: the segmented buttons sit at 44 inside a 50 px container, and the "···" and "?" buttons become 44x44. Give links a `min-height:44px` inline-flex box. Phone Emergency gets 44 px.
- The FlightDeck map's geometry stays as is. Give each panel an invisible hit rect padded to at least 44 px where the neighbors allow. Also add a "Panels in this area" list under the map (a disclosure, closed by default) as a second path to each panel. It's keyboard friendly and doesn't change the drawing.

### F16. Light and dark parity is incomplete.
- No light board exists for StartOptions (the `T.light` object is defined but never rendered), StripOpen, the Where pop-over, FlightSetup or anything under 1440.
- R4 Menu exists only in light.
- In light mode the selected states rely almost entirely on weight. The segmented selected chip #e2e6ec on #ffffff is 1.25:1, and the selected nav item #e4ecfd on #f5f7fa is 1.10:1. Dark has the same weakness: #2a323e on #161b23 is 1.34:1.
- Light status icons: skipped #c98a12 on #fff is **2.95** (needs 3). To-do ring #8a93a0 is 3.11, which barely passes. Dark to-do ring #5a6372 on #161b23 is **2.85**.

Text tokens pass in both themes: light muted #5b6472 is 5.05 to 5.57 on all its surfaces, amber #8a5a00 is 5.42 to 5.93, and link #1d4fd8 is 5.85.

**Fix:**
- Render every board in both themes before handoff.
- Add a 1.5 px accent inset ring to selected segmented chips and nav items (#2563eb light at 5.17, #4c8dff dark).
- Light skipped icon `#a86f00` (4.25). Light to-do ring `#7a8391` (3.83). Dark to-do ring `#6b7482` (3.66).

### F17. There's no spacing or type scale. Values drift board to board.
- **Header height:** 60 (checklist) vs 64 (Flight and Flight deck pages).
- **Phase row:** 64 (R3) vs 60 (R4) vs 58 (tablet).
- **Main padding:** 22/40, 20/36, 24/40, 22/32.
- **Aside width:** 540, 520, 470, 420.
- **Flight nav width:** 240 vs 230.
- **Step grid:** first column 44 vs 40, value column 36% vs 34%.
- **Row padding:** 12, 10, 6, with no rule.
- **Type sizes used:** 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 22, 24, 26, 30, 32. That's 15 sizes.
- **h1:** 32 (R3) vs 30 (R4) vs 26 (tablet).
- **Section labels:** 11 vs 12 px.
- **Menu group names:** "TRAIN" (R3) vs "LEARN" (R4).
- **Flight page headers:** the Weather header has Import and Descriptions, Start options has neither.

**Fix:**
- **Spacing:** a 4 px base with steps 4, 8, 12, 16, 24, 32, 40. Rows get 12 vertical padding, panels 16 to 24, page gutters 32 desktop, 20 tablet, 16 phone.
- **Type:** 12 / 14 / 16 / 20 / 24 / 32 (with 15 for mono values if needed), one h1 size per breakpoint.
- **Shared metrics:** one header height (60), one aside width at 1440 (520), one Flight-page nav width (232).
- **Flight page header:** the same right-hand controls on every section, or none.

### F18. Tracked caps are used for UI chrome as well as for cockpit text.
"CHECK", "IF NOT", "NEW PANEL · FORWARD OVERHEAD · P5", "THE CONTROLS · FUEL PANEL", "CLEARANCE", "METAR · THE CURRENT REPORT, AS PRINTED", "FIND IT IN YOUR OFP" and others are set in 12 px caps with 0.06em tracking, the same treatment as panel placards and settings values. For a trainee that blurs what's printed in the airplane and what's app chrome.

**Fix:** reserve caps for text that appears in the airplane or its documents: placards, annunciator legends, CDU page names, METAR groups, and the step setting values. Set app labels in sentence case, 13 px, weight 600: "Check", "If not", "New panel: forward overhead (P5)", "The controls", "Find it in your OFP". That makes caps mean "you'll see this in the flight deck", which is itself teaching.

### F19. The Flight page Weather rows have no CDU destination chips, and the OFP schematic is hard to see in light mode.
**Board:** R4 Main.

Owner decision: the flight plan has CDU destination chips. R3 Fuel and weights had them ("CDU · PERF INIT · ZFW"). The Weather rows have none, though wind and temperature go into the CDU (TAKEOFF REF) and the altimeter goes to the EFIS BARO.

The OFP page schematic blocks are #e2e6ec on #ffffff (1.25:1) in light mode, and the highlighted section is #e4ecfd (1.19:1). Only the 2 px accent ring identifies it. Dark mode has the same problem: #2a323e on #1b212b is 1.25:1.

**Fix:**
- Add chips: Wind "CDU · TAKEOFF REF · WIND", Temperature "CDU · TAKEOFF REF · OAT", Altimeter "EFIS · BARO" (labeled as a control, not the CDU).
- Schematic blocks: light #c3cad4 (about 1.6, decorative), with the highlighted block in solid accent at 30 percent and a 2 px ring.
- Make each Weather row a real button (whole row, `aria-pressed`) so "Select a row below" has an obvious target.

### F20. The R4 Checklist aside lost the setting value. The aside reads as a map caption.
**Board:** R4 Checklist.

R4 shrank "Fuel pump switches" plus its value to a 13 px kicker, "Current step · 6 of 9 · Fuel pump switches". The setting is on screen in the highlighted row. But the aside, where the eyes go to read the drawing, no longer says what to set.

**Fix:** under the kicker, add a single line with the value in 15 px mono (the F12 resolved value, one or two lines). That costs about 24 px, which the budget in F1 allows.

### F21. Designer notes and placeholders are in the UI.
"Scrolls sideways" (TabletLandscape), "Stage names here are placeholders; the agent uses the pack's real Preflight stages." (StartOptions), "Layout and panel drawings come from 3.0's panel data, unchanged." (FlightDeck caption), "Observed 17:53 UTC · sample data" (R4 Main).

**Fix:** move these to the canvas notes. Replace the FlightDeck caption with user copy: "Select a panel to open it. Turn on Name every panel to label them all."

### F22. Smaller issues
- **R4 FlightDeck:** the map figure is fixed at 520 px tall but the scaled map is about 278 px tall, leaving about 240 px empty. Size the figure to the map's aspect ratio and move "Light meanings for this area" up under it as a disclosure.
- **Light meanings swatch:** hard-coded amber #f2bd5c with 9 px text. Raise it to 11 px. The ratio is fine at 8.08.
- **Labels under changed switches:** blue #a9caff or #1d4fd8 ("AFT 1 · ON"). The owner's rule is that values are neutral text. Color the switch drawing only and keep the label neutral (#8b94a3 dark, #4a5462 light).
- **Exterior inspection:** with the toggle off, the 18-stop diagram can't be reached for study. Make Find return the stage, and let StartOptions offer "Preview the inspection".
- **Keyboard hint:** omits Left arrow (back). Use "Space done · ← back · S skip".
- **StartOptions:** `max-width:980px` left-aligns the content while the Weather page fills the width. Pick one rule.
- **Power source:** the inline "Ground power at the gate / No ground power" choice at Power up (3.0 How to use, step 2) isn't drawn. Keep it in the checklist as well as in Start options.

### Is it clearly better than 3.0?
Yes, in these ways: real Done and Skip buttons, the collapsed done group, an explicit skipped state, switch direction, unlit lenses, the one-line location when the map is hidden, and OFP teaching.

Not yet, in these ways:
- The panel drawings are smaller and less legible than 3.0's (F1, F2).
- Open steps across pages (F3), terms (F7), Essentials editing (F4) and the strip's SimBrief import (F11) are gone.
- The tablets are weaker than desktop on reachability (F5).

Until F1 to F5 are fixed, a 3.0 user would call this a regression on the screens they use most.

---

## 3. Where progressive disclosure could go further (nothing mid-step hidden)

1. **Open steps list** (F3): a count that opens a list grouped by stage, instead of hunting.
2. **Terms** (F7): dotted-underline pop-overs plus a one-line "Terms: N1, CDU ⌄". Nothing permanent is added.
3. **How to do it:** expand in place under Why, not as a separate panel. Remember the open or closed state per user, not per step.
4. **Lens taps:** tap any lens in a step drawing to open its light meaning in a pop-over. This brings the 166 light meanings into the step without a trip to the Flight deck page.
5. **Done group:** already collapsed ("4 done · Landing gear pins to Door lights, Show"). Do the same for later phases' checklist sections. Keep the current phase's challenge-response checklist collapsed to a header with its count until the procedure is done, then open it automatically.
6. **Conditional values** (F12): show the resolved line by default, with "Why OFF?" opening the condition and the field link.
7. **Flight strip, open:** on desktop, put Ground, Tower and Departure frequency behind "Frequencies ⌄", as Phone already does. The strip then opens to about 200 px instead of about 300 px and covers less of the step.
8. **Flight page descriptions:** with Descriptions off, keep a 1-line truncated description that expands on row focus. That's better for learners than all or nothing.
9. **"Find it in your OFP":** collapsed, the aside shows only the OFP section order with the selected section highlighted. "Show close-up" reveals the excerpt. That leaves room for the CDU chip explanation.
10. **Flight deck page:** "Panels in this area" list (F15) and "Light meanings for this area" as disclosures under the map.
11. **Phone current step:** collapse Check and If not to their single lines (they're already single lines). Put the "?" explanations in a bottom sheet with a medium detent, so the Done bar stays visible.
12. **Menu:** the long footer note ("Flight setup and exterior inspection: Flight strip › All flight data…") can become a "Where did Flight setup go? ⌄" disclosure, shown only on the first few opens.

---

## 4. Top 5 changes before handoff

1. **Make the panel drawings legible in both themes (F1, F2).** Size SVGs by width with aspect-ratio height, and give the controls figure the free aside height. Render labels at 11 px or more and lens legends at 10 px or more. Change the dark stroke, lens and rest tokens to #6b7482 and #2a303a, and the light stroke to #8a93a0, so every outline, lens and switch is at least 3:1. Layout and proportions are untouched.
2. **Restore the lost 3.0 features at every size (F3, F4, F7, F8, F11).** Open steps across earlier pages, Essentials add/remove plus E on every row, terms a step uses, How to do it in the map-hidden state, and the SimBrief import in the strip, with the offline OFP drop drawn.
3. **Bring the tablets to parity and draw the missing overlays (F5, F6).** Find, step mode, Hide map, How to do it, phase-tab scroll cue. Draw the tablet drawer, strip open state and Where pop-over. Re-anchor the desktop pop-over so Check and If not stay visible. Add the captain-seat view on phone.
4. **Close the coverage gaps with boards (table, F16).** Light mode at every size. Phone and tablet Flight page, Flight deck page and menu with Flight deck plus Reset flight plan. Timers, CDU walkthrough, briefings, Flow view, Emergency, Flight log, Settings, exterior inspection and the Descent approach data, at least at desktop and phone.
5. **Lock a token system and the responsive rules (F14, F15, F17, F18, F9, F10).**
   - **Tokens:** a 4 px spacing scale, six type sizes, one header height and one aside width.
   - **Accessibility:** 44 px minimum on every control.
   - **Responsive:** `scale = max(1, min(w/1440, h/900))` with defined breakpoints below 1440 and a capped content measure on ultrawide.
   - **Wording:** sentence-case UI labels, with caps reserved for cockpit text. One name each for clearing and for the strip fields.
