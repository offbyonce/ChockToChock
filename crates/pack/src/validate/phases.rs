// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! The phases file: options, list placement and stages.

use std::collections::BTreeSet;

use checklist_core::{Quick, UserOption};

use super::index::Index;
use super::{Issue, error};
use crate::Pack;

pub(super) fn check_phases(pack: &Pack, idx: &Index, issues: &mut Vec<Issue>) {
    let Some(file) = &pack.content.phases else {
        return;
    };
    if let Some(turnaround) = &file.turnaround {
        idx.check_show_if("turnaround", turnaround, issues);
    }
    let uses_turnaround = pack
        .checklists
        .iter()
        .flat_map(|l| &l.steps)
        .find(|s| s.quick == Some(Quick::Turnaround));
    if let (Some(step), None) = (uses_turnaround, &file.turnaround) {
        issues.push(error(
            step.id.as_str(),
            "is in Essentials on a turnaround, but phases.yaml declares no `turnaround`",
        ));
    }
    for option in &file.options {
        idx.check_option_value(&option.id, &option.id, &option.default, issues);
        check_choices(option, issues);
        for phase in &option.ask_in {
            idx.require_phase(&option.id, phase, issues);
        }
    }
    let mut placed = BTreeSet::new();
    for phase in &file.phases {
        for list in &phase.lists {
            if !idx.lists.contains(list.as_str()) {
                issues.push(error(&phase.id, format!("refers to unknown list `{list}`")));
            }
            if !placed.insert(list.as_str()) {
                issues.push(error(list, "list is placed in more than one phase"));
            }
        }
    }
    for list in &idx.lists {
        if !placed.contains(list) {
            issues.push(error(*list, "list is not placed in any phase"));
        }
    }
    for stage in &file.stages {
        for place in &stage.phases {
            idx.require_phase(&stage.title, &place.phase, issues);
            if let Some(show_if) = &place.show_if {
                idx.check_show_if(&stage.title, show_if, issues);
            }
        }
    }
}

/// Choices, when given, show each value of the option exactly once.
fn check_choices(option: &UserOption, issues: &mut Vec<Issue>) {
    if option.choices.is_empty() {
        return;
    }
    let mut seen = BTreeSet::new();
    for choice in &option.choices {
        if !option.values.contains(&choice.value) {
            issues.push(error(
                &option.id,
                format!("choice for unknown value `{}`", choice.value),
            ));
        } else if !seen.insert(choice.value.as_str()) {
            issues.push(error(
                &option.id,
                format!("more than one choice for value `{}`", choice.value),
            ));
        }
    }
    for value in &option.values {
        if !seen.contains(value.as_str()) {
            issues.push(error(&option.id, format!("no choice for value `{value}`")));
        }
    }
}

#[cfg(test)]
mod tests {
    use checklist_core::{OptionChoice, OptionTest, ShowIf};

    use super::super::test_support::{assert_flags, pack, step};

    #[test]
    fn stage_places_are_checked_too() {
        let mut p = pack(vec![step("a")], vec![]);
        if let Some(file) = &mut p.content.phases {
            file.stages[0].phases[0].show_if = Some(ShowIf::Test(OptionTest::Equals {
                option: "power".into(),
                equals: "nope".into(),
            }));
        }
        assert_flags(&p, "has no value");
    }

    #[test]
    fn every_list_sits_in_exactly_one_phase() {
        let mut p = pack(vec![step("a")], vec![]);
        if let Some(file) = &mut p.content.phases {
            file.phases[0].lists.clear();
        }
        assert_flags(&p, "not placed in any phase");

        let mut p = pack(vec![step("a")], vec![]);
        if let Some(file) = &mut p.content.phases {
            file.phases[0].lists = vec!["preflight".into(), "preflight".into(), "ghost".into()];
        }
        assert_flags(&p, "more than one phase");
        assert_flags(&p, "unknown list");
    }

    #[test]
    fn a_turnaround_step_needs_a_turnaround_declaration() {
        let mut s = step("a");
        s.quick = Some(checklist_core::Quick::Turnaround);
        let mut p = pack(vec![s.clone()], vec![]);
        assert_flags(&p, "declares no `turnaround`");
        if let Some(file) = &mut p.content.phases {
            file.turnaround = Some(ShowIf::Test(OptionTest::Equals {
                option: "power".into(),
                equals: "nope".into(),
            }));
        }
        assert_flags(&p, "has no value `nope`");
        let mut p = pack(vec![s], vec![]);
        if let Some(file) = &mut p.content.phases {
            file.turnaround = Some(ShowIf::Test(OptionTest::Equals {
                option: "power".into(),
                equals: "apu".into(),
            }));
        }
        assert!(
            crate::validate_pack(&p)
                .iter()
                .all(|i| !i.message.contains("turnaround"))
        );
    }

    #[test]
    fn choices_cover_each_value_once_and_ask_in_names_phases() {
        let choice = |value: &str| OptionChoice {
            value: value.into(),
            label: value.to_uppercase(),
            description: None,
        };
        let mut p = pack(vec![step("a")], vec![]);
        if let Some(file) = &mut p.content.phases {
            file.options[0].choices = vec![choice("gpu"), choice("gpu"), choice("solar")];
            file.options[0].ask_in = vec!["preflight".into(), "ghost".into()];
        }
        assert_flags(&p, "more than one choice for value `gpu`");
        assert_flags(&p, "choice for unknown value `solar`");
        assert_flags(&p, "no choice for value `apu`");
        assert_flags(&p, "unknown phase `ghost`");

        let mut p = pack(vec![step("a")], vec![]);
        if let Some(file) = &mut p.content.phases {
            file.options[0].choices = vec![choice("gpu"), choice("apu")];
            file.options[0].ask_in = vec!["preflight".into()];
        }
        assert!(
            crate::validate_pack(&p)
                .iter()
                .all(|i| !i.message.contains("choice"))
        );
    }

    #[test]
    fn an_option_default_must_be_one_of_its_values() {
        let mut p = pack(vec![step("a")], vec![]);
        if let Some(file) = &mut p.content.phases {
            file.options[0].default = "solar".into();
        }
        assert_flags(&p, "has no value `solar`");
    }
}
