// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! The checklist domain, with no I/O, no async and no clock.
//!
//! Everything aircraft-specific arrives as data (from packs); nothing in here
//! names an aircraft. Anything time-based is passed in by the caller, which keeps
//! every rule testable without a window.

pub mod model;
pub mod session;

pub use model::{Checklist, SourceRef, Step, StepId};
pub use session::{Progress, Session, SessionError};
