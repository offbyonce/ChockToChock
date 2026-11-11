// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Types generated from Rust (ts-rs) under ./generated; do not edit them by hand. Run
// `just bindings` after changing a Rust type. Calls into Rust go through the host the interface
// is mounted with (packages/ui/src/host.ts), so this package has no runtime dependencies.

import type { AppError } from "./generated/AppError";
import type { AppInfo } from "./generated/AppInfo";
import type { Checklist } from "./generated/Checklist";

export type { Ofp } from "./generated/Ofp";
export type { Step } from "./generated/Step";
export type { StepId } from "./generated/StepId";
export type { AppError, AppInfo, Checklist };

/** True when a rejected call carries one of our tagged errors. */
export function isAppError(value: unknown): value is AppError {
  return typeof value === "object" && value !== null && "kind" in value;
}
