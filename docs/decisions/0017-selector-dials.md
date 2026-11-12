<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0017: Selector dials: every position named, at its real angle

Date: 2026-11-11. Status: proposed. Extends [0014](0014-pack-format-3.md).

## Context
Drawings labeled only the ends and the target of a selector with more than four positions, so
middle ticks had no name. And every dial spread its positions over the top, while real dials do
not: the electrical meters selectors run from STBY PWR at the lower left, over the top, to TEST
at the bottom (DC) or the lower right (AC).

## Decision
- A selector may have `angles`: one per position, in degrees clockwise from straight up, as on the
  panel, each larger than the last, within -180 to 180. Without it, the positions spread over the
  top of the dial as before. Pack format 3; no version change.
- Drawings label every position of a selector, each at its own angle.
- `pack check` flags a selector with a position without a name, and `angles` that do not match
  its positions in number, range or order, or that sit on anything but a selector.
- The iFly electrical meters selectors take their real geometry and rest at AUX BAT and APU GEN
  cold and dark, both pointing straight up, as the vendor's cold and dark panel shows.

## Consequences
- Dials read as on the panel, and no tick is left without a name.
- Crowded dials may need their section's text a step smaller; the layout keeps every label clear.
