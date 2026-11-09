<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0008: Learn to operate the airplane like a trained crew

Date: 2026-11-09. Status: accepted.

## Context
The app grew out of an interactive checklist. Checklists alone do not teach a pilot how a crew
works: how to brief a flight, who does what, what to do when something fails, or how to manage an
approach. Other tools already cover much of what surrounds a flight in the simulator, and decision
0006 rules out a connection to it. We need a clear line between what this app teaches and what it
leaves to those tools.

## Decision
The scope is learning to operate the airplane like a trained crew, not just checklists. Phase 2
parity with 3.0 still ships first. Phase 3 then adds:

- **Briefings built from the SimBrief plan:** threats, runway, departure and what-ifs.
- **Crew roles:** pilot flying and pilot monitoring duties, standard callouts, and
  challenge-and-response, taught within flow practice and briefings.
- **Non-normal scenarios** built on the memory items.
- **A technique layer:** descent planning, energy management, stabilized-approach criteria and
  autopilot mode use.
- **A structured course** with spaced repetition.

Out of scope:

| Not building | Why |
| --- | --- |
| Performance calculations | Takeoff and landing numbers depend on data we do not hold, and a wrong number teaches something dangerous. SimBrief and the aircraft's own tools already produce them; we teach how to use them. |
| Charts and navigation data | They are licensed and change every cycle, and dedicated providers do this well. We use only what the flight plan or the pilot supplies. |
| ATC | The simulator, online networks and dedicated tools already provide it. Radio phraseology and ATC interaction are left to those tools. |
| Virtual first officer or copilot voice features | To work well they must follow the airplane's state, which needs a sim connection, and sim-linked tools do it better. We teach both crew roles instead. |
| Anything needing a sim connection | Decision 0006: the app must be usable alongside the simulator on any platform, including Xbox and PlayStation. |

## Consequences
- Phase 3 is larger, so each feature still needs review with beta pilots before it ships.
- Briefings can only use what the plan and the pilot provide. What-ifs prompt the pilot to think;
  they do not compute performance or read charts.
- All new content follows the rules in decision 0007, including the overlap check and
  maintainer review.
- Crew-role training has no simulated partner. The pilot practices both roles against the app.
