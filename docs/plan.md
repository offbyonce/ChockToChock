<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# Chock to Chock: foundation plan

Status as of 9 Nov 2026. Phase 0 (foundation) is complete; Phase 1 is next. The reasoning
behind each choice is in `docs/decisions/`.

## What we are building

A free, open-source training app for flight simulator pilots. It teaches procedures:
checklists, flows, memory items, CDU (flight computer) set-up and how aircraft systems work,
and it imports a SimBrief flight plan. It grows out of the single-file HTML "iFly 737 MAX 8
Interactive Checklist" (3.0), published free on flightsim.to. The goal is to learn to operate the
airplane like a trained crew, not just to run checklists (decision 0008).

- **One engine, many aircraft.** Aircraft are data packs. The iFly 737 MAX 8 is the first;
  PMDG 737-800 and 777 and Airbus types come later, reusing shared library packs.
- **A training app, not a sim add-on.** It never connects to the simulator, so it works beside
  MSFS on PC, Mac, Xbox or PlayStation (decision 0006).
- **Windows first** for installers; macOS and Linux build from the same code. Mobile later.

## Decisions (all made by the maintainer)

| Topic | Decision |
| --- | --- |
| Name and identifier | Chock to Chock, `com.offbyonce.chocktochock` |
| Licenses | Code MIT; pack content CC BY-NC-SA 4.0; REUSE per file (0003) |
| Contributions | Public repo `offbyonce/ChockToChock`; DCO sign-off, no CLA |
| Stack | Tauri 2.12, Rust 1.99 (pinned), Svelte 5 + Vite + TypeScript 6 (0001) |
| Pack format | Authored in YAML, read strictly with `serde-saphyr`, shipped as JSON (0002) |
| Saved data | SQLite owned by Rust (`crates/store`) |
| Updates | Notify-only via GitHub Releases first; in-app updater later (0005) |
| Signing | Unsigned beta; revisit (SignPath Foundation, if eligible) before wider release |
| Sim link | None (0006 supersedes 0004) |
| Scope | Learn to operate the airplane like a trained crew; what we do not build is listed below (0008) |
| References | List-level `basis` tags, no per-step citations or third-party links (0007); rules in `CONTRIBUTING.md`, enforced in part by `pack check` |

## Architecture

```
Svelte interface  ──typed commands──▶  Tauri shell (thin)
                                            │
                     ┌──────────────┬───────┴──────┐
                     ▼              ▼              ▼
                   pack          simbrief        store
                     └──────────────┴──────────────┘
                                    ▼
                             checklist-core  (pure: no I/O, no Tauri)
```

- Rust types are the source of truth; JSON Schemas (`schemars`) and TypeScript types (`ts-rs`)
  are generated, and `just ci` fails if the committed copies drift.
- Errors cross into the interface as a tagged `AppError { kind, message }`.
- The interface may call only the commands listed in `build.rs` and granted in
  `capabilities/main.json`. No file or network access from the interface; strict CSP.

## Packs

- `pack.json` manifest: id, semver version, schema version, engine range, dependencies,
  kind (aircraft or library), license, attribution, list of content files.
- Content in YAML, one file per checklist, flow or topic, with stable namespaced ids
  (`ifly-737-max8.preflight.irs-mode`).
- Shared content via **library packs** (e.g. `lib.boeing-737-controls`, `lib.boeing-cdu`).
  A pack may extend at most one parent and override items **by id** (change, remove, insert
  before/after), never by list position. Conflicts are build errors. Packs are flattened to
  one JSON per aircraft before the app uses them.
- `pack check` enforces: valid schema, unique ids, a `basis` on every list, no banned or
  unapproved links, no images or PDFs (0007; being updated in Phase 1). Golden snapshot tests
  catch unintended content changes.
- Planned additions (Phase 1 and later): phases and flows, panel maps, CDU page models,
  timers, memory items, systems models, aliases from 3.0 ids.

## Quality

- `just ci` runs: formatting (rustfmt, Biome), clippy with `-D warnings` (pedantic), svelte-check,
  all tests (nextest, doctests, Vitest), generated-file drift check, `pack check`,
  `cargo deny`, `pnpm audit`.
- GitHub Actions runs `just ci` on Windows, macOS and Ubuntu 22.04 for every push and pull
  request, plus a REUSE license check. `main` is protected: pull requests with all four checks
  passing; no force-push or deletion.
- Git hooks (lefthook): formatting and lint before commit; DCO sign-off required; tests before
  push.

## Phases

| Phase | Work | Gate |
| --- | --- | --- |
| 0. Foundation (done) | Workspace, crates, shell, interface skeleton, CI, licenses, docs | `just ci` green on Linux, Windows, macOS |
| 1. Engine and iFly pack | Extend the pack format; move all 3.0 content into `packs/ifly-737-max8` with no sources or links carried over | Every 3.0 step accounted for; `pack check` passes |
| 2. Desktop app parity | Interface matching 3.0: phases, flows, panel states, SimBrief import, CDU walkthrough, timers, saved progress, 3.0 import, notify-only updates | A Windows beta that does everything 3.0 does |
| 3. Training features | Memory items trainer, flow practice, systems explorer and step-to-system links, scenario drills, progress tracking. Also: briefings built from the SimBrief plan (threats, runway, departure, what-ifs); crew roles (pilot flying and pilot monitoring duties, standard callouts, challenge-and-response) taught within flow practice and briefings; non-normal scenarios built on the memory items; a technique layer (descent planning, energy management, stabilized-approach criteria, autopilot mode use); a structured course with spaced repetition | Each feature reviewed with beta pilots |
| 4. More aircraft | Library packs; PMDG 737 and 777; Airbus family; macOS and Linux releases; in-app updates; mobile later | Each aircraft passes `pack check` and review |

Phase 2 parity ships before any Phase 3 work.

## Out of scope

Performance calculations, charts and navigation data, ATC, virtual first officer or copilot voice
features, and anything needing a sim connection. The reason for each is in decision 0008.

## Open items

- Low-severity Dependabot alert on the default branch (check and fix or document).
- `pnpm/action-setup@v4` warns about running on an old Node version; move to a newer release.
- Updater signing key: generate and back up before the first release (0005).
- Add `reuse lint` to `just ci`, with a clear message when the tool is missing.
- Download EASA AD 2021-0039R2 when EASA's site is back.
- Pilot review of the procedure open list.
