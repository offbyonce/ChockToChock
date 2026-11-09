<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# Contributing to Chock to Chock

Thanks for helping. This page covers the three things every contribution needs:
a sign-off, passing checks, and (for checklist content) acceptable sources.

## 1. Sign off every commit (DCO)

By signing off, you certify the
[Developer Certificate of Origin](https://developercertificate.org/): you wrote the change
or have the right to submit it, under this project's licences. Add the line with:

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

## 3. Checklist content rules

These protect the project and other people's work. `pack check` enforces most of them
automatically, and the maintainer reviews every content change.

**Every step needs a source** (`sources:` in the YAML) that says where its wording or
value came from. Good sources are ones anyone may read: the aircraft developer's own
public tutorials and documentation, the simulator's own documentation, public regulator
material, or your own observation in the simulator (say so).

**Manufacturer manuals (FCOM, FCTM, QRH) you have legitimate access to** may be used as a
reference to check facts, for example one supplied under licence with an aircraft you
bought, as long as that licence allows it. Facts and procedures (switch positions, the order
of steps, how a system works) are not protected by copyright, but the manual's wording,
tables, diagrams and layout are. So:

- write every step and explanation in your own words, and draw your own diagrams;
- do not mirror a manual's structure or phrasing closely;
- cite it by name only (e.g. "Boeing 737 FCOM, as supplied with <aircraft>"), never a link
  to a copy.

**Never use, link or cite:**

- Copies of FCOMs, FCTMs, QRHs or airline SOPs posted online without the owner's permission
- Document-sharing and flashcard sites: Scribd, Studylib, Brainscape, SlideShare,
  Manualshive, and archive.org copies of manuals
- Paid or members-only content you would need to copy to reproduce

**Never include:**

- Images, screenshots, PDFs or any files from an aircraft developer, manufacturer or simulator
- Text, tables or diagrams copied from a manual or tutorial. Switch names, positions and
  values are facts and are fine.

If you are unsure about a source, ask in your pull request before writing the content.

## Reporting problems

Open an issue with the aircraft, the checklist and step, what you expected, and your
source. For security problems, see [SECURITY.md](SECURITY.md) instead.
