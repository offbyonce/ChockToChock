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
  `height`, `shapes` and `places`, all in the frame's own units.
- A shape is a list of `points` (one point is a dot, two a line, more a path, `closed` to close
  it), a `role` (`structure`, `glass`, `seat`, `eye` or `sight`) and an optional `label`. Roles
  set only how the renderer colors and draws a shape; they name no aircraft part, so any pack can
  draw its own airplane.
- A place puts one overview region (by its `name`) in the frame as a rectangle. The renderer
  highlights the place of the step's region; in the side view it draws a sight line from the `eye`
  shape to it.
- Both views carry the caption "Schematic, not to scale", as 3.0's flight deck drawings did.
- `pack check` holds the frames to the pack: every place names a real region, no region twice, and
  every shape and place lies inside its frame; a side frame with places has an `eye`.
- The iFly pack's frames are our own schematic, written from the regions' `locate` sentences and
  the overview's arrangement, never traced from any manual or image.
- Pack format 4, with 0022. All new fields are optional.

## Consequences
- The Where is this? views are drawn by the shared renderer in both themes and at any size.
- Another aircraft pack supplies its own frames; a pack without them shows the map only.
