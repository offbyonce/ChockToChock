// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

/**
 * Whether a step row carries the blue bar: the bar marks the step the side pane shows, as in 3.0.
 * That is the selected step, or the current step while nothing is selected.
 */
export const marksStep = (selected: string | null, id: string, state: string) =>
  selected === null ? state === "current" : selected === id;
