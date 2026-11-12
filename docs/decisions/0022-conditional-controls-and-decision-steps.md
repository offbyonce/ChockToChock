<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0022: Conditional controls and decision steps

Date: 2026-11-12. Status: proposed. Replaces the "SEE STEP" caption in
[0018](0018-lights-and-switch-values.md); the rest of 0018 stands. Builds on the options of
[0014](0014-pack-format-3.md).

## Context
Under 0018 a target whose position depends on a condition says `open: true`, and the drawing shows
the knob between positions with the caption "SEE STEP". The fuel pump step is the main case: its
center pumps are ON only with more than 1,000 lb (about 450 kg) in the center tank. The approved
design ([rules.md](../design/rules.md) §5, [content-to-validate.md](../design/content-to-validate.md))
draws such a control with a dashed outline while the condition is unknown, and in its resolved
position once the pilot has answered a decision step.

## Decision
- **Conditional controls.** A target with `open: true` is drawn with a dashed accent outline and
  no position, and the drawing's legend reads "dashed: only if the step's condition applies". The
  caption "SEE STEP" goes.
- **Decision steps.** A step may `decide` an option (the options of 0014 in `phases.yaml`, with
  their values, choices and labels). The step shows the option's choices in its row; the pilot's
  answer sets the option for this flight and ticks the step. An option may name a flight field
  that `suggests` its answer; the app then offers that answer and the pilot confirms it. Until the
  pack has a sourced way to work the answer out, there is no suggestion and the pilot answers.
  The center tank step needs a sourced wing-tank capacity before it suggests anything.
- **Targets that depend on a decision.** A target may carry `when: { option, equals }`, the same
  shape as a step's `show_if`. A step may list several targets for one item as long as their
  `when` conditions do not overlap. While the option is unanswered, the item is drawn as
  conditional (above); once it is answered, the matching target is drawn resolved, as an ordinary
  `set`, and the step's value shows the resolved setting with a link to the decision that set it.
- **Checks.** `pack check` holds a `decide` to a real option, `when` to a real option and one of
  its values, the targets for one item to non-overlapping conditions that, when not all values
  are covered, leave an `open: true` fallback; and the wording rules of 0018 to each target.
- **Pack format 4.** Older apps must refuse a pack they would misread (0014). A format 3 app would
  draw the fuel step without its resolution and show the decision step as an ordinary step, so
  `SCHEMA_VERSION` becomes 4. Packs at 3 still load. All new fields are optional.

## Consequences
- The fuel pump step can show "MAIN PUMPS ON · CENTER PUMPS OFF" and draw the center switches at
  OFF once the center tank question is answered.
- The center tank step and its wording are content: they are sourced and checked under 0007 before
  they ship (Phase 2 step 7, batch 7).
- Options are no longer asked only above a phase's lists; a step can ask one too.
