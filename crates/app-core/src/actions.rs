// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! The only way the interface changes a flight (decision 0012). More actions arrive with the
//! batches that need them (panel states, plans, timers).

use checklist_core::{FillField, Millis, StepId};
use pack::FlatPack;
use serde::Deserialize;
use ts_rs::TS;

use crate::AppError;
use crate::flight::Flight;
use crate::values::{allowed, blank_key};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, TS)]
#[serde(tag = "type", rename_all = "camelCase", deny_unknown_fields)]
#[ts(export)]
pub enum Action {
    /// The pilot confirmed the step in the cockpit.
    Tick {
        step: StepId,
        #[ts(type = "number")]
        at: Millis,
    },
    Untick {
        step: StepId,
    },
    /// Skipped for now; it stays open.
    Skip {
        step: StepId,
        #[ts(type = "number")]
        at: Millis,
    },
    /// Back to do: the skip mark goes.
    Unskip {
        step: StepId,
    },
    /// Clear every tick and skip in one list.
    ClearList {
        list: String,
    },
    SetOption {
        option: String,
        value: String,
    },
    /// A flight value, typed by the pilot. An empty value removes it (decision 0019).
    SetValue {
        field: FillField,
        value: String,
    },
    /// Several flight values at once, such as those a SimBrief plan fills in; each replaces the
    /// value it covers, and the whole change is one step to undo (decision 0019).
    SetValues {
        #[ts(type = "Partial<Record<FillField, string>>")]
        values: std::collections::BTreeMap<FillField, String>,
    },
    /// What the pilot typed into a step's blank (`index` from 0). Empty removes the entry, so the
    /// blank shows the linked flight value again.
    SetBlank {
        step: StepId,
        index: usize,
        value: String,
    },
    /// The clearance and other notes from the flight strip.
    SetNotes {
        notes: String,
    },
    /// The iFly panel state the flight starts from, or none. It sets the state's options and
    /// ticks and skips the steps it has already set; picking another state, or none, takes back
    /// only the marks the earlier state made. The pilot's own ticks and skips are never changed.
    SetStartState {
        state: Option<String>,
        #[ts(type = "number")]
        at: Millis,
    },
    /// A new flight: ticks, values, notes and timers are cleared; option choices are kept.
    Clear,
}

fn invalid(message: impl Into<String>) -> AppError {
    AppError::InvalidAction(message.into())
}

/// Takes back the earlier state's own marks, then applies the new state's options, ticks and
/// skips to steps the pilot has not marked.
fn set_start_state(
    pack: &FlatPack,
    flight: &mut Flight,
    state: Option<String>,
    at: Millis,
) -> Result<(), AppError> {
    let list_of = |step: &StepId| pack.lists.iter().find(|l| l.contains(step));
    let chosen = match &state {
        Some(id) => Some(
            pack.content
                .start_states
                .iter()
                .flat_map(|f| &f.states)
                .find(|s| &s.id == id)
                .ok_or_else(|| invalid(format!("no panel state `{id}`")))?
                .clone(),
        ),
        None => None,
    };
    for step in std::mem::take(&mut flight.state_ticks) {
        if let Some(list) = list_of(&step) {
            let _ = flight.session.untick(list, &step);
        }
    }
    for step in std::mem::take(&mut flight.state_skips) {
        if let Some(list) = list_of(&step) {
            let _ = flight.session.unskip(list, &step);
        }
    }
    if let Some(s) = chosen {
        for (option, value) in &s.set {
            set_option(pack, flight, option.clone(), value.clone())?;
        }
        for step in &s.done {
            let Some(list) = list_of(step) else { continue };
            if flight.session.done_at(step).is_none() && !flight.session.is_skipped(step) {
                flight
                    .session
                    .tick(list, step, at)
                    .map_err(|e| invalid(e.to_string()))?;
                flight.state_ticks.insert(step.clone());
            }
        }
        for step in &s.skip {
            let Some(list) = list_of(step) else { continue };
            if flight.session.done_at(step).is_none() && !flight.session.is_skipped(step) {
                flight
                    .session
                    .skip(list, step, at)
                    .map_err(|e| invalid(e.to_string()))?;
                flight.state_skips.insert(step.clone());
            }
        }
    }
    flight.start_state = state;
    Ok(())
}

fn set_value(flight: &mut Flight, field: FillField, value: &str) -> Result<(), AppError> {
    if matches!(field, FillField::Manual | FillField::RouteLeg) {
        return Err(invalid(format!("`{field:?}` is not a flight value")));
    }
    // Kept as typed: the interface saves on every keystroke, so a space typed between words must
    // stay. A blank value removes the entry.
    if value.trim().is_empty() {
        flight.values.remove(&field);
        return Ok(());
    }
    if let Some(ok) = allowed(field)
        && !ok.contains(&value)
    {
        return Err(invalid(format!(
            "`{value}` is not one of {}",
            ok.join(", ")
        )));
    }
    flight.values.insert(field, value.to_owned());
    Ok(())
}

