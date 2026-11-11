// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! `aliases.yaml`: old saved ids and what replaced them.

data_types! {
    /// What an alias renames. Covers everything the previous release saved.
    #[derive(Copy, Eq, PartialOrd, Ord)]
    #[serde(rename_all = "snake_case")]
    pub enum AliasKind {
        /// `to` is a step id.
        Step,
        /// `to` is a briefing item id.
        BriefingItem,
        /// A value typed into a blank. `to` is `<step-id>#<n>`, counting blanks from 0.
        Blank,
        /// `to` is a start state id.
        StartState,
        /// A chosen option. `from` and `to` are `<option>:<value>`.
        OptionValue,
        /// A saved flight value. `to` is a name from the app's fill list.
        FillField,
        /// `to` is a list id.
        List,
        /// `to` is a phase id.
        Phase,
        /// `to` is a zone id.
        Zone,
    }

    /// An old saved id and what replaced it. Lets saved progress carry over.
    pub struct Alias {
        pub kind: AliasKind,
        pub from: String,
        pub to: String,
    }

    /// `aliases.yaml`.
    pub struct AliasesFile {
        pub aliases: Vec<Alias>,
    }
}
