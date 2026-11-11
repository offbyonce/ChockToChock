// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! The checklist domain, with no I/O, no async and no clock.
//!
//! Everything aircraft-specific arrives as data (from packs); nothing in here
//! names an aircraft. Anything time-based is passed in by the caller, which keeps
//! every rule testable without a window.

/// Adds the derives every pack data type needs, and rejects unknown fields when reading.
macro_rules! data_types {
    ($($item:item)*) => {
        $(
            #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
            #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
            #[cfg_attr(feature = "ts", derive(ts_rs::TS), ts(export))]
            #[serde(deny_unknown_fields)]
            $item
        )*
    };
}

pub mod content;
pub mod model;
pub mod session;

pub use content::{
    Alias, AliasKind, AliasesFile, BriefItem, Briefing, BriefingsFile, CduField, CduFile,
    CduInline, CduPage, CduPart, CduRouteCount, CduRow, CduScreen, CduStyle, CduStyled, CduText,
    CduWalkthrough, CduWhen, Control, ControlsFile, ExplainNote, Fault, FaultsFile, GlossaryEntry,
    GlossaryFile, HowNote, IntroParagraph, LightColor, LightNote, LightsFile, NotesFile, Overview,
    OverviewArea, OverviewCaption, Panel, PanelItem, PanelItemKind, PanelSection, Phase,
    PhasesFile, RouteLegCount, RouteLegPart, Stage, StagePhase, StartState, StartStatesFile,
    UserOption, Zone,
};
pub use model::{
    AutoStart, AutoStartPick, Checklist, Confirmation, ConfirmationKind, FillField, HintPart,
    ListKind, MAX_ROUTE_LEG, OptionTest, Quick, ShowIf, Step, StepId, Timer, WalkaroundPoint,
};
pub use session::{Progress, Session, SessionError};
