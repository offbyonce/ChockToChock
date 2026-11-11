<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0014: Pack format 3: option choices, map regions and target positions

Date: 2026-11-11. Status: accepted. Extends [0002](0002-pack-format.md).

## Context
On the batch 0 review, three things 3.0 showed were missing because the pack could not express
them, and the engine must not name an aircraft: the option cards ("Ground power at the gate" with
a sentence under it), the orientation captions and panel codes on the flight deck map ("AFT,
TOWARD THE CABIN", "P5") with "Where is this?", and controls drawn at the position a step asks
for.

## Decision
- **Pack format 3.** `SCHEMA_VERSION` becomes 3. Packs at 2 still load; an app that understands
  only 2 refuses a pack at 3 instead of misreading it. All new fields are optional.
- **Options** (`phases.yaml`) may have a `description`, `choices` and `ask_in`:
  - `choices` give each value a `label` and an optional `description`. When present, every value
    has exactly one choice.
  - `ask_in` lists the phases that show the choice above their lists. Choices with descriptions
    show as cards, others as a row with a segmented control, as 3.0 did.
- **Overview regions** (`controls.yaml`, under `overview`): each has a `name`, an optional `code`,
  a rectangle in the overview's own layout, optional `top`, `bottom`, `left` and `right` captions,
  and an optional `locate` sentence (where to look from the seat). A map of a zone shows the
  areas whose centers fall in the first region that holds the zone's area. Names are unique and
  each region lies inside the overview.
- **Target positions** need no new field. Rust reads them from the step's `expect` and each panel
  item's `positions`: the target is the one position the value names as a whole word. If it names
  none or more than one, no position is drawn. Drawings label it as the target ("SET TO ON"),
  never as the simulator's state (decision 0006).
- **Step numbers** shown in lists are positions among the steps that show, computed in Rust. The
  pack's optional `number` is not used for display.

## Consequences
- Wording for choices and regions lives in the pack, in our own words, and can be translated or
  changed without touching code.
- The iFly pack moves to format 3 and fills in all three options and six regions.
- Validation, schemas, bindings and the pack snapshots cover the new fields.
