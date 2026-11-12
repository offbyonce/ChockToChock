// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Flight values as steps show them (decision 0019): the text of each blank and a step's hint,
//! and the values a unit choice may take.

use checklist_core::{FillField, Step};

use crate::flight::Flight;

/// The key of a step's n-th blank (from 0) in [`Flight::blanks`].
pub fn blank_key(step: &Step, n: usize) -> String {
    format!("{}#{n}", step.id)
}

/// The text for each of a step's blanks: the pilot's entry for that blank, else the linked
/// flight value, else empty.
pub fn blank_values(flight: &Flight, step: &Step) -> Vec<String> {
    step.fill
        .iter()
        .enumerate()
        .map(|(n, field)| {
            flight
                .blanks
                .get(&blank_key(step, n))
                .or_else(|| flight.values.get(field))
                .cloned()
                .unwrap_or_default()
        })
        .collect()
}

/// A step's hint, as 3.0 built it: each part with a value, joined with " · ". Empty when no part
/// has a value.
pub fn hint(flight: &Flight, step: &Step) -> String {
    step.hint
        .iter()
        .filter_map(|h| {
            flight
                .values
                .get(&h.field)
                .filter(|v| !v.is_empty())
                .map(|v| format!("{}{v}", h.prefix))
        })
        .collect::<Vec<_>>()
        .join(" · ")
}

/// The values a choice field may take, or `None` for a field that takes any text.
pub fn allowed(field: FillField) -> Option<&'static [&'static str]> {
    match field {
        FillField::WeightUnit => Some(&["LB", "KG"]),
        FillField::QnhUnit => Some(&["inHg", "hPa"]),
        FillField::MinimumsType => Some(&["BARO", "RADIO"]),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::actions::{Action, apply};
    use crate::flight::new_flight;
    use crate::test_support::ifly_pack;
    use checklist_core::StepId;
    use pack::FlatPack;

    fn step<'a>(pack: &'a FlatPack, id: &str) -> &'a Step {
        pack.lists
            .iter()
            .flat_map(|l| &l.steps)
            .find(|s| s.id.as_str() == id)
            .unwrap()
    }

    fn set(pack: &FlatPack, flight: Flight, field: FillField, value: &str) -> Flight {
        let action = Action::SetValue {
            field,
            value: value.into(),
        };
        apply(pack, flight, action).unwrap()
    }

    #[test]
    fn a_blank_shows_the_pilots_entry_else_the_flight_value_else_nothing() {
        let pack = ifly_pack();
        let mcp = step(&pack, "ifly-737-max8.before-start.mcp-check");
        let flight = set(&pack, new_flight(&pack), FillField::V2, "145");
        assert_eq!(blank_values(&flight, mcp), ["145", "", ""]);
        let typed = apply(
            &pack,
            flight,
            Action::SetBlank {
                step: StepId::new("ifly-737-max8.before-start.mcp-check"),
                index: 0,
                value: " 147 ".into(),
            },
        )
        .unwrap();
        assert_eq!(blank_values(&typed, mcp), ["147", "", ""]);
        // Emptying the entry brings the flight value back.
        let back = apply(
            &pack,
            typed,
            Action::SetBlank {
                step: StepId::new("ifly-737-max8.before-start.mcp-check"),
                index: 0,
                value: String::new(),
            },
        )
        .unwrap();
        assert_eq!(blank_values(&back, mcp), ["145", "", ""]);
    }

    #[test]
    fn a_hint_leaves_out_parts_with_no_value() {
        let pack = ifly_pack();
        let radios = step(&pack, "ifly-737-max8.preflight.radio-tuning-panels");
        let flight = new_flight(&pack);
        assert_eq!(hint(&flight, radios), "");
        let flight = set(&pack, flight, FillField::DepartureFrequency, "125.600");
        assert_eq!(hint(&flight, radios), "DEP 125.600");
        let flight = set(&pack, flight, FillField::GroundFrequency, "121.700");
        assert_eq!(hint(&flight, radios), "GND 121.700 · DEP 125.600");
    }

    #[test]
    fn unit_choices_take_only_their_values_and_an_empty_value_removes() {
        let pack = ifly_pack();
        let flight = set(&pack, new_flight(&pack), FillField::QnhUnit, "hPa");
        assert_eq!(flight.values[&FillField::QnhUnit], "hPa");
        for (field, bad) in [
            (FillField::QnhUnit, "mb"),
            (FillField::MinimumsType, "DH"),
            (FillField::WeightUnit, "kg"),
        ] {
            let action = Action::SetValue {
                field,
                value: bad.into(),
            };
            assert!(
                apply(&pack, flight.clone(), action).is_err(),
                "{field:?} {bad}"
            );
        }
        let cleared = set(&pack, flight, FillField::QnhUnit, "");
        assert!(!cleared.values.contains_key(&FillField::QnhUnit));
    }

    #[test]
    fn blanks_must_exist_and_manual_is_not_a_flight_value() {
        let pack = ifly_pack();
        let flight = new_flight(&pack);
        let no_blank = Action::SetBlank {
            step: StepId::new("ifly-737-max8.before-start.mcp-check"),
            index: 3,
            value: "1".into(),
        };
        assert!(apply(&pack, flight.clone(), no_blank).is_err());
        let manual = Action::SetValue {
            field: FillField::Manual,
            value: "1".into(),
        };
        assert!(apply(&pack, flight, manual).is_err());
    }
}
