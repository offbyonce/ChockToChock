// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! `pack.json`: who a pack is, what it needs, and under what terms it ships.

use std::collections::BTreeMap;

use schemars::JsonSchema;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "snake_case")]
pub enum PackKind {
    /// A flyable aircraft (e.g. `ifly-737-max8`).
    Aircraft,
    /// Shared content other packs include (e.g. `lib.boeing-cdu`).
    Library,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PackManifest {
    /// Optional path to `pack.schema.json`, so editors offer autocomplete. Ignored by the app.
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<String>,
    /// Lower-case id with dots and dashes, e.g. `ifly-737-max8` or `lib.boeing-cdu`.
    pub id: String,
    pub name: String,
    #[schemars(with = "String")]
    #[cfg_attr(feature = "ts", ts(type = "string"))]
    pub version: Version,
    /// Pack format version; the app rejects packs newer than it understands.
    pub schema_version: u32,
    pub kind: PackKind,
    /// App versions this pack works with, e.g. `>=0.1, <1`.
    #[schemars(with = "String")]
    #[cfg_attr(feature = "ts", ts(type = "string"))]
    pub engine: VersionReq,
    /// Other packs this one includes, with the versions it accepts.
    #[serde(default)]
    #[schemars(with = "BTreeMap<String, String>")]
    #[cfg_attr(feature = "ts", ts(type = "Record<string, string>"))]
    pub dependencies: BTreeMap<String, VersionReq>,
    /// SPDX license id for the pack content, e.g. `CC-BY-NC-SA-4.0`.
    pub license: String,
    /// Credit and terms notes.
    #[serde(default)]
    pub attribution: Vec<String>,
    /// The `basis` tags this pack's lists may use: kinds of material, never document names.
    #[serde(default)]
    pub basis_tags: Vec<String>,
    /// Checklist files, in display order, relative to the pack folder.
    pub checklists: Vec<String>,
    /// Phases, stages and options. Required for an aircraft pack.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phases: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub controls: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Panel files, in display order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub panels: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lights: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub faults: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub glossary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub briefings: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_states: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cdu: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aliases: Option<String>,
}

impl PackManifest {
    /// Every data file this manifest lists, relative to the pack folder.
    pub fn data_files(&self) -> Vec<&str> {
        let single = [
            &self.phases,
            &self.controls,
            &self.notes,
            &self.lights,
            &self.faults,
            &self.glossary,
            &self.briefings,
            &self.start_states,
            &self.cdu,
            &self.aliases,
        ];
        self.checklists
            .iter()
            .chain(&self.panels)
            .chain(single.into_iter().flatten())
            .map(String::as_str)
            .collect()
    }
}
