// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! `pack.json`: who a pack is, what it needs, and under what terms it ships.

use std::collections::BTreeMap;

use schemars::JsonSchema;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PackKind {
    /// A flyable aircraft (e.g. `ifly-737-max8`).
    Aircraft,
    /// Shared content other packs include (e.g. `lib.boeing-cdu`).
    Library,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PackManifest {
    /// Optional path to `pack.schema.json`, so editors offer autocomplete. Ignored by the app.
    #[serde(rename = "$schema", default, skip_serializing_if = "Option::is_none")]
    pub json_schema: Option<String>,
    /// Lower-case id with dots and dashes, e.g. `ifly-737-max8` or `lib.boeing-cdu`.
    pub id: String,
    pub name: String,
    #[schemars(with = "String")]
    pub version: Version,
    /// Pack format version; the app rejects packs newer than it understands.
    pub schema_version: u32,
    pub kind: PackKind,
    /// App versions this pack works with, e.g. `>=0.1, <1`.
    #[schemars(with = "String")]
    pub engine: VersionReq,
    /// Other packs this one includes, with the versions it accepts.
    #[serde(default)]
    #[schemars(with = "BTreeMap<String, String>")]
    pub dependencies: BTreeMap<String, VersionReq>,
    /// SPDX licence id for the pack content, e.g. `CC-BY-NC-SA-4.0`.
    pub license: String,
    /// Credit and terms notes, e.g. which aircraft vendor's variables are read.
    #[serde(default)]
    pub attribution: Vec<String>,
    /// Checklist files, in display order, relative to the pack folder.
    pub checklists: Vec<String>,
}
