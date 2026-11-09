<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0007: Pack content model and reference policy

Date: 2026-11-08. Status: proposed. Supersedes the "every step sourced" rule and the per-step
sources in [0002](0002-pack-format.md); the rest of 0002 stands.

## Context
Phase 1 moves all 3.0 content into a pack. The current model has only checklists of steps with
text, expected response, note and sources. 3.0 has 26 phases, 65 lists and 510 steps, plus
controls, panels, timers, a CDU walkthrough and more. It keeps one global reference list and
links no step to a source, so per-step citations could not be carried over honestly.

## Decision: references and accuracy
- **No per-step citations and no URLs to third-party material in the repo.** The 3.0 reference
  list does not carry over. `SourceRef` and `sources:` are removed.
- **Each list has a `basis`** naming only the kind of material it was checked against, for
  example `manufacturer-docs`, `ifly-docs`, `sim-observed`. No document names or links. A pack
  declares its allowed tags in `pack.json`, so the engine names no aircraft. A list has one or
  more tags.
- **Optional `status` of `draft` or `checked`** on steps and panel light notes, and a
  `sim_simplified` flag where the simulator simplifies the real aircraft. The 10 light notes 3.0
  marks "confirm" start as `draft`.
- **Imported 3.0 lists get basis `prior-release` and every step starts as `draft`.** Each list
  moves to a real basis and `checked` as the maintainer reviews it.
- **Manufacturer manuals (FCOM, QRH, FCTM) may be read as references for accuracy, wherever the
  copy is hosted, including the banned sites.** Facts only. Never copy text, tables, diagrams or
  layout, and never closely paraphrase. Write from our own flow structure, not line by line
  alongside a manual.
- **Second-hand study material is not used as a basis at all, for quality reasons.** That covers
  Brainscape, SlideShare, AviationHunt and memory-item compilations.
- **Manuals are never stored in the repo, uploaded, linked or cited.** Linking the banned sites
  or second-hand study material stays banned.

## Decision: content model
Types live in `checklist-core`; loading and checks in `pack`. `schemaVersion` becomes 2.
Content stays in YAML, one file per list or topic, listed by kind in `pack.json`.

