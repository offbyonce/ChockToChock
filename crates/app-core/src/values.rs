// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Flight values as steps show them (decision 0019): the text of each blank and a step's hint,
//! and the values a unit choice may take.

use checklist_core::{FillField, Step};
use serde::Serialize;
use ts_rs::TS;

use crate::flight::Flight;

/// One part of the flight strip's summary ("RWY 16L"), and the field its pop-out focuses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct StripPart {
    pub field: FillField,
    pub text: String,
}

/// The flight strip's summary, as 3.0 built it: the route, runway, departure with its
/// transition, initial altitude, squawk and frequencies, each only when known.
pub fn strip(flight: &Flight) -> Vec<StripPart> {
    let v = |f: FillField| {
        flight
            .values
            .get(&f)
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
    };
    let mut out = vec![];
    let mut push = |field, text: String| out.push(StripPart { field, text });
    let route: Vec<&str> = [FillField::Origin, FillField::Destination]
        .into_iter()
        .filter_map(v)
        .collect();
    if !route.is_empty() {
        push(FillField::Origin, route.join(" → "));
    }
    if let Some(r) = v(FillField::Runway) {
        push(FillField::Runway, format!("RWY {r}"));
    }
    if let Some(sid) = v(FillField::Sid) {
        let text = match v(FillField::Transition) {
            Some(t) => format!("{sid}.{t}"),
            None => sid.to_owned(),
        };
        push(FillField::Sid, text);
    }
    if let Some(a) = v(FillField::InitialAltitude) {
        push(FillField::InitialAltitude, format!("{a} FT"));
    }
    if let Some(sq) = v(FillField::Squawk) {
        push(FillField::Squawk, format!("SQK {sq}"));
    }
    if let Some(g) = v(FillField::GroundFrequency) {
        push(FillField::GroundFrequency, format!("GND {g}"));
    }
    if let Some(d) = v(FillField::DepartureFrequency) {
        push(FillField::DepartureFrequency, format!("DEP {d}"));
    }
    out
}

/// The last line of the flight's notes, for the strip, or empty.
pub fn last_note(flight: &Flight) -> String {
    flight
        .notes
        .trim()
        .lines()
        .last()
        .unwrap_or_default()
        .trim()
        .to_owned()
}

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

/// The values the Arrival data box asks for, as 3.0's did.
pub const ARRIVAL: [FillField; 5] = [
    FillField::LandingAltitude,
    FillField::LandingFlaps,
    FillField::Vref,
    FillField::Autobrake,
    FillField::Minimums,
];

/// Whether a phase shows the Arrival data box: it is the first phase, in flight order, whose
/// steps have blanks for arrival values (the descent, in the iFly pack, as in 3.0).
pub fn asks_arrival<'a>(
    pack: &pack::FlatPack,
    order: impl IntoIterator<Item = &'a str>,
    id: &str,
) -> bool {
    let needs = |phase: &str| {
        pack.content
            .phases
            .iter()
            .flat_map(|p| &p.phases)
            .find(|p| p.id == phase)
            .into_iter()
            .flat_map(|p| &p.lists)
            .filter_map(|l| pack.lists.iter().find(|x| &x.id == l))
            .flat_map(|l| &l.steps)
            .any(|s| s.fill.iter().any(|f| ARRIVAL.contains(f)))
    };
    order.into_iter().find(|p| needs(p)) == Some(id)
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
                value: "147".into(),
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
    fn the_strip_sums_up_the_clearance_as_3_0_did() {
        let pack = ifly_pack();
        let flight = new_flight(&pack);
        assert_eq!(strip(&flight), []);
        let mut flight = flight;
        for (f, v) in [
            (FillField::Destination, "KSFO"),
            (FillField::Runway, "16L"),
            (FillField::Sid, "SUMMA2"),
            (FillField::Transition, "SEA"),
            (FillField::Squawk, "4521"),
        ] {
            flight = set(&pack, flight, f, v);
        }
        let texts: Vec<String> = strip(&flight).into_iter().map(|p| p.text).collect();
        assert_eq!(texts, ["KSFO", "RWY 16L", "SUMMA2.SEA", "SQK 4521"]);
        let flight = set(&pack, flight, FillField::Origin, "KSEA");
        assert_eq!(strip(&flight)[0].text, "KSEA → KSFO");
        let notes = Action::SetNotes {
            notes: "Taxi via A\nClimb 7000 \n\n".into(),
        };
        let flight = apply(&pack, flight, notes).unwrap();
        assert_eq!(last_note(&flight), "Climb 7000");
    }

    #[test]
    fn the_arrival_box_shows_on_the_first_phase_needing_arrival_values() {
        let pack = ifly_pack();
        let order = ["before-start", "descent", "approach", "landing"];
        assert!(asks_arrival(&pack, order, "descent"));
        assert!(
            !asks_arrival(&pack, order, "landing"),
            "landing flaps come later"
        );
        assert!(!asks_arrival(&pack, order, "before-start"));
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
        let cleared = set(&pack, flight, FillField::QnhUnit, " ");
        assert!(!cleared.values.contains_key(&FillField::QnhUnit));
        // A space typed between words stays: the interface saves on every keystroke.
        let route = set(&pack, cleared, FillField::Route, "SUMMA2 ");
        assert_eq!(route.values[&FillField::Route], "SUMMA2 ");
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
