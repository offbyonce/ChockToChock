<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0007: Pack content model and reference policy

Date: 2026-11-08. Status: accepted. Supersedes the "every step sourced" rule and the per-step
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
  moves to a real basis as its procedure section is validated and the maintainer approves the
  batch.
- **Facts are checked against vendor documentation, as described under Procedure validation. No
  manual text, link or citation enters the repo.** Facts only. Never copy text, tables, diagrams or
  layout, and never closely paraphrase. Write from our own flow structure, not line by line
  alongside a manual.
- **Second-hand study material is not used as a basis at all, for quality reasons.** That covers
  Brainscape, SlideShare, AviationHunt and memory-item compilations.
- **Airline SOPs stay out as a basis.** Manufacturer procedures are the baseline.
- **Manuals are never stored in the repo, uploaded, linked or cited.** Linking the banned sites
  or second-hand study material stays banned.

## Decision: content model
Types live in `checklist-core`; loading and checks in `pack`. `schemaVersion` becomes 2.
Content stays in YAML, one file per list or topic, listed by kind in `pack.json`.

| 3.0 | Pack |
| --- | --- |
| `STAGES`, `PHASES` | `phases.yaml`: stages holding ordered phases; `reference: true` for the three reference phases; `show_if` on a phase's place in a stage replaces rules hard-coded in the app (one phase can sit in either of two stages) |
| Lists | `kind: checklist`, `flow` or `non-normal`; `memory: true`; `when`, `why`, `basis` |
| `{h}` header rows | `section` on the step that follows |
| Row number, title, value, control(s) | `number` (absent for 3.0's `0`), `text`, `expect`, `controls` |
| `___` in values | Kept in `expect`; optional `fill` gives one field per blank, in order, from the app's fixed list (see below) |
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
| `alt` | Not carried over: 3.0's app reads it, but no step sets it. Bringing back alternative steps would need a new decision |
| `C`, `ZONES` | `controls.yaml`, zones; a control's zone is optional (replaces `__briefing`) |
| `PANELS` | `panels/*.yaml` with named item kinds, not single letters. Coordinates are kept as our own layout; no images or screenshots in the repo, and our drawings stay in our own outline style |
| `LIGHTS21` | Light notes keyed by stable locator, not list position; carry `status` |
| `CAUSE21` | Fault causes keyed by step id; the 2 entries that match no step are dropped and reported |
| `GLOSS`, `BRIEFS` | `glossary.yaml`, `briefings.yaml` |
| `PSTATES` | `start-states.yaml`, using new step ids |
| CDU walkthrough (in app code) | `cdu.yaml`: pages, rows, keys to highlight, action, check, note, named fill fields. Calculations (AIRAC, rounding, route legs) go in Rust, not the pack |
| `SOURCES` | Not carried over |
| Position-based 3.0 ids | New stable ids plus `aliases.yaml` from old to new |

Practice-question text waits for Phase 3. Systems content is also Phase 3 and must be labeled
real aircraft, simulator model, or both.

**`show_if` is deliberately small.** A test is "option equals value" or "option is one of
[values]". Tests can be combined with `all_of` or `any_of`, one level deep. Nothing else: no
negation, comparisons, step state or expressions. Options and values must be declared by the pack.
This covers every visibility rule in 3.0, including those that hide a step on a turnaround,
because each option's values are listed.

```yaml
show_if: {option: power, equals: gpu}
show_if: {option: approach, one_of: [ils, rnav]}
show_if:
  any_of:
    - {option: power, equals: gpu}
    - {option: start, equals: turnaround}
```

**`fill` names come from a fixed list the app defines** (for example `zfw`, `v1`, `cost_index`),
held as a Rust enum so the schema and bindings carry it. Packs cannot add names, and `pack check`
rejects unknown ones. `manual` marks a blank with no linked field. `hint` and the CDU fill fields
use the same list. A new name needs an app release.

**Step ids are `<pack-id>.<phase>.<short-name>`**, for example
`ifly-737-max8.preflight.irs-mode`. The short name uses lower-case letters, digits and dashes, and
ids are unique within a pack. **An id never changes once published**, because saved progress and
overrides depend on it. If a step is reworded or moved to another phase, its id stays, so the
phase part records where it was first published. A removed step's id is retired and never reused.

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
  note and alias references all resolve; `show_if` stays within its grammar; `fill` names are on
  the app's list; step ids start with the pack id and follow the format above.

## Decision: text-overlap check (not built yet)
`pack overlap <pack-dir> --refs <folder>`, with `just overlap` as the shortcut. Logic in
`crates/pack`.

- **References** are plain `.txt` files in a folder outside the repo. The tool refuses a folder
  inside the repo. The folder must be private on the machine that runs the check and never synced
  anywhere public. PDF extraction is done separately, so no new dependency is needed.
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

## Procedure validation
Added 2026-11-09.

- **Vendor documentation is the primary basis,** because the app teaches the iFly 737 MAX 8. The
  iFly tutorial and manuals come first. The FAA, EASA and accident documents and the cross-check
  checklists are secondary checks.
- **There is no FCOM reading.** Manuals (FCOM, QRH, FCTM) are never downloaded, stored, linked or
  cited, in the repo or the private folders.
- **The private overlap folder holds only text copies of documents the maintainer obtained
  legitimately** (public records such as NTSB dockets, accident reports and FAA/EASA documents;
  vendor documentation the maintainer owns; files downloaded under a site's normal terms) and public
  web pages that 3.0 cited, saved privately for the overlap check only. Those web pages are not
  cross-checks and never a basis for a step. FCOM and QRH copies of unknown origin are never placed
  there.
- **Every list is validated, by procedure section, starting with power-up.** Order and setting
  differences are fixed against the tutorial, and "why" notes are written in our own words.
- **A step is marked `checked` when it clearly matches the vendor documentation.** `checked` means
  the action and setting match the vendor documentation; not every word of the explanation is
  sourced, and it does not mean a pilot has reviewed it. Simulator-only behavior is labeled with `simulator_model: true`
  on the step whose note describes it; `sim_simplified` keeps its narrower meaning, that the
  simulator simplifies the real aircraft.
- **Where sources conflict and the tutorial does not settle it, the step stays `draft`** and goes
  on an open list for an optional pilot review later.
- **Findings are recorded as facts in our own words,** for example: "AC power is not applied until
  the electric hydraulic pumps are OFF". Never copy wording, tables or layout.
- **Public-record airline checklists and community checklists may be used as cross-checks,** as
  well as in the overlap check. Never use them as the basis for a step, never copy them, and never
  name the airline, the community authors or the specific checklists in the repo. Specifics go only
  in the private findings log.
- **Public pilot forum discussions may be used as a cross-check for open-list items only.** Other
  developers' product tutorials do not count. Never use them as a basis, and never name, link or
  copy them in the repo. Specifics go in the private log.
- **FAA AC 120-71B and the NASA normal checklist report inform how we explain flows and checklists,
  and our "why" notes.** Draw ideas only, never their wording.
- **Fixes are written from our own flow structure** and explain why the step matters where useful
  (for example, ground-crew safety).
- **The findings log and the open list live in `~/private`, never in the repo.** Each procedure
  section is reported as a batch, a short summary of changes plus the open list, and the maintainer
  approves each batch.

## Consequences
- Pilots see what kind of material a list was checked against, but cannot trace an individual step.
- 3.0 wording was written alongside its old references. Dropping the list does not clear that
  wording, so imported text goes through the overlap check and starts as `draft`.
- Removing `sources` breaks the old format; only the example pack is affected.
- The model is larger, so the app can show 3.0's behavior from data alone.

## Changes needed (not yet made)
**`CONTRIBUTING.md`:** replace "Every step needs a source" with `basis`, `status` and
`sim_simplified`. Rewrite the manuals paragraph to the rules above, removing "legitimate access"
and "cite it by name". Split "Never use, link or cite" in three: the maintainer checks facts against
the manufacturer's manuals, and no manual text, link or citation enters the repo; second-hand study material is never a basis; linking any banned site or
second-hand material is banned. Airline SOPs stay banned. Add the overlap check and the
`prior-release` and `draft` starting point. State the step id format and that published ids never
change.

**`AGENTS.md`:** rewrite the first three content rules to match. Keep "never add images, PDFs or
files". Add `just overlap` to the commands.

**`pack check` and model:** remove `SourceRef`, `Step.sources` and the source checks in
`validate.rs`, and update their tests. Replace `BLOCKED_SOURCE_DOMAINS` with domains plus path
prefixes, adding `aviationhunt.com`. Add the text scan, the empty link allow-list and the new
rules. Regenerate schemas and bindings. Remove `sources:` from the example pack and add `basis`.

**Also affected:** `docs/plan.md` (the `pack check` list and the Sources row);
`docs/briefs/phase-1.md` (step 5 is no longer needed, and steps 2, 4, 6 and 8 mention sources);
the decision index; `lefthook.yml` for the hook, which needs the maintainer's go-ahead.
