// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Each step's explicit targets (decision 0018): they name real panel items on the step's
//! controls, in states those items have, and they agree with the step's wording, so the two
//! cannot drift apart.

use std::collections::{BTreeMap, BTreeSet};

use checklist_core::{
    LightState, PanelItemKind, Step, clauses, locate, named_position, names, names_position,
    says_light_on, switch_value,
};

use super::{Issue, error};
use crate::Pack;

pub(super) fn check_step_targets(pack: &Pack, issues: &mut Vec<Issue>) {
    let panels = &pack.content.panels;
    let options: BTreeMap<&str, Vec<&str>> = pack
        .content
        .phases
        .iter()
        .flat_map(|p| &p.options)
        .map(|o| (o.id.as_str(), o.values.iter().map(String::as_str).collect()))
        .collect();
    let glossary: BTreeSet<&str> = pack
        .content
        .glossary
        .iter()
        .flat_map(|g| &g.terms)
        .map(|t| t.term.as_str())
        .collect();
    for step in pack.checklists.iter().flat_map(|l| &l.steps) {
        check_step(panels, step, issues);
        check_decision(&options, step, issues);
        for term in &step.terms {
            if !glossary.contains(term.as_str()) {
                issues.push(error(
                    step.id.as_str(),
                    format!("term `{term}` is not in the glossary"),
                ));
            }
        }
    }
}

/// Decision steps and the targets that depend on them (decision 0022): a step decides a real
/// option; a `when` names a real option and one of its values; one item's conditional targets
/// depend on one option, never twice on one value, and leave an `open` target for the time before
/// the answer unless they cover every value.
fn check_decision(options: &BTreeMap<&str, Vec<&str>>, step: &Step, issues: &mut Vec<Issue>) {
    let at = step.id.as_str();
    if let Some(option) = &step.decide
        && !options.contains_key(option.as_str())
    {
        issues.push(error(at, format!("`decide` names no option `{option}`")));
    }
    let mut by_item: BTreeMap<&str, Vec<&checklist_core::StepTarget>> = BTreeMap::new();
    for t in &step.targets {
        by_item.entry(t.at.as_str()).or_default().push(t);
    }
    for (item, targets) in by_item {
        let conditional: Vec<_> = targets.iter().filter_map(|t| t.when.as_ref()).collect();
        if conditional.is_empty() {
            continue;
        }
        let mut answers = BTreeSet::new();
        for w in &conditional {
            match options.get(w.option.as_str()) {
                None => issues.push(error(
                    at,
                    format!("`{item}`: `when` names no option `{}`", w.option),
                )),
                Some(values) if !values.contains(&w.equals.as_str()) => issues.push(error(
                    at,
                    format!(
                        "`{item}`: `{}` is not a value of option `{}`",
                        w.equals, w.option
                    ),
                )),
                Some(_) => {}
            }
            if !answers.insert(w.equals.as_str()) {
                issues.push(error(
                    at,
                    format!("`{item}` has two targets for `{} = {}`", w.option, w.equals),
                ));
            }
        }
        let option = conditional[0].option.as_str();
        if conditional.iter().any(|w| w.option != option) {
            issues.push(error(
                at,
                format!("`{item}`'s targets depend on more than one option"),
            ));
        }
        let covered = options
            .get(option)
            .is_some_and(|values| values.iter().all(|v| answers.contains(v)));
        let fallbacks: Vec<_> = targets.iter().filter(|t| t.when.is_none()).collect();
        if !covered && !fallbacks.iter().any(|t| t.open) {
            issues.push(error(
                at,
                format!(
                    "`{item}` depends on `{option}`, but has no `open` target for before the answer"
                ),
            ));
        }
    }
}

