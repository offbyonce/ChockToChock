The owner approved a sixth design round, which changes how a step shows its panel and where it is. The handoff is in ~/private/design-r6/. Read ROUND6.md first, then the boards.

Standing rule, restated: the boards are the specification. Match them exactly, in both themes and at every size. The one exception is panel drawings: your renderer's panels are higher quality than the boards' and win. The fuel panel on the round 6 boards is a stand-in. The new label-free panel preview must come from your renderer, from the same panel data, at the same quality as your full panels, just without text. Don't copy the boards' outline. If a board can't be matched exactly, stop and ask before building anything different.

**First, finish batch 2 with these answers:**
1. Page scrollbar: approved. The list and the pane scroll separately with hidden scrollbars, as long as wheel, trackpad, touch and keyboard scrolling feel natural and the edge fades show there's more. Fix the 12 px offsets, the Where is this? position and the PhoneWhere band.
2. CTR lights out of room at 1440: no longer matters. Round 6 removes the drawing from the pane, so don't do any more pane-fit work.
3. Remove the "New panel" tag everywhere. Round 6 also removes the map figure from the pane; leave that for batch 3.
4. Guard glyph: make closed and open obviously different at a glance (closed covers the switch; open is raised on its hinge, clear of it). Show before and after in both themes.
5. Don't build the "Show panel" pop-over. Round 6 replaces it.

Redo the affected pairs, push phase-2/step-7-batch-2, and send the PR text.

**Then batch 3:**
- Copy ~/private/design-r6 into docs/design/round6/, and mark the superseded round 5 boards as superseded in the README.
- Update coverage.md as ROUND6.md says.
- Add the approved tokens (slide-out widths and scrim colors) with Locator.dc.html as the source.
- Start with the dense-panel gate in ROUND6.md: label-free previews of the three densest panels plus the fuel panel, both themes, three sizes, each beside your full panel. Send them and wait for the owner's review before building the rest of round 6 or the rest of batch 3.

Batch 3 then covers the core components and the desktop checklist from round 5 (the step list, header, phase track, flight strip and open-steps list, as already briefed), together with round 6's side pane, slide-out, control card, Flow side pane and Settings row.

Reply with any conflict you see between round 6 and an accepted decision or the pack format, and your questions, before you start batch 3.
