// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! References from faults, glossary, briefings, start states and aliases.

use std::collections::BTreeSet;

use super::aliases::check_alias;
use super::ids::unique;
use super::index::Index;
use super::{Issue, error};
use crate::Pack;

pub(super) fn check_other_references(pack: &Pack, idx: &Index, issues: &mut Vec<Issue>) {
    let c = &pack.content;
    if let Some(file) = &c.faults {
        for fault in &file.faults {
            for step in &fault.steps {
                idx.require_step("faults", step, issues);
            }
        }
    }
    if let Some(file) = &c.glossary {
        unique(
            "glossary term",
            file.terms.iter().map(|t| t.term.as_str()),
            issues,
        );
    }
    if let Some(file) = &c.briefings {
        unique(
            "briefing",
            file.briefings.iter().map(|b| b.id.as_str()),
            issues,
        );
        for briefing in &file.briefings {
            idx.require_phase(&briefing.id, &briefing.phase, issues);
        }
    }
    if let Some(file) = &c.start_states {
        for state in &file.states {
            idx.require_phase(&state.id, &state.phase, issues);
            for (option, value) in &state.set {
                idx.check_option_value(&state.id, option, value, issues);
            }
            for step in state.done.iter().chain(&state.skip) {
                idx.require_step(&state.id, step, issues);
            }
        }
    }
    if let Some(file) = &c.aliases {
        let mut seen = BTreeSet::new();
        for alias in &file.aliases {
            if !seen.insert((alias.kind, alias.from.as_str())) {
                issues.push(error(&alias.from, "alias id is used twice in this pack"));
            }
            check_alias(alias, idx, issues);
        }
    }
}

#[cfg(test)]
mod tests {
    use checklist_core::AliasKind;

    use super::super::test_support::{PACK, alias, alias_pack, assert_flags, messages};

    #[test]
    fn the_same_old_id_may_appear_once_per_kind() {
        let step_id = format!("{PACK}.preflight.a");
        let p = alias_pack(vec![
            alias(AliasKind::Step, "preflight", &step_id),
            alias(AliasKind::Phase, "preflight", "preflight"),
        ]);
        assert_eq!(messages(&p), "");

        let p = alias_pack(vec![
            alias(AliasKind::Step, "preflight", &step_id),
            alias(AliasKind::Step, "preflight", &step_id),
        ]);
        assert_flags(&p, "alias id is used twice");
    }
}
