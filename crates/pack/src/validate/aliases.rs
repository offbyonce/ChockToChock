// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Aliases: old ids that map to current ones.

use std::collections::BTreeSet;

use checklist_core::{Alias, AliasKind, FillField};
use serde_json::Value;

use super::index::Index;
use super::{Issue, error};

pub(super) fn check_alias(alias: &Alias, idx: &Index, issues: &mut Vec<Issue>) {
    let at = alias.from.as_str();
    let to = alias.to.as_str();
    let found = |set: &BTreeSet<&str>| set.contains(to);
    let unknown = |what: &str| error(at, format!("refers to unknown {what} `{to}`"));
    match alias.kind {
        AliasKind::Step if !found(&idx.steps) => issues.push(unknown("step")),
        AliasKind::BriefingItem if !found(&idx.briefing_items) => {
            issues.push(unknown("briefing item"));
        }
        AliasKind::StartState if !found(&idx.start_states) => issues.push(unknown("start state")),
        AliasKind::List if !found(&idx.lists) => issues.push(unknown("list")),
        AliasKind::Phase if !found(&idx.phases) => issues.push(unknown("phase")),
        AliasKind::Zone if !found(&idx.zones) => issues.push(unknown("zone")),
        AliasKind::Blank => {
            let blank = to.rsplit_once('#').and_then(|(step, n)| {
                let n = n.parse::<usize>().ok()?;
                (n < *idx.blanks.get(step)?).then_some(())
            });
            if blank.is_none() {
                issues.push(unknown("blank"));
            }
        }
        AliasKind::OptionValue => match to.split_once(':') {
            Some((option, value)) => idx.check_option_value(at, option, value, issues),
            None => issues.push(error(at, "option alias must be `<option>:<value>`")),
        },
        AliasKind::FillField
            if !matches!(
                serde_json::from_value::<FillField>(Value::String(to.to_owned())),
                Ok(f) if f != FillField::Manual && f != FillField::RouteLeg
            ) =>
        {
            issues.push(unknown("fill field"));
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use checklist_core::AliasKind;

    use super::super::test_support::{PACK, alias, alias_pack, assert_flags, messages};

    #[test]
    fn aliases_of_every_kind_resolve() {
        let step_id = format!("{PACK}.preflight.a");
        let blank = format!("{step_id}#1");
        let p = alias_pack(vec![
            alias(AliasKind::Step, "preflight-0-0", &step_id),
            alias(AliasKind::BriefingItem, "b-0-0", "departure.runway"),
            alias(AliasKind::Blank, "preflight-0-0-v1", &blank),
            alias(AliasKind::StartState, "bes", "ready"),
            alias(AliasKind::OptionValue, "power:ground", "power:gpu"),
            alias(AliasKind::FillField, "toflaps", "takeoff_flaps"),
            alias(AliasKind::List, "m:Smoke", "preflight"),
            alias(AliasKind::Phase, "f:preflight", "preflight"),
            alias(AliasKind::Zone, "c:Overhead", "overhead"),
        ]);
        assert_eq!(messages(&p), "");
    }

    #[test]
    fn aliases_with_unknown_targets_are_errors() {
        let step_id = format!("{PACK}.preflight.a");
        let past_end = format!("{step_id}#2");
        for (a, expected) in [
            (alias(AliasKind::Step, "x", "ghost"), "unknown step"),
            (
                alias(AliasKind::BriefingItem, "x", "ghost"),
                "unknown briefing item",
            ),
            (alias(AliasKind::Blank, "x", &past_end), "unknown blank"),
            (alias(AliasKind::Blank, "x", "ghost#0"), "unknown blank"),
            (
                alias(AliasKind::StartState, "x", "ghost"),
                "unknown start state",
            ),
            (
                alias(AliasKind::OptionValue, "x", "power:solar"),
                "has no value",
            ),
            (
                alias(AliasKind::OptionValue, "x", "power"),
                "must be `<option>:<value>`",
            ),
            (
                alias(AliasKind::FillField, "x", "ghost"),
                "unknown fill field",
            ),
            (alias(AliasKind::List, "x", "ghost"), "unknown list"),
            (alias(AliasKind::Phase, "x", "ghost"), "unknown phase"),
            (alias(AliasKind::Zone, "x", "ghost"), "unknown zone"),
        ] {
            assert_flags(&alias_pack(vec![a]), expected);
        }
    }
}