fn check_step(panels: &[checklist_core::Panel], step: &Step, issues: &mut Vec<Issue>) {
    let at = step.id.as_str();
    let drawable = panels
        .iter()
        .flat_map(|p| &p.sections)
        .flat_map(|s| &s.items)
        .any(|i| i.controls.iter().any(|c| step.controls.contains(c)));
    if drawable && step.targets.is_empty() {
        issues.push(error(
            at,
            "the step's controls are drawn, but it has no `targets`",
        ));
    }
    let mut seen = BTreeSet::new();
    let lights: Vec<&str> = step
        .targets
        .iter()
        .filter_map(|t| locate(panels, &t.at))
        .filter_map(|(s, i)| s.items.get(i))
        .filter(|i| i.kind == PanelItemKind::Light)
        .map(|i| i.label.as_str())
        .collect();
    let switch_positions: Vec<&str> = step
        .targets
        .iter()
        .filter_map(|t| locate(panels, &t.at))
        .filter_map(|(s, i)| s.items.get(i))
        .filter(|i| i.kind != PanelItemKind::Light)
        .filter_map(|i| i.positions.as_deref())
        .flat_map(|p| p.split(',').map(str::trim))
        .collect();
    let value = switch_value(&step.expect, lights.iter().copied(), &switch_positions);
    for t in &step.targets {
        // One item may have one target per answer of a decision (decision 0022); those are
        // checked in `check_decision`.
        if t.when.is_none() && !seen.insert(t.at.as_str()) {
            issues.push(error(at, format!("target `{}` is listed twice", t.at)));
        }
        let Some((section, index)) = locate(panels, &t.at) else {
            issues.push(error(
                at,
                format!("target `{}` does not match any panel item", t.at),
            ));
            continue;
        };
        let Some(item) = section.items.get(index) else {
            continue;
        };
        if !item.controls.iter().any(|c| step.controls.contains(c)) {
            issues.push(error(
                at,
                format!("target `{}` is not on one of the step's controls", t.at),
            ));
        }
        let is_light = item.kind == PanelItemKind::Light;
        if t.open && t.set.is_some() {
            issues.push(error(
                at,
                format!("`{}` leaves its position `open` and also `set`s one", t.at),
            ));
        }
        if is_light && t.set.is_some() {
            issues.push(error(
                at,
                format!("`{}`: a light takes `light`, not `set`", t.at),
            ));
        }
        if !is_light && t.light.is_some() {
            issues.push(error(at, format!("`{}`: only a light takes `light`", t.at)));
        }
        check_wording(at, step, item, t, &value, issues);
    }
}

