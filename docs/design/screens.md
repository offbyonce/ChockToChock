# Screens

Board names refer to files in `boards/`. Every screen exists in dark and light.

## Desktop checklist (`Checklist`, `ChecklistLight`)

- **Header (60):** menu, app name and aircraft, stage tabs (Preflight to Post-flight) with an amber dot on any stage holding an open step, Find (Ctrl K), Checklist / Flow segmented control, Emergency (red).
- **Phase track (60):** phase tabs with progress bars (done green, current blue ring with count and amber dot when it has an open step), Full · Detailed mode pill, overall count (41/366), Saved time.
- **Flight strip (48, closed):** route, Runway, SID, Initial altitude, Squawk, Departure frequency (dash when empty), Notes count, chevron. The whole strip opens the strip panel.
- **Main column (content max 1040):** phase label and title, "2 open steps ⌄" (opens the open-steps list), the procedure card:
  - header with done and skipped counts;
  - collapsed done group with Show;
  - skipped row: amber dashed icon, Skipped pill, Do it now;
  - **decision step** (Center tank fuel): CHECK QUANTITY with the answer "1,000 lb (450 kg) or less", answered from the SimBrief plan and changeable;
  - **current step:** title, E marker, location, resolved value ("MAIN PUMPS ON · CENTER PUMPS OFF") with a link to the decision that set it, Done, Skip for now, keyboard hint (Space done · ← back · S skip), row menu;
  - to-do rows with E markers; a value still needing data shows "— enter takeoff trim" linking to its Flight field;
  - **row menu:** Make this the current step, Mark done, Skip for now, Add to / Remove from Essentials.
- Below the procedure: the phase's challenge-and-response checklist, collapsed with its count until the procedure is done.
- **Side pane (520):** step label and value, Check, If not, Why with How to do it ⌄ and Terms ⌄, map figure (on a panel change) with New panel, area name, position text, Where is this?, Hide map, then the controls drawing taking the remaining height, with legend "Blue: set by this step · lights drawn unlit · tap a light for its meaning".
- **Map hidden (`MapHidden`):** the map figure becomes a one-line location with Where is this? and Show map. Nothing else moves out.

## Desktop overlays

- **Open steps (`OpenSteps`):** pop-over from "2 open steps". Every open step from every stage, grouped by stage and phase, each with its setting and Go to step. Footer: skipped steps stay open; the Flight log lists any left at the end.
- **Where is this? (`WherePop`):** pop-over that opens leftward over the step list so the side pane stays visible. Title "Where is the forward overhead panel?", the plain-language line, the side view (sight line from the seat) and the captain-seat view (area highlighted, no panel detail), a legend, and Open in Flight deck.
- **Flight strip open (`StripOpen`):** drops below the strip. Clearance fields (Runway, SID, Transition, Initial altitude, Squawk), Frequencies ⌄ (ground, tower, departure), Notes from ATC, then a footer: Import latest from SimBrief, where the values came from, Reset flight plan, All flight data ›.

## Wide (`Wide`, `WideLight`)

The desktop layout at 1600 x 900 scaled 1.6, filling 2560 x 1440. Same layout, larger.

## Flight pages

Shared header: Checklist back button, title and route, SimBrief import time, Descriptions On / Off. Section list (232): Overview, Route, Fuel and weights, Cruise and levels, Weather, Clearance, Takeoff (with "2 to enter"), Approach, then Start options and SimBrief and OFP file.

- **Weather (`Weather`):** airport tabs (departure, destination, alternate, en route winds), the METAR as printed with the selected group highlighted, decoded rows as buttons (value, raw group, description, destination chip such as CDU · TAKEOFF REF or EFIS · BARO KNOB, ?), TAF disclosure. Side panel "Find it in your OFP": the section order with a schematic page and the weather section highlighted, a close-up of the printed line with the group marked, the note that order depends on the SimBrief layout, and Open my OFP on SimBrief (opens SimBrief; nothing stored).
- **SimBrief and OFP file (`SimBrief`):** username field and Import latest flight, what is and is never brought in, offline drop zone "Drop a saved OFP file here" with Choose file (read once, only flight data kept), last import, Reset flight plan.
- **Start options (`Start`):** Exterior inspection toggle with Preview the inspection, Power source (also asked at Power up), iFly panel state (Cold & Dark, GPU Connected, On GRD Power, On APU Power, Before Engine Start, Ready to Taxi, On Runway), Turnaround toggle.
- **Approach (`Descent`):** approach type (ILS, RNAV, Visual) that changes the Approach steps, and rows for arrival runway, STAR, minimums, transition level and landing speed, each with a description and destination chip.
- Descriptions Off keeps label, value and chip; ? keeps the longer note.

