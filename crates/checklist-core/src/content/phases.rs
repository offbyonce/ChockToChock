// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! `phases.yaml`: options, stages and phases.

use crate::model::ShowIf;

data_types! {
    /// A choice the pilot makes that decides what is shown, e.g. the power source.
    pub struct UserOption {
        pub id: String,
        pub title: String,
        pub values: Vec<String>,
        /// The value used until the pilot chooses. Must be one of `values`.
        pub default: String,
    }

    /// A phase's place in a stage.
    pub struct StagePhase {
        pub phase: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub show_if: Option<ShowIf>,
    }

    /// A group of phases, e.g. "Preflight".
    pub struct Stage {
        pub title: String,
        pub phases: Vec<StagePhase>,
    }

    /// One paragraph of a phase's introduction.
    pub struct IntroParagraph {
        /// A short lead-in shown before the text, e.g. "When to use them".
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub heading: Option<String>,
        pub text: String,
    }

    /// One part of the flight, holding its lists in order.
    pub struct Phase {
        pub id: String,
        pub title: String,
        /// Reference material, outside the normal flow of a flight.
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        pub reference: bool,
        /// Shown at the top of the phase.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub intro: Vec<IntroParagraph>,
        /// Ids of the lists in this phase.
        pub lists: Vec<String>,
    }

    /// `phases.yaml`.
    pub struct PhasesFile {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub options: Vec<UserOption>,
        pub stages: Vec<Stage>,
        pub phases: Vec<Phase>,
    }
}
