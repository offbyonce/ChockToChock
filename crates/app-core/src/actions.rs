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
    /// A new flight: ticks, values, notes and timers are cleared; option choices are kept.
    Clear,
}

fn invalid(message: impl Into<String>) -> AppError {
    AppError::InvalidAction(message.into())
}

fn set_value(flight: &mut Flight, field: FillField, value: &str) -> Result<(), AppError> {
    if matches!(field, FillField::Manual | FillField::RouteLeg) {
        return Err(invalid(format!("`{field:?}` is not a flight value")));
    }
    let value = value.trim().to_owned();
    if value.is_empty() {
        flight.values.remove(&field);
        return Ok(());
    }
    if let Some(ok) = allowed(field)
        && !ok.contains(&value.as_str())
    {
        return Err(invalid(format!(
            "`{value}` is not one of {}",
            ok.join(", ")
        )));
    }
    flight.values.insert(field, value);
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
    let value = value.trim();
    if value.is_empty() {
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
            flight
                .session
                .tick(list, &step, at)
                .map_err(|e| invalid(e.to_string()))?;
        }
        Action::Untick { step } => {
            let list = list_of(&step)?;
            flight
                .session
                .untick(list, &step)
                .map_err(|e| invalid(e.to_string()))?;
        }
        Action::Skip { step, at } => {
            let list = list_of(&step)?;
            flight
                .session
                .skip(list, &step, at)
                .map_err(|e| invalid(e.to_string()))?;
        }
        Action::Unskip { step } => {
            let list = list_of(&step)?;
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
        Action::SetBlank { step, index, value } => {
            set_blank(pack, &mut flight, &step, index, &value)?;
        }
        Action::SetNotes { notes } => flight.notes = notes,
        Action::Clear => {
            flight.start_state = None;
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
