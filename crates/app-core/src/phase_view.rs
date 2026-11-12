// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! One phase as the interface shows it: its lists with the steps that show for the pilot's
//! options, each step's state, progress, the current step, and where the phase sits in the rail
//! (decision 0012). The interface reads the step text from the pack by id and adds no logic.

use checklist_core::{ListKind, Progress, StepId, StepState, counts};
use pack::FlatPack;
use serde::Serialize;
use ts_rs::TS;

use crate::error::AppError;
use crate::flight::Flight;
use crate::rail::rail;
use crate::settings::Settings;
use crate::targets::{ItemTarget, step_targets};
use crate::values::{StripPart, asks_arrival, blank_values, hint, last_note, strip};
use crate::visibility::{Shown, in_essentials, shown_steps};

/// One step in a list, as shown.
#[derive(Debug, Clone, PartialEq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct StepRow {
    pub id: StepId,
    /// The number shown: the step's position among the steps that show, from 1. The pack's own
    /// `number` is not used for this; 3.0 numbered by position too, and the import kept only the
    /// numbers 3.0 stored, so many steps have none.
    pub number: usize,
    pub state: StepState,
    /// False for a step the simulator does not model: it can be ticked but is not counted.
    pub counts: bool,
    /// The panel items the step uses and the position it asks for, for the control drawings.
    pub targets: Vec<ItemTarget>,
    /// In Essentials (the pack's list, with the pilot's changes).
    pub essential: bool,
    /// The text for each of the step's blanks, in order; empty where nothing is known yet
    /// (decision 0019).
    pub values: Vec<String>,
    /// The step's hint from flight values ("GND 121.700 · DEP 125.600"), or empty.
    pub hint: String,
}

/// One list in a phase.
#[derive(Debug, Clone, PartialEq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PhaseList {
    pub id: String,
    pub title: String,
    pub kind: ListKind,
    pub steps: Vec<StepRow>,
    pub progress: Progress,
    /// Every counted step is done; only then does the interface show the completion call.
    pub complete: bool,
    /// Steps that show in Full but not in Essentials ("Show 3 more steps from Full").
    pub hidden: usize,
    /// The phase's current step, if it is in this list: the first step of the phase, in list
    /// order, that is neither done nor skipped. A phase has one current step at most.
    pub current: Option<StepId>,
}

/// A neighboring phase, for the previous and next buttons.
#[derive(Debug, Clone, PartialEq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PhaseLink {
    pub id: String,
    pub title: String,
}

/// One phase, ready to show.
#[derive(Debug, Clone, PartialEq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PhaseView {
    pub id: String,
    pub title: String,
    pub reference: bool,
    /// The stage the phase shows in for these options, if any.
    pub stage: Option<String>,
    /// 1-based position in that stage, and how many phases the stage shows.
    pub position: Option<(usize, usize)>,
    pub previous: Option<PhaseLink>,
    pub next: Option<PhaseLink>,
    pub lists: Vec<PhaseList>,
    pub progress: Progress,
    pub complete: bool,
    /// The flight strip's summary (decision 0019); empty when nothing is known yet.
    pub strip: Vec<StripPart>,
    /// The last line of the flight's notes, shown under the strip, or empty.
    pub note: String,
    /// The phase shows the Arrival data box (decision 0019).
    pub arrival: bool,
}

/// The phase `id` for this flight.
///
/// # Errors
/// `InvalidAction` if the pack has no phase with that id.
/// A step's row apart from its id, number and state.
fn step_row(
    pack: &FlatPack,
    flight: &Flight,
    shown: &Shown,
    step: &checklist_core::Step,
) -> StepRow {
    StepRow {
        id: step.id.clone(),
        number: 0,
        state: StepState::ToDo,
        counts: counts(step),
        targets: step_targets(pack, step),
        essential: in_essentials(step, &flight.options, shown),
        values: blank_values(flight, step),
        hint: hint(flight, step),
    }
}

