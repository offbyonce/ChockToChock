<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0016: Levers, handles, and lights a step expects on

Date: 2026-11-11. Status: accepted. Extends [0014](0014-pack-format-3.md) and
[0015](0015-resting-positions.md).

## Context
The importer typed the parking brake, the fire handles and the start, speedbrake, gear and flap
levers as guarded toggle switches; 3.0 drew them as handles and levers by renaming them in its
code. And when a step sets a control whose light shows that setting (the parking brake and its
light), nothing in the pack said the light should show on: the step's value names the control's
position, not the light.

## Decision
- Panel items may be a `lever` (moved along a slot) or a `handle` (pulled). Pack format 3; no
  version change. The iFly items are re-typed and unguarded.
- A light may have `lit_at`: the position, on another item it shares a control with, that turns it
  on. When a step asks for that position, the drawing shows the light on, filled in its color,
  with "EXPECTED ON" under it: what the step expects, never the simulator's state (0006). A
  light without `lit_at` shows on when the step's value says so (ON, LIT, ILLUMINATED), as 3.0
  read it. `pack check` requires `lit_at` to name such a position.
- Lights are all one size (50 by 20 units, as in 3.0), their words wrapped inside, so lights side
  by side line up.
- The iFly parking brake's positions are RELEASED and SET (resting at SET cold and dark, as
  measured), and its light is lit at SET.

## Consequences
- Drawings match 3.0's kinds of controls and show the lights a step expects on.
- Which light follows which control is data the pack can correct, light by light.
