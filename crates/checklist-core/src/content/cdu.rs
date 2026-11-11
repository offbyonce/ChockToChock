// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! `cdu.yaml`: CDU walkthroughs and the text they show.

use crate::model::FillField;

data_types! {
    /// How text appears on the CDU screen.
    #[derive(Copy, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum CduStyle {
        /// System data the pilot does not enter.
        Data,
        /// Values the system has calculated or accepted.
        Accent,
        /// Values the pilot enters.
        User,
    }

    /// Fixed text in a style.
    pub struct CduStyled {
        pub text: String,
        pub style: CduStyle,
    }

    /// Which half of a route row a field shows.
    #[derive(Copy, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum RouteLegPart {
        Via,
        To,
    }

    /// A live flight value. Calculations happen in the app, never in the pack.
    pub struct CduField {
        pub field: FillField,
        /// For `route_leg`: the row, from 1 to `MAX_ROUTE_LEG`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub index: Option<u32>,
        /// For `route_leg`: which half of the row.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub part: Option<RouteLegPart>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub style: Option<CduStyle>,
        /// Literal text before the value, e.g. `RW`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub before: Option<String>,
        /// Literal text after the value, e.g. a degree sign.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub after: Option<String>,
        /// Shown when the value is not known.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub empty: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub empty_style: Option<CduStyle>,
    }

    /// A count the app works out from the route.
    #[derive(Copy, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum RouteLegCount {
        /// How many pages the route legs and departure need, at least 1.
        Pages,
        /// How many route rows do not fit on the first page.
        Overflow,
    }

    /// A number calculated from the route. The rows per page are the aircraft's, so they
    /// come from the pack.
    pub struct CduRouteCount {
        pub route_legs: RouteLegCount,
        pub per_page: u32,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub style: Option<CduStyle>,
        /// Literal text before the number.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub before: Option<String>,
        /// Literal text after the number.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub after: Option<String>,
        /// Shown instead when the count is zero.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub empty: Option<String>,
    }

    /// Text or a live value, in any style. Mix several in a row to style them differently.
    #[serde(untagged)]
    pub enum CduInline {
        Text(String),
        Styled(CduStyled),
        Field(CduField),
        RouteCount(CduRouteCount),
    }

    /// Shows one group of parts when flight values are known and another when they are not,
    /// e.g. `<SEL>` beside a runway, or a placeholder until there is a runway. Does not nest.
    pub struct CduWhen {
        /// Every one of these must be known for `show` to appear.
        pub when_known: Vec<FillField>,
        pub show: Vec<CduInline>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub otherwise: Vec<CduInline>,
    }

    /// One piece of CDU text.
    #[serde(untagged)]
    pub enum CduPart {
        Text(String),
        Styled(CduStyled),
        Field(CduField),
        RouteCount(CduRouteCount),
        When(CduWhen),
    }
}

/// CDU text: parts joined in order, so live values never need a template language.
pub type CduText = Vec<CduPart>;

data_types! {
    /// One line select row: a small label line above a value line.
    pub struct CduRow {
        /// Draws a divider instead of the labels; empty text means a plain line.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub separator: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub left_label: CduText,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub left: CduText,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub right_label: CduText,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub right: CduText,
    }

    /// What the CDU shows on one walkthrough page.
    pub struct CduScreen {
        pub title: CduText,
        pub page: CduText,
        pub rows: Vec<CduRow>,
        /// Line select keys to highlight, e.g. `2L`.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub highlight: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub scratchpad: CduText,
    }

    /// One step of a CDU walkthrough.
    pub struct CduPage {
        /// The CDU page name, e.g. `PERF INIT`.
        pub page: String,
        /// The function key to highlight.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub key: Option<String>,
        pub title: String,
        pub action: CduText,
        pub check: CduText,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub note: CduText,
        pub screen: CduScreen,
    }

    /// A page-by-page guide a step can open.
    pub struct CduWalkthrough {
        pub id: String,
        pub title: String,
        pub pages: Vec<CduPage>,
    }

    /// `cdu.yaml`.
    pub struct CduFile {
        pub walkthroughs: Vec<CduWalkthrough>,
    }
}
