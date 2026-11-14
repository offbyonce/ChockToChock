<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0024: Outside flight data, starting with SayIntentions (Beta)

Date: 2026-11-13. Status: accepted (owner, 2026-11-13). A phase after Phase 2 step 7; not started.

## Context
SayIntentions, an AI ATC add-on for the simulator, publishes a local `flight.json` on the pilot's
computer during a flight, with the ATC-assigned runway, SID, STAR, gate and clearance status.
Reading it lets the strip and the Flight pages follow what ATC assigned, and flags what differs
from the plan or from what the pilot briefed. Nobody on the project has a SayIntentions account,
so the integration cannot be tested against a live flight. Decision 0006 (the app never connects
to the simulator) stands: SayIntentions is an add-on's file, not the simulator.

## Decision
- **A separate module, versioned on its own, outside the engine.** A provider-neutral Rust crate,
  `flight-sources`: a value from an outside source (value, source, time), the merge rule into the
  flight, and change detection against the plan and against what was briefed. It has no network
  code and names no provider.
- **One adapter per provider in the web layer,** first `packages/source-sayintentions`. It reads
  only the flight fields of the local `flight.json` (runway, SID, STAR, gate, clearance status),
  tolerates missing, renamed or extra fields, and never stores or sends the pilot's api_key
  (which the file contains) or anything personal. No developer key is used or committed, so
  SayIntentions' cloud API is out of scope. Later adapters plug into the same crate.
- **ATC's clearance always wins.** An outside value replaces the planned value and the pilot's own
  entry, shown as "From ATC · time" and "Changed from X" each time. The pilot may still edit the
  field; the next ATC value replaces the edit again.
- **A change to a briefed item reopens the briefing step** as an open step, everywhere open steps
  show, with "Re-brief: N changes", "Changed since you briefed: was X" on each changed row and
  "Briefed again". The strip shows a dismissible amber notice; the open step is not dismissible.
- **Source tags are neutral grey:** "ATC · time", "From your plan", "Your entry", "FMC".
- **Beta, off by default:** Settings › Outside data, "Use SayIntentions ATC data (Beta)", with a
  note that it has not been tested against a live SayIntentions flight, a feedback link, and "Copy
  what Chock to Chock read" (only the flight fields used). With SayIntentions not running, the
  checklist works exactly as without it.

## Consequences
- The integration is unverified until users confirm it; the Beta label stays until they do.
- The engine and the pack format do not change; outside values enter the flight through the
  crate's merge rule.
- A GitHub issue template for SayIntentions feedback, warning people to remove their api_key
  before pasting `flight.json`, is drafted for the owner to add.
