// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! The phase rail: stages and the phases that show in them for the pilot's options, with
//! progress, plus the reference phases behind the Emergency button (decision 0012).

use checklist_core::Progress;
use pack::FlatPack;
use serde::Serialize;
use ts_rs::TS;

use crate::flight::Flight;
use crate::settings::Settings;
use crate::visibility::{Shown, passes, shown_steps};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct RailPhase {
    pub id: String,
    pub title: String,
    pub progress: Progress,
    /// Every counted step is done.
    pub complete: bool,
    /// Steps still open (skipped or to do).
    pub open: usize,
    /// Of those, the ones the pilot skipped.
    pub skipped: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct RailStage {
    pub title: String,
    pub phases: Vec<RailPhase>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Rail {
    pub stages: Vec<RailStage>,
    /// Reference phases (memory items, quick reference, non-normal procedures).
    pub reference: Vec<RailPhase>,
}

fn phase_entry(pack: &FlatPack, flight: &Flight, shown: Shown, id: &str) -> Option<RailPhase> {
    let phase = pack
        .content
        .phases
        .as_ref()?
        .phases
        .iter()
        .find(|p| p.id == id)?;
    let progress = phase
        .lists
        .iter()
        .filter_map(|list_id| pack.lists.iter().find(|l| &l.id == list_id))
        .map(|list| {
            flight
                .session
                .progress(shown_steps(list, &flight.options, shown, phase.reference))
        })
        .fold(Progress::default(), Progress::plus);
    Some(RailPhase {
        id: phase.id.clone(),
        title: phase.title.clone(),
        progress,
        complete: progress.complete(),
        open: progress.open(),
        skipped: progress.skipped,
    })
}

/// The rail for this flight, counting the steps the pilot chose to see (Full or Essentials).
pub fn rail(pack: &FlatPack, flight: &Flight, settings: &Settings) -> Rail {
    let shown = Shown::new(pack, settings);
    let Some(file) = &pack.content.phases else {
        return Rail {
            stages: vec![],
            reference: vec![],
        };
    };
    let stages = file
        .stages
        .iter()
        .map(|stage| RailStage {
            title: stage.title.clone(),
            phases: stage
                .phases
                .iter()
                .filter(|place| {
                    place
                        .show_if
                        .as_ref()
                        .is_none_or(|s| passes(s, &flight.options))
                })
                .filter_map(|place| phase_entry(pack, flight, shown, &place.phase))
                // In Essentials, a phase with no Essentials steps is left out, as in 3.0.
                .filter(|p| !shown.essentials || p.progress.total > 0)
                .collect(),
        })
        .collect();
    let reference = file
        .phases
        .iter()
        .filter(|p| p.reference)
        .filter_map(|p| phase_entry(pack, flight, shown, &p.id))
        .collect();
    Rail { stages, reference }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::{Action, apply};
    use crate::flight::new_flight;
    use crate::test_support::ifly_pack;

    fn titles(rail: &Rail, stage: &str) -> Vec<String> {
        rail.stages
            .iter()
            .find(|s| s.title == stage)
            .unwrap()
            .phases
            .iter()
            .map(|p| p.id.clone())
            .collect()
    }

    #[test]
    fn the_power_option_decides_which_power_phase_shows() {
        let pack = ifly_pack();
        let flight = new_flight(&pack);
        let gpu = apply(
            &pack,
            flight.clone(),
            Action::SetOption {
                option: "power".into(),
                value: "gpu".into(),
            },
        )
        .unwrap();
        let apu = apply(
            &pack,
            flight,
            Action::SetOption {
                option: "power".into(),
                value: "apu".into(),
            },
        )
        .unwrap();
        assert!(
            titles(&rail(&pack, &gpu, &Settings::default()), "Preflight")
                .contains(&"gpu-power".to_owned())
        );
        assert!(
            !titles(&rail(&pack, &apu, &Settings::default()), "Preflight")
                .contains(&"gpu-power".to_owned())
        );
        assert!(
            titles(&rail(&pack, &apu, &Settings::default()), "Preflight")
                .contains(&"apu-power".to_owned())
        );
        assert_eq!(rail(&pack, &gpu, &Settings::default()).reference.len(), 3);
    }

    #[test]
    fn a_skipped_step_keeps_its_phase_open() {
        let pack = ifly_pack();
        let first = pack
            .lists
            .iter()
            .find(|l| l.id == "exterior-exterior-inspection")
            .unwrap();
        let mut flight = new_flight(&pack);
        for (n, step) in first.steps.iter().enumerate() {
            let action = if n == 0 {
                Action::Skip {
                    step: step.id.clone(),
                    at: 1,
                }
            } else {
                Action::Tick {
                    step: step.id.clone(),
                    at: 2,
                }
            };
            flight = apply(&pack, flight, action).unwrap();
        }
        let phase = rail(&pack, &flight, &Settings::default()).stages[0]
            .phases
            .iter()
            .find(|p| p.id == "exterior")
            .unwrap()
            .clone();
        assert!(!phase.complete);
        assert_eq!((phase.open, phase.skipped), (1, 1));
        let step = first.steps[0].id.clone();
        flight = apply(&pack, flight, Action::Tick { step, at: 3 }).unwrap();
        let phase = rail(&pack, &flight, &Settings::default()).stages[0]
            .phases
            .iter()
            .find(|p| p.id == "exterior")
            .unwrap()
            .clone();
        assert!(phase.complete);
    }
}
