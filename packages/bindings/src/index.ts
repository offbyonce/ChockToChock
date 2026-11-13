// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Types generated from Rust (ts-rs) under ./generated; do not edit them by hand. Run
// `just bindings` after changing a Rust type. Calls into Rust go through the host the interface
// is mounted with (packages/ui/src/host.ts), so this package has no runtime dependencies.

import type { AppError } from "./generated/AppError";
import type { AppInfo } from "./generated/AppInfo";
import type { Checklist } from "./generated/Checklist";

export type { Action } from "./generated/Action";
export type { Control } from "./generated/Control";
export type { ControlsFile } from "./generated/ControlsFile";
export type { Detail } from "./generated/Detail";
export type { ExplainNote } from "./generated/ExplainNote";
export type { FillField } from "./generated/FillField";
export type { FlatPack } from "./generated/FlatPack";
export type { Flight } from "./generated/Flight";
export type { ItemTarget } from "./generated/ItemTarget";
export type { LabelAnchor } from "./generated/LabelAnchor";
export type { ListKind } from "./generated/ListKind";
export type { ListView } from "./generated/ListView";
export type { Loaded } from "./generated/Loaded";
export type { MapInSteps } from "./generated/MapInSteps";
export type { OpenStep } from "./generated/OpenStep";
export type { OptionChoice } from "./generated/OptionChoice";
export type { Overview } from "./generated/Overview";
export type { OverviewArea } from "./generated/OverviewArea";
export type { OverviewRegion } from "./generated/OverviewRegion";
export type { OverviewViews } from "./generated/OverviewViews";
export type { PackSummary } from "./generated/PackSummary";
export type { Panel } from "./generated/Panel";
export type { PanelItem } from "./generated/PanelItem";
export type { PanelSection } from "./generated/PanelSection";
export type { PhaseLink } from "./generated/PhaseLink";
export type { PhaseList } from "./generated/PhaseList";
export type { PhaseView } from "./generated/PhaseView";
export type { Plan } from "./generated/Plan";
export type { PlanValues } from "./generated/PlanValues";
export type { Progress } from "./generated/Progress";
export type { Rail } from "./generated/Rail";
export type { RailPhase } from "./generated/RailPhase";
export type { RailStage } from "./generated/RailStage";
export type { Reply } from "./generated/Reply";
export type { SavedPlan } from "./generated/SavedPlan";
export type { Settings } from "./generated/Settings";
export type { Step } from "./generated/Step";
export type { StepId } from "./generated/StepId";
export type { StepRow } from "./generated/StepRow";
export type { StepState } from "./generated/StepState";
export type { StripPart } from "./generated/StripPart";
export type { Theme } from "./generated/Theme";
export type { UserOption } from "./generated/UserOption";
export type { ViewFrame } from "./generated/ViewFrame";
export type { ViewLabel } from "./generated/ViewLabel";
export type { ViewPlace } from "./generated/ViewPlace";
export type { ViewRole } from "./generated/ViewRole";
export type { ViewShape } from "./generated/ViewShape";
export type { Zone } from "./generated/Zone";
export type { AppError, AppInfo, Checklist };

/** True when a rejected call carries one of our tagged errors. */
export function isAppError(value: unknown): value is AppError {
  return typeof value === "object" && value !== null && "kind" in value;
}
