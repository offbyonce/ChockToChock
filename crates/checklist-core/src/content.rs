// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Everything in a pack other than the lists of steps: phases, controls, panels, notes and
//! the CDU walkthrough. Each `...File` type is the top level of one YAML file.

use std::collections::BTreeMap;

use crate::model::{FillField, ShowIf, Status, StepId};

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

    /// One part of the flight, holding its lists in order.
    pub struct Phase {
        pub id: String,
        pub title: String,
        /// Reference material, outside the normal flow of a flight.
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        pub reference: bool,
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

    /// An area of the cockpit.
    pub struct Zone {
        pub id: String,
        pub area: String,
        pub title: String,
        pub short_title: String,
    }

    /// A switch, lever or other control a step refers to.
    pub struct Control {
        pub id: String,
        pub name: String,
        /// Absent for controls with no place in the cockpit, such as a briefing.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zone: Option<String>,
        pub location: String,
        pub function: String,
    }

    /// `controls.yaml`.
    pub struct ControlsFile {
        pub zones: Vec<Zone>,
        pub controls: Vec<Control>,
    }

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

    /// What a panel item is, which decides how it is drawn.
    #[derive(Copy, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum PanelItemKind {
        GuardedSwitch,
        ToggleSwitch,
        Selector,
        PushButton,
        Light,
        GreenLight,
        BlueLight,
        Gauge,
        Readout,
        DisplayUnit,
    }

    /// One item on a panel drawing. Positions are our own layout.
    pub struct PanelItem {
        pub kind: PanelItemKind,
        pub label: String,
        pub x: f64,
        pub y: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub control: Option<String>,
        /// The positions it can be set to, as text.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub positions: Option<String>,
        pub description: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub width: Option<f64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub height: Option<f64>,
    }

    /// A labelled section of a panel.
    pub struct PanelSection {
        pub id: String,
        pub title: String,
        pub x: f64,
        pub y: f64,
        pub width: f64,
        pub height: f64,
        pub items: Vec<PanelItem>,
    }

    /// One `panels/*.yaml` file.
    pub struct Panel {
        pub id: String,
        pub title: String,
        pub width: f64,
        pub height: f64,
        pub sections: Vec<PanelSection>,
    }

    /// What a panel light means.
    pub struct LightNote {
        /// `<panel>/<section>/<label>/<n>`, where n counts items with that label from 1.
        pub locator: String,
        pub meaning: String,
        pub when: String,
        pub action: String,
        /// True when seeing this light is normal.
        pub normal: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<Status>,
    }

    /// `lights.yaml`.
    pub struct LightsFile {
        pub lights: Vec<LightNote>,
    }

    /// The likely causes when a step does not give the expected result.
    pub struct Fault {
        pub steps: Vec<StepId>,
        pub causes: Vec<String>,
        pub action: String,
    }

    /// `faults.yaml`.
    pub struct FaultsFile {
        pub faults: Vec<Fault>,
    }

    /// One term and its meaning.
    pub struct GlossaryEntry {
        pub term: String,
        pub meaning: String,
    }

    /// `glossary.yaml`.
    pub struct GlossaryFile {
        pub terms: Vec<GlossaryEntry>,
    }

    /// One line of a briefing: the letter of its mnemonic and the rest of the word.
    pub struct BriefItem {
        /// Unique in the pack. Saved progress uses it, so it never changes once published.
        pub id: String,
        pub lead: String,
        pub rest: String,
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

    /// A live flight value. Calculations happen in the app, never in the pack.
    pub struct CduField {
        pub field: FillField,
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

    /// Text or a live value, in any style. Mix several in a row to style them differently.
    #[serde(untagged)]
    pub enum CduInline {
        Text(String),
        Styled(CduStyled),
        Field(CduField),
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
