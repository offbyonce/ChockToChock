// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Packs: folders of data that describe an aircraft (or a shared library of content).
//!
//! Authors write pack content in YAML. This crate reads it strictly into the typed
//! model from `checklist-core`, validates them, and writes plain JSON for the app.

pub mod load;
pub mod manifest;
pub mod overlap;
pub mod schema;
pub mod validate;

pub use load::{LoadError, load_pack_dir, parse_checklist_yaml, parse_yaml};
pub use manifest::{PackKind, PackManifest};
pub use validate::{Issue, Severity, validate_pack};

/// The pack schema version this build of the app understands.
pub const SCHEMA_VERSION: u32 = 2;

/// Everything a pack holds besides its manifest and lists. Each part is optional.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct PackContent {
    pub phases: Option<checklist_core::PhasesFile>,
    pub controls: Option<checklist_core::ControlsFile>,
    pub notes: Option<checklist_core::NotesFile>,
    pub panels: Vec<checklist_core::Panel>,
    pub lights: Option<checklist_core::LightsFile>,
    pub faults: Option<checklist_core::FaultsFile>,
    pub glossary: Option<checklist_core::GlossaryFile>,
    pub briefings: Option<checklist_core::BriefingsFile>,
    pub start_states: Option<checklist_core::StartStatesFile>,
    pub cdu: Option<checklist_core::CduFile>,
    pub aliases: Option<checklist_core::AliasesFile>,
}

/// A loaded pack: its manifest plus every checklist and other content it defines.
#[derive(Debug, Clone, PartialEq)]
pub struct Pack {
    pub manifest: PackManifest,
    pub checklists: Vec<checklist_core::Checklist>,
    pub content: PackContent,
    /// Files in the pack folder that the manifest does not list (checked for images).
    pub other_files: Vec<std::path::PathBuf>,
}
