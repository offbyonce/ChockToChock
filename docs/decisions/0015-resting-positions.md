<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0015: Resting positions for controls a step does not use

Date: 2026-11-11. Status: accepted. Extends [0014](0014-pack-format-3.md).

## Context
Control drawings showed a position only for the controls a step uses: the step's target. 3.0
also drew the other controls on the panel at a resting position (CAB/UTIL at OFF, the meter
selectors with a pointer), which gives context. The app has no simulator link (decision 0006),
so a resting position is a fact about the airplane, not the simulator's state, and it must come
from pack data rather than code.

## Decision
- A panel item may have an optional `rest`: where it rests when the airplane is cold and dark.
  It must be one of the item's `positions`. Pack format 3 (0014) covers it; no version change.
- Drawings show a control the step does not use at its resting position, dimmed: a hollow,
  muted knob or pointer. The step's own target stays solid in the accent color, so the two never
  look alike. A control without a `rest`, such as a spring-loaded switch, shows no position.
- `pack check` requires resting positions to agree with the cold and dark start state (the start
  state that opens on the first phase of the first stage): every step that state counts as done
  must name, as its target, the resting position of each control it uses.
- The iFly pack's resting positions follow 3.0's defaults, corrected to 3.0's measured cold and
  dark panel state. The check also found two steps about the electric hydraulic pumps that
  pointed at the engine pump switches; both now point at the electric pumps.
- Later (Phase 2, step 7, batch 3): draw each control at its expected position for the start
  state and the steps the pilot has confirmed, still never as live simulator state.

## Consequences
- Drawings carry the context 3.0 had, from data the pack can correct.
- A resting position that contradicts the cold and dark start state fails `pack check`.
