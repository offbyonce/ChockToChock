// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! What a step asks each of its panel items to be, for the control drawings: exactly the targets
//! the pack states for the step (decision 0018). Nothing here reads a step's wording, and none of
//! it is the simulator's state, which the app does not know (decision 0006).

use checklist_core::{LightState, Step, locate};
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
}

/// The panel items a step involves, from its targets in the pack.
pub fn step_targets(pack: &FlatPack, step: &Step) -> Vec<ItemTarget> {
    step.targets
        .iter()
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
        let targets = step_targets(&pack, step);
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
    fn a_step_naming_one_control_targets_only_it() {
        // TERR's control is the whole EFIS panel; only the TERR button is the step's.
        let pack = ifly_pack();
        let step = pack
            .lists
            .iter()
            .flat_map(|l| &l.steps)
            .find(|s| s.text.starts_with("TERR ("))
            .unwrap();
        let targets = step_targets(&pack, step);
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
        let targets = step_targets(&pack, step);
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
        for t in step_targets(&pack, step) {
            let (label, _) = label_of(&pack, &t);
            assert_eq!(t.lit, label.contains("VALVE"), "{label}");
        }
    }

    #[test]
    fn a_switch_step_that_checks_its_light_keeps_it_unlit() {
        // "ELT SWITCH — GUARD CLOSED, LIGHT OUT": the light is the step's, and out.
        let pack = ifly_pack();
        let step = step_where(&pack, |s| s.text == "ELT SWITCH");
        let lights: Vec<ItemTarget> = step_targets(&pack, step)
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
        let lit: Vec<&str> = step_targets(&pack, step)
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
            for t in step_targets(&pack, step).iter().filter(|t| t.lit) {
                let (label, _) = label_of(&pack, t);
                for clause in step.expect.split([',', ';']) {
                    if names(clause, label) {
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
            let kept = step_targets(&pack, step)
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
                ["memory", "quick-reference", "procedures"].contains(&phase)
                    && !s.controls.is_empty()
                    && step_targets(&pack, s).is_empty()
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
            let target = step_targets(&pack, step)
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
        let targets = step_targets(&pack, step);
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
}