pub fn phase_view(
    pack: &FlatPack,
    flight: &Flight,
    settings: &Settings,
    id: &str,
) -> Result<PhaseView, AppError> {
    let shown = Shown::new(pack, settings);
    let phase = pack
        .content
        .phases
        .as_ref()
        .and_then(|file| file.phases.iter().find(|p| p.id == id))
        .ok_or_else(|| AppError::InvalidAction(format!("no phase `{id}` in this pack")))?;

    let mut lists: Vec<PhaseList> = phase
        .lists
        .iter()
        .filter_map(|list_id| pack.lists.iter().find(|l| &l.id == list_id))
        .map(|list| {
            let visible: Vec<_> =
                shown_steps(list, &flight.options, shown, phase.reference).collect();
            let steps: Vec<StepRow> = flight
                .session
                .states(visible.iter().copied())
                .into_iter()
                .zip(&visible)
                .enumerate()
                .map(|(i, ((id, state), step))| StepRow {
                    id: id.clone(),
                    number: i + 1,
                    state,
                    ..step_row(pack, flight, &shown, step)
                })
                .collect();
            let progress = flight.session.progress(visible.iter().copied());
            let hidden =
                crate::visibility::visible_steps(list, &flight.options).count() - visible.len();
            PhaseList {
                id: list.id.clone(),
                title: list.title.clone(),
                kind: list.kind.clone(),
                current: steps
                    .iter()
                    .find(|s| s.state == StepState::Current)
                    .map(|s| s.id.clone()),
                steps,
                hidden,
                progress,
                complete: progress.complete(),
            }
        })
        .collect();
    // One current step per phase: the first list's. Later lists' first open steps are to do.
    let mut seen = false;
    for list in &mut lists {
        if seen {
            list.current = None;
            for step in &mut list.steps {
                if step.state == StepState::Current {
                    step.state = StepState::ToDo;
                }
            }
        } else if list.current.is_some() {
            seen = true;
        }
    }
    let progress = lists
        .iter()
        .map(|l| l.progress)
        .fold(Progress::default(), Progress::plus);

    // Where the phase sits in the rail for these options.
    let rail = rail(pack, flight, settings);
    let order: Vec<(&str, &str, &str)> = rail
        .stages
        .iter()
        .flat_map(|s| {
            s.phases
                .iter()
                .map(move |p| (s.title.as_str(), p.id.as_str(), p.title.as_str()))
        })
        .collect();
    let link = |i: usize| {
        order.get(i).map(|(_, id, title)| PhaseLink {
            id: (*id).to_owned(),
            title: (*title).to_owned(),
        })
    };
    let at = order.iter().position(|(_, p, _)| *p == id);
    let stage = at.map(|i| order[i].0.to_owned());
    let position = stage.as_ref().and_then(|title| {
        let in_stage: Vec<_> = order.iter().filter(|(s, _, _)| s == title).collect();
        let n = in_stage.iter().position(|(_, p, _)| *p == id)?;
        Some((n + 1, in_stage.len()))
    });

    Ok(PhaseView {
        id: phase.id.clone(),
        title: phase.title.clone(),
        reference: phase.reference,
        stage,
        position,
        previous: at.and_then(|i| i.checked_sub(1)).and_then(link),
        next: at.and_then(|i| link(i + 1)),
        lists,
        complete: progress.complete(),
        progress,
        strip: strip(flight),
        note: last_note(flight),
        arrival: asks_arrival(pack, order.iter().map(|(_, p, _)| *p), &phase.id),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::{Action, apply};
    use crate::flight::new_flight;
    use crate::test_support::ifly_pack;

    fn first_steps(view: &PhaseView, n: usize) -> Vec<StepId> {
        view.lists[0]
            .steps
            .iter()
            .take(n)
            .map(|s| s.id.clone())
            .collect()
    }

    #[test]
    fn a_step_row_carries_its_blank_values_and_hint() {
        let pack = ifly_pack();
        let mut flight = new_flight(&pack);
        for (field, value) in [
            (checklist_core::FillField::V2, "145"),
            (checklist_core::FillField::Heading, "163"),
            (checklist_core::FillField::InitialAltitude, "5000"),
        ] {
            let action = Action::SetValue {
                field,
                value: value.into(),
            };
            flight = apply(&pack, flight, action).unwrap();
        }
        let view = phase_view(&pack, &flight, &Settings::default(), "before-start").unwrap();
        let row = view
            .lists
            .iter()
            .flat_map(|l| &l.steps)
            .find(|s| s.id.as_str() == "ifly-737-max8.before-start.mcp-check")
            .unwrap();
        assert_eq!(row.values, ["145", "163", "5000"]);
        assert_eq!(row.hint, "");
    }

    #[test]
    fn a_new_flight_starts_on_the_first_step() {
        let pack = ifly_pack();
        let flight = new_flight(&pack);
        let view = phase_view(&pack, &flight, &Settings::default(), "cold-dark").unwrap();
        assert_eq!(view.stage.as_deref(), Some("Preflight"));
        assert_eq!(view.position, Some((1, 4)));
        assert!(view.previous.is_none());
        assert_eq!(view.next.as_ref().unwrap().id, "gpu-power");
        let list = &view.lists[0];
        assert_eq!(list.steps[0].state, StepState::Current);
        assert!(list.steps[1..].iter().all(|s| s.state == StepState::ToDo));
        assert_eq!(list.current.as_ref(), Some(&list.steps[0].id));
        assert!(!view.complete);
    }

    #[test]
    fn ticking_moves_current_and_a_skip_keeps_the_list_open() {
        let pack = ifly_pack();
        let mut flight = new_flight(&pack);
        let view = phase_view(&pack, &flight, &Settings::default(), "cold-dark").unwrap();
        let ids: Vec<StepId> = view.lists[0].steps.iter().map(|s| s.id.clone()).collect();
        let [first, second] = [ids[0].clone(), ids[1].clone()];
        flight = apply(&pack, flight, Action::Tick { step: first, at: 1 }).unwrap();
        flight = apply(
            &pack,
            flight,
            Action::Skip {
                step: second,
                at: 2,
            },
        )
        .unwrap();
        let view = phase_view(&pack, &flight, &Settings::default(), "cold-dark").unwrap();
        let states: Vec<_> = view.lists[0].steps.iter().map(|s| s.state).collect();
        assert_eq!(
            states[..3],
            [StepState::Done, StepState::Skipped, StepState::Current]
        );
        for id in &ids[2..] {
            flight = apply(
                &pack,
                flight,
                Action::Tick {
                    step: id.clone(),
                    at: 3,
                },
            )
            .unwrap();
        }
        let view = phase_view(&pack, &flight, &Settings::default(), "cold-dark").unwrap();
        assert!(
            !view.lists[0].complete,
            "a skipped step keeps the list open"
        );
        assert_eq!(view.lists[0].current, None);
        flight = apply(
            &pack,
            flight,
            Action::Tick {
                step: ids[1].clone(),
                at: 4,
            },
        )
        .unwrap();
        assert!(
            phase_view(&pack, &flight, &Settings::default(), "cold-dark")
                .unwrap()
                .complete
        );
    }

    #[test]
    fn neighbors_follow_the_options() {
        let pack = ifly_pack();
        let flight = new_flight(&pack);
        let apu = apply(
            &pack,
            flight.clone(),
            Action::SetOption {
                option: "power".into(),
                value: "apu".into(),
            },
        )
        .unwrap();
        let gpu_next = phase_view(&pack, &flight, &Settings::default(), "cold-dark")
            .unwrap()
            .next
            .unwrap();
        let apu_next = phase_view(&pack, &apu, &Settings::default(), "cold-dark")
            .unwrap()
            .next
            .unwrap();
        assert_eq!(gpu_next.id, "gpu-power");
        assert_eq!(apu_next.id, "apu-power");
        assert_ne!(
            first_steps(
                &phase_view(&pack, &apu, &Settings::default(), "apu-power").unwrap(),
                1
            ),
            vec![]
        );
    }

    #[test]
    fn reference_phases_have_no_place_in_the_rail() {
        let pack = ifly_pack();
        let view = phase_view(&pack, &new_flight(&pack), &Settings::default(), "memory").unwrap();
        assert!(view.reference);
        assert_eq!(view.stage, None);
        assert_eq!(view.position, None);
        assert!(view.previous.is_none() && view.next.is_none());
    }

    #[test]
    fn essentials_shows_the_packs_list_with_turnaround_steps_only_on_a_turnaround() {
        let pack = ifly_pack();
        let flight = new_flight(&pack);
        let full = Settings::default();
        let ess = Settings {
            essentials: true,
            ..Settings::default()
        };
        let count = |f: &Flight, s: &Settings, phase: &str| -> usize {
            phase_view(&pack, f, s, phase)
                .unwrap()
                .lists
                .iter()
                .map(|l| l.steps.len())
                .sum()
        };
        // Fewer steps in Essentials, every one marked essential, the rest counted as hidden.
        assert!(count(&flight, &ess, "preflight") < count(&flight, &full, "preflight"));
        let v = phase_view(&pack, &flight, &ess, "preflight").unwrap();
        assert!(v.lists.iter().flat_map(|l| &l.steps).all(|s| s.essential));
        let hidden: usize = v.lists.iter().map(|l| l.hidden).sum();
        assert_eq!(
            count(&flight, &ess, "preflight") + hidden,
            count(&flight, &full, "preflight")
        );
        // A phase with no Essentials steps leaves the rail.
        let stage_ids = |s: &Settings| -> Vec<String> {
            crate::rail::rail(&pack, &flight, s)
                .stages
                .iter()
                .flat_map(|st| st.phases.iter().map(|p| p.id.clone()))
                .collect()
        };
        assert!(stage_ids(&full).contains(&"exterior".to_owned()));
        assert!(!stage_ids(&ess).contains(&"exterior".to_owned()));
        // Steps in Essentials only on a turnaround show once the start option says so.
        let turn = apply(
            &pack,
            flight.clone(),
            Action::SetOption {
                option: "start".into(),
                value: "turn".into(),
            },
        )
        .unwrap();
        let ids = |f: &Flight| -> std::collections::BTreeSet<StepId> {
            let r = crate::rail::rail(&pack, f, &ess);
            r.stages
                .iter()
                .flat_map(|s| &s.phases)
                .flat_map(|p| phase_view(&pack, f, &ess, &p.id).unwrap().lists)
                .flat_map(|l| l.steps)
                .map(|s| s.id)
                .collect()
        };
        let turnaround_only: Vec<_> = pack
            .lists
            .iter()
            .flat_map(|l| &l.steps)
            .filter(|s| s.quick == Some(checklist_core::Quick::Turnaround))
            .map(|s| s.id.clone())
            .collect();
        assert_ne!(turnaround_only.len(), 0);
        let (cold, turned) = (ids(&flight), ids(&turn));
        assert!(turnaround_only.iter().all(|id| !cold.contains(id)));
        assert!(turnaround_only.iter().any(|id| turned.contains(id)));
    }

    #[test]
    fn the_pilots_changes_to_essentials_win() {
        let pack = ifly_pack();
        let flight = new_flight(&pack);
        let full = phase_view(&pack, &flight, &Settings::default(), "cold-dark").unwrap();
        let steps = &full.lists[0].steps;
        let out = steps.iter().find(|s| !s.essential).unwrap().id.clone();
        let inn = steps.iter().find(|s| s.essential).unwrap().id.clone();
        let mut s = Settings {
            essentials: true,
            ..Settings::default()
        };
        let changes = s
            .essentials_changes
            .entry(pack.manifest.id.clone())
            .or_default();
        changes.added.insert(out.clone());
        changes.removed.insert(inn.clone());
        let ids: Vec<_> = phase_view(&pack, &flight, &s, "cold-dark").unwrap().lists[0]
            .steps
            .iter()
            .map(|r| r.id.clone())
            .collect();
        assert!(ids.contains(&out) && !ids.contains(&inn));
    }

    #[test]
    fn clearing_a_list_and_unskipping_reopen_steps() {
        let pack = ifly_pack();
        let s = Settings::default();
        let mut flight = new_flight(&pack);
        let view = phase_view(&pack, &flight, &s, "cold-dark").unwrap();
        let ids: Vec<StepId> = view.lists[0].steps.iter().map(|r| r.id.clone()).collect();
        flight = apply(
            &pack,
            flight,
            Action::Tick {
                step: ids[0].clone(),
                at: 1,
            },
        )
        .unwrap();
        flight = apply(
            &pack,
            flight,
            Action::Skip {
                step: ids[1].clone(),
                at: 1,
            },
        )
        .unwrap();
        flight = apply(
            &pack,
            flight,
            Action::Unskip {
                step: ids[1].clone(),
            },
        )
        .unwrap();
        assert!(!flight.session.is_skipped(&ids[1]));
        flight = apply(
            &pack,
            flight,
            Action::Skip {
                step: ids[1].clone(),
                at: 2,
            },
        )
        .unwrap();
        flight = apply(
            &pack,
            flight,
            Action::ClearList {
                list: view.lists[0].id.clone(),
            },
        )
        .unwrap();
        let after = phase_view(&pack, &flight, &s, "cold-dark").unwrap();
        assert_eq!(
            after.lists[0].progress.done + after.lists[0].progress.skipped,
            0
        );
        assert!(
            apply(
                &pack,
                flight,
                Action::ClearList {
                    list: "nope".into()
                }
            )
            .is_err()
        );
    }

    #[test]
    fn a_phase_has_one_current_step() {
        // Before start has a procedure and a checklist; both once showed a current step.
        let pack = ifly_pack();
        let mut flight = new_flight(&pack);
        let current = |f: &Flight| -> Vec<StepId> {
            phase_view(&pack, f, &Settings::default(), "before-start")
                .unwrap()
                .lists
                .iter()
                .flat_map(|l| &l.steps)
                .filter(|s| s.state == StepState::Current)
                .map(|s| s.id.clone())
                .collect()
        };
        let view = phase_view(&pack, &flight, &Settings::default(), "before-start").unwrap();
        assert!(view.lists.len() > 1);
        assert_eq!(current(&flight), vec![view.lists[0].steps[0].id.clone()]);
        assert!(view.lists[1..].iter().all(|l| l.current.is_none()));
        // Once the first list is done, the current step moves to the next list.
        for s in &view.lists[0].steps {
            flight = apply(
                &pack,
                flight,
                Action::Tick {
                    step: s.id.clone(),
                    at: 1,
                },
            )
            .unwrap();
        }
        let view = phase_view(&pack, &flight, &Settings::default(), "before-start").unwrap();
        assert_eq!(current(&flight), vec![view.lists[1].steps[0].id.clone()]);
    }

    #[test]
    fn every_step_that_shows_is_numbered_in_order() {
        // Before start's procedure has steps with no number in the pack (Landing gear pins,
        // TERR, Electric hydraulic pump switches); they once showed unnumbered.
        let pack = ifly_pack();
        let flight = new_flight(&pack);
        for phase in ["before-start", "cold-dark", "preflight", "memory"] {
            for list in phase_view(&pack, &flight, &Settings::default(), phase)
                .unwrap()
                .lists
            {
                let numbers: Vec<usize> = list.steps.iter().map(|s| s.number).collect();
                let expected: Vec<usize> = (1..=list.steps.len()).collect();
                assert_eq!(numbers, expected, "{}", list.id);
            }
        }
    }

    #[test]
    fn numbers_follow_the_steps_that_show_for_the_options() {
        let pack = ifly_pack();
        let flight = new_flight(&pack);
        let turn = apply(
            &pack,
            flight.clone(),
            Action::SetOption {
                option: "start".into(),
                value: "turn".into(),
            },
        )
        .unwrap();
        for f in [&flight, &turn] {
            for stage in crate::rail::rail(&pack, f, &Settings::default()).stages {
                for p in stage.phases {
                    for list in phase_view(&pack, f, &Settings::default(), &p.id)
                        .unwrap()
                        .lists
                    {
                        let last = list.steps.last().map_or(0, |s| s.number);
                        assert_eq!(last, list.steps.len(), "{}", list.id);
                    }
                }
            }
        }
    }

    #[test]
    fn an_unknown_phase_is_an_error() {
        let pack = ifly_pack();
        assert!(matches!(
            phase_view(&pack, &new_flight(&pack), &Settings::default(), "nope"),
            Err(AppError::InvalidAction(_))
        ));
    }
}
