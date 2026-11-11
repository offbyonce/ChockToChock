// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Types generated from Rust (ts-rs) under ./generated; do not edit them by hand. Run
// `just bindings` after changing a Rust type. Calls into Rust go through the host the interface
// is mounted with (packages/ui/src/host.ts), so this package has no runtime dependencies.

import type { AppError } from "./generated/AppError";
import type { AppInfo } from "./generated/AppInfo";
import type { Checklist } from "./generated/Checklist";

export type { Action } from "./generated/Action";
export type { Detail } from "./generated/Detail";
export type { FlatPack } from "./generated/FlatPack";
export type { Flight } from "./generated/Flight";
export type { ListView } from "./generated/ListView";
export type { Loaded } from "./generated/Loaded";
export type { Ofp } from "./generated/Ofp";
export type { PackSummary } from "./generated/PackSummary";
export type { Progress } from "./generated/Progress";
export type { Rail } from "./generated/Rail";
export type { RailPhase } from "./generated/RailPhase";
export type { RailStage } from "./generated/RailStage";
export type { Reply } from "./generated/Reply";
export type { Settings } from "./generated/Settings";
export type { Step } from "./generated/Step";
export type { StepId } from "./generated/StepId";
export type { StepState } from "./generated/StepState";
export type { Theme } from "./generated/Theme";
export type { AppError, AppInfo, Checklist };

/** True when a rejected call carries one of our tagged errors. */
export function isAppError(value: unknown): value is AppError {
  return typeof value === "object" && value !== null && "kind" in value;
}
