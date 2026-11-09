<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# Contributing to Chock to Chock

Thanks for helping. This page covers the three things every contribution needs:
a sign-off, passing checks, and (for checklist content) following the content rules.

## 1. Sign off every commit (DCO)

By signing off, you certify the
[Developer Certificate of Origin](https://developercertificate.org/): you wrote the change
or have the right to submit it, under this project's licenses. Add the line with:

```sh
git commit -s
```

That appends `Signed-off-by: Your Name <you@example.com>`. Commits without it are rejected
by the git hook and in review. Your contribution is licensed under MIT (code) or
CC BY-NC-SA 4.0 (checklist content in `packs/`), the same as the rest of the project.

## 2. Pass the checks

Run `just ci` before opening a pull request. It formats, lints, tests, validates packs and
checks dependencies. The git hooks (installed by `pnpm install`) run the quick parts for you.

- Commit messages follow [Conventional Commits](https://www.conventionalcommits.org/):
  `feat: …`, `fix: …`, `docs: …`, `pack(ifly-737-max8): …`.
- Every bug fix comes with a test that would have caught it.
- Keep logic in `crates/`. The Tauri shell and the interface stay thin.
- Never put an aircraft name in engine code. Aircraft differences belong in pack data.
- American English everywhere: code identifiers, pack field names, error messages, docs and pack content (color, behavior, license as a noun, optimize, pressurization).
  Exceptions keep their exact spelling: SPDX license IDs, file names such as `LICENSE` and `LICENSES/`, and anything quoted from outside sources.

## 3. Checklist content rules

These protect the project and other people's work. `pack check` enforces most of them
automatically, and the maintainer reviews every content change. The reasoning is in
[decision 0007](docs/decisions/0007-pack-content-model.md); `pack check` is being updated to
match, and until that lands it still expects per-step sources.

**No citations and no third-party links.** Steps do not cite sources, and packs contain no URLs
to third-party material. A first-party link needs the maintainer's approval.

**Every list has a `basis`** naming only the kind of material it was checked against, such as
`manufacturer-docs` or `sim-observed`, from the tags the pack declares. Never a document name or
link.

**Steps may carry a `status`** of `draft` or `checked`, and `sim_simplified` where the simulator
simplifies the real aircraft. Content imported from the 3.0 checklist starts with basis
`prior-release` and every step as `draft`; each list moves to a real basis and `checked` when the
maintainer reviews it.

**Manufacturer manuals (FCOM, FCTM, QRH)** may be read as references for accuracy, wherever the
copy is hosted. Facts and procedures (switch positions, the order of steps, how a system works)
are not protected by copyright, but a manual's wording, tables, diagrams and layout are. So:

- use facts only, and write every step and explanation in your own words;
- never copy text, tables, diagrams or layout, and never closely paraphrase;
- write from our own flow structure, not line by line alongside a manual;
- draw your own diagrams;
- never store, upload, link or cite a manual.

**Never used as a basis:**

- Second-hand study material: Brainscape, SlideShare, AviationHunt and memory-item compilations
- Airline SOPs
- Paid or members-only content you would need to copy to reproduce

**Never link,** anywhere in the project: Scribd, Studylib, Brainscape, SlideShare, Manualshive,
archive.org, AviationHunt, NTSB docket documents, or any other second-hand study material.

**Never include:**

- Images, screenshots, PDFs or any files from an aircraft developer, manufacturer or simulator
- Text, tables or diagrams copied from a manual or tutorial. Switch names, positions and
  values are facts and are fine.

**Step ids** are `<pack-id>.<phase>.<short-name>`, for example
`ifly-737-max8.preflight.irs-mode`. An id never changes once published, even if the step is
reworded or moved. A removed step's id is retired and never reused.

**Overlap check (planned, not built yet).** `just overlap` will flag any run of 8 or more words
matching reference text kept in a private folder outside the repo, before content is committed.
That folder is never committed, uploaded or synced anywhere public.

If you are unsure whether something is acceptable, ask in your pull request before writing the
content.

## Reporting problems

Open an issue with the aircraft, the checklist and step, what you expected, and your
source. For security problems, see [SECURITY.md](SECURITY.md) instead.
