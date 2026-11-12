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
- **The saved plan** keeps three things, under its own storage key and format number: the parsed
  plan (the fields the app uses, as a Rust type), SimBrief's raw plan file as fetched or loaded,
  and the plan's printable OFP document, for reading offline. With them go the time SimBrief made
  the plan and the time it was loaded, for its age.
- **Importing a plan overwrites** the flight values the plan covers, as 3.0 did. A notice says
  how many were filled and asks the pilot to check the runway and departure against the clearance,
  with Undo.
- 3.0's iFly panel-state picker on the Start card arrives with panel states in batch 3.

## Consequences
- The flight's saved format does not change: the new fields are new keys in `values`.
- The saved plan can be large (the raw file and the OFP document); it lives apart from the flight,
  so a flight never depends on it.
