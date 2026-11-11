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
- **0008's out-of-scope table** gains a full EFB and video courses, and its row for anything
  needing a sim connection now also names automatic checking of cockpit switches, pointing to
  decision 0006.

## Consequences
- The fuel module may show calculations, but only on the pilot's plan or on clearly invented
  training numbers, labeled as training, never as planning advice.
- Fuel rules are facts from regulators' published rules, checked as any content is (decision
  0007), and written in our own words.
- Requests for takeoff or landing performance stay declined, with the reason in 0008.
