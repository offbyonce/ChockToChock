<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0019: Flight data: values, units, saved plan and import

Date: 2026-11-11. Status: proposed. Extends [0012](0012-app-state.md).

## Context
Batch 2 brings Flight setup, the flight strip, values in steps and SimBrief loading. The flight
already has room for values, blank entries and notes (0012), but nothing sets or shows them, the
unit choices 3.0 offers have no place, and 0012 leaves open what the saved plan holds and what an
import does to values the pilot typed.

## Decision
- **Unit choices belong to the flight.** Two fill fields join `weight_unit`: `qnh_unit` (`inHg`
  or `hPa`) and `minimums_type` (`BARO` or `RADIO`). A SimBrief plan sets `weight_unit` (`LB` or
  `KG`) for its flight. Each is checked against its allowed values when set.
- **Actions.** `setValue` (a fill field; an empty value removes it), `setBlank` (a step's n-th
  blank; empty removes the entry) and `setNotes`. `clear` keeps its meaning. Undo after Clear
  flight or an import restores the flight the interface held before; Rust needs no undo action.
- **Values are resolved in Rust.** Each step row in the phase view carries the text for each of
  its blanks (the pilot's entry for that blank, else the linked flight value, else empty) and its
  hint, built as 3.0 built it: parts with no value are left out, the rest joined with " · ".
- **The saved plan is compact and structured,** built in Rust from SimBrief's data under its own
  storage key and format number. It keeps the fields the app uses now and the sections a future
  flight plan view needs: the route, the navlog fixes (with airway, altitude, wind and fuel), the
  fuel blocks, weights, times, alternates, and METAR and TAF. It keeps the plan's identity, the
  time SimBrief made it and the time it was loaded (for its age). Personal fields (pilot name,
  pilot ID) are left out. SimBrief's raw file and its printable OFP document are neither stored
  nor shown. A plan file the pilot drops is parsed the same way and then discarded. Its size is
  measured with a real long-haul plan and kept compact.
- **A later flight plan view** (phase 3's flight plan module) rebuilds the plan in our own layout
  from this data, with training insights, and works offline.
- **Importing a plan overwrites** the flight values the plan covers, as 3.0 did. A notice says
  how many were filled and asks the pilot to check the runway and departure against the clearance,
  with Undo.
- 3.0's iFly panel-state picker on the Start card arrives with panel states in batch 3.

## Consequences
- The flight's saved format does not change: the new fields are new keys in `values`.
- The saved plan lives apart from the flight, so a flight never depends on it, and holds no
  personal fields and none of SimBrief's own text.
