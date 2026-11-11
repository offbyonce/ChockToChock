// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! `notes.yaml`: shared how-to notes and explanations.

data_types! {
    /// Shared "how to do it" instructions.
    pub struct HowNote {
        pub id: String,
        pub steps: Vec<String>,
    }

    /// A shared explanation of a test or action.
    pub struct ExplainNote {
        pub id: String,
        pub what: String,
        pub why: String,
        /// What a good result looks like.
        pub success: String,
        /// What a bad result can mean.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub failure: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub check: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub check_detail: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub if_not: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub if_not_detail: Option<String>,
    }

    /// `notes.yaml`.
    pub struct NotesFile {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub how: Vec<HowNote>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub explain: Vec<ExplainNote>,
    }
}
