// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! App operations that do not depend on where the app runs (decision 0010). The web host
//! (WebAssembly) and a later desktop shell call these and add no logic of their own.

mod actions;
mod app;
mod error;
mod flight;
mod open_steps;
mod packs;
mod phase_view;
mod plans;
mod rail;
mod saved;
mod settings;
mod targets;
#[cfg(test)]
mod test_support;
mod visibility;

pub use actions::{Action, apply};
pub use app::{AppInfo, app_info, example_checklist};
pub use error::AppError;
pub use flight::{FLIGHT_FORMAT, Flight, new_flight};
pub use open_steps::{OpenStep, open_steps};
pub use packs::load_pack;
pub use phase_view::{PhaseLink, PhaseList, PhaseView, StepRow, phase_view};
pub use plans::{parse_plan, plan_url};
pub use rail::{Rail, RailPhase, RailStage, rail};
pub use saved::{Loaded, load_flight, load_settings};
pub use settings::{
    Detail, EssentialsChanges, ListView, MAX_DIMMING, SETTINGS_FORMAT, Settings, Theme,
};
pub use targets::{ItemTarget, step_targets};
pub use visibility::{Shown, in_essentials, passes, shown_steps, step_shows, visible_steps};
