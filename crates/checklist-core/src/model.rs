// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Checklist data shapes. These Rust types are the single source of truth:
//! the pack JSON Schema and the TypeScript types are generated from them.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// A stable step id: `<pack-id>.<phase>.<short-name>`, e.g. `demo-aircraft.preflight.irs-mode`.
/// An id never changes once published, even if the step is reworded or moved to another phase.
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

/// The highest route row a CDU field can refer to.
pub const MAX_ROUTE_LEG: u32 = 30;

data_types! {
    /// What kind of list this is.
    #[serde(rename_all = "snake_case")]
    pub enum ListKind {
        /// Read and confirm after the work is done.
        Checklist,
        /// A sequence of actions done from memory or by following along.
        Flow,
        /// For abnormal situations.
        #[serde(rename = "non-normal")]
        NonNormal,
    }

    /// What a step or light note was confirmed against (decision 0013, which defines each kind).
    /// The list grows by adding a kind there and here.
    #[derive(Copy, Eq, PartialOrd, Ord)]
    #[serde(rename_all = "kebab-case")]
    pub enum ConfirmationKind {
        /// Matches the aircraft developer's documentation for the simulated aircraft.
        VendorDocs,
        /// Matches an official public document from an aviation authority.
        RegulatorDocs,
    }

    /// An internal maintenance record (decision 0013): the kind of source and the day it was
    /// recorded, never who or which document. The app never shows it.
    pub struct Confirmation {
        pub kind: ConfirmationKind,
        /// `YYYY-MM-DD`.
        pub date: String,
    }

    /// When a step still shows in quick mode.
    #[derive(Copy, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum Quick {
        Always,
        Turnaround,
    }

    /// One test against a pilot-chosen option. The options are declared by the pack.
    #[serde(untagged)]
    pub enum OptionTest {
        Equals { option: String, equals: String },
        OneOf { option: String, one_of: Vec<String> },
    }

    /// When something is shown. Deliberately small: tests combine one level deep, with no
    /// negation, comparisons, step state or expressions.
    #[serde(untagged)]
    pub enum ShowIf {
        Test(OptionTest),
        AllOf { all_of: Vec<OptionTest> },
        AnyOf { any_of: Vec<OptionTest> },
    }

    /// The fixed list of flight values the app can fill in. Packs cannot add names; a new one
    /// needs an app release.
    #[derive(Copy, Eq, PartialOrd, Ord)]
    #[serde(rename_all = "snake_case")]
    pub enum FillField {
        /// A blank the pilot types in, with no linked value. Only valid in a step's `fill`.
        Manual,
        Origin,
        Destination,
        FlightNumber,
        Route,
        Runway,
        Sid,
        Transition,
        InitialAltitude,
        Squawk,
        GroundFrequency,
        DepartureFrequency,
        Qnh,
        Heading,
        TakeoffFlaps,
        LandingFlaps,
        V1,
        Vr,
        V2,
        Vref,
        Cg,
        TakeoffTrim,
        Autobrake,
        Minimums,
        LandingAltitude,
        MinimumTakeoffFuel,
        Fuel,
        Zfw,
        TakeoffWeight,
        Passengers,
        Cargo,
        Reserves,
        CostIndex,
        CruiseLevel,
        CruiseWind,
        AssumedTemperature,
        ThrustReductionAltitude,
        AccelerationAltitude,
        IsaDeviation,
        TransitionAltitude,
        TransitionLevel,
        /// `LB` or `KG`, for fuel and weights.
        WeightUnit,
        /// `inHg` or `hPa`, for the QNH.
        QnhUnit,
        /// `BARO` or `RADIO`, for the minimums.
        MinimumsType,
        /// Fuel in thousands, rounded up to the next tenth (calculated).
        FuelThousands,
        /// Fuel plus zero fuel weight, in thousands (calculated).
        GrossWeight,
        /// Outside air temperature at top of climb (calculated).
        TopOfClimbOat,
        /// Latitude and longitude of the departure airport.
        OriginPosition,
        /// The navigation database cycle (calculated).
        NavDatabase,
        /// The dates that cycle is active (calculated).
        NavActiveDates,
        UtcTime,
        UtcDate,
        /// One row of the route. Only valid in CDU text, with an `index` and a `part`.
        RouteLeg,
    }

    /// One piece of a hint shown beside a step, e.g. `GND` and the ground frequency.
    pub struct HintPart {
        /// Literal text before the value, including any space, e.g. `SQUAWK `.
        pub prefix: String,
        pub field: FillField,
    }

    /// A stop on the walkaround drawing.
    pub struct WalkaroundPoint {
        pub x: u32,
        pub y: u32,
    }

    /// Which of several ticked steps starts a timer.
    #[derive(Copy, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum AutoStartPick {
        Earliest,
        Latest,
    }

    /// Starts a timer from the time earlier steps were ticked.
    pub struct AutoStart {
        pub from: Vec<StepId>,
        pub pick: AutoStartPick,
    }

    /// A wait attached to a step.
    pub struct Timer {
        pub seconds: u32,
        pub label: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub note: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub auto_start: Option<AutoStart>,
    }

    /// The state a step expects a light in.
    #[derive(Copy, Eq)]
    #[serde(rename_all = "snake_case")]
    pub enum LightState {
        On,
        Out,
    }

    /// One panel item a step involves, and what the step asks of it (decision 0018).
    pub struct StepTarget {
        /// The item, as `panel/section/LABEL/n`: the n-th item (from 1) with that label in the
        /// section, as light notes name them.
        pub at: String,
        /// The position the step sets a switch, selector, lever or handle to: one of its
        /// `positions`.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub set: Option<String>,
        /// The state the step expects a light in. Leave it out for a light that may cycle (window
        /// heat ON lights), so a drawing never claims a state the light may not be in.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub light: Option<LightState>,
        /// The step's wording names one of the item's positions, but leaves it open: a condition
        /// ("ON IF MORE THAN 1,000 LB"), an option, one engine of two, or the word belongs to
        /// something else. The drawing claims no position, and `pack check` accepts the wording.
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        pub open: bool,
        /// Applies only when a decision step has answered an option with this value (decision
        /// 0022). Until it is answered, the item's target without `when` (`open: true`) applies.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub when: Option<TargetWhen>,
    }

    /// The answer a target depends on: an option and one of its values (decision 0022).
    pub struct TargetWhen {
        pub option: String,
        pub equals: String,
    }

    /// One challenge and response line.
    // These flags are independent facts from pack data; a step can have any mix of them.
    #[allow(clippy::struct_excessive_bools)]
    pub struct Step {
        pub id: StepId,
        /// Printed number; absent for steps that are not numbered.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub number: Option<u32>,
        /// Starts a new titled section at this step.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub section: Option<String>,
        /// The challenge, e.g. "IRS mode selectors".
        pub text: String,
        /// The expected response or position, e.g. "NAV". Always text, never a boolean.
        /// `___` marks a blank the pilot fills in.
        pub expect: String,
        /// Ids from the pack's controls file.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub controls: Vec<String>,
        /// The panel items the step involves and the state it asks of each, which the control
        /// drawings show exactly (decision 0018). `pack check` holds them to the step's wording.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub targets: Vec<StepTarget>,
        /// One field per `___` in `expect`, in order.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub fill: Vec<FillField>,
        /// Flight values shown beside the expected response.
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub hint: Vec<HintPart>,
        /// Id of a CDU walkthrough this step can open.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub opens: Option<String>,
        /// In a memory list: read this step rather than recalling it.
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        pub non_recall: bool,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub quick: Option<Quick>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub show_if: Option<ShowIf>,
        /// A decision step: it asks the pilot this option (by id), and the answer sets it for the
        /// flight. Targets elsewhere may depend on the answer (decision 0022).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub decide: Option<String>,
        /// Optional note shown under the step.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub note: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub why: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub check: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub check_detail: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub if_not: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub if_not_detail: Option<String>,
        /// Id of a shared "how to do it" note.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub how: Option<String>,
        /// Id of a shared explanation.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub explain: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub walkaround: Option<WalkaroundPoint>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub look_for: Option<String>,
        /// Overrides the location text of the step's first control.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub location: Option<String>,
        /// What the crew says.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub callout: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub timer: Option<Timer>,
        /// Panel lights this step refers to: control id, then light labels.
        #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
        pub panel_lights: BTreeMap<String, Vec<String>>,
        /// The simulator does not model this step, so it is left out of progress.
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        pub not_simulated: bool,
        /// Internal record of what this step was confirmed against (decision 0013).
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub confirmed_by: Vec<Confirmation>,
        /// The simulator simplifies the real aircraft here.
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        pub sim_simplified: bool,
        /// The step's note describes how the simulator behaves, not the real aircraft.
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        pub simulator_model: bool,
    }

    /// An ordered list of steps: a checklist, a flow or a non-normal procedure.
    pub struct Checklist {
        pub id: String,
        pub title: String,
        pub kind: ListKind,
        /// Kinds of material this list was checked against, from the tags the pack declares.
        /// Never a document name or a link.
        pub basis: Vec<String>,
        /// Part of the memory items.
        #[serde(default, skip_serializing_if = "std::ops::Not::not")]
        pub memory: bool,
        /// When to use this list.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub when: Option<String>,
        /// Why this list matters.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub why: Option<String>,
        pub steps: Vec<Step>,
    }
}

impl Step {
    /// A step with only the required fields set.
    pub fn new(id: impl Into<String>, text: impl Into<String>, expect: impl Into<String>) -> Self {
        Self {
            id: StepId::new(id),
            number: None,
            section: None,
            text: text.into(),
            expect: expect.into(),
            controls: Vec::new(),
            targets: Vec::new(),
            fill: Vec::new(),
            hint: Vec::new(),
            opens: None,
            non_recall: false,
            quick: None,
            show_if: None,
            decide: None,
            note: None,
            why: None,
            check: None,
            check_detail: None,
            if_not: None,
            if_not_detail: None,
            how: None,
            explain: None,
            walkaround: None,
            look_for: None,
            location: None,
            callout: None,
            timer: None,
            panel_lights: BTreeMap::new(),
            not_simulated: false,
            confirmed_by: Vec::new(),
            sim_simplified: false,
            simulator_model: false,
        }
    }
}

impl Checklist {
    pub fn step(&self, id: &StepId) -> Option<&Step> {
        self.steps.iter().find(|s| &s.id == id)
    }

    pub fn contains(&self, id: &StepId) -> bool {
        self.step(id).is_some()
    }
}
