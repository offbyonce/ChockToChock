// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Everything in a pack other than the lists of steps: phases, controls, panels, notes and
//! the CDU walkthrough. Each `...File` type is the top level of one YAML file.

mod aliases;
mod briefings;
mod cdu;
mod controls;
mod faults;
mod glossary;
mod lights;
mod notes;
mod panels;
mod phases;
mod start_states;

pub use aliases::{Alias, AliasKind, AliasesFile};
pub use briefings::{BriefItem, Briefing, BriefingsFile};
pub use cdu::{
    CduField, CduFile, CduInline, CduPage, CduPart, CduRouteCount, CduRow, CduScreen, CduStyle,
    CduStyled, CduText, CduWalkthrough, CduWhen, RouteLegCount, RouteLegPart,
};
pub use controls::{
    Control, ControlsFile, Overview, OverviewArea, OverviewCaption, OverviewRegion, OverviewViews,
    ViewFrame, ViewPlace, ViewRole, ViewShape, Zone,
};
pub use faults::{Fault, FaultsFile};
pub use glossary::{GlossaryEntry, GlossaryFile};
pub use lights::{LightNote, LightsFile};
pub use notes::{ExplainNote, HowNote, NotesFile};
pub use panels::{LightColor, Panel, PanelItem, PanelItemKind, PanelSection};
pub use phases::{IntroParagraph, OptionChoice, Phase, PhasesFile, Stage, StagePhase, UserOption};
pub use start_states::{StartState, StartStatesFile};
