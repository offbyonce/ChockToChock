// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! The one error type that crosses into the interface. Each crate keeps its own
//! error enum; this converts them into a `{ kind, message }` the interface can branch on.

use serde::Serialize;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS, thiserror::Error)]
#[serde(tag = "kind", content = "message")]
#[ts(export)]
pub enum AppError {
    /// A pack failed to load or did not pass validation.
    #[error("{0}")]
    PackInvalid(String),
}

impl From<pack::LoadError> for AppError {
    fn from(e: pack::LoadError) -> Self {
        Self::PackInvalid(e.to_string())
    }
}
