// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Each step's explicit targets (decision 0018): they name real panel items on the step's
//! controls, in states those items have, and they agree with the step's wording, so the two
//! cannot drift apart.

use std::collections::BTreeSet;

use checklist_core::{
    LightState, PanelItemKind, Step, locate, named_position, names, names_position, says_light_on,
    switch_value,
};

use super::{Issue, error};
use crate::Pack;

pub(super) fn check_step_targets(pack: &Pack, issues: &mut Vec<Issue>) {
    let panels = &pack.content.panels;
    for step in pack.checklists.iter().flat_map(|l| &l.steps) {
        check_step(panels, step, issues);
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
        if !seen.insert(t.at.as_str()) {
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
        } else if !names_position(value, set) {
            issues.push(error(
                at,
                format!(
                    "`{}` is set to `{set}`, but the step's wording does not say so",
                    t.at
                ),
            ));
        }
    } else if !is_light
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
        let says_out = step.expect.split([',', ';']).any(|c| {
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
}
