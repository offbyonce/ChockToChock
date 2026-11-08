<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# Chock to Chock

Interactive cockpit checklists for flight simulators, from chocks off to chocks on.

Chock to Chock is a free training app for flight simulator pilots. It teaches the
procedures: checklists, flows, memory items and how the aircraft's systems work, and it
imports your SimBrief flight plan. It never connects to the simulator, so it works
alongside MSFS on a PC, a Mac, an Xbox or a PlayStation. Aircraft are data packs, so new
aircraft can be added without changing the app.

> **Status:** early development (Phase 0: foundation). Not ready for use yet. The current
> release for the iFly 737 MAX 8 is the single-file HTML checklist on flightsim.to.

**For simulator use only.** Never use these checklists for real-world flight.

## Repository layout

| Path | What it is |
| --- | --- |
| `crates/checklist-core` | The checklist domain: steps and sessions. No I/O. |
| `crates/pack` | Pack loading (YAML), validation and schemas |
| `crates/pack-cli` | The `pack` command for checking packs |
| `crates/simbrief` | SimBrief flight plan parsing |
| `crates/store` | Saved data (SQLite) |
| `apps/desktop/src-tauri` | The desktop app shell (Tauri 2) |
| `apps/desktop/ui` | The interface (Svelte 5) |
| `packages/bindings` | Typed calls from the interface to Rust; types generated from Rust |
| `packs/` | Aircraft and library packs (checklist content) |
| `schemas/` | JSON Schemas for pack authors (generated) |
| `docs/` | Setup guide and decision records |

## Getting started

See [docs/setup.md](docs/setup.md) to install the tools, then:

```sh
just setup   # install packages
just dev     # run the app
just ci      # run every check
```

## Contributing

Contributions are welcome, especially checklist fixes and new aircraft. Please read
[CONTRIBUTING.md](CONTRIBUTING.md) first: every commit needs a sign-off, and checklist
content has strict rules about sources. This is a hobby project, so responses are
best-effort.

## Licences

- **Code** is MIT ([LICENSE](LICENSE)).
- **Checklist content** in `packs/` is
  [CC BY-NC-SA 4.0](LICENSES/CC-BY-NC-SA-4.0.txt): share and adapt it with credit, but
  not commercially, and keep the same licence on anything you make from it.
  Want to use the checklists commercially? Contact the maintainer.

Each file's licence is recorded with [REUSE](https://reuse.software).

Aircraft and product names belong to their owners. Chock to Chock is not affiliated with
Microsoft, iFly, PMDG, Navigraph/SimBrief or any aircraft developer, and ships none of
their files, images or manuals.
