// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Commands the interface may call. Each one is listed in `build.rs` and granted in
//! `capabilities/main.json`; anything else is refused.

use std::path::Path;

use checklist_core::Checklist;
use serde::Serialize;
use ts_rs::TS;

use crate::AppError;

/// Basic facts for the About box and the window title.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
    pub pack_schema_version: u32,
}

#[tauri::command]
pub fn app_info() -> AppInfo {
    AppInfo {
        name: "Chock to Chock".to_owned(),
        version: env!("CARGO_PKG_VERSION").to_owned(),
        pack_schema_version: pack::SCHEMA_VERSION,
    }
}

/// The bundled format-demo checklist, parsed through the same strict loader as real packs.
#[tauri::command]
pub fn example_checklist() -> Result<Checklist, AppError> {
    const YAML: &str = include_str!("../../../../packs/example-trainer/before-start.yaml");
    Ok(pack::parse_checklist_yaml(
        YAML,
        Path::new("example-trainer/before-start.yaml"),
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_checklist_loads() {
        let list = example_checklist().unwrap();
        assert_eq!(list.id, "before-start");
        assert_eq!(list.steps.len(), 3, "the example pack has three steps");
    }

    #[test]
    fn errors_serialise_as_kind_and_message() {
        let json = serde_json::to_string(&AppError::PackInvalid("bad".into())).unwrap();
        assert_eq!(json, r#"{"kind":"PackInvalid","message":"bad"}"#);
    }
}
