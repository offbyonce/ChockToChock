// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! `glossary.yaml`: terms and their meanings.

data_types! {
    /// One term and its meaning.
    pub struct GlossaryEntry {
        pub term: String,
        pub meaning: String,
    }

    /// `glossary.yaml`.
    pub struct GlossaryFile {
        pub terms: Vec<GlossaryEntry>,
    }
}
