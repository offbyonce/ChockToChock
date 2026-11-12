<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0018: Which lights a step points at, and when they show on

Date: 2026-11-11. Status: proposed. Amends [0016](0016-levers-handles-and-lit-lights.md).

## Context
Decision 0016 shows a light without `lit_at` on when the step's value says ON. A step that sets
switches ON, such as the fuel pumps, then showed the pumps' LOW PRESSURE lights as expected on,
though those lights go out when the pumps run.

## Decision
- A light filed under the same entry as the switches a step sets is part of that step only when
  the step speaks of lights (LIGHT, LIGHTS, or the light's name) or expects it on. Setting the
  fuel pumps ON points at the pump switches, not their LOW PRESSURE lights; "ELT SWITCH: GUARD
  CLOSED, LIGHT OUT" keeps the ELT light. A step about lights alone keeps them all.
- A light without `lit_at` shows on from the step's value only when that value names no position
  of a switch the step uses, and a value about several lights is read clause by clause: "VALVE
  LIGHTS ON, FILTER BYPASS OUT" shows the valve lights on and FILTER BYPASS out.
- A switch's position is read from the value with its lights' names taken out, so "PASS OXY ON
  LIGHT OUT" does not read as the passenger oxygen switch at ON.
- A step that names some of its controls (TERR on the EFIS panel) points at those only, and the
  lights it speaks of or expects on.
- `lit_at` names exactly which position lights a light. The iFly pack gives it to the three green
  gear lights (gear lever DN), SPEED BRAKE ARMED (speedbrake ARMED) and PASS OXY ON (passenger
  oxygen ON). The window heat ON lights get none: they can cycle off when the window is warm.

## Consequences
- Drawings highlight what a step acts on or checks, and never show a light on that the step's
  switches turn off or that the step says is out. Tests check every step in the pack for both.
- A light that a switch turns on needs `lit_at` to show on.
