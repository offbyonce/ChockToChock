// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Controls, panels and lights.

use std::collections::BTreeMap;

use checklist_core::{PanelItem, PanelItemKind, named_position};

use super::confirmations::check_confirmations;
use super::ids::unique;
use super::index::Index;
use super::{Issue, error};
use crate::Pack;

pub(super) fn check_panels_and_lights(pack: &Pack, idx: &Index, issues: &mut Vec<Issue>) {
    let c = &pack.content;
    if let Some(file) = &c.controls {
        for control in &file.controls {
            if let Some(zone) = &control.zone
                && !idx.zones.contains(zone.as_str())
            {
                issues.push(error(
                    &control.id,
                    format!("refers to unknown zone `{zone}`"),
                ));
            }
        }
    }
    if let Some(overview) = c.controls.as_ref().and_then(|f| f.overview.as_ref()) {
        for area in &overview.areas {
            if !idx.zones.contains(area.zone.as_str()) {
                issues.push(error(
                    "overview",
                    format!("refers to unknown zone `{}`", area.zone),
                ));
            }
        }
        unique(
            "overview region",
            overview.regions.iter().map(|r| r.name.as_str()),
            issues,
        );
        for region in &overview.regions {
            let inside = region.x >= 0.0
                && region.y >= 0.0
                && region.width > 0.0
                && region.height > 0.0
                && region.x + region.width <= overview.width
                && region.y + region.height <= overview.height;
            if !inside {
                issues.push(error(
                    &region.name,
                    "region must have a size and lie inside the overview",
                ));
            }
        }
    }
    unique("panel", c.panels.iter().map(|p| p.id.as_str()), issues);
    let mut labels: BTreeMap<(&str, &str, &str), usize> = BTreeMap::new();
    for panel in &c.panels {
        unique(
            &format!("panel `{}` section", panel.id),
            panel.sections.iter().map(|s| s.id.as_str()),
            issues,
        );
        for section in &panel.sections {
            for item in &section.items {
                *labels
                    .entry((&panel.id, &section.id, &item.label))
                    .or_default() += 1;
                for control in &item.controls {
                    idx.require_control(&section.id, control, issues);
                }
                check_panel_item(&section.id, item, issues);
            }
        }
    }
    check_rest_against_cold_start(pack, issues);
    let Some(file) = &c.lights else {
        return;
    };
    unique(
        "light note",
        file.lights.iter().map(|l| l.locator.as_str()),
        issues,
    );
    for light in &file.lights {
        if !locator_resolves(&light.locator, &labels) {
            issues.push(error(&light.locator, "does not match any panel item"));
        }
        check_confirmations(&light.locator, &light.confirmed_by, issues);
    }
}

fn check_panel_item(at: &str, item: &PanelItem, issues: &mut Vec<Issue>) {
    let label = &item.label;
    let is_light = item.kind == PanelItemKind::Light;
    if is_light && item.color.is_none() {
        issues.push(error(at, format!("light `{label}` needs a `color`")));
    }
    if !is_light && item.color.is_some() {
        issues.push(error(at, format!("`{label}`: only a light has a `color`")));
    }
    if let Some(rest) = &item.rest {
        let named = item
            .positions
            .as_deref()
            .is_some_and(|p| p.split(',').any(|x| x.trim() == rest));
        if !named {
            issues.push(error(
                at,
                format!("`{label}`: rest `{rest}` is not one of its positions"),
            ));
        }
    }
    if item.guarded
        && !matches!(
            item.kind,
            PanelItemKind::ToggleSwitch | PanelItemKind::PushButton
        )
    {
        issues.push(error(
            at,
            format!("`{label}`: only a toggle switch or push button can be `guarded`"),
        ));
    }
}