## Flight deck reference (`Deck`) and menu (`Menu`)

- **Flight deck:** area list with the side-view locator (Aft overhead, Forward overhead P5, Glareshield P7, Main panel P1 to P3, Forward electronic P9, Control stand, Aft electronic P8); the area map, unlabeled except the selected panel, with Name every panel; Panels in this area as a list (keyboard and small-target route); Light meanings for this area; the selected panel at resting positions; light meanings; steps that use the panel.
- **Menu (drawer, 360):** Steps (Full / Essentials, Detailed / Simple), Learn (Practice, How procedures work), Reference (Flight deck, Glossary, References), This flight and the app (Flight log, Settings, Take the tour), then, behind a divider at the bottom, Reset flight plan with Undo note. Flight setup is reached from the strip, not the menu.

## Other views (`Views` with `view` prop)

- **Flow:** step position and progress, open-steps link, 48 px title, 32 px value, Check and If not side by side, Why with How to do it and Terms, Done next step (56 tall), Back, Skip for now, keyboard hint, Next preview. Side pane: location line and the controls drawing.
- **Emergency reference:** red-tinted header, Memory items / Quick reference / Non-normal tabs, item list, the selected item's numbered steps, Then open the quick reference checklist, Practice these from memory. Titles and steps come from the pack unchanged.
- **Step timer:** a timer chip on the step row (ring and time left), started automatically by the step that triggers it, Pause, Restart, chime at zero, and a running-timers bar that persists across pages. Covers engine warm-up and cool-down, the APU wait and IRS alignment.
- **CDU walkthrough:** page progress (IDENT to PRE-FLT COMPLETE), a schematic CDU with the user's values filled in and the key to press highlighted, the entry instruction ("Type 121.4, then press 3L"), where the value came from with Change it, Entered next and Back. Opened from the CDU step and returns there.
- **Briefing:** a briefing step in the flow with prompts and the flight's values filled in, Briefed and Skip for now. The approach briefing works the same at Descent.
- **Exterior inspection:** top view of the airplane with the 18 numbered stops at the 3.0 positions (done, current, to do), the current stop's name, where, Check list, the every-stop reminder, Done next stop, Skip for now, Next.
- **Flight log:** this flight's timestamped ticks with open steps marked, Show every tick, earlier flights, and actions: Log flight and start new, Save copy, Export CSV, Export JSON, Load log file, Print or save as PDF.
- **Settings:** Appearance (System, Light, Dark), Text and controls size, Map in steps (Always, New panel, On request, Off), Timer chime, Back up and Restore, Clear all progress.

## Tablet (`Tablet` with `orient` and `overlay` props)

- **Landscape (1180 x 820):** header with stage picker (amber dot), Find (44 icon button), Checklist / Flow, Emergency; phase tabs that scroll sideways with a fade and a chevron, plus the mode pill; strip; list with E markers and the current step's Done and Skip; side pane (440) with value, Check, If not, Why, location line with Where is this? and Hide, controls drawing.
- **Portrait (820 x 1180):** no side pane. The current step expands in place: Check and If not side by side, Why, location line, controls drawing (max 480 wide). Done and Skip in a bottom bar.
- **Overlays:** Where is this? pop-over (side view, captain-seat view, area map), strip panel, menu drawer (with saved time and overall count in its header).

## Phone (`Mobile` with `screen` prop)

- **Checklist:** header (menu, stage and phase picker with amber dot, Find, Emergency), strip (route, squawk, notes), progress bar with count and "2 open", list with the current step expanded: title, E, value, controls drawing, Check, If not, Why with How to do it and Terms, location line opening Where is this?. Bottom bar: Done, Skip for now.
- **Open steps:** bottom sheet, grouped by stage.
- **Where is this?:** sheet with the plain-language line, side view, captain-seat view, a link to the panel map in Flight deck, and the step's Check pinned at the foot.
- **Flight strip:** sheet with the fields, Frequencies ⌄, notes, Import latest from SimBrief, Reset flight plan, All flight data ›.
- **Menu:** drawer with Checklist / Flow, Full / Essentials, Detailed / Simple, the same groups as desktop, and Reset flight plan at the bottom.
- **Flight page:** section picker, airport tabs, the METAR, decoded rows (the selected one shows its description and chip), Find it in your OFP as a disclosure.
- **Flight deck:** area picker, area map, Panels in this area, the selected panel, light meanings, steps that use it.
