// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Commands the interface may call. Each one is listed in `build.rs` and granted in
//! `capabilities/main.json`; anything else is refused. Each wraps one `app-core` function.

use app_core::{AppError, AppInfo};
use checklist_core::Checklist;

#[tauri::command]
pub fn app_info() -> AppInfo {
    app_core::app_info()
}

#[tauri::command]
pub fn example_checklist() -> Result<Checklist, AppError> {
    app_core::example_checklist()
}
