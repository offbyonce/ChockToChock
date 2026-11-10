<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# Working on Chock to Chock (for AI agents and contributors)

Read this first, then `docs/plan.md` (what we are building and why) and the brief for the
current phase in `docs/briefs/`. Decision records in `docs/decisions/` explain past choices;
do not reverse one without the maintainer's agreement.

## What this project is

A free training app for flight simulator pilots: checklists, flows, memory items, CDU
walkthroughs and aircraft systems. Desktop first (Tauri 2, Rust core, Svelte 5), Windows as the
main target, macOS and Linux too. Aircraft are data packs in `packs/`; the engine never names an
aircraft. **The app never connects to the simulator** (decision 0006).

## Non-negotiable rules

**Content and sources** (full rules in `CONTRIBUTING.md`):

- No per-step citations and no URLs to third-party material (decision 0007). Each list has a
  `basis` tag naming only the kind of material it was checked against. Never link the banned
  sites or second-hand study material; `pack check` enforces this.
- Facts are checked against vendor documentation. No manual text, link or citation enters the
  repo. Facts only: never copy text, tables, diagrams or layout, never closely
  paraphrase, and write from our own flow structure. Never store, upload, link or cite them.
- Second-hand study material (Brainscape, SlideShare, AviationHunt, memory-item compilations) and
  airline SOPs are never used as a basis.
- Never add images, screenshots, PDFs or any files from an aircraft developer, manufacturer or
  simulator. Switch names, positions and values are facts and are fine.
- Label systems content as real aircraft, simulator model, or both.

**Code:**

- Logic lives in `crates/`. The Tauri shell (`apps/desktop/src-tauri`) and the interface stay
  thin. No aircraft names in engine code; differences come from pack data.
- Rust types are the single source of truth. Regenerate derived files, never edit them by hand:
  `just schemas` (JSON Schemas) and `just bindings` (TypeScript types).
- Every bug fix comes with a test that would have caught it.
- No `unsafe`. No new dependency without saying why; versions live once, in the root
  `Cargo.toml` and `pnpm-workspace.yaml`.

**Git:**

- Work on a branch, never directly on `main`. Never force-push, rewrite history or delete branches
  you did not create.
- Sign off every commit: `git commit -s`. Conventional Commit messages (`feat:`, `fix:`,
  `docs:`, `test:`, `chore:`, `ci:`, `pack(<id>):`).
- Commit messages carry no session links or session ids (no Claude-Session or similar trailers) and
  no machine names. Co-Authored-By lines are fine.
- Run `just ci` before every push. If it fails, fix it; never skip hooks or disable checks.
- Do not edit `.github/workflows/`, `deny.toml`, `lefthook.yml`, licenses or the decision records
  without the maintainer's explicit go-ahead.
- Never commit secrets, keys, tokens, personal names or email addresses, machine names or IP
  addresses. Test fixtures are invented and use generic names ("Pilot").

## How to work

- **One step at a time.** Propose a step, do it, show the result (command output, test results),
  and wait for the maintainer to confirm before starting the next one. Do not queue up extra
  tasks before the current one works.
- **Ask before deciding** anything that changes the data model, the pack format, a public
  interface, licenses or the plan. Write a short decision record for choices that matter.
- **Say plainly when unsure,** and verify instead of guessing: run the command, read the file,
  check the docs.
- Keep explanations short and in plain language; say which machine a command runs on.
- American English everywhere: code identifiers, pack field names, error messages, docs and pack content (color, behavior, license as a noun, optimize, pressurization).
  Exceptions keep their exact spelling: SPDX license IDs, file names such as `LICENSE` and `LICENSES/`, and anything quoted from outside sources.

## Repository map

| Path | What it is |
| --- | --- |
| `crates/checklist-core` | Domain: checklists, steps, sessions. No I/O. |
| `crates/pack` | Pack manifest, strict YAML loading (`serde-saphyr`), validation, schemas |
| `crates/pack-cli` | `pack check <dir>`, `pack schema <dir>`, `pack overlap <dir>` |
| `crates/simbrief` | SimBrief OFP parsing |
| `crates/store` | SQLite saved data and migrations |
| `apps/desktop/src-tauri` | Tauri shell: commands, capabilities, config |
| `apps/desktop/ui` | Svelte 5 interface |
| `packages/bindings` | Typed command wrappers; `src/generated` comes from Rust |
| `tools/import-3x` | One-way converter from the 3.0 checklist to pack YAML |
| `packs/` | Content packs (CC BY-NC-SA 4.0) |
| `schemas/` | Generated JSON Schemas |
| `docs/` | Plan, briefs, setup guide, decision records |

## Commands

```sh
just setup     # pnpm install + cargo fetch (once after cloning)
just dev       # run the app (needs a desktop session)
just test      # Rust and interface tests
just packs     # validate every pack
just ci        # everything; must pass before a push
just overlap   # compare packs with private references (CHOCKTOCHOCK_OVERLAP_REFS)
```

Rust is pinned in `rust-toolchain.toml` (1.99). Newer clippy versions add lints, so always run
`just ci` with the pinned toolchain, not an older one.