/// Resting positions must agree with the cold and dark start state (the start state that opens
/// on the first phase of the first stage): a step that state counts as done is one whose
/// controls already rest where the step asks.
fn check_rest_against_cold_start(pack: &Pack, issues: &mut Vec<Issue>) {
    let c = &pack.content;
    let first = c
        .phases
        .as_ref()
        .and_then(|f| f.stages.first())
        .and_then(|s| s.phases.first())
        .map(|p| p.phase.as_str());
    let Some(cold) = c
        .start_states
        .as_ref()
        .and_then(|f| f.states.iter().find(|s| Some(s.phase.as_str()) == first))
    else {
        return;
    };
    let steps: BTreeMap<_, _> = pack
        .checklists
        .iter()
        .flat_map(|l| &l.steps)
        .map(|s| (&s.id, s))
        .collect();
    for id in &cold.done {
        let Some(step) = steps.get(id) else { continue };
        for section in c.panels.iter().flat_map(|p| &p.sections) {
            for item in &section.items {
                let (Some(rest), Some(positions)) = (&item.rest, &item.positions) else {
                    continue;
                };
                if !item.controls.iter().any(|k| step.controls.contains(k)) {
                    continue;
                }
                let Some(target) = named_position(&step.expect, positions) else {
                    continue;
                };
                let target = positions.split(',').nth(target).unwrap_or_default().trim();
                if target != rest {
                    issues.push(error(
                        &section.id,
                        format!(
                            "`{}` rests at `{rest}`, but start state `{}` counts `{}` as done, which sets it to `{target}`",
                            item.label, cold.id, step.id
                        ),
                    ));
                }
            }
        }
    }
}

/// `<panel>/<section>/<label>/<n>`. Labels can contain slashes, so split from both ends.
fn locator_resolves(locator: &str, labels: &BTreeMap<(&str, &str, &str), usize>) -> bool {
    let mut front = locator.splitn(3, '/');
    let (Some(panel), Some(section), Some(rest)) = (front.next(), front.next(), front.next())
    else {
        return false;
    };
    let Some((label, n)) = rest.rsplit_once('/') else {
        return false;
    };
    let Ok(n) = n.parse::<usize>() else {
        return false;
    };
    n >= 1
        && labels
            .get(&(panel, section, label))
            .is_some_and(|c| n <= *c)
}

#[cfg(test)]
mod tests {
    use checklist_core::{
        Control, ControlsFile, LightColor, LightNote, LightsFile, Overview, OverviewArea,
        OverviewRegion, Panel, PanelItem, PanelItemKind, PanelSection, StartState, StartStatesFile,
    };

    use super::super::test_support::{alias_pack, assert_flags, messages, pack, step};
    use crate::Pack;

    fn panel_pack() -> Pack {
        let mut p = pack(vec![step("a")], vec![]);
        p.content.controls = Some(ControlsFile {
            zones: vec![],
            controls: vec![Control {
                id: "bat".into(),
                name: "Battery".into(),
                zone: None,
                location: "Overhead".into(),
                function: "Connects the battery".into(),
            }],
            overview: None,
        });
        p.content.panels = vec![Panel {
            id: "oh".into(),
            title: "Overhead".into(),
            width: 100.0,
            height: 100.0,
            sections: vec![PanelSection {
                id: "elec".into(),
                title: "Electrical".into(),
                x: 0.0,
                y: 0.0,
                width: 10.0,
                height: 10.0,
                items: vec![PanelItem {
                    kind: PanelItemKind::Light,
                    label: "A/B LOW".into(),
                    color: Some(LightColor::Amber),
                    guarded: false,
                    x: 1.5,
                    y: 2.0,
                    controls: vec!["bat".into()],
                    positions: None,
                    rest: None,
                    description: "Low".into(),
                    width: None,
                    height: None,
                }],
            }],
        }];
        p
    }

    /// The battery as a toggle switch resting at `rest`, and a cold start that counts a step
    /// setting it OFF as done.
    fn rest_pack(rest: &str) -> Pack {
        let mut p = panel_pack();
        let item = &mut p.content.panels[0].sections[0].items[0];
        item.kind = PanelItemKind::ToggleSwitch;
        item.color = None;
        item.positions = Some("OFF, ON".into());
        item.rest = Some(rest.into());
        let id = p.checklists[0].steps[0].id.clone();
        p.checklists[0].steps[0].controls = vec!["bat".into()];
        p.checklists[0].steps[0].expect = "OFF".into();
        p.content.start_states = Some(StartStatesFile {
            states: vec![StartState {
                id: "cold".into(),
                title: "Cold".into(),
                description: "Cold".into(),
                set: std::collections::BTreeMap::default(),
                phase: "preflight".into(),
                done: vec![id],
                skip: vec![],
            }],
        });
        p
    }

