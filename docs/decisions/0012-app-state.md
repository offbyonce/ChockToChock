<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0012: App state for the web beta: one flight, rules in Rust, skipped stays open

Date: 2026-11-11. Status: accepted. Builds on [0010](0010-web-build-tooling.md).

## Context
Phase 2, step 7 builds the interface for 3.0 parity: phases, checklists and flows, options,
Essentials, iFly panel states, flight values and the flight strip, SimBrief, timers, the CDU
walkthrough, practice and the flight log. All of these follow rules (which steps show, what
counts as done, when a timer starts, what a CDU line says) that belong in `crates/`, where they
are tested, not in the interface. Two rules today are not what the interface needs: a skipped
step counts as finished, and ticks carry no time.

## Decision
- **Skipped stays open.** `checklist-core` reports progress as done, skipped and to do. A list is
  complete only when every counted step is done; a skipped step is still open. Each step's state
  is one of to do, current (the first step that is neither done nor skipped), done or skipped.
  Steps the simulator does not model (`not_simulated`) are shown and can be ticked, with a short
  "not modeled in the simulator" note, but do not count toward progress or completion.
- **Ticks and skips record a time.** `Session` keeps, for each done or skipped step, the time it
  happened, in milliseconds since 1970 as passed in by the caller. The core still has no clock.
- **One flight, as a Rust type.** `app-core` holds a `Flight`: the pack id and version, the
  pilot's options (power source, start type, approach), the panel state started from, the
  session, flight values (keyed by the app's fill fields, from the pilot or from SimBrief), the
  pilot's blank entries, clearance notes, and timers started by hand. `Settings` holds what
  outlives a flight: theme, dimming, default detail level, list view, Full or Essentials and the
  pilot's changes to Essentials, the flight deck map, the timer chime, the SimBrief username and
  whether to load the latest plan on opening. A `Logbook` keeps the flights the pilot has filed
  (built in step 7, batch 7). The last SimBrief plan is saved on its own with the time it was
  loaded, for offline use and its age; importing it copies its values into the flight, so a
  flight never depends on it. All of these are kept on the device only, and step 8's export uses
  the same types.
- **Saved-data formats.** Each saved type carries a format number. An older format is migrated
  forward automatically when it is read, and every migration has tests. A newer or unreadable
  format is never silently discarded: the app keeps it as it is, tells the pilot, and offers to
  save a copy (the export, once step 8 adds it).
- **Changes are actions.** The interface changes a flight only by sending an action (tick, untick,
  skip, set an option, start from a panel state, set a value or a blank, import a plan, start a
  timer, clear). `app-core` applies it and returns the new flight, so rules such as "ticking a
  skipped step un-skips it" live in one place.
- **Rules live in `app-core`,** as functions of the pack, the flight, the settings and the time:
  which phases and steps show (options, Essentials, the pilot's changes); each step's state, the
  current step, progress and completion; open steps on earlier phases; what a panel state ticks
  and skips; filled values, including calculated ones; hints; timer state; CDU page text; practice
  questions; and friendly SimBrief errors.
- **The boundary.** The WebAssembly module keeps each loaded pack in memory, so the pack crosses
  once. Flights, settings, actions and views cross as JSON, with types from ts-rs. The interface
  keeps the current flight and settings, passes them in with each call, and saves what comes back
  through the host's storage. It may read the pack directly for drawings and reference text, but
  never decides a rule itself.

## Consequences
- More of `app-core` is tested in Rust, and the interface stays a view of what Rust returns.
- `Session`'s saved shape changes. Nothing saved has shipped, so no migration is needed; the
  parked `crates/store` needs its tests updated if it is revived.
- Every call passes the flight (tens of kilobytes at most); views are computed for the screen in
  view, not the whole pack.
- Undo (for example after clearing a flight) is the interface keeping the previous flight.
