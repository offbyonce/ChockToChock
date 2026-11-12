// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Open steps, as in 3.0: on every phase the pilot has started (a step done or skipped), the
//! steps not yet done, each marked skipped or missed, in the order the phases come (decision
//! 0012). Asked for a phase, only the phases before it count, as 3.0's "from earlier pages".

use checklist_core::StepId;
use pack::FlatPack;
use serde::Serialize;
use ts_rs::TS;

use crate::flight::Flight;
use crate::rail::rail;
use crate::settings::Settings;
use crate::visibility::{Shown, shown_steps};

/// One open step and where it is.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct OpenStep {
    pub step: StepId,
    pub phase: String,
    pub phase_title: String,
    pub list_title: String,
    /// Skipped by the pilot; otherwise missed (left to do on a phase the pilot moved past).
    pub skipped: bool,
}

/// The flight's open steps for the pilot's choices. With `before`, only phases ahead of that
/// phase in the flight count, and never the phase itself or the reference phases.
pub fn open_steps(
    pack: &FlatPack,
    flight: &Flight,
    settings: &Settings,
    before: Option<&str>,
) -> Vec<OpenStep> {
    let shown = Shown::new(pack, settings);
    let Some(file) = &pack.content.phases else {
        return vec![];
    };
    let r = rail(pack, flight, settings);
    let flow: Vec<&str> = r
        .stages
        .iter()
        .flat_map(|s| &s.phases)
        .map(|p| p.id.as_str())
        .collect();
    let order: Vec<&str> = match before {
        Some(id) => flow.iter().copied().take_while(|p| *p != id).collect(),
        None => flow
            .iter()
            .copied()
            .chain(r.reference.iter().map(|p| p.id.as_str()))
            .collect(),
    };
    let mut out = vec![];
    for id in order {
        let Some(phase) = file.phases.iter().find(|p| p.id == id) else {
            continue;
        };
        let steps: Vec<(&checklist_core::Checklist, &checklist_core::Step)> = phase
            .lists
            .iter()
            .filter_map(|l| pack.lists.iter().find(|x| &x.id == l))
            .flat_map(|list| {
                shown_steps(list, &flight.options, shown, phase.reference).map(move |s| (list, s))
            })
            .collect();
        let started = steps
            .iter()
            .any(|(_, s)| flight.session.is_done(&s.id) || flight.session.is_skipped(&s.id));
        if !started {
            continue;
        }
        for (list, step) in steps {
            if !flight.session.is_done(&step.id) {
                out.push(OpenStep {
                    step: step.id.clone(),
                    phase: phase.id.clone(),
                    phase_title: phase.title.clone(),
                    list_title: list.title.clone(),
                    skipped: flight.session.is_skipped(&step.id),
                });
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::{Action, apply};
    use crate::flight::new_flight;
    use crate::phase_view::phase_view;
    use crate::test_support::ifly_pack;

    #[test]
    fn open_steps_are_the_undone_steps_of_started_phases() {
        let pack = ifly_pack();
        let s = Settings::default();
        let mut flight = new_flight(&pack);
        let cold = phase_view(&pack, &flight, &s, "cold-dark").unwrap().lists[0]
            .steps
            .clone();
        assert!(
            open_steps(&pack, &flight, &s, None).is_empty(),
            "nothing started yet"
        );
        flight = apply(
            &pack,
            flight,
            Action::Tick {
                step: cold[0].id.clone(),
                at: 1,
            },
        )
        .unwrap();
        flight = apply(
            &pack,
            flight,
            Action::Skip {
                step: cold[1].id.clone(),
                at: 2,
            },
        )
        .unwrap();
        let open = open_steps(&pack, &flight, &s, None);
        assert_eq!(
            open.len(),
            cold.len() - 1,
            "every undone step of the started phase"
        );
        assert!(open[0].skipped && open[0].step == cold[1].id);
        assert!(open[1..].iter().all(|o| !o.skipped), "the rest are missed");
        // From a later phase they count as earlier pages; on the phase itself they do not.
        assert_eq!(
            open_steps(&pack, &flight, &s, Some("gpu-power")).len(),
            open.len()
        );
        assert_eq!(open_steps(&pack, &flight, &s, Some("cold-dark")).len(), 0);
        // Ticked or unskipped, a skipped step is no longer marked skipped.
        flight = apply(
            &pack,
            flight,
            Action::Unskip {
                step: cold[1].id.clone(),
            },
        )
        .unwrap();
        assert!(
            open_steps(&pack, &flight, &s, None)
                .iter()
                .all(|o| !o.skipped)
        );
    }
}