    #[test]
    fn a_rest_position_is_one_of_the_positions_and_agrees_with_the_cold_start() {
        assert!(!messages(&rest_pack("OFF")).contains("rest"));
        assert_flags(
            &rest_pack("MAYBE"),
            "rest `MAYBE` is not one of its positions",
        );
        assert_flags(
            &rest_pack("ON"),
            "rests at `ON`, but start state `cold` counts",
        );
    }

    fn light(locator: &str) -> LightsFile {
        LightsFile {
            lights: vec![LightNote {
                locator: locator.into(),
                meaning: "M".into(),
                when: "W".into(),
                action: "A".into(),
                normal: false,
                confirmed_by: vec![],
            }],
        }
    }

    #[test]
    fn light_locators_resolve_even_with_slashes_in_labels() {
        let mut p = panel_pack();
        p.content.lights = Some(light("oh/elec/A/B LOW/1"));
        assert_eq!(messages(&p), "");
        for bad in [
            "oh/elec/A/B LOW/2",
            "oh/elec/A/B LOW/0",
            "oh/nope/X/1",
            "oh/elec",
        ] {
            let mut p = panel_pack();
            p.content.lights = Some(light(bad));
            assert_flags(&p, "does not match any panel item");
        }
    }

    #[test]
    fn overview_areas_must_name_known_zones() {
        let mut p = alias_pack(vec![]);
        let area = |zone: &str| OverviewArea {
            zone: zone.into(),
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
            label: None,
        };
        if let Some(file) = &mut p.content.controls {
            file.overview = Some(Overview {
                width: 100.0,
                height: 100.0,
                captions: vec![],
                areas: vec![area("overhead"), area("overhead"), area("ghost")],
                regions: vec![],
            });
        }
        let joined = messages(&p);
        assert!(joined.contains("unknown zone `ghost`"), "{joined}");
        assert!(!joined.contains("`overhead`"), "{joined}");
    }

    #[test]
    fn overview_regions_are_unique_and_inside_the_map() {
        let mut p = alias_pack(vec![]);
        let region = |name: &str, x: f64| OverviewRegion {
            name: name.into(),
            code: None,
            x,
            y: 0.0,
            width: 50.0,
            height: 50.0,
            top: None,
            bottom: None,
            left: None,
            right: None,
            locate: None,
        };
        if let Some(file) = &mut p.content.controls {
            file.overview = Some(Overview {
                width: 100.0,
                height: 100.0,
                captions: vec![],
                areas: vec![],
                regions: vec![
                    region("Overhead", 0.0),
                    region("Overhead", 10.0),
                    region("Wide", 80.0),
                ],
            });
        }
        let joined = messages(&p);
        assert!(joined.contains("Overhead"), "{joined}");
        assert!(joined.contains("lie inside the overview"), "{joined}");
    }

    #[test]
    fn panel_items_must_name_known_controls() {
        let mut p = panel_pack();
        p.content.panels[0].sections[0].items[0]
            .controls
            .push("nope".into());
        assert_flags(&p, "unknown control");
    }

    #[test]
    fn lights_need_a_color_and_only_lights_have_one() {
        let mut p = panel_pack();
        p.content.panels[0].sections[0].items[0].color = None;
        assert_flags(&p, "needs a `color`");

        let mut p = panel_pack();
        p.content.panels[0].sections[0].items[0].kind = PanelItemKind::Gauge;
        assert_flags(&p, "only a light has a `color`");
    }

    #[test]
    fn only_switches_and_buttons_can_be_guarded() {
        for kind in [PanelItemKind::ToggleSwitch, PanelItemKind::PushButton] {
            let mut p = panel_pack();
            let item = &mut p.content.panels[0].sections[0].items[0];
            item.kind = kind;
            item.color = None;
            item.guarded = true;
            assert_eq!(messages(&p), "");
        }
        let mut p = panel_pack();
        p.content.panels[0].sections[0].items[0].guarded = true;
        assert_flags(&p, "can be `guarded`");
    }
}
