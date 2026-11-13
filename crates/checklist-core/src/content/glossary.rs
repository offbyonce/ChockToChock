// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! `glossary.yaml`: terms and their meanings.

data_types! {
    /// One term and its meaning.
    pub struct GlossaryEntry {
        pub term: String,
        pub meaning: String,
        /// `false` for a unit or a plain abbreviation (LB, FT, ENG): it stays in the glossary but is
        /// never one of a step's terms, which name what a learner needs (panels and systems).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub step_term: Option<bool>,
    }

    /// `glossary.yaml`.
    pub struct GlossaryFile {
        pub terms: Vec<GlossaryEntry>,
    }
}
