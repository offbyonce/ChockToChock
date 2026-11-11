<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# Decision records

One short file per architecture decision: what we chose, why, and what we gave up.
Never rewrite an accepted record; add a new one that supersedes it.

| # | Decision | Status |
| --- | --- | --- |
| [0001](0001-app-stack.md) | Tauri 2, a Rust core and a Svelte 5 interface | Accepted; delivery amended by 0009 |
| [0002](0002-pack-format.md) | Packs authored in YAML, read strictly, shipped as JSON | Accepted; per-step sources superseded by 0007 |
| [0003](0003-licensing-and-contributions.md) | MIT code, CC BY-NC-SA 4.0 content, DCO sign-off | Accepted |
| [0004](0004-sim-link.md) | Sim link behind a trait; pure-Rust SimConnect client first | Superseded by 0006 |
| [0005](0005-updates-and-signing.md) | Notify-only updates via GitHub Releases; unsigned beta | Accepted; applies to a desktop build only (0009) |
| [0006](0006-instruction-not-sim-control.md) | Teach procedures and systems; no connection to the simulator | Accepted |
| [0007](0007-pack-content-model.md) | Pack content model; list-level basis tags instead of per-step sources | Accepted; step status replaced by 0013 |
| [0008](0008-platform-scope.md) | Learn to operate the airplane like a trained crew; what we do not build | Accepted; amended by 0011 |
| [0009](0009-web-first-delivery.md) | Web first: a hosted app that installs and works offline | Accepted |
| [0010](0010-web-build-tooling.md) | Web build layout and tooling | Accepted |
| [0011](0011-scope-boundaries.md) | Scope boundaries for fuel training and the out-of-scope list | Accepted |
| [0012](0012-app-state.md) | App state for the web beta: one flight, rules in Rust, skipped stays open | Accepted |
| [0013](0013-step-presentation-and-provenance.md) | Every step presented the same way; provenance kept as an internal record | Accepted |
| [0014](0014-pack-format-3.md) | Pack format 3: option choices, map regions and target positions | Accepted |
| [0015](0015-resting-positions.md) | Resting positions for controls a step does not use | Accepted |

Template: Context, Decision, Consequences. Keep it under a page.
