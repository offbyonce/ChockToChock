// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Packs: folders of data that describe an aircraft (or a shared library of content).
//!
//! Authors write pack content in YAML. This crate reads it strictly into the typed
//! model from `checklist-core`, validates them, and writes plain JSON for the app.

pub mod load;
pub mod manifest;
pub mod schema;
pub mod validate;

pub use load::{LoadError, load_pack_dir, parse_checklist_yaml, parse_yaml};
pub use manifest::{PackKind, PackManifest};
pub use validate::{Issue, Severity, validate_pack};

/// The pack schema version this build of the app understands.
pub const SCHEMA_VERSION: u32 = 3;

/// Everything a pack holds besides its manifest and lists. Each part is optional.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(deny_unknown_fields)]
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

/// A pack as the app receives it: everything in one JSON file, written by `pack build` once the
/// pack has passed `pack check`. Library packs and overrides will be resolved into it when they
/// exist, so the app never reads YAML or follows references between packs.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(deny_unknown_fields)]
pub struct FlatPack {
    pub manifest: PackManifest,
    pub lists: Vec<checklist_core::Checklist>,
    pub content: PackContent,
}

impl From<Pack> for FlatPack {
    fn from(pack: Pack) -> Self {
        Self {
            manifest: pack.manifest,
            lists: pack.checklists,
            content: pack.content,
        }
    }
}

/// One built pack in `index.json`, which `pack build` writes beside the packs so the app can
/// find them without naming any aircraft.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(deny_unknown_fields)]
pub struct PackSummary {
    pub id: String,
    pub name: String,
    #[cfg_attr(feature = "ts", ts(type = "string"))]
    pub version: semver::Version,
    pub kind: PackKind,
}

impl From<&PackManifest> for PackSummary {
    fn from(m: &PackManifest) -> Self {
        Self {
            id: m.id.clone(),
            name: m.name.clone(),
            version: m.version.clone(),
            kind: m.kind,
        }
    }
}
