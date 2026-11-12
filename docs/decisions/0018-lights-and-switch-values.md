<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0018: A value about a switch does not light its lights

Date: 2026-11-11. Status: proposed. Amends [0016](0016-levers-handles-and-lit-lights.md).

## Context
Decision 0016 shows a light without `lit_at` on when the step's value says ON. A step that sets
switches ON, such as the fuel pumps, then showed the pumps' LOW PRESSURE lights as expected on,
though those lights go out when the pumps run.

## Decision
- A light without `lit_at` shows on from the step's value only when that value names no position
  of a switch the step uses. "ALL ON" for the fuel pumps is about the switches; "VALVE LIGHTS ON"
  for a step with only lights still lights them.
- `lit_at` is unchanged: it names exactly which position lights a light.

## Consequences
- Drawings no longer show a light on that the step's switches turn off.
- A light that a switch turns on needs `lit_at` to show on, as the parking brake light has.
