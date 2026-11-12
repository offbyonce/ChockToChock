// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! What a step asks each of its panel items to be set to, for the control drawings. This is the
//! step's target, read from its expected value; it is never the simulator's state, which the app
//! does not know (decision 0006).

use checklist_core::{Step, named_position, names};
use pack::FlatPack;
use serde::Serialize;
use ts_rs::TS;

/// One panel item a step uses, and the position the step asks for, if its value names exactly one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct ItemTarget {
    /// The panel section the item is in.
    pub section: String,
    /// The item's index in that section.
    pub item: usize,
    /// The index of the target position in the item's `positions`, when the step names one.
    pub position: Option<usize>,
    /// A light the step expects on: its `lit_at` position is the step's target on an item it
    /// shares a control with, or (with no `lit_at`) the step's value says it is on, as 3.0 read it.
    pub lit: bool,
}

/// Whether a value says a light is on, as 3.0 read it: ON, LIT, ILLUMINATED and the like, and
/// not OUT, OFF or DARK.
fn says_on(text: &str) -> bool {
    let words = ["ON", "ILLUMINATED", "LIT", "COMES ON"];
    let out = ["OUT", "OFF", "EXTINGUISHED", "DARK"];
    words.iter().any(|w| names(text, w)) && !out.iter().any(|w| names(text, w))
}

/// The panel items a step uses, with the position it asks for and the lights it expects on.
pub fn step_targets(pack: &FlatPack, step: &Step) -> Vec<ItemTarget> {
    let mut out = vec![];
    let used: Vec<(&checklist_core::PanelItem, Option<usize>)> = pack
        .content
        .panels
        .iter()
        .flat_map(|p| &p.sections)
        .flat_map(|s| &s.items)
        .filter(|e| e.controls.iter().any(|c| step.controls.contains(c)))
        .map(|e| {
            (
                e,
                e.positions
                    .as_deref()
                    .and_then(|p| named_position(&step.expect, p)),
            )
        })
        .collect();
    for panel in &pack.content.panels {
        for section in &panel.sections {
            for (item, entry) in section.items.iter().enumerate() {
                if !entry.controls.iter().any(|c| step.controls.contains(c)) {
                    continue;
                }
                let position = entry
                    .positions
                    .as_deref()
                    .and_then(|p| named_position(&step.expect, p));
                let lit = entry.kind == checklist_core::PanelItemKind::Light
                    && match &entry.lit_at {
                        Some(at) => used.iter().any(|(o, t)| {
                            o.controls.iter().any(|c| entry.controls.contains(c))
                                && t.and_then(|t| o.positions.as_deref()?.split(',').nth(t))
                                    .is_some_and(|p| p.trim() == at)
                        }),
                        None => says_on(&step.expect),
                    };
                out.push(ItemTarget {
                    section: section.id.clone(),
                    item,
                    position,
                    lit,
                });
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::ifly_pack;

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
