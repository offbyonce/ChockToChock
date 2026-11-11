// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Which phases and steps show for the pilot's options (decision 0012).

use std::collections::BTreeMap;

use checklist_core::{Checklist, OptionTest, ShowIf, Step};

fn test_passes(test: &OptionTest, options: &BTreeMap<String, String>) -> bool {
    match test {
        OptionTest::Equals { option, equals } => options.get(option) == Some(equals),
        OptionTest::OneOf { option, one_of } => {
            options.get(option).is_some_and(|v| one_of.contains(v))
        }
    }
}

/// Whether a `show_if` holds for these option choices.
pub fn passes(show_if: &ShowIf, options: &BTreeMap<String, String>) -> bool {
    match show_if {
        ShowIf::Test(test) => test_passes(test, options),
        ShowIf::AllOf { all_of } => all_of.iter().all(|t| test_passes(t, options)),
        ShowIf::AnyOf { any_of } => any_of.iter().any(|t| test_passes(t, options)),
    }
}

/// Whether a step shows for these option choices.
pub fn step_shows(step: &Step, options: &BTreeMap<String, String>) -> bool {
    step.show_if.as_ref().is_none_or(|s| passes(s, options))
}

/// The steps of a list that show for these option choices, in order.
pub fn visible_steps<'a>(
    list: &'a Checklist,
    options: &'a BTreeMap<String, String>,
) -> impl Iterator<Item = &'a Step> {
    list.steps.iter().filter(move |s| step_shows(s, options))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn options(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| ((*k).into(), (*v).into()))
            .collect()
    }

    fn eq(option: &str, value: &str) -> OptionTest {
        OptionTest::Equals {
            option: option.into(),
            equals: value.into(),
        }
    }

    #[test]
    fn tests_combine_one_level_deep() {
        let gpu_cold = options(&[("power", "gpu"), ("start", "cold")]);
        assert!(passes(&ShowIf::Test(eq("power", "gpu")), &gpu_cold));
        assert!(!passes(&ShowIf::Test(eq("power", "apu")), &gpu_cold));
        assert!(passes(
            &ShowIf::AllOf {
                all_of: vec![eq("power", "gpu"), eq("start", "cold")]
            },
            &gpu_cold
        ));
        assert!(!passes(
            &ShowIf::AllOf {
                all_of: vec![eq("power", "gpu"), eq("start", "turn")]
            },
            &gpu_cold
        ));
        assert!(passes(
            &ShowIf::AnyOf {
                any_of: vec![eq("power", "apu"), eq("start", "cold")]
            },
            &gpu_cold
        ));
        let one_of = OptionTest::OneOf {
            option: "power".into(),
            one_of: vec!["apu".into(), "gpu".into()],
        };
        assert!(passes(&ShowIf::Test(one_of), &gpu_cold));
        assert!(!passes(&ShowIf::Test(eq("missing", "x")), &gpu_cold));
    }

    #[test]
    fn steps_without_a_condition_always_show() {
        let mut step = Step::new("demo.p.a", "A", "ON");
        assert!(step_shows(&step, &options(&[])));
        step.show_if = Some(ShowIf::Test(eq("power", "gpu")));
        assert!(!step_shows(&step, &options(&[("power", "apu")])));
    }
}
