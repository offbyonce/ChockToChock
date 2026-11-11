// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! `start-states.yaml`: ready-made starting points.

use std::collections::BTreeMap;

use crate::model::StepId;

data_types! {
    /// A ready-made point to start from, with earlier steps already accounted for.
    pub struct StartState {
        pub id: String,
        pub title: String,
        pub description: String,
        /// Options this start selects.
        #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
        pub set: BTreeMap<String, String>,
        /// The phase to open.
        pub phase: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub done: Vec<StepId>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub skip: Vec<StepId>,
    }

    /// `start-states.yaml`.
    pub struct StartStatesFile {
        pub states: Vec<StartState>,
    }
}
