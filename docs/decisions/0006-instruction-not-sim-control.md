<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0006: Teach procedures and systems; no connection to the simulator

Date: 2026-11-08. Status: accepted. Supersedes [0004](0004-sim-link.md).

## Context
A sim link (reading cockpit switches to tick steps automatically) would need SimConnect,
which only works on Windows, raises license questions about Microsoft's DLL, depends on
each aircraft developer's SDK terms, and breaks when the sim changes. Consoles (Xbox,
PS5) allow no outside connection at all. Other add-ons already compete on automation.

## Decision
- Chock to Chock does not connect to the simulator. It is a training app: checklists,
  flows, memory items, systems explanations and practice modes.
- The `sim-link` crate and the step `when:` sim condition were removed. Nothing in the
  code or packs reads simulator variables.
- The effort goes into instructional quality, including how the aircraft's systems work
  and interact.

## Consequences
- Works the same for PC, Mac, Xbox and PlayStation pilots, and later on phones and tablets.
- No SimConnect, DLL or vendor-SDK licensing questions.
- No automatic ticking; pilots tick steps themselves, which is also how training works.
- If a sim link is ever wanted again, 0004 records the earlier design.