| 3.0 | Pack |
| --- | --- |
| `STAGES`, `PHASES` | `phases.yaml`: stages holding ordered phases; `reference: true` for the three reference phases; `show_if` replaces rules hard-coded in the app |
| Lists | `kind: checklist`, `flow` or `non-normal`; `memory: true`; `when`, `why`, `basis` |
| `{h}` header rows | `section` on the step that follows |
| Row number, title, value, control(s) | `number` (absent for 3.0's `0`), `text`, `expect`, `controls` |
| `___` in values | Kept in `expect`; optional `fill` names the flight-strip field |
| Per-id hints and CDU button in app code | `hint` and `opens` on the step |
| `(NON-RECALL)` in values | `non_recall: true`, marker removed from `expect` |
| `q`, `qt` | `quick: always` or `turnaround` |
| `gpu`, `apuNow`, `appr` | `show_if` over `options` the pack declares (power, approach type, start type) |
| `x` detail text | `why`, `check`, `check_detail`, `if_not`, `if_not_detail` |
| `d` | `note` (existing field) |
| `how`, `why` keys, `HOW`, `WHY` | `how` and `explain` ids pointing at shared `notes.yaml` |
| `x`, `y` numbers on exterior steps | `walkaround: {x, y}` |
| `look`, `where`, `say` | `look_for`, `location`, `callout` |
| `tm` | `timer`: seconds, label, note, optional auto-start from step ids (earliest or latest) |
| `pcl` | `panel_lights` |
| `ns` | `not_simulated: true`, left out of progress as in 3.0 (separate from `sim_simplified`) |
| `add`, `a` | Dropped: build-time marker and id-numbering aid |
| `C`, `ZONES` | `controls.yaml`, zones; a control's zone is optional (replaces `__briefing`) |
| `PANELS` | `panels/*.yaml` with named item kinds, not single letters. Coordinates are kept as our own layout; no images or screenshots in the repo, and our drawings stay in our own outline style |
| `LIGHTS21` | Light notes keyed by stable locator, not list position; carry `status` |
| `CAUSE21` | Fault causes keyed by step id; the 2 entries that match no step are dropped and reported |
| `GLOSS`, `BRIEFS` | `glossary.yaml`, `briefings.yaml` |
| `PSTATES` | `start-states.yaml`, using new step ids |
| CDU walkthrough (in app code) | `cdu.yaml`: pages, rows, keys to highlight, action, check, note, named fill fields. Calculations (AIRAC, rounding, route legs) go in Rust, not the pack |
| `SOURCES` | Not carried over |
| Position-based 3.0 ids | New stable ids plus `aliases.yaml` from old to new |

Practice-question text waits for Phase 3. Systems content is also Phase 3 and must be labelled
real aircraft, simulator model, or both.

## Decision: `pack check`
- Drop "every step has a source".
- Keep blocking banned domains, and images and PDFs.
- Add `aviationhunt.com` to the banned domains.
- Add `data.ntsb.gov/Docket` as a blocked path prefix (host plus case-insensitive path).
- With `sources` gone, links can only appear in text, so scan every authored string in the pack
  and manifest for URLs and bare banned domains.
- Any other URL is an error unless it is on an allow-list in `pack check`. The list starts empty
  and holds only first-party links the maintainer approves. A banned link can never be
  allow-listed.
- New: every list has a `basis` from the declared tags; `status`, `show_if`, `opens`, `controls`,
  note and alias references all resolve.

## Decision: text-overlap check (proposed, not built)
`pack overlap <pack-dir> --refs <folder>`, with `just overlap` as the shortcut. Logic in
`crates/pack`.

- **References** are plain `.txt` files in a folder outside the repo. The tool refuses a folder
  inside the repo. PDF extraction is done separately, so no new dependency is needed.
- **Method:** lower-case, strip punctuation, collapse spaces, then flag any run of 8 or more words
  found in both. Reference text is held in memory only; no index is written.
- **Two comparisons:** each prose field alone, and each list's step text and expected responses
  joined in order. Checklist rows are short, so a copied list would pass a field-only check.
- **Output:** pack file, step or list id, reference file name and the matching words. Non-zero
  exit if any match is not allowed.
- **Allowed matches:** `tools/overlap/allow.txt` holds the run and a reason, for standard phrases
  and switch names. The run is our own committed text, so this exposes nothing new.
- **When it runs:** a pre-commit hook when `packs/` is staged, and over all imported 3.0 text
  before that content is first committed. It cannot run in CI, because references are never
  uploaded. A contributor without references gets a "not run" notice, and the maintainer runs it
  before merging.
- **Limits:** it finds exact runs only. Close paraphrase, copied structure and layout still need
  human review.

## Consequences
- Pilots see what kind of material a list was checked against, but cannot trace an individual step.
- 3.0 wording was written alongside its old references. Dropping the list does not clear that
  wording, so imported text goes through the overlap check and starts as `draft`.
- Removing `sources` breaks the old format; only the example pack is affected.
- The model is larger, so the app can show 3.0's behaviour from data alone.

## Changes needed (not yet made)
**`CONTRIBUTING.md`:** replace "Every step needs a source" with `basis`, `status` and
`sim_simplified`. Rewrite the manuals paragraph to the rules above, removing "legitimate access"
and "cite it by name". Split "Never use, link or cite" in three: manufacturer manuals may be read
wherever hosted; second-hand study material is never a basis; linking any banned site or
second-hand material is banned. Airline SOPs stay banned. Add the overlap check and the
`prior-release` and `draft` starting point.

**`AGENTS.md`:** rewrite the first three content rules to match. Keep "never add images, PDFs or
files". Add `just overlap` to the commands.

**`pack check` and model:** remove `SourceRef`, `Step.sources` and the source checks in
`validate.rs`, and update their tests. Replace `BLOCKED_SOURCE_DOMAINS` with domains plus path
prefixes, adding `aviationhunt.com`. Add the text scan, the empty link allow-list and the new
rules. Regenerate schemas and bindings. Remove `sources:` from the example pack and add `basis`.

**Also affected:** `docs/plan.md` (the `pack check` list and the Sources row);
`docs/briefs/phase-1.md` (step 5 is no longer needed, and steps 2, 4, 6 and 8 mention sources);
the decision index; `lefthook.yml` for the hook, which needs the maintainer's go-ahead.
