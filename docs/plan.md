<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# Chock to Chock: foundation plan

Status as of 10 Nov 2026. Phase 0 (foundation) and Phase 1 (engine and iFly pack) are complete;
Phase 2 (web beta) is next. The reasoning behind each choice is in `docs/decisions/`.

## What we are building

A free, open-source training app for flight simulator pilots. It teaches procedures:
checklists, flows, memory items, CDU (flight computer) set-up and how aircraft systems work,
and it imports a SimBrief flight plan. It grows out of the single-file HTML "iFly 737 MAX 8
Interactive Checklist" (3.0), published free on flightsim.to. The goal is to learn to operate the
airplane like a trained crew, not just to run checklists (decision 0008).

- **One engine, many aircraft.** Aircraft are data packs. The iFly 737 MAX 8 is the first; the
  next ones follow the order under "Aircraft order", reusing shared library packs.
- **A training app, not a sim add-on.** It never connects to the simulator, so it works beside
  MSFS on PC, Mac, Xbox or PlayStation (decision 0006).
- **Web first.** A hosted web app that installs to a home screen and works offline after the
  first visit, on desktop, tablet and phone (decision 0009). A desktop build may follow later.

## Decisions (all made by the maintainer)

| Topic | Decision |
| --- | --- |
| Name and identifier | Chock to Chock, `com.offbyonce.chocktochock` |
| Licenses | Code MIT; pack content CC BY-NC-SA 4.0; REUSE per file (0003) |
| Contributions | Public repo `offbyonce/ChockToChock`; DCO sign-off, no CLA |
| Stack | Rust 1.99 (pinned), Svelte 5 + Vite + TypeScript 6 (0001); Rust runs in the browser as WebAssembly (0009); Tauri 2.12 for a later desktop build |
| Delivery | Hosted web app on GitHub Pages, installable, offline after the first visit (0009) |
| Privacy | No backend, accounts, analytics, cookies or third-party resources; SimBrief called from the user's browser (0009) |
| Pack format | Authored in YAML, read strictly with `serde-saphyr`, shipped as JSON (0002) |
| Saved data | Browser storage, with export and import in a versioned file format defined in Rust (0009); SQLite (`crates/store`) kept for a desktop build |
| Updates | Web: each deploy is the update (0009). Desktop build: notify-only via GitHub Releases (0005) |
| Signing | Not needed for the web app; revisit with a desktop build (0005) |
| Sim link | None (0006 supersedes 0004) |
| Scope | Learn to operate the airplane like a trained crew; what we do not build is listed below (0008) |
| References | List-level `basis` tags, no per-step citations or third-party links (0007); rules in `CONTRIBUTING.md`, enforced in part by `pack check` |

## Architecture

```
Shared Svelte interface  ──typed calls──▶  host adapter (thin)
                                              │
              Phase 2: web host (WebAssembly) │ later: Tauri shell
                     ┌──────────────┬─────────┴────┐
                     ▼              ▼              ▼
                checklist-core   simbrief    browser storage (web)
                (pure: no I/O)   (parsing)   or store (desktop)

pack + pack-cli run at build time: YAML in, checked, flattened JSON out.
```

- Rust types are the source of truth; JSON Schemas (`schemars`) and TypeScript types (`ts-rs`)
  are generated, and `just ci` fails if the committed copies drift.
- Errors cross into the interface as a tagged `AppError { kind, message }`.
- The interface reaches storage, files and the network only through a small host adapter, so the
  same interface can run in the browser now and in a desktop shell later (0009).
