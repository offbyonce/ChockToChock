// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

/**
 * How the slide-out opens (ROUND6.md): on the panel (Show panel, a preview, P), with Where it is
 * expanded (the location line, W), or on one control's card (tapping a control).
 */
export type Opening = {
  entry: "panel" | "where" | "control";
  /** The panel to show, when the step's controls are on several. */
  panel?: string;
  /** The control whose card opens. */
  control?: { section: string; item: number };
};
