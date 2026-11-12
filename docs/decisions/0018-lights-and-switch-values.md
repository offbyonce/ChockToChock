<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0018: Explicit step targets

Date: 2026-11-11. Status: accepted. Extends [0014](0014-pack-format-3.md); replaces the wording
rules of [0016](0016-levers-handles-and-lit-lights.md) for drawings.

## Context
Drawings worked out what a step asks of each panel item by reading the step's wording, and each
new rule patched the last guess. One misread showed the passenger oxygen switch at ON (which
drops the cabin masks) because the light it names, PASS OXY ON, contains the word ON.

## Decision
- A step may list `targets`: each names one panel item by its locator, `panel/section/LABEL/n`
  (the n-th item with that label in the section, as light notes name them), with `set: <position>`
  for a switch, selector, lever or handle, or `light: on | out` for a light. A target with neither
  only highlights the item (a screen, a button, a switch whose position the step leaves open).
  Where the wording names a position the step does not ask for (a condition, "ON IF MORE THAN
  1,000 LB"; an option; one engine of two; or a word that belongs to something else), the target
  says `open: true`: the drawing claims no position (the control is highlighted with its knob
  between positions and the caption "SEE STEP"), and the check accepts the wording.
- Drawings show exactly these targets. Nothing reads a step's wording at render time.
- A light that may cycle (the window heat ON lights, which go out when the window is warm) gets
  no `light` state, so a drawing never claims a state the light may not be in.
- `pack check` holds targets to the pack: each names a real item on one of the step's controls,
  `set` is one of its positions and only on a non-light, `light` only on a light, no target twice,
  and a step whose controls are drawn has targets. It also holds them to the step's wording: a
  `set` position the wording does not name, a position the wording names that the target lacks, a
  light expected on that the wording says is out, or out that the wording says is on, are errors.
  Light names are taken out of the wording before positions are read.
- `pack suggest-targets <pack-dir>` drafts targets from wording, with every place its reading was
  not certain, for an author to check and keep. It is how the iFly pack's targets were first
  written; the drafts leave a state out wherever the reading was unsure.
- Pack format 3; the field is optional, so no version change.

## Consequences
- What a drawing shows is data an author can see, check and correct step by step.
- Steps and their targets cannot drift apart unnoticed.
- `lit_at` remains as the suggester's hint for lights a switch turns on.