- Web: a strict content security policy in a `<meta>` tag; no third-party resources at runtime.
  Desktop (later): the interface may call only the commands listed in `build.rs` and granted in
  `capabilities/main.json`.

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
| 2. Web beta with 3.0 parity | Hosted web app (0009) matching 3.0: phases, flows, panel states, SimBrief import, CDU walkthrough, timers, saved progress with export and import, 3.0 import. Also: installable and offline, first-launch "simulator training only" notice, "not yet verified" marker on draft steps, in-app privacy note | A public web beta that does everything 3.0 does and works offline after the first visit |
| 3. Training features | Memory items trainer, flow practice, systems explorer and step-to-system links, scenario drills, progress tracking. Also: briefings built from the SimBrief plan (threats, runway, departure, what-ifs); crew roles (pilot flying and pilot monitoring duties, standard callouts, challenge-and-response) taught within flow practice and briefings; non-normal scenarios built on the memory items; a technique layer (descent planning, energy management, stabilized-approach criteria, autopilot mode use); a structured course with spaced repetition; a flight plan and fuel module (a walkthrough of the pilot's own SimBrief plan, fuel and weight drills, find-the-error exercises, and the difference between EASA and US fuel rules); a CDU and EFB trainer; a refresher mode on top of spaced repetition for pilots returning after time away. The non-normal trainer depends on the pilot review of the procedure open list, because most non-normal steps are still `draft` | Each feature reviewed with beta pilots |
| 4. More aircraft | Library packs first in each family, then the aircraft in the order under "Aircraft order"; a desktop build if wanted (0009, later options) | Each aircraft passes `pack check` and review |

Phase 2 parity ships before any Phase 3 work.

## Aircraft order (Phase 4)

Chosen by how popular each aircraft is with simulator pilots and how well its vendor documents it
publicly, which is what steps are checked against (decision 0007). Each family starts with its
library pack.

1. **PMDG 737 NG** (Boeing 737 library): among the most flown airliners in community surveys,
   available on consoles, with public vendor tutorials, and it shares most procedures with the MAX.
2. **FlyByWire A32NX** (starts the Airbus A320 library): free and very widely used, with the best
   public documentation of any of these aircraft, so the A320 library is built and checked on it.
3. **Fenix A320 family** (A320 library): the most popular airliner in community surveys. Its
   public documentation is thinner, so it builds on the library checked with the A32NX, with its
   differences noted.
4. **iniBuilds A320neo** (A320 library): included with the simulator and available on consoles,
   so it reaches pilots who fly the default aircraft.
5. **PMDG 777** (Boeing widebody library): a popular widebody, available on consoles.
6. **iniBuilds A350, then Headwind A330-900** (Airbus widebody library): a popular recent release,
   then a free and widely downloaded aircraft.
7. **ATR 72** (turboprop library): included with the simulator.

## What Phase 2 changes for the store crate and the desktop app

- **`crates/store`:** not used by the web app, because its bundled SQLite (through `rusqlite`)
  does not build for WebAssembly. It stays in the repo, unchanged, for a desktop build. The saved-progress export file becomes
  the shared format: a versioned Rust type that the web app writes and reads now, and a desktop
  build can import later.
- **`apps/desktop`:** parked, not released in Phase 2. Its interface code moves into a shared
  interface package used by `apps/web` now and by `apps/desktop` later. Whether its Tauri shell
  keeps building in CI during Phase 2 is settled in the Phase 2 brief, step 1.

## Out of scope

Performance calculations, charts and navigation data or a full EFB, ATC, virtual first officer or
copilot voice features, automatic checking of cockpit switches, and anything needing a sim
connection. The reasons are in decisions 0006 and 0008. Also not planned: video courses. Others make
them, and this app's strength is interactive practice with reasons.

## Open items

- GitHub Pages deploy workflow (maintainer; Phase 2 brief).
- Move CI from `ubuntu-22.04` to `ubuntu-latest` (possible once the Tauri shell is parked, 0010), together
  with updating the required check name "ci (ubuntu-22.04)" in the `main` ruleset (maintainer).
- Low-severity Dependabot alert on the default branch (check and fix or document).
- `pnpm/action-setup@v4` warns about running on an old Node version; move to a newer release.
- Updater signing key: generate and back up before the first release (0005).
- Add `reuse lint` to `just ci`, with a clear message when the tool is missing.
- Download EASA AD 2021-0039R2 when EASA's site is back.
- Pilot review of the procedure open list.
