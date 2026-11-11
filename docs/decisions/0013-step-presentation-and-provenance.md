<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0013: Every step presented the same way; provenance kept as an internal record

Date: 2026-11-11. Status: accepted. Amends [0007](0007-pack-content-model.md).

## Context
Decision 0007 gave steps and panel light notes a `status` of `draft` or `checked`, and the
Phase 2 brief planned a "not yet verified" marker on draft steps. A step that is in the pack
belongs there: showing some steps as less trustworthy than others confuses pilots and suggests a
precision the project cannot offer. What the maintainer needs is a record of what each step was
confirmed against, for maintenance, not a label in the app.

## Decision
- **Every step in the pack is presented the same way.** There is no draft status and no
  verification badge or marker anywhere in the interface. Practice and the reference screens use
  every step.
- **Provenance is an internal maintenance record, never shown in the interface:**
  - Each list keeps its required `basis` (0007): the kinds of material the list was checked
    against.
  - A step or panel light note may carry an optional `confirmed_by` record: a list of entries,
    each with a confirmation `kind` and the `date` it was recorded (YYYY-MM-DD). It never says
    who confirmed it or which document; those details go in the maintainer's private log.
  - `pack check` never treats a missing record as an error or a warning.
  - Both stay in the flattened pack, as part of the data, but the interface never displays them.
- **Confirmation kinds** are an open list. Each kind is defined in one line here before it is
  first used, and is recorded only for what it honestly is:
  - `vendor-docs`: matches the aircraft developer's documentation for the simulated aircraft.
  - `regulator-docs`: matches an official public document from an aviation authority, such as
    an FAA or EASA airworthiness directive, accident report or advisory circular.
- Later kinds, such as `pilot-review`, `sim-observed` or `user-report`, are added to this list
  with their one-line meaning when first needed.
- `draft` and `checked` are retired. Every step marked `checked` under 0007 matched the vendor
  documentation, so it becomes one `vendor-docs` entry dated when it was marked `checked`. A
  `draft` step gets no record.
- The private open list stays a list of content to improve. It labels nothing in the app.
- The earlier "cross-checked" and "verified tag" ideas are not adopted.
- Users can report a problem with a step from the app, with no backend (Phase 2 brief).

## Consequences
- The model loses `Status` and gains `Confirmation` and `ConfirmationKind`; schemas, bindings,
  validation, the converter, snapshots and docs follow. Step ids do not change.
- Adding a kind means a line here and a value in the model, reviewed like any change.
- Unlike 0007's consequences, pilots no longer see a list's basis in the app; it and the
  confirmation records are for maintainers and reviewers of the public repository.
