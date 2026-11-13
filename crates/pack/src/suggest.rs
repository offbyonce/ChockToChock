// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Proposes a step's targets from its wording, for an author to check and keep (decision 0018).
//! Drawings never read wording: they show the targets a pack states. This only drafts them, and
//! says where its reading was not certain.

use checklist_core::{
    LightState, Panel, PanelItem, PanelItemKind, Step, StepTarget, clauses, locator,
    named_position, names, says_light_on, switch_value,
};
use serde::Serialize;

/// Proposed targets for one step, and why any of them may be wrong.
#[derive(Debug, Clone, Serialize)]
pub struct Suggestion {
    pub targets: Vec<StepTarget>,
    /// Each place the reading was not certain, in words an author can check.
    pub flags: Vec<String>,
}

struct Found<'a> {
    panel: &'a Panel,
    section: &'a checklist_core::PanelSection,
    index: usize,
    item: &'a PanelItem,
}

/// Whether a step speaks of lights: it says LIGHT or LIGHTS, or names the light.
fn mentions_light(step: &Step, label: &str) -> bool {
    let text = format!("{} {}", step.text, step.expect);
    ["LIGHT", "LIGHTS", "LT", "LTS"]
        .iter()
        .any(|w| names(&text, w))
        || names(&text, label)
}

fn positions_of(item: &PanelItem) -> Vec<String> {
    item.positions
        .as_deref()
        .map(|p| p.split(',').map(|x| x.trim().to_owned()).collect())
        .unwrap_or_default()
}

/// Each switch's position, where the wording (light names taken out) names exactly one.
fn read_positions(found: &[Found], value: &str, flags: &mut Vec<String>) -> Vec<Option<String>> {
    found
        .iter()
        .map(|f| {
            let ps = positions_of(f.item);
            let p = f.item.positions.as_deref()?;
            if let Some(i) = named_position(value, p) {
                return ps.get(i).cloned();
            }
            if f.item.kind != PanelItemKind::Light && ps.len() > 1 {
                let named = ps.iter().filter(|x| names(value, x)).count();
                let why = if named > 1 {
                    "names more than one of"
                } else {
                    "names none of"
                };
                flags.push(format!(
                    "{}: the wording {why} its positions ({})",
                    f.item.label,
                    ps.join(", ")
                ));
            }
            None
        })
        .collect()
}

/// A light's state: on when a switch the step sets lights it (`lit_at`), else as the wording says,
/// clause by clause; none (flagged) where the wording does not say.
fn light_state(
    step: &Step,
    f: &Found,
    lit_at: bool,
    sets_a_switch: bool,
    lights_kept: usize,
    flags: &mut Vec<String>,
) -> Option<LightState> {
    if lit_at {
        return Some(LightState::On);
    }
    let label = &f.item.label;
    let (on, sure) = says_light_on(&step.expect, label);
    let state = if sets_a_switch {
        // A value naming a switch position is about the switch; its light's state is known only
        // if a clause speaks of it.
        let clause_out = clauses(&step.expect).into_iter().any(|c| {
            (names(c, label) || names(c, "LIGHT") || names(c, "LIGHTS"))
                && (names(c, "OUT") || names(c, "OFF"))
        });
        if clause_out {
            Some(LightState::Out)
        } else if on && sure {
            Some(LightState::On)
        } else {
            None
        }
    } else {
        let out = ["OUT", "OFF", "EXTINGUISHED", "DARK"]
            .iter()
            .any(|w| names(&step.expect, w));
        if on {
            Some(LightState::On)
        } else if out {
            Some(LightState::Out)
        } else {
            None
        }
    };
    match state {
        None => flags.push(format!("{label}: light state not stated")),
        Some(s) if !sets_a_switch && !sure && lights_kept > 1 => flags.push(format!(
            "{label}: light state read from wording that does not name it ({})",
            if s == LightState::On { "on" } else { "out" }
        )),
        Some(_) => {}
    }
    state
}

/// A step naming some of its controls (TERR on the EFIS panel) keeps those, and lights with a
/// state.
fn keep_named(
    step: &Step,
    found: &[Found],
    out: Vec<(usize, StepTarget)>,
    flags: &mut Vec<String>,
) -> Vec<(usize, StepTarget)> {
    let keep = |(n, t): &(usize, StepTarget)| {
        names(&step.text, &found[*n].item.label)
            || (found[*n].item.kind == PanelItemKind::Light && t.light.is_some())
    };
    if !out
        .iter()
        .any(|(n, _)| names(&step.text, &found[*n].item.label))
    {
        return out;
    }
    let dropped: Vec<&str> = out
        .iter()
        .filter(|x| !keep(x))
        .map(|(n, _)| found[*n].item.label.as_str())
        .collect();
    if !dropped.is_empty() {
        flags.push(format!(
            "only the controls the step names are kept; left out: {}",
            dropped.join(", ")
        ));
    }
    out.into_iter().filter(|x| keep(x)).collect()
}

