// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! The index of ids that content files can refer to, and the checks that look ids up in it.

use std::collections::{BTreeMap, BTreeSet};

use checklist_core::{OptionTest, ShowIf, StepId};

use super::ids::unique;
use super::{BLANK, Issue, error};
use crate::Pack;

/// The ids content files can refer to.
pub(super) struct Index<'a> {
    pub(super) pack_id: &'a str,
    pub(super) tags: BTreeSet<&'a str>,
    pub(super) lists: BTreeSet<&'a str>,
    pub(super) steps: BTreeSet<&'a str>,
    pub(super) blanks: BTreeMap<&'a str, usize>,
    pub(super) briefing_items: BTreeSet<&'a str>,
    pub(super) start_states: BTreeSet<&'a str>,
    pub(super) phases: BTreeSet<&'a str>,
    pub(super) options: BTreeMap<&'a str, BTreeSet<&'a str>>,
    pub(super) zones: BTreeSet<&'a str>,
    pub(super) controls: BTreeSet<&'a str>,
    pub(super) how: BTreeSet<&'a str>,
    pub(super) explain: BTreeSet<&'a str>,
    pub(super) walkthroughs: BTreeSet<&'a str>,
}

impl<'a> Index<'a> {
    pub(super) fn build(pack: &'a Pack, issues: &mut Vec<Issue>) -> Self {
        let c = &pack.content;
        let phases = c.phases.as_ref();
        let controls = c.controls.as_ref();
        let notes = c.notes.as_ref();
        let mut options = BTreeMap::new();
        let all_options = phases.map_or(&[][..], |p| p.options.as_slice());
        unique("option", all_options.iter().map(|o| o.id.as_str()), issues);
        for option in all_options {
            let values = unique(
                &format!("option `{}` value", option.id),
                option.values.iter().map(String::as_str),
                issues,
            );
            options.insert(option.id.as_str(), values);
        }
        Self {
            pack_id: &pack.manifest.id,
            tags: unique(
                "basis tag",
                pack.manifest.basis_tags.iter().map(String::as_str),
                issues,
            ),
            lists: unique(
                "list",
                pack.checklists.iter().map(|l| l.id.as_str()),
                issues,
            ),
            steps: unique(
                "step",
                pack.checklists
                    .iter()
                    .flat_map(|l| &l.steps)
                    .map(|s| s.id.as_str()),
                issues,
            ),
            phases: unique(
                "phase",
                phases.iter().flat_map(|p| &p.phases).map(|p| p.id.as_str()),
                issues,
            ),
            options,
            zones: unique(
                "zone",
                controls
                    .iter()
                    .flat_map(|c| &c.zones)
                    .map(|z| z.id.as_str()),
                issues,
            ),
            controls: unique(
                "control",
                controls
                    .iter()
                    .flat_map(|c| &c.controls)
                    .map(|c| c.id.as_str()),
                issues,
            ),
            how: unique(
                "how note",
                notes.iter().flat_map(|n| &n.how).map(|n| n.id.as_str()),
                issues,
            ),
            explain: unique(
                "explanation",
                notes.iter().flat_map(|n| &n.explain).map(|n| n.id.as_str()),
                issues,
            ),
            blanks: pack
                .checklists
                .iter()
                .flat_map(|l| &l.steps)
                .map(|s| (s.id.as_str(), s.expect.matches(BLANK).count()))
                .collect(),
            briefing_items: unique(
                "briefing item",
                c.briefings
                    .iter()
                    .flat_map(|f| &f.briefings)
                    .flat_map(|b| &b.items)
                    .map(|i| i.id.as_str()),
                issues,
            ),
            start_states: unique(
                "start state",
                c.start_states
                    .iter()
                    .flat_map(|f| &f.states)
                    .map(|s| s.id.as_str()),
                issues,
            ),
            walkthroughs: unique(
                "walkthrough",
                c.cdu
                    .iter()
                    .flat_map(|f| &f.walkthroughs)
                    .map(|w| w.id.as_str()),
                issues,
            ),
        }
    }

    pub(super) fn require_step(&self, at: &str, id: &StepId, issues: &mut Vec<Issue>) {
        if !self.steps.contains(id.as_str()) {
            issues.push(error(at, format!("refers to unknown step `{id}`")));
        }
    }

    pub(super) fn require_phase(&self, at: &str, id: &str, issues: &mut Vec<Issue>) {
        if !self.phases.contains(id) {
            issues.push(error(at, format!("refers to unknown phase `{id}`")));
        }
    }

    pub(super) fn require_control(&self, at: &str, id: &str, issues: &mut Vec<Issue>) {
        if !self.controls.contains(id) {
            issues.push(error(at, format!("refers to unknown control `{id}`")));
        }
    }

    pub(super) fn check_option_value(
        &self,
        at: &str,
        option: &str,
        value: &str,
        issues: &mut Vec<Issue>,
    ) {
        match self.options.get(option) {
            None => issues.push(error(at, format!("refers to unknown option `{option}`"))),
            Some(values) if !values.contains(value) => issues.push(error(
                at,
                format!("option `{option}` has no value `{value}`"),
            )),
            Some(_) => {}
        }
    }

    fn check_test(&self, at: &str, test: &OptionTest, issues: &mut Vec<Issue>) {
        match test {
            OptionTest::Equals { option, equals } => {
                self.check_option_value(at, option, equals, issues);
            }
            OptionTest::OneOf { option, one_of } => {
                if one_of.is_empty() {
                    issues.push(error(at, "`one_of` must list at least one value"));
                }
                for value in one_of {
                    self.check_option_value(at, option, value, issues);
                }
            }
        }
    }

    pub(super) fn check_show_if(&self, at: &str, show_if: &ShowIf, issues: &mut Vec<Issue>) {
        let tests = match show_if {
            ShowIf::Test(test) => std::slice::from_ref(test),
            ShowIf::AllOf { all_of: tests } | ShowIf::AnyOf { any_of: tests } => tests.as_slice(),
        };
        if tests.is_empty() {
            issues.push(error(
                at,
                "`all_of` and `any_of` must list at least one test",
            ));
        }
        for test in tests {
            self.check_test(at, test, issues);
        }
    }
}

#[cfg(test)]
mod tests {
    use checklist_core::{OptionTest, ShowIf};

    use super::super::test_support::{assert_flags, pack, step};

    #[test]
    fn show_if_must_use_declared_options_and_values() {
        for (show_if, expected) in [
            (
                ShowIf::Test(OptionTest::Equals {
                    option: "nope".into(),
                    equals: "gpu".into(),
                }),
                "unknown option",
            ),
            (
                ShowIf::Test(OptionTest::OneOf {
                    option: "power".into(),
                    one_of: vec!["nope".into()],
                }),
                "has no value",
            ),
            (
                ShowIf::Test(OptionTest::OneOf {
                    option: "power".into(),
                    one_of: vec![],
                }),
                "at least one value",
            ),
            (ShowIf::AllOf { all_of: vec![] }, "at least one test"),
            (ShowIf::AnyOf { any_of: vec![] }, "at least one test"),
        ] {
            let mut s = step("a");
            s.show_if = Some(show_if);
            assert_flags(&pack(vec![s], vec![]), expected);
        }
    }
}
