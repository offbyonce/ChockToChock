// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! The only way the interface changes a flight (decision 0012). More actions arrive with the
//! batches that need them (panel states, values, plans, timers).

use checklist_core::{Millis, StepId};
use pack::FlatPack;
use serde::Deserialize;
use ts_rs::TS;

use crate::AppError;
use crate::flight::Flight;

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
    SetOption {
        option: String,
        value: String,
    },
    /// A new flight: ticks, values, notes and timers are cleared; option choices are kept.
    Clear,
}

fn invalid(message: impl Into<String>) -> AppError {
    AppError::InvalidAction(message.into())
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
        Action::SetOption { option, value } => {
            let known = pack
                .content
                .phases
                .iter()
                .flat_map(|p| &p.options)
                .find(|o| o.id == option);
            match known {
                None => return Err(invalid(format!("unknown option `{option}`"))),
                Some(o) if !o.values.contains(&value) => {
                    return Err(invalid(format!("option `{option}` has no value `{value}`")));
                }
                Some(_) => {
                    flight.options.insert(option, value);
                }
            }
        }
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
