<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0011: Scope boundaries for fuel training and the out-of-scope list

Date: 2026-11-11. Status: accepted. Amends [0008](0008-platform-scope.md).

## Context
Phase 3 now includes a flight plan and fuel module: a walkthrough of the pilot's own SimBrief
plan, fuel and weight drills, find-the-error exercises, and the difference between EASA and US
fuel rules. Decision 0008 rules out performance calculations, because a wrong number teaches
something dangerous. Fuel and weight drills involve arithmetic, so the line between the two needs
to be drawn. The roadmap also named more things the app will not build than 0008's table lists.

## Decision
- **Allowed, for teaching:** fuel and weight drills and comparisons of fuel rules, when they use
  the pilot's own plan figures (from their SimBrief plan) and published fuel rules. Examples: how
  the fuel blocks add up, how zero-fuel, takeoff and landing weights relate, and how reserves
  differ between rule sets. The app explains and checks the plan's figures; it never produces
  figures for a real flight.
- **Still out:** takeoff and landing performance calculations (speeds, thrust, flap settings,
  runway limits), as in 0008.
- **Out of scope.** This table replaces the one in 0008, which stays as it was accepted. It adds a
  full EFB and video courses, and names automatic checking of cockpit switches under the sim
  connection row.

| Not building | Why |
| --- | --- |
| Performance calculations | Takeoff and landing numbers depend on data we do not hold, and a wrong number teaches something dangerous. SimBrief and the aircraft's own tools already produce them; we teach how to use them. Fuel and weight teaching is allowed as above. |
| Charts and navigation data | They are licensed and change every cycle, and dedicated providers do this well. We use only what the flight plan or the pilot supplies. |
| A full EFB | An EFB bundles charts, navigation data and performance tools, which this table already rules out, and dedicated tools do it well. We teach how to use the aircraft's own EFB instead. |
| ATC | The simulator, online networks and dedicated tools already provide it. Radio phraseology and ATC interaction are left to those tools. |
| Virtual first officer or copilot voice features | To work well they must follow the airplane's state, which needs a sim connection, and sim-linked tools do it better. We teach both crew roles instead. |
| Anything needing a sim connection, including automatic checking of cockpit switches | Decision 0006: the app must be usable alongside the simulator on any platform, including Xbox and PlayStation. |
| Video courses | Others make them, and this app's strength is interactive practice with the reasons behind each step. |

## Consequences
- The fuel module may show calculations, but only on the pilot's plan or on clearly invented
  training numbers, labeled as training, never as planning advice.
- Fuel rules are facts from regulators' published rules, checked as any content is (decision
  0007), and written in our own words.
- Requests for takeoff or landing performance stay declined, with the reason in 0008.
