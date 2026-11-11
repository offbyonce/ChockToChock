// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! What a step asks each of its panel items to be set to, for the control drawings. This is the
//! step's target, read from its expected value; it is never the simulator's state, which the app
//! does not know (decision 0006).

use checklist_core::{Step, named_position};
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
}

/// The panel items a step uses, with the position it asks for.
pub fn step_targets(pack: &FlatPack, step: &Step) -> Vec<ItemTarget> {
    let mut out = vec![];
    for panel in &pack.content.panels {
        for section in &panel.sections {
            for (item, entry) in section.items.iter().enumerate() {
                if entry.controls.iter().any(|c| step.controls.contains(c)) {
                    out.push(ItemTarget {
                        section: section.id.clone(),
                        item,
                        position: entry
                            .positions
                            .as_deref()
                            .and_then(|p| named_position(&step.expect, p)),
                    });
                }
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
