// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! App operations that do not depend on where the app runs (decision 0010). The web host
//! (WebAssembly) and a later desktop shell call these and add no logic of their own.

mod app;
mod error;

pub use app::{AppInfo, app_info, example_checklist};
pub use error::AppError;
