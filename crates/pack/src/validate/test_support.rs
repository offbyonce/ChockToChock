// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Small packs the validation tests build on.

use std::path::PathBuf;

use checklist_core::{
    Alias, AliasKind, AliasesFile, BriefItem, Briefing, BriefingsFile, Checklist, ControlsFile,
    ListKind, Phase, PhasesFile, Stage, StagePhase, StartState, StartStatesFile, Step, UserOption,
    Zone,
};

use super::validate_pack;
use crate::manifest::{PackKind, PackManifest};
use crate::{Pack, PackContent, SCHEMA_VERSION};

pub(super) const PACK: &str = "demo-aircraft";

pub(super) fn manifest() -> PackManifest {
    PackManifest {
        json_schema: None,
        id: PACK.into(),
        name: "Demo".into(),
        version: semver::Version::new(0, 1, 0),
        schema_version: SCHEMA_VERSION,
        kind: PackKind::Aircraft,
        engine: semver::VersionReq::parse(">=0.1").unwrap(),
        dependencies: std::collections::BTreeMap::default(),
        license: "CC-BY-NC-SA-4.0".into(),
        attribution: vec![],
        basis_tags: vec!["sim-observed".into()],
        checklists: vec!["preflight.yaml".into()],
        phases: Some("phases.yaml".into()),
        controls: None,
        notes: None,
        panels: vec![],
        lights: None,
        faults: None,
        glossary: None,
        briefings: None,
        start_states: None,
        cdu: None,
        aliases: None,
    }
}

pub(super) fn step(short: &str) -> Step {
    Step::new(format!("{PACK}.preflight.{short}"), "Battery", "ON")
}

pub(super) fn phases() -> PhasesFile {
    PhasesFile {
        options: vec![UserOption {
            id: "power".into(),
            title: "Power".into(),
            values: vec!["gpu".into(), "apu".into()],
            default: "gpu".into(),
            description: None,
            choices: vec![],
            ask_in: vec![],
        }],
        stages: vec![Stage {
            title: "Preflight".into(),
            phases: vec![StagePhase {
                phase: "preflight".into(),
                show_if: None,
            }],
        }],
        phases: vec![Phase {
            id: "preflight".into(),
            title: "Preflight".into(),
            reference: false,
            intro: vec![],
            lists: vec!["preflight".into()],
        }],
    }
}

pub(super) fn pack(steps: Vec<Step>, other_files: Vec<&str>) -> Pack {
    Pack {
        manifest: manifest(),
        checklists: vec![Checklist {
            id: "preflight".into(),
            title: "Preflight".into(),
            kind: ListKind::Checklist,
            basis: vec!["sim-observed".into()],
            memory: false,
            when: None,
            why: None,
            steps,
        }],
        content: PackContent {
            phases: Some(phases()),
            ..PackContent::default()
        },
        other_files: other_files.into_iter().map(PathBuf::from).collect(),
    }
}

pub(super) fn messages(p: &Pack) -> String {
    validate_pack(p)
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n")
}

pub(super) fn assert_flags(p: &Pack, expected: &str) {
    let joined = messages(p);
    assert!(
        joined.contains(expected),
        "wanted `{expected}` in:\n{joined}"
    );
}

pub(super) fn alias(kind: AliasKind, from: &str, to: &str) -> Alias {
    Alias {
        kind,
        from: from.into(),
        to: to.into(),
    }
}

pub(super) fn alias_pack(aliases: Vec<Alias>) -> Pack {
    let mut s = step("a");
    s.expect = "V1 ___ VR ___".into();
    let mut p = pack(vec![s], vec![]);
    p.content.controls = Some(ControlsFile {
        zones: vec![Zone {
            id: "overhead".into(),
            area: "Overhead".into(),
            title: "Overhead".into(),
            short_title: "OH".into(),
        }],
        controls: vec![],
        overview: None,
    });
    p.content.briefings = Some(BriefingsFile {
        briefings: vec![Briefing {
            id: "departure".into(),
            title: "Departure".into(),
            mnemonic: "R".into(),
            phase: "preflight".into(),
            items: vec![BriefItem {
                id: "departure.runway".into(),
                lead: "R".into(),
                rest: "UNWAY".into(),
                detail: None,
            }],
        }],
    });
    p.content.start_states = Some(StartStatesFile {
        states: vec![StartState {
            id: "ready".into(),
            title: "Ready".into(),
            description: "Ready".into(),
            set: std::collections::BTreeMap::default(),
            phase: "preflight".into(),
            done: vec![],
            skip: vec![],
        }],
    });
    p.content.aliases = Some(AliasesFile { aliases });
    p
}
