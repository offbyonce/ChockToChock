<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# Phase 1 brief: the engine and the iFly 737 MAX 8 pack

**Goal:** every piece of content in the 3.0 HTML checklist exists as validated pack data in
`packs/ifly-737-max8`, with no sources or links carried over (decision 0007), so Phase 2 can
build the interface on it.

**Gate:** every 3.0 step is accounted for (moved, merged or deliberately dropped, with a
reason), `pack check` passes, and `just ci` is green.

Work one step at a time (see `AGENTS.md`): finish a step, show the result, wait for the
maintainer's go-ahead.

## Where the 3.0 content is

The 3.0 source is **not** in this repo and must not be committed to it wholesale (it contains
links to sources we no longer allow). The maintainer provides a local copy; its path is given to
you separately. The parts that matter:

| 3.0 file | Holds |
| --- | --- |
| `src/data.js` | Base content: `ZONES` (panel areas), `PHASES` (checklists and flows by phase), `BRIEFS`, `SOURCES`, memory items, glossary (`GLOSS`), "why" and "when" notes, reference lists |
| `src/v20.js` to `src/v25.js` | Patches applied in order on top of `data.js` (fixes, renames, inserts) |
| `src/data21.js`, `src/data22.js` | 2.1 and 3.0 additions, including timers and the CDU walkthrough data |
| `src/words22.js`, `src/pstate22.js` | Wording updates and panel states |
| `src/build.py` | The order the files are combined in |
| `tests/` | 3.0's own tests; useful to see what the data should look like |

3.0 has about 510 steps once all layers are applied. Because later files patch earlier ones,
read the **resolved** data (run the files in their build order in Node and dump the result),
not `data.js` on its own.

## Steps

1. **Inventory (no code changes).** Run the 3.0 data layers in Node, in `build.py` order, and
   dump the resolved data to a scratch JSON file outside the repo. Report: the top-level
   structures, the counts (phases, checklists, flows, steps, memory items, sources), and the
   kinds of fields steps carry. Propose how each maps onto packs.
2. **Pack format design (decision record).** From the inventory, propose the additions to the
   Rust model in `checklist-core` and `pack`: phases, flows versus checklists, step kinds,
   panel zones and states, timers, CDU walkthrough, list-level `basis`, step `status`, memory
   items, glossary, per-step "why" and "when", and an alias table from 3.0 ids to new ids. Write it as
   `docs/decisions/0007-pack-content-model.md` and wait for approval before coding.
3. **Implement the model.** Rust types, YAML loading, validation rules, regenerated schemas and
   bindings, tests (including property tests where cheap). Update the example pack to use
   the new features. `just ci` green.
4. **Converter.** A small, re-runnable script (Node is fine, under `tools/import-3x/`) that reads
   the resolved 3.0 data, and the CDU walkthrough from `template.html`, from a path given on the
   command line and writes YAML into `packs/ifly-737-max8/`. It carries over no sources or links.
   Every list gets basis `prior-release`, every step `draft`, and the 10 "confirm" light notes
   `draft`. It normalizes imported 3.0 text to American spelling.
5. **Build the text-overlap check.** `pack overlap` and `just overlap` as designed in decision
   0007, with tests. Add the `overlap` pre-commit hook to `lefthook.yml` in this step, once
   `just overlap` exists.
6. **Generate and check the pack.** Run the converter, then `pack check`. Fix errors in the
   converter or the model, not by hand-editing generated YAML (unless a step is deliberately
   rewritten, which the report must list). Run the text-overlap check (0007) over the imported
   text before it is first committed.
7. **Parity report.** A table of every 3.0 step: new id, status (moved, merged, dropped), and
   reason for anything not moved one-for-one, plus a list of the words the converter changed
   to American spelling. Snapshot tests on the flattened pack.
8. **Hand back.** Branch ready for review, with the parity report and the lists still at basis
   `prior-release`.

## Out of scope for Phase 1

- Interface work beyond showing that the new pack loads (that is Phase 2).
- Systems models content (designed for in step 2, written in Phase 3).
- Library packs (`lib.boeing-*`) unless the inventory shows a clear split; note candidates
  instead.

## Notes for later phases

**Phase 2:**

- The LEGS page of the CDU walkthrough must render correctly with no departure set, with a test.
  The converted page assumes one is set, so without it the rows would sit one line off.
- Add CDU comparison tests for partly filled flight data. The converter's comparison covers only
  no flight data and full flight data.
- The guided tour is app text and belongs here.

**Phase 3:**

- Aliases for practice scores saved per cockpit area, along with the practice text. One area
  covers several zones, so they need their own mapping.
