// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Checklist data shapes. These Rust types are the single source of truth:
//! the pack JSON Schema and the TypeScript types are generated from them.

use serde::{Deserialize, Serialize};

/// A stable, namespaced step id such as `ifly-max8.preflight.irs-align`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(transparent)]
pub struct StepId(pub String);

impl StepId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for StepId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Where a step's wording or value came from. Every step needs at least one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(deny_unknown_fields)]
pub struct SourceRef {
    /// Human-readable name, e.g. "iFly 737 MAX 8 tutorial, p. 12".
    pub title: String,
    /// Optional link. Checked against the blocked-source list by `pack check`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// One challenge and response line.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub id: StepId,
    /// The challenge, e.g. "IRS mode selectors".
    pub text: String,
    /// The expected response or position, e.g. "NAV". Always text, never a boolean.
    pub expect: String,
    /// Optional note shown under the step.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(default)]
    pub sources: Vec<SourceRef>,
}

/// An ordered list of steps for one phase of flight.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
#[serde(deny_unknown_fields)]
pub struct Checklist {
    pub id: String,
    pub title: String,
    pub steps: Vec<Step>,
}

impl Checklist {
    pub fn step(&self, id: &StepId) -> Option<&Step> {
        self.steps.iter().find(|s| &s.id == id)
    }

    pub fn contains(&self, id: &StepId) -> bool {
        self.step(id).is_some()
    }
}
