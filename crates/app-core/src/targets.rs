// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! What a step asks each of its panel items to be, for the control drawings: exactly the targets
//! the pack states for the step (decision 0018). Nothing here reads a step's wording, and none of
//! it is the simulator's state, which the app does not know (decision 0006).

use std::collections::BTreeMap;

use checklist_core::{LightState, Step, StepTarget, locate};
use pack::FlatPack;
use serde::Serialize;
use ts_rs::TS;

/// One panel item a step involves, as the pack states it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ItemTarget {
    /// The panel section the item is in.
    pub section: String,
    /// The item's index in that section.
    pub item: usize,
    /// The index of the position the step sets the item to, in the item's `positions`.
    pub position: Option<usize>,
    /// A light the step expects on.
    pub lit: bool,
    /// The step leaves the position to a condition the app does not know yet ("CENTER PUMPS ON IF
    /// MORE THAN 1,000 LB"): the drawing shows it dashed, with no position (decision 0022).
    pub open: bool,
    /// The position comes from a decision step's answer (decision 0022): the drawing shows it
    /// resolved, and the step can link to the decision.
    pub resolved: bool,
}

/// The panel items a step involves, from its targets in the pack, for the flight's answers: an
/// item whose targets depend on a decision takes the target for the answer, and until there is
/// one, its `open` target.
pub fn step_targets(
    pack: &FlatPack,
    options: &BTreeMap<String, String>,
    step: &Step,
) -> Vec<ItemTarget> {
    let mut shown: Vec<&StepTarget> = vec![];
    for t in &step.targets {
        let answered = t
            .when
            .as_ref()
            .map(|w| options.get(&w.option) == Some(&w.equals));
        match answered {
            Some(true) => {
                // The answer's target replaces the item's target for before the answer.
                shown.retain(|s| s.at != t.at);
                shown.push(t);
            }
            Some(false) => {}
            None => {
                let decided = step.targets.iter().any(|o| {
                    o.at == t.at
                        && o.when
                            .as_ref()
                            .is_some_and(|w| options.get(&w.option) == Some(&w.equals))
                });
                if !decided {
                    shown.push(t);
                }
            }
        }
    }
    shown
        .into_iter()
        .filter_map(|t| {
            let (section, item) = locate(&pack.content.panels, &t.at)?;
            let entry = section.items.get(item)?;
            let position = t.set.as_deref().and_then(|set| {
                entry
                    .positions
                    .as_deref()?
                    .split(',')
                    .position(|p| p.trim() == set)
            });
            Some(ItemTarget {
                section: section.id.clone(),
                item,
                position,
                lit: t.light == Some(LightState::On),
                open: t.open,
                resolved: t.when.is_some(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::ifly_pack;
    use checklist_core::{PanelItemKind, named_position, names, says_on};

    #[test]
    fn a_value_names_one_position_as_a_whole_word() {
        assert_eq!(named_position("ON, GUARD CLOSED", "OFF, ON"), Some(1));
        assert_eq!(
            named_position("AUTO, GUARD CLOSED", "BAT, OFF, AUTO"),
            Some(2)
        );
        assert_eq!(named_position("on", "OFF, ON"), Some(1));
        // "ON" inside another word does not count.
        assert_eq!(named_position("ONCE STABLE", "OFF, ON"), None);
    }

    #[test]
    fn a_value_naming_several_positions_or_none_has_no_target() {
        assert_eq!(
            named_position("ENG 1 AND 2 ON, ELEC 1 AND 2 OFF", "OFF, ON"),
            None
        );
        assert_eq!(named_position("CHECK", "OFF, ON"), None);
        assert_eq!(named_position("ON", ""), None);
    }

    #[test]
    fn setting_the_parking_brake_lights_its_light() {
        let pack = ifly_pack();
        let step = pack
            .lists
            .iter()
            .flat_map(|l| &l.steps)
            .find(|s| s.controls.iter().any(|c| c == "pbrake") && s.expect == "SET")
            .unwrap();
        let targets = step_targets(&pack, &BTreeMap::new(), step);
        let section = pack
            .content
            .panels
            .iter()
            .flat_map(|p| &p.sections)
            .find(|s| s.id == "lo")
            .unwrap();
        let light = targets
            .iter()
            .find(|t| t.section == "lo" && section.items[t.item].label == "PARK BRAKE LIGHT")
            .unwrap();
        assert!(light.lit, "the light shows on when the step sets the brake");
        let handle = targets
            .iter()
            .find(|t| t.section == "lo" && section.items[t.item].label == "PARKING BRAKE")
            .unwrap();
        assert!(!handle.lit);
        assert!(handle.position.is_some(), "SET is the handle's target");
    }

    #[test]
    fn a_position_left_to_a_condition_reaches_the_drawing_as_open() {
        // The center fuel pumps are ON only with enough center fuel; the drawing must say so
        // rather than show them at rest like a control the step does not set.
        let pack = ifly_pack();
        let step = pack
            .lists
            .iter()
            .flat_map(|l| &l.steps)
            .find(|s| s.id.as_str() == "ifly-737-max8.before-start.fuel-pump-switches")
            .unwrap();
        let targets = step_targets(&pack, &BTreeMap::new(), step);
        let section = pack
            .content
            .panels
            .iter()
            .flat_map(|p| &p.sections)
            .find(|s| s.id == targets[0].section)
            .unwrap();
        let center: Vec<&ItemTarget> = targets
            .iter()
            .filter(|t| section.items[t.item].label.starts_with("CTR"))
            .collect();
        assert_eq!(center.len(), 2);
        assert!(center.iter().all(|t| t.open && t.position.is_none()));
        assert!(
            targets
                .iter()
                .filter(|t| t.position.is_some())
                .all(|t| !t.open)
        );
    }

    #[test]
    fn a_step_naming_one_control_targets_only_it() {
        // TERR's control is the whole EFIS panel; only the TERR button is the step's.
        let pack = ifly_pack();
        let step = pack
            .lists
            .iter()
            .flat_map(|l| &l.steps)
            .find(|s| s.text.starts_with("TERR ("))
            .unwrap();
        let targets = step_targets(&pack, &BTreeMap::new(), step);
        let labels: Vec<&str> = targets
            .iter()
            .map(|t| {
                pack.content
                    .panels
                    .iter()
                    .flat_map(|p| &p.sections)
                    .find(|s| s.id == t.section)
                    .map(|s| s.items[t.item].label.as_str())
                    .unwrap()
            })
            .collect();
        assert!(
            !labels.is_empty() && labels.iter().all(|l| *l == "TERR"),
            "{labels:?}"
        );
    }

    #[test]
    fn switching_the_fuel_pumps_on_does_not_light_their_low_pressure_lights() {
        // "ALL ON" names the pump switches' position; their LOW PRESSURE lights go out.
        let pack = ifly_pack();
        let step = pack
            .lists
            .iter()
            .flat_map(|l| &l.steps)
            .find(|s| s.id.as_str() == "ifly-737-max8.before-start.fuel-pump-switches")
            .unwrap();
        let targets = step_targets(&pack, &BTreeMap::new(), step);
        assert!(
            targets.iter().any(|t| t.position.is_some()),
            "the switches are set ON"
        );
        assert!(targets.iter().all(|t| !t.lit), "no light is expected on");
        assert!(
            targets
                .iter()
                .all(|t| label_of(&pack, t).1 != PanelItemKind::Light),
            "the step points at the switches only"
        );
    }

    fn label_of<'a>(pack: &'a FlatPack, t: &ItemTarget) -> (&'a str, PanelItemKind) {
        let item = pack
            .content
            .panels
            .iter()
            .flat_map(|p| &p.sections)
            .find(|s| s.id == t.section)
            .map(|s| &s.items[t.item])
            .unwrap();
        (item.label.as_str(), item.kind)
    }

    fn step_where(pack: &FlatPack, f: impl Fn(&Step) -> bool) -> &Step {
        pack.lists
            .iter()
            .flat_map(|l| &l.steps)
            .find(|s| f(s))
            .unwrap()
    }

    #[test]
    fn a_step_about_several_lights_reads_each_one() {
        // "VALVE LIGHTS ON, FILTER BYPASS OUT": the valve lights on, FILTER BYPASS not.
        let pack = ifly_pack();
        let step = step_where(&pack, |s| s.expect == "VALVE LIGHTS ON, FILTER BYPASS OUT");
        for t in step_targets(&pack, &BTreeMap::new(), step) {
            let (label, _) = label_of(&pack, &t);
            assert_eq!(t.lit, label.contains("VALVE"), "{label}");
        }
    }

    #[test]
    fn a_switch_step_that_checks_its_light_keeps_it_unlit() {
        // "ELT SWITCH — GUARD CLOSED, LIGHT OUT": the light is the step's, and out.
        let pack = ifly_pack();
        let step = step_where(&pack, |s| s.text == "ELT SWITCH");
        let lights: Vec<ItemTarget> = step_targets(&pack, &BTreeMap::new(), step)
            .into_iter()
            .filter(|t| label_of(&pack, t).1 == PanelItemKind::Light)
            .collect();
        assert!(!lights.is_empty(), "the ELT light is the step's");
        assert!(lights.iter().all(|t| !t.lit), "and it is out");
    }

    #[test]
    fn gear_down_lights_the_three_green_lights_only() {
        let pack = ifly_pack();
        let step = step_where(&pack, |s| {
            s.text == "LANDING GEAR LEVER" && s.expect == "DN"
        });
        let lit: Vec<&str> = step_targets(&pack, &BTreeMap::new(), step)
            .iter()
            .filter(|t| t.lit)
            .map(|t| label_of(&pack, t).0)
            .collect();
        assert_eq!(lit, ["LEFT GEAR", "NOSE GEAR", "RIGHT GEAR"]);
    }

    #[test]
    fn no_light_is_shown_on_where_the_step_says_it_is_out() {
        // Across every step: a lit light's own clause in the value never says OUT or OFF.
        let pack = ifly_pack();
        for step in pack.lists.iter().flat_map(|l| &l.steps) {
            for t in step_targets(&pack, &BTreeMap::new(), step)
                .iter()
                .filter(|t| t.lit)
            {
                let (label, _) = label_of(&pack, t);
                for clause in checklist_core::clauses(&step.expect) {
                    // A clause saying both OFF and ON ("OFF WHEN BOTH LOW PRESSURE LIGHTS COME
                    // ON") is about a switch and a light; it does not say the light is out.
                    if names(clause, label) && !names(clause, "ON") {
                        assert!(
                            !names(clause, "OUT") && !names(clause, "OFF"),
                            "{}: {label} shown on, but `{clause}`",
                            step.id
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn every_step_that_speaks_of_lights_keeps_them() {
        // No entry is missed: a step mentioning lights, on controls that have lights, shows them.
        let pack = ifly_pack();
        for step in pack.lists.iter().flat_map(|l| &l.steps) {
            let text = format!("{} {}", step.text, step.expect);
            if !names(&text, "LIGHT") && !names(&text, "LIGHTS") {
                continue;
            }
            let has_lights = pack
                .content
                .panels
                .iter()
                .flat_map(|p| &p.sections)
                .flat_map(|s| &s.items)
                .any(|i| {
                    i.kind == PanelItemKind::Light
                        && i.controls.iter().any(|c| step.controls.contains(c))
                });
            if !has_lights {
                continue;
            }
            let kept = step_targets(&pack, &BTreeMap::new(), step)
                .iter()
                .any(|t| label_of(&pack, t).1 == PanelItemKind::Light);
            assert!(kept, "{} speaks of lights but shows none", step.id);
        }
    }

    #[test]
    fn every_emergency_step_with_a_control_has_a_drawing() {
        // Memory items, quick reference and non-normal steps showed no drawing when their
        // controls (the control column, the oxygen masks, the PA) were on no panel.
        let pack = ifly_pack();
        let missing: Vec<&str> = pack
            .lists
            .iter()
            .flat_map(|l| &l.steps)
            .filter(|s| {
                let phase = s.id.as_str().split('.').nth(1).unwrap_or_default();
                // A control no panel draws (the control column) has nothing to show.
                let drawn = pack
                    .content
                    .panels
                    .iter()
                    .flat_map(|p| &p.sections)
                    .flat_map(|s| &s.items)
                    .any(|i| i.controls.iter().any(|c| s.controls.contains(c)));
                ["memory", "quick-reference", "procedures"].contains(&phase)
                    && drawn
                    && step_targets(&pack, &BTreeMap::new(), s).is_empty()
            })
            .map(|s| s.id.as_str())
            .collect();
        assert_eq!(missing, Vec::<&str>::new());
    }

    #[test]
    fn the_flight_control_step_targets_each_switch_by_its_own_name() {
        // The switches read A ON and B ON, as on the panel; one value names both.
        let pack = ifly_pack();
        let step = pack
            .lists
            .iter()
            .flat_map(|l| &l.steps)
            .find(|s| s.id.as_str() == "ifly-737-max8.preflight.flight-control-switches")
            .unwrap();
        let section = pack
            .content
            .panels
            .iter()
            .flat_map(|p| &p.sections)
            .find(|s| s.id == "fctl")
            .unwrap();
        for (label, name) in [("FLT CONTROL A", "A ON"), ("FLT CONTROL B", "B ON")] {
            let item = section.items.iter().position(|i| i.label == label).unwrap();
            let target = step_targets(&pack, &BTreeMap::new(), step)
                .into_iter()
                .find(|t| t.section == "fctl" && t.item == item)
                .unwrap();
            let positions: Vec<&str> = section.items[item]
                .positions
                .as_deref()
                .unwrap()
                .split(", ")
                .collect();
            assert_eq!(target.position.map(|p| positions[p]), Some(name), "{label}");
        }
    }

    #[test]
    fn a_value_saying_on_lights_a_light_without_lit_at() {
        assert!(says_on("ILLUMINATED"));
        assert!(says_on("COMES ON"));
        assert!(!says_on("OUT"));
        assert!(!says_on("ON THEN OFF"));
    }

    #[test]
    fn the_battery_step_targets_the_battery_switch_on() {
        let pack = ifly_pack();
        let step = pack
            .lists
            .iter()
            .flat_map(|l| &l.steps)
            .find(|s| s.id.0 == "ifly-737-max8.cold-dark.battery-switch")
            .unwrap();
        let targets = step_targets(&pack, &BTreeMap::new(), step);
        let battery = targets.iter().find(|t| t.section == "elecu").unwrap();
        let item = &pack
            .content
            .panels
            .iter()
            .flat_map(|p| &p.sections)
            .find(|s| s.id == "elecu")
            .unwrap()
            .items[battery.item];
        assert_eq!(item.label, "BATTERY");
        assert_eq!(battery.position, Some(1), "ON of OFF, ON");
    }

    /// The fuel pump step with an invented decision, built in memory for the test only (the
    /// shipped pack has none until the center tank step is sourced): option `invented_center`,
    /// `more` or `less`, with CTR L drawn ON or OFF by the answer.
    fn with_invented_decision() -> (FlatPack, checklist_core::Step) {
        let pack = ifly_pack();
        let mut step = pack
            .lists
            .iter()
            .flat_map(|l| &l.steps)
            .find(|s| s.id.as_str() == "ifly-737-max8.before-start.fuel-pump-switches")
            .unwrap()
            .clone();
        let ctr = step
            .targets
            .iter()
            .find(|t| t.at == "oh/fuel/CTR L/1")
            .unwrap()
            .clone();
        for (set, equals) in [("ON", "more"), ("OFF", "less")] {
            step.targets.push(checklist_core::StepTarget {
                set: Some(set.into()),
                open: false,
                when: Some(checklist_core::TargetWhen {
                    option: "invented_center".into(),
                    equals: equals.into(),
                }),
                ..ctr.clone()
            });
        }
        (pack, step)
    }

    #[test]
    fn a_decided_target_is_drawn_resolved_and_until_then_conditional() {
        let (pack, step) = with_invented_decision();
        let ctr_l = |options: &BTreeMap<String, String>| {
            let targets = step_targets(&pack, options, &step);
            let section = pack
                .content
                .panels
                .iter()
                .flat_map(|p| &p.sections)
                .find(|s| s.id == "fuel")
                .unwrap();
            let found: Vec<ItemTarget> = targets
                .into_iter()
                .filter(|t| t.section == "fuel" && section.items[t.item].label == "CTR L")
                .collect();
            assert_eq!(found.len(), 1, "one target per item: {found:?}");
            found[0].clone()
        };
        // Unanswered: conditional, no position.
        let before = ctr_l(&BTreeMap::new());
        assert!(before.open && !before.resolved && before.position.is_none());
        // Answered "less": resolved at OFF (the first position).
        let options = BTreeMap::from([("invented_center".to_string(), "less".to_string())]);
        let after = ctr_l(&options);
        assert!(after.resolved && !after.open);
        assert_eq!(after.position, Some(0));
        // The other answer draws ON.
        let options = BTreeMap::from([("invented_center".to_string(), "more".to_string())]);
        assert_eq!(ctr_l(&options).position, Some(1));
    }
}
