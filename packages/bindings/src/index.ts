// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// The only place the interface calls into Rust. One function per command, so command
// names and types are written once. Types under ./generated come from Rust (ts-rs);
// do not edit them by hand. Run `just bindings` after changing a Rust type.

import { invoke } from "@tauri-apps/api/core";
import type { AppError } from "./generated/AppError";
import type { AppInfo } from "./generated/AppInfo";
import type { Checklist } from "./generated/Checklist";

export type { Ofp } from "./generated/Ofp";
export type { Step } from "./generated/Step";
export type { StepId } from "./generated/StepId";
export type { AppError, AppInfo, Checklist };

/** True when a rejected command carries one of our tagged errors. */
export function isAppError(value: unknown): value is AppError {
  return typeof value === "object" && value !== null && "kind" in value;
}

export function appInfo(): Promise<AppInfo> {
  return invoke<AppInfo>("app_info");
}

export function exampleChecklist(): Promise<Checklist> {
  return invoke<Checklist>("example_checklist");
}
