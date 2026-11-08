// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Packs: folders of data that describe an aircraft (or a shared library of content).
//!
//! Authors write checklists in YAML. This crate reads them strictly into the typed
//! model from `checklist-core`, validates them, and writes plain JSON for the app.

pub mod load;
pub mod manifest;
pub mod schema;
pub mod validate;

pub use load::{LoadError, load_pack_dir, parse_checklist_yaml};
pub use manifest::{PackKind, PackManifest};
pub use validate::{Issue, Severity, validate_pack};

/// The pack schema version this build of the app understands.
pub const SCHEMA_VERSION: u32 = 1;

/// A loaded pack: its manifest plus every checklist it defines.
#[derive(Debug, Clone, PartialEq)]
pub struct Pack {
    pub manifest: PackManifest,
    pub checklists: Vec<checklist_core::Checklist>,
    /// Files in the pack folder that are not manifest or checklist data (checked for images).
    pub other_files: Vec<std::path::PathBuf>,
}