/// Where a step sets some switches, another switch it neither sets nor names (the ignition
/// selector on an engine start switch step) is not the step's.
fn drop_unnamed_switches(
    step: &Step,
    found: &[Found],
    out: Vec<(usize, StepTarget)>,
) -> Vec<(usize, StepTarget)> {
    if !out.iter().any(|(_, t)| t.set.is_some()) {
        return out;
    }
    let text = format!("{} {}", step.text, step.expect);
    out.into_iter()
        .filter(|(n, t)| {
            found[*n].item.kind == PanelItemKind::Light
                || t.set.is_some()
                || names(&text, &found[*n].item.label)
        })
        .collect()
}

/// Draft the targets of a step from its controls and wording.
pub fn suggest(panels: &[Panel], step: &Step) -> Suggestion {
    let mut flags = vec![];
    let found: Vec<Found> = panels
        .iter()
        .flat_map(|panel| {
            panel.sections.iter().flat_map(move |section| {
                section
                    .items
                    .iter()
                    .enumerate()
                    .map(move |(index, item)| Found {
                        panel,
                        section,
                        index,
                        item,
                    })
            })
        })
        .filter(|f| f.item.controls.iter().any(|c| step.controls.contains(c)))
        .collect();
    let is_light = |f: &Found| f.item.kind == PanelItemKind::Light;
    let switch_positions: Vec<&str> = found
        .iter()
        .filter(|f| !is_light(f))
        .filter_map(|f| f.item.positions.as_deref())
        .flat_map(|p| p.split(',').map(str::trim))
        .collect();
    let value = switch_value(
        &step.expect,
        found
            .iter()
            .filter(|f| is_light(f))
            .map(|f| f.item.label.as_str()),
        &switch_positions,
    );
    let set = read_positions(&found, &value, &mut flags);
    let sets_a_switch = found
        .iter()
        .zip(&set)
        .any(|(f, s)| !is_light(f) && s.is_some());
    let only_lights = !found.iter().any(|g| !is_light(g));
    let speaks = |f: &Found| only_lights || mentions_light(step, &f.item.label);
    let lights_kept = found.iter().filter(|f| is_light(f) && speaks(f)).count();
    let mut out: Vec<(usize, StepTarget)> = vec![];
    let mut left_out = vec![];
    for (n, f) in found.iter().enumerate() {
        let at = locator(f.panel, f.section, f.index);
        if !is_light(f) {
            out.push((
                n,
                StepTarget {
                    at,
                    set: set[n].clone(),
                    light: None,
                    open: false,
                    when: None,
                },
            ));
            continue;
        }
        let lit_at = f.item.lit_at.as_deref().is_some_and(|lit| {
            found.iter().zip(&set).any(|(g, s)| {
                g.item.controls.iter().any(|c| f.item.controls.contains(c))
                    && s.as_deref() == Some(lit)
            })
        });
        if !lit_at && !speaks(f) {
            left_out.push(f.item.label.clone());
            continue;
        }
        let light = light_state(step, f, lit_at, sets_a_switch, lights_kept, &mut flags);
        out.push((
            n,
            StepTarget {
                at,
                set: None,
                light,
                open: false,
                when: None,
            },
        ));
    }
    if !left_out.is_empty() {
        left_out.dedup();
        flags.push(format!(
            "lights left out, as the step does not speak of them: {}",
            left_out.join(", ")
        ));
    }
    let out = keep_named(step, &found, out, &mut flags);
    let out = drop_unnamed_switches(step, &found, out);
    if out.is_empty() && !found.is_empty() {
        flags.push("no target found".into());
    }
    flags.dedup();
    Suggestion {
        targets: out.into_iter().map(|(_, t)| t).collect(),
        flags,
    }
}

#[cfg(test)]
mod tests {
    use checklist_core::LightState;

    use super::suggest;

    fn ifly() -> crate::Pack {
        let dir =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../packs/ifly-737-max8");
        crate::load_pack_dir(&dir).unwrap()
    }

    fn step<'a>(pack: &'a crate::Pack, id: &str) -> &'a checklist_core::Step {
        pack.checklists
            .iter()
            .flat_map(|l| &l.steps)
            .find(|s| s.id.as_str() == id)
            .unwrap()
    }

    #[test]
    fn a_light_name_containing_on_does_not_set_its_switch() {
        // "GUARD CLOSED, PASS OXY ON LIGHT OUT": the switch is not at ON, and the light is out.
        let pack = ifly();
        let s = suggest(
            &pack.content.panels,
            step(&pack, "ifly-737-max8.preflight.passenger-oxygen-switch"),
        );
        let switch = s
            .targets
            .iter()
            .find(|t| t.at.contains("PASS OXYGEN"))
            .unwrap();
        assert_eq!(switch.set, None);
        let light = s
            .targets
            .iter()
            .find(|t| t.at.contains("PASS OXY ON"))
            .unwrap();
        assert_eq!(light.light, Some(LightState::Out));
    }

    #[test]
    fn a_switch_step_leaves_out_its_lights_and_says_so() {
        let pack = ifly();
        let s = suggest(
            &pack.content.panels,
            step(&pack, "ifly-737-max8.before-start.fuel-pump-switches"),
        );
        assert!(s.targets.iter().all(|t| !t.at.contains("LOW PRESSURE")));
        assert!(
            s.flags.iter().any(|f| f.starts_with("lights left out")),
            "{:?}",
            s.flags
        );
    }
}
