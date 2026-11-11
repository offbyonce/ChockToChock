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
    /// A SimBrief plan could not be requested or read.
    #[error("{0}")]
    FlightPlan(String),
    /// The interface asked for a change the flight cannot make (a bug, not a pilot error).
    #[error("{0}")]
    InvalidAction(String),
}

impl From<pack::LoadError> for AppError {
    fn from(e: pack::LoadError) -> Self {
        Self::PackInvalid(e.to_string())
    }
}

impl From<simbrief::OfpError> for AppError {
    fn from(e: simbrief::OfpError) -> Self {
        Self::FlightPlan(e.to_string())
    }
}
