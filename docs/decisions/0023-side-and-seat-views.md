<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0023: Side and seat views from pack data

Date: 2026-11-12. Status: accepted. Extends the overview of [0014](0014-pack-format-3.md); part of
pack format 4 with [0022](0022-conditional-controls-and-decision-steps.md).

## Context
"Where is this?" in the approved design ([screens.md](../design/screens.md), the WherePop board)
shows two views besides the flight deck map: a side view (how high and how far, with the sight
line from the seat) and a captain-seat view (which side, with the area highlighted and no panel
detail). Decision [0021](0021-design-system-and-layouts.md) has one renderer draw every drawing
from pack data. The pack has only the flattened overview map: region rectangles with no height,
depth or seat.

## Decision
- The overview may have `views` with a `side` and a `seat` frame. Each frame has a `width` and
  `height`, `shapes`, `places` and `labels`, all in the frame's own units.
- A shape is an outline through its `points` (a curve is given as many points), `closed` or not,
  with a `role`: `structure` (the airplane's outline, filled), `glass`, `seat` or `eye` (one point,
  where the side view's sight line starts). Roles set only how the renderer colors a shape; they
  name no aircraft part, so any pack can draw its own airplane.
- A place puts one overview region (by its `name`) in the frame as one or more closed outlines,
  with an optional `label_at` for its name. The renderer highlights the step's region (blue, a heavy
  outline), names it there when the view does, and in the side view draws a dashed sight line from
  the eye to it, ending in a dot.
- A label is fixed text at a point, with an `anchor` (start, middle or end): "WINDSHIELD",
  "CABIN →", "YOU", "CAPTAIN SIDE".
- Both views carry the caption "Schematic, not to scale", as 3.0's flight deck drawings did.
- `pack check` holds the frames to the pack: every place names a real region, once, with outlines of
  three points or more; every shape, outline and label lies inside its frame; a side frame with
  places has one `eye`.
- The iFly pack's frames are the approved design's side and seat views (the WherePop board in
  [docs/design/boards](../design/boards)), carried over shape for shape: the design is the
  reference, never a manual or image.
- Pack format 4, with 0022. All new fields are optional.

## Consequences
- The Where is this? views are drawn by the shared renderer in both themes and at any size.
- Another aircraft pack supplies its own frames; a pack without them shows the map only.