fn set_option(
    pack: &FlatPack,
    flight: &mut Flight,
    option: String,
    value: String,
) -> Result<(), AppError> {
    let known = pack
        .content
        .phases
        .iter()
        .flat_map(|p| &p.options)
        .find(|o| o.id == option);
    match known {
        None => Err(invalid(format!("unknown option `{option}`"))),
        Some(o) if !o.values.contains(&value) => {
            Err(invalid(format!("option `{option}` has no value `{value}`")))
        }
        Some(_) => {
            flight.options.insert(option, value);
            Ok(())
        }
    }
}

fn set_blank(
    pack: &FlatPack,
    flight: &mut Flight,
    step: &StepId,
    index: usize,
    value: &str,
) -> Result<(), AppError> {
    let found = pack
        .lists
        .iter()
        .flat_map(|l| &l.steps)
        .find(|s| &s.id == step)
        .ok_or_else(|| invalid(format!("unknown step `{step}`")))?;
    if index >= found.fill.len() {
        return Err(invalid(format!("step `{step}` has no blank {index}")));
    }
    let key = blank_key(found, index);
    if value.trim().is_empty() {
        flight.blanks.remove(&key);
    } else {
        flight.blanks.insert(key, value.to_owned());
    }
    Ok(())
}

/// Apply an action and return the changed flight.
pub fn apply(pack: &FlatPack, mut flight: Flight, action: Action) -> Result<Flight, AppError> {
    if flight.pack != pack.manifest.id {
        return Err(invalid(format!(
            "this flight belongs to pack {}",
            flight.pack
        )));
    }
    let list_of = |step: &StepId| {
        pack.lists
            .iter()
            .find(|l| l.contains(step))
            .ok_or_else(|| invalid(format!("unknown step `{step}`")))
    };
    match action {
        Action::Tick { step, at } => {
            let list = list_of(&step)?;
            // A mark the pilot makes is theirs: a later state change leaves it alone.
            flight.state_ticks.remove(&step);
            flight.state_skips.remove(&step);
            flight
                .session
                .tick(list, &step, at)
                .map_err(|e| invalid(e.to_string()))?;
        }
        Action::Untick { step } => {
            let list = list_of(&step)?;
            flight.state_ticks.remove(&step);
            flight
                .session
                .untick(list, &step)
                .map_err(|e| invalid(e.to_string()))?;
        }
        Action::Skip { step, at } => {
            let list = list_of(&step)?;
            flight.state_ticks.remove(&step);
            flight.state_skips.remove(&step);
            flight
                .session
                .skip(list, &step, at)
                .map_err(|e| invalid(e.to_string()))?;
        }
        Action::Unskip { step } => {
            let list = list_of(&step)?;
            flight.state_skips.remove(&step);
            flight
                .session
                .unskip(list, &step)
                .map_err(|e| invalid(e.to_string()))?;
        }
        Action::ClearList { list } => {
            let list = pack
                .lists
                .iter()
                .find(|l| l.id == list)
                .ok_or_else(|| invalid(format!("unknown list `{list}`")))?;
            flight.session.clear_list(list);
        }
        Action::SetOption { option, value } => set_option(pack, &mut flight, option, value)?,
        Action::SetValue { field, value } => set_value(&mut flight, field, &value)?,
        Action::SetValues { values } => {
            for (field, value) in &values {
                set_value(&mut flight, *field, value)?;
            }
        }
        Action::SetBlank { step, index, value } => {
            set_blank(pack, &mut flight, &step, index, &value)?;
        }
        Action::SetNotes { notes } => flight.notes = notes,
        Action::SetStartState { state, at } => set_start_state(pack, &mut flight, state, at)?,
        Action::Clear => {
            flight.start_state = None;
            flight.state_ticks.clear();
            flight.state_skips.clear();
            flight.session.clear();
            flight.values.clear();
            flight.blanks.clear();
            flight.notes.clear();
            flight.timers.clear();
        }
    }
    Ok(flight)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flight::new_flight;

    #[test]
    fn a_panel_state_marks_its_steps_and_a_change_takes_back_only_its_own() {
        let pack = crate::test_support::ifly_pack();
        let states = &pack.content.start_states.as_ref().unwrap().states;
        let cd = states.iter().find(|s| s.id == "cd").unwrap().clone();
        let gpu = states.iter().find(|s| s.id == "gpu").unwrap().clone();
        let hand = cd.done[0].clone();
        // The pilot ticks one of the state's steps by hand first.
        let f = apply(
            &pack,
            new_flight(&pack),
            Action::Tick {
                step: hand.clone(),
                at: 1,
            },
        )
        .unwrap();
        let f = apply(
            &pack,
            f,
            Action::SetStartState {
                state: Some("gpu".into()),
                at: 2,
            },
        )
        .unwrap();
        assert_eq!(f.start_state.as_deref(), Some("gpu"));
        assert_eq!(f.options["power"], "gpu");
        for step in &gpu.done {
            assert!(f.session.done_at(step).is_some(), "{step} ticked");
        }
        assert!(
            !f.state_ticks.contains(&hand),
            "the pilot's own tick is not the state's"
        );
        assert_eq!(f.session.done_at(&hand), Some(1));
        // Back to Cold & Dark: the steps only GPU set are open again; the pilot's tick stays.
        let only_gpu: Vec<_> = gpu
            .done
            .iter()
            .filter(|s| !cd.done.contains(s))
            .cloned()
            .collect();
        assert_ne!(only_gpu.len(), 0);
        let f = apply(
            &pack,
            f,
            Action::SetStartState {
                state: Some("cd".into()),
                at: 3,
            },
        )
        .unwrap();
        for step in &only_gpu {
            assert!(f.session.done_at(step).is_none(), "{step} taken back");
        }
        assert_eq!(f.session.done_at(&hand), Some(1));
        // A step the state ticked and the pilot then ticks again by hand becomes theirs.
        let other = cd.done.iter().find(|s| **s != hand).unwrap().clone();
        let f = apply(
            &pack,
            f,
            Action::Untick {
                step: other.clone(),
            },
        )
        .unwrap();
        let f = apply(
            &pack,
            f,
            Action::Tick {
                step: other.clone(),
                at: 4,
            },
        )
        .unwrap();
        let f = apply(&pack, f, Action::SetStartState { state: None, at: 5 }).unwrap();
        assert_eq!(f.start_state, None);
        assert_eq!(f.session.done_at(&other), Some(4));
        assert_eq!(f.session.done_at(&hand), Some(1));
        assert!(f.state_ticks.is_empty() && f.state_skips.is_empty());
        assert!(
            apply(
                &pack,
                f,
                Action::SetStartState {
                    state: Some("nope".into()),
                    at: 6
                }
            )
            .is_err()
        );
    }
    use crate::test_support::example_pack;

    fn step() -> StepId {
        StepId::new("example-trainer.before-start.battery")
    }

    #[test]
    fn tick_skip_and_clear_change_the_flight() {
        let pack = example_pack();
        let flight = new_flight(&pack);
        let skipped = apply(
            &pack,
            flight,
            Action::Skip {
                step: step(),
                at: 5,
            },
        )
        .unwrap();
        assert!(skipped.session.is_skipped(&step()));
        let ticked = apply(
            &pack,
            skipped,
            Action::Tick {
                step: step(),
                at: 9,
            },
        )
        .unwrap();
        assert!(!ticked.session.is_skipped(&step()), "ticking un-skips");
        assert_eq!(ticked.session.done_at(&step()), Some(9));
        let cleared = apply(&pack, ticked, Action::Clear).unwrap();
        assert!(!cleared.session.is_done(&step()));
    }

    #[test]
    fn options_must_be_declared_and_wrong_steps_are_refused() {
        let pack = example_pack();
        let flight = new_flight(&pack);
        let option = pack.content.phases.as_ref().unwrap().options[0].clone();
        let other = option
            .values
            .iter()
            .find(|v| **v != option.default)
            .unwrap()
            .clone();
        let changed = apply(
            &pack,
            flight.clone(),
            Action::SetOption {
                option: option.id.clone(),
                value: other.clone(),
            },
        )
        .unwrap();
        assert_eq!(changed.options[&option.id], other);
        let bad_value = Action::SetOption {
            option: option.id,
            value: "nope".into(),
        };
        assert!(matches!(
            apply(&pack, flight.clone(), bad_value),
            Err(AppError::InvalidAction(_))
        ));
        let bad_step = Action::Tick {
            step: StepId::new("example-trainer.x.y"),
            at: 1,
        };
        assert!(matches!(
            apply(&pack, flight.clone(), bad_step),
            Err(AppError::InvalidAction(_))
        ));
        let mut foreign = flight;
        foreign.pack = "other".into();
        assert!(matches!(
            apply(&pack, foreign, Action::Clear),
            Err(AppError::InvalidAction(_))
        ));
    }

    #[test]
    fn actions_arrive_as_tagged_json() {
        let action: Action =
            serde_json::from_str(r#"{"type":"tick","step":"a.b.c","at":12}"#).unwrap();
        assert_eq!(
            action,
            Action::Tick {
                step: StepId::new("a.b.c"),
                at: 12
            }
        );
        assert!(serde_json::from_str::<Action>(r#"{"type":"tick","step":"a.b.c"}"#).is_err());
    }
}