/// A target agrees with the step's wording: a `set` position the wording names, a position the
/// wording names is set, and a light's state is not contradicted by the clause naming it (a
/// clause saying both OFF and ON, "OFF WHEN BOTH LOW PRESSURE LIGHTS COME ON", contradicts
/// neither).
fn check_wording(
    at: &str,
    step: &Step,
    item: &checklist_core::PanelItem,
    t: &checklist_core::StepTarget,
    value: &str,
    issues: &mut Vec<Issue>,
) {
    let is_light = item.kind == PanelItemKind::Light;
    let positions: Vec<&str> = item
        .positions
        .as_deref()
        .map(|p| p.split(',').map(str::trim).collect())
        .unwrap_or_default();
    if let Some(set) = &t.set {
        if !positions.contains(&set.as_str()) {
            issues.push(error(
                at,
                format!("`{}`: `{set}` is not one of its positions", t.at),
            ));
        } else if t.when.is_none() && !names_position(value, set) {
            issues.push(error(
                at,
                format!(
                    "`{}` is set to `{set}`, but the step's wording does not say so",
                    t.at
                ),
            ));
        }
    } else if !is_light
        && !t.open
        && let Some(n) = item
            .positions
            .as_deref()
            .and_then(|p| named_position(value, p))
    {
        issues.push(error(
            at,
            format!(
                "the step's wording sets `{}` to `{}`, but its target has no `set`",
                t.at, positions[n]
            ),
        ));
    }
    if let Some(light) = t.light {
        let says_out = clauses(&step.expect).into_iter().any(|c| {
            names(c, &item.label) && (names(c, "OUT") || names(c, "OFF")) && !names(c, "ON")
        });
        let (on, sure) = says_light_on(&step.expect, &item.label);
        match light {
            LightState::On if says_out => issues.push(error(
                at,
                format!(
                    "`{}` is expected on, but the step's wording says it is out",
                    t.at
                ),
            )),
            LightState::Out if on && sure => issues.push(error(
                at,
                format!(
                    "`{}` is expected out, but the step's wording says it is on",
                    t.at
                ),
            )),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use checklist_core::{LightState, StepTarget};

    use super::super::test_support::{assert_flags, messages};
    use crate::Pack;

    /// A pack with a battery switch and its light, and one step setting the switch.
    fn pack_with(expect: &str, targets: Vec<StepTarget>) -> Pack {
        let mut p = super::super::panels::tests::panel_pack();
        let item = &mut p.content.panels[0].sections[0].items[0];
        item.kind = checklist_core::PanelItemKind::ToggleSwitch;
        item.color = None;
        item.label = "BATTERY".into();
        item.positions = Some("OFF, ON".into());
        let mut light = item.clone();
        light.kind = checklist_core::PanelItemKind::Light;
        light.color = Some(checklist_core::LightColor::Amber);
        light.positions = None;
        light.label = "BAT DISCHARGE".into();
        p.content.panels[0].sections[0].items.push(light);
        p.checklists[0].steps[0].controls = vec!["bat".into()];
        p.checklists[0].steps[0].expect = expect.into();
        p.checklists[0].steps[0].targets = targets;
        p
    }

    fn target(at: &str, set: Option<&str>, light: Option<LightState>) -> StepTarget {
        StepTarget {
            at: at.into(),
            set: set.map(Into::into),
            light,
            open: false,
            when: None,
        }
    }

    #[test]
    fn targets_agree_with_the_wording() {
        let ok = pack_with(
            "ON, BAT DISCHARGE OUT",
            vec![
                target("oh/elec/BATTERY/1", Some("ON"), None),
                target("oh/elec/BAT DISCHARGE/1", None, Some(LightState::Out)),
            ],
        );
        assert!(!messages(&ok).contains("target"), "{}", messages(&ok));
        assert_flags(
            &pack_with("ON", vec![target("oh/elec/BATTERY/1", Some("OFF"), None)]),
            "is set to `OFF`, but the step's wording does not say so",
        );
        assert_flags(
            &pack_with("ON", vec![target("oh/elec/BATTERY/1", None, None)]),
            "the step's wording sets `oh/elec/BATTERY/1` to `ON`, but its target has no `set`",
        );
        assert_flags(
            &pack_with(
                "ON, BAT DISCHARGE OUT",
                vec![
                    target("oh/elec/BATTERY/1", Some("ON"), None),
                    target("oh/elec/BAT DISCHARGE/1", None, Some(LightState::On)),
                ],
            ),
            "is expected on, but the step's wording says it is out",
        );
    }

    #[test]
    fn a_position_left_open_is_accepted_but_not_with_a_set() {
        let mut open = target("oh/elec/BATTERY/1", None, None);
        open.open = true;
        assert!(!messages(&pack_with("ON IF NEEDED", vec![open.clone()])).contains("target"));
        open.set = Some("ON".into());
        assert_flags(
            &pack_with("ON IF NEEDED", vec![open]),
            "leaves its position `open` and also `set`s one",
        );
    }

    #[test]
    fn targets_name_real_items_in_states_they_have() {
        assert_flags(
            &pack_with("ON", vec![target("oh/elec/NOPE/1", Some("ON"), None)]),
            "does not match any panel item",
        );
        assert_flags(
            &pack_with("ON", vec![target("oh/elec/BATTERY/1", Some("HALF"), None)]),
            "`HALF` is not one of its positions",
        );
        assert_flags(
            &pack_with(
                "ON",
                vec![target("oh/elec/BATTERY/1", None, Some(LightState::On))],
            ),
            "only a light takes `light`",
        );
        assert_flags(&pack_with("ON", vec![]), "it has no `targets`");
        assert_flags(
            &pack_with(
                "ON",
                vec![
                    target("oh/elec/BATTERY/1", Some("ON"), None),
                    target("oh/elec/BATTERY/1", Some("ON"), None),
                ],
            ),
            "is listed twice",
        );
    }

    /// An invented decision: the "power" option of the test pack (gpu or apu) decides the battery.
    fn when(set: Option<&str>, equals: &str) -> StepTarget {
        StepTarget {
            when: Some(checklist_core::TargetWhen {
                option: "power".into(),
                equals: equals.into(),
            }),
            ..target("oh/elec/BATTERY/1", set, None)
        }
    }

    fn open() -> StepTarget {
        StepTarget {
            open: true,
            ..target("oh/elec/BATTERY/1", None, None)
        }
    }

    #[test]
    fn targets_may_depend_on_a_decision_with_an_open_target_before_the_answer() {
        let ok = pack_with(
            "ON IF ON GROUND POWER",
            vec![open(), when(Some("ON"), "gpu"), when(Some("OFF"), "apu")],
        );
        assert!(!messages(&ok).contains("BATTERY"), "{}", messages(&ok));
        // Every answer covered: no open target needed.
        let covered = pack_with(
            "ON IF ON GROUND POWER",
            vec![when(Some("ON"), "gpu"), when(Some("OFF"), "apu")],
        );
        assert!(
            !messages(&covered).contains("BATTERY"),
            "{}",
            messages(&covered)
        );
    }

    #[test]
    fn decisions_and_their_targets_are_held_to_the_pack() {
        let mut p = pack_with(
            "ON IF ON GROUND POWER",
            vec![open(), when(Some("ON"), "gpu")],
        );
        p.checklists[0].steps[0].decide = Some("centre".into());
        assert_flags(&p, "`decide` names no option `centre`");
        assert_flags(
            &pack_with(
                "ON IF ON GROUND POWER",
                vec![open(), when(Some("ON"), "battery")],
            ),
            "`battery` is not a value of option `power`",
        );
        assert_flags(
            &pack_with(
                "ON IF ON GROUND POWER",
                vec![open(), when(Some("ON"), "gpu"), when(Some("OFF"), "gpu")],
            ),
            "has two targets for `power = gpu`",
        );
        assert_flags(
            &pack_with("ON IF ON GROUND POWER", vec![when(Some("ON"), "gpu")]),
            "depends on `power`, but has no `open` target for before the answer",
        );
        let mut other = when(Some("OFF"), "apu");
        if let Some(w) = other.when.as_mut() {
            w.option = "start".into();
        }
        assert_flags(
            &pack_with(
                "ON IF ON GROUND POWER",
                vec![open(), when(Some("ON"), "gpu"), other],
            ),
            "targets depend on more than one option",
        );
        assert_flags(
            &pack_with("ON IF ON GROUND POWER", vec![open(), open()]),
            "target `oh/elec/BATTERY/1` is listed twice",
        );
    }

    #[test]
    fn a_steps_terms_are_in_the_glossary() {
        let mut p = pack_with("ON", vec![target("oh/elec/BATTERY/1", Some("ON"), None)]);
        p.content.glossary = Some(checklist_core::GlossaryFile {
            terms: vec![checklist_core::GlossaryEntry {
                term: "APU".into(),
                meaning: "Auxiliary power unit.".into(),
            }],
        });
        p.checklists[0].steps[0].terms = vec!["APU".into(), "XPDR".into()];
        let joined = messages(&p);
        assert!(
            joined.contains("term `XPDR` is not in the glossary"),
            "{joined}"
        );
        assert!(!joined.contains("term `APU`"), "{joined}");
    }
}
