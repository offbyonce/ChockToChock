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
use crate::visibility::visible_steps;

/// One step in a list, as shown.
#[derive(Debug, Clone, PartialEq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct StepRow {
    pub id: StepId,
    pub state: StepState,
    /// False for a step the simulator does not model: it can be ticked but is not counted.
    pub counts: bool,
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
    /// The first step that is neither done nor skipped.
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
}

/// The phase `id` for this flight.
///
/// # Errors
/// `InvalidAction` if the pack has no phase with that id.
pub fn phase_view(pack: &FlatPack, flight: &Flight, id: &str) -> Result<PhaseView, AppError> {
    let phase = pack
        .content
        .phases
        .as_ref()
        .and_then(|file| file.phases.iter().find(|p| p.id == id))
        .ok_or_else(|| AppError::InvalidAction(format!("no phase `{id}` in this pack")))?;

    let lists: Vec<PhaseList> = phase
        .lists
        .iter()
        .filter_map(|list_id| pack.lists.iter().find(|l| &l.id == list_id))
        .map(|list| {
            let visible: Vec<_> = visible_steps(list, &flight.options).collect();
            let steps: Vec<StepRow> = flight
                .session
                .states(visible.iter().copied())
                .into_iter()
                .zip(&visible)
                .map(|((id, state), step)| StepRow {
                    id: id.clone(),
                    state,
                    counts: counts(step),
                })
                .collect();
            let progress = flight.session.progress(visible.iter().copied());
            PhaseList {
                id: list.id.clone(),
                title: list.title.clone(),
                kind: list.kind.clone(),
                current: steps
                    .iter()
                    .find(|s| s.state == StepState::Current)
                    .map(|s| s.id.clone()),
                steps,
                progress,
                complete: progress.complete(),
            }
        })
        .collect();
    let progress = lists
        .iter()
        .map(|l| l.progress)
        .fold(Progress::default(), Progress::plus);

    // Where the phase sits in the rail for these options.
    let rail = rail(pack, flight);
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
    fn a_new_flight_starts_on_the_first_step() {
        let pack = ifly_pack();
        let flight = new_flight(&pack);
        let view = phase_view(&pack, &flight, "cold-dark").unwrap();
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
        let view = phase_view(&pack, &flight, "cold-dark").unwrap();
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
        let view = phase_view(&pack, &flight, "cold-dark").unwrap();
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
        let view = phase_view(&pack, &flight, "cold-dark").unwrap();
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
        assert!(phase_view(&pack, &flight, "cold-dark").unwrap().complete);
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
        let gpu_next = phase_view(&pack, &flight, "cold-dark")
            .unwrap()
            .next
            .unwrap();
        let apu_next = phase_view(&pack, &apu, "cold-dark").unwrap().next.unwrap();
        assert_eq!(gpu_next.id, "gpu-power");
        assert_eq!(apu_next.id, "apu-power");
        assert_ne!(
            first_steps(&phase_view(&pack, &apu, "apu-power").unwrap(), 1),
            vec![]
        );
    }

    #[test]
    fn reference_phases_have_no_place_in_the_rail() {
        let pack = ifly_pack();
        let view = phase_view(&pack, &new_flight(&pack), "memory").unwrap();
        assert!(view.reference);
        assert_eq!(view.stage, None);
        assert_eq!(view.position, None);
        assert!(view.previous.is_none() && view.next.is_none());
    }

    #[test]
    fn an_unknown_phase_is_an_error() {
        let pack = ifly_pack();
        assert!(matches!(
            phase_view(&pack, &new_flight(&pack), "nope"),
            Err(AppError::InvalidAction(_))
        ));
    }
}
