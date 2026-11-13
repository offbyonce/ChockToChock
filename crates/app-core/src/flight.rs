// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! One flight in progress (decision 0012). Saved on the device with a format number.

use std::collections::BTreeMap;

use checklist_core::{FillField, Millis, Session, StepId};
use pack::FlatPack;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// The saved format of a [`Flight`]. Raise it, with a migration and tests, when the shape changes.
pub const FLIGHT_FORMAT: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export)]
pub struct Flight {
    pub format: u32,
    /// The pack this flight belongs to, and its version when the flight started.
    pub pack: String,
    pub pack_version: String,
    /// The pilot's choice for each option the pack declares, e.g. the power source.
    pub options: BTreeMap<String, String>,
    /// The panel state the flight started from, if any.
    pub start_state: Option<String>,
    pub session: Session,
    /// Flight values, from the pilot or from SimBrief.
    pub values: BTreeMap<FillField, String>,
    /// What the pilot typed into blanks with no linked value: `<step-id>#<n>` to text.
    pub blanks: BTreeMap<String, String>,
    /// Clearance and other notes from the flight strip.
    pub notes: String,
    /// Timers the pilot started by hand: step to start time.
    #[ts(type = "Record<string, number>")]
    pub timers: BTreeMap<StepId, Millis>,
}

/// A new flight for a pack, with each option at its default, except options a decision step
/// asks: they stay unanswered until the pilot answers (decision 0022).
pub fn new_flight(pack: &FlatPack) -> Flight {
    let decided: Vec<&str> = pack
        .lists
        .iter()
        .flat_map(|l| &l.steps)
        .filter_map(|s| s.decide.as_deref())
        .collect();
    let options = pack
        .content
        .phases
        .iter()
        .flat_map(|p| &p.options)
        .filter(|o| !decided.contains(&o.id.as_str()))
        .map(|o| (o.id.clone(), o.default.clone()))
        .collect();
    Flight {
        format: FLIGHT_FORMAT,
        pack: pack.manifest.id.clone(),
        pack_version: pack.manifest.version.to_string(),
        options,
        start_state: None,
        session: Session::new(),
        values: BTreeMap::new(),
        blanks: BTreeMap::new(),
        notes: String::new(),
        timers: BTreeMap::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_support::ifly_pack;

    #[test]
    fn an_option_a_decision_step_asks_starts_unanswered() {
        // An invented decision in memory: a step that asks the power option.
        let mut pack = ifly_pack();
        assert!(new_flight(&pack).options.contains_key("power"));
        pack.lists[0].steps[0].decide = Some("power".into());
        let flight = new_flight(&pack);
        assert!(
            !flight.options.contains_key("power"),
            "{:?}",
            flight.options
        );
        // Options no step decides keep their defaults.
        assert!(!flight.options.is_empty());
    }
}
