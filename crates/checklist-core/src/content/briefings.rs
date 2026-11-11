// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! `briefings.yaml`: crew briefings.

data_types! {
    /// One line of a briefing: the letter of its mnemonic and the rest of the word.
    pub struct BriefItem {
        /// Unique in the pack. Saved progress uses it, so it never changes once published.
        pub id: String,
        pub lead: String,
        pub rest: String,
        /// What to cover under this line.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub detail: Option<String>,
    }

    /// A crew briefing, shown in one phase.
    pub struct Briefing {
        pub id: String,
        pub title: String,
        pub mnemonic: String,
        pub phase: String,
        pub items: Vec<BriefItem>,
    }

    /// `briefings.yaml`.
    pub struct BriefingsFile {
        pub briefings: Vec<Briefing>,
    }
}
