// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Reading packs from disk. YAML is parsed strictly: unknown fields, duplicate keys
//! and YAML 1.1 "yes/no" booleans are all rejected, with line and column in the error.

use std::path::{Path, PathBuf};

use checklist_core::Checklist;

use serde::de::DeserializeOwned;

use crate::manifest::PackManifest;
use crate::{Pack, PackContent};

#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    #[error("cannot read {path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("{path}: {message}")]
    Yaml { path: PathBuf, message: String },
    #[error("{path}: {source}")]
    Json {
        path: PathBuf,
        source: serde_json::Error,
    },
}

fn options() -> serde_saphyr::Options {
    // Options is non-exhaustive, so set fields on a default rather than building a literal.
    // Duplicate keys are already an error by default.
    let mut options = serde_saphyr::Options::default();
    options.strict_booleans = true;
    options
}

/// Parse one pack data file from YAML text. `origin` is only used in error messages.
pub fn parse_yaml<T: DeserializeOwned>(text: &str, origin: &Path) -> Result<T, LoadError> {
    serde_saphyr::from_str_with_options(text, options()).map_err(|e| LoadError::Yaml {
        path: origin.to_path_buf(),
        message: e.to_string(),
    })
}

/// Parse one checklist from YAML text. `origin` is only used in error messages.
pub fn parse_checklist_yaml(text: &str, origin: &Path) -> Result<Checklist, LoadError> {
    parse_yaml(text, origin)
}

fn read(path: &Path) -> Result<String, LoadError> {
    std::fs::read_to_string(path).map_err(|source| LoadError::Io {
        path: path.to_path_buf(),
        source,
    })
}

fn load_file<T: DeserializeOwned>(dir: &Path, rel: &str) -> Result<T, LoadError> {
    let path = dir.join(rel);
    parse_yaml(&read(&path)?, &path)
}

fn load_optional<T: DeserializeOwned>(
    dir: &Path,
    rel: Option<&String>,
) -> Result<Option<T>, LoadError> {
    rel.map(|r| load_file(dir, r)).transpose()
}

/// Load `pack.json` and every file it lists from a pack folder.
pub fn load_pack_dir(dir: &Path) -> Result<Pack, LoadError> {
    let manifest_path = dir.join("pack.json");
    let manifest: PackManifest =
        serde_json::from_str(&read(&manifest_path)?).map_err(|source| LoadError::Json {
            path: manifest_path.clone(),
            source,
        })?;

    let checklists = manifest
        .checklists
        .iter()
        .map(|rel| load_file(dir, rel))
        .collect::<Result<Vec<Checklist>, _>>()?;
    let content = PackContent {
        phases: load_optional(dir, manifest.phases.as_ref())?,
        controls: load_optional(dir, manifest.controls.as_ref())?,
        notes: load_optional(dir, manifest.notes.as_ref())?,
        panels: manifest
            .panels
            .iter()
            .map(|rel| load_file(dir, rel))
            .collect::<Result<_, _>>()?,
        lights: load_optional(dir, manifest.lights.as_ref())?,
        faults: load_optional(dir, manifest.faults.as_ref())?,
        glossary: load_optional(dir, manifest.glossary.as_ref())?,
        briefings: load_optional(dir, manifest.briefings.as_ref())?,
        start_states: load_optional(dir, manifest.start_states.as_ref())?,
        cdu: load_optional(dir, manifest.cdu.as_ref())?,
        aliases: load_optional(dir, manifest.aliases.as_ref())?,
    };

    let mut other_files = Vec::new();
    collect_files(dir, dir, &mut other_files)?;
    let listed = manifest.data_files();
    other_files
        .retain(|p| p != Path::new("pack.json") && !listed.iter().any(|c| Path::new(c) == p));
    other_files.sort();

    Ok(Pack {
        manifest,
        checklists,
        content,
        other_files,
    })
}

fn collect_files(root: &Path, dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), LoadError> {
    let entries = std::fs::read_dir(dir).map_err(|source| LoadError::Io {
        path: dir.to_path_buf(),
        source,
    })?;
    for entry in entries {
        let path = entry
            .map_err(|source| LoadError::Io {
                path: dir.to_path_buf(),
                source,
            })?
            .path();
        if path.is_dir() {
            collect_files(root, &path, out)?;
        } else if let Ok(rel) = path.strip_prefix(root) {
            out.push(rel.to_path_buf());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(yaml: &str) -> Result<Checklist, String> {
        parse_checklist_yaml(yaml, Path::new("test.yaml")).map_err(|e| e.to_string())
    }

    const VALID: &str = "\
id: preflight
title: Preflight
kind: checklist
basis: [sim-observed]
steps:
  - id: demo.preflight.irs
    text: IRS mode selectors
    expect: NAV
    status: draft
    sim_simplified: true
  - id: demo.preflight.yaw-damper
    text: Yaw damper
    expect: NO
";

    #[test]
    fn valid_yaml_parses_and_keeps_text_values() {
        let list = parse(VALID).unwrap();
        assert_eq!(list.steps.len(), 2);
        // `NO` must stay text: a switch position, not a boolean.
        assert_eq!(list.steps[1].expect, "NO");
    }

    #[test]
    fn misspelled_field_is_rejected_with_location() {
        let err = parse("id: p\ntitle: P\nsteps:\n  - id: a\n    text: T\n    expcet: NAV\n")
            .unwrap_err();
        assert!(err.contains("unknown field `expcet`"), "{err}");
        assert!(err.contains("line 6"), "{err}");
    }

    #[test]
    fn duplicate_key_is_rejected() {
        let err = parse("id: p\ntitle: P\ntitle: Q\nsteps: []\n").unwrap_err();
        assert!(err.contains("duplicate"), "{err}");
    }

    fn with_step_field(field: &str) -> String {
        format!(
            "id: p\ntitle: P\nkind: flow\nbasis: [a]\nsteps:\n  - id: a\n    text: T\n    expect: E\n    {field}\n"
        )
    }

    #[test]
    fn simulator_model_is_read_separately_from_sim_simplified() {
        let list = parse(&with_step_field("simulator_model: true")).unwrap();
        assert!(list.steps[0].simulator_model);
        assert!(!list.steps[0].sim_simplified);
    }

    #[test]
    fn list_needs_a_kind_and_a_basis() {
        let err = parse("id: p\ntitle: P\nbasis: [a]\nsteps: []\n").unwrap_err();
        assert!(err.contains("kind"), "{err}");
        let err = parse("id: p\ntitle: P\nkind: flow\nsteps: []\n").unwrap_err();
        assert!(err.contains("basis"), "{err}");
    }

    #[test]
    fn per_step_sources_are_gone() {
        assert!(parse(&with_step_field("sources: []")).is_err());
    }

    #[test]
    fn show_if_accepts_each_form_of_the_grammar() {
        for field in [
            "show_if: {option: power, equals: gpu}",
            "show_if: {option: approach, one_of: [ils, rnav]}",
            "show_if: {all_of: [{option: power, equals: gpu}, {option: approach, one_of: [ils]}]}",
            "show_if: {any_of: [{option: power, equals: gpu}]}",
        ] {
            let list = parse(&with_step_field(field)).unwrap_or_else(|e| panic!("{field}: {e}"));
            assert!(list.steps[0].show_if.is_some(), "{field}");
        }
    }

    #[test]
    fn show_if_rejects_anything_outside_the_grammar() {
        for field in [
            "show_if: {option: power, not_equals: gpu}",
            "show_if: {not: {option: power, equals: gpu}}",
            "show_if: {option: power, equals: gpu, one_of: [gpu]}",
            "show_if: {all_of: [{any_of: [{option: power, equals: gpu}]}]}",
            "show_if: power == gpu",
        ] {
            assert!(parse(&with_step_field(field)).is_err(), "{field}");
        }
    }

    #[test]
    fn fill_names_come_from_the_app() {
        let list = parse(&with_step_field("fill: [zfw, v1, cost_index, manual]")).unwrap();
        assert_eq!(list.steps[0].fill.len(), 4);
        let err = parse(&with_step_field("fill: [my_field]")).unwrap_err();
        assert!(err.contains("my_field"), "{err}");
    }

    #[test]
    fn cdu_text_mixes_styles_and_groups_do_not_nest() {
        use checklist_core::{CduFile, CduPart};
        let doc = |action: &str| {
            format!(
                "walkthroughs:\n  - id: w\n    title: W\n    pages:\n      - page: P\n        title: T\n        action: {action}\n        check: []\n        screen: {{title: [], page: [], rows: []}}\n"
            )
        };
        let good = doc(
            "[{when_known: [runway], show: ['<SEL> ', {field: runway, style: user}], otherwise: [{text: '(none)', style: data}]}]",
        );
        let file: CduFile = parse_yaml(&good, Path::new("c.yaml")).unwrap();
        let part = &file.walkthroughs[0].pages[0].action[0];
        assert!(matches!(part, CduPart::When(w) if w.show.len() == 2));

        let route = doc(
            "[{field: route_leg, index: 30, part: via}, {route_legs: overflow, per_page: 5, empty: none}]",
        );
        let file: CduFile = parse_yaml(&route, Path::new("c.yaml")).unwrap();
        assert!(matches!(
            file.walkthroughs[0].pages[0].action[1],
            CduPart::RouteCount(_)
        ));
        let retired = doc("[{field: route_leg1_via}]");
        assert!(parse_yaml::<CduFile>(&retired, Path::new("c.yaml")).is_err());

        let nested = doc("[{when_known: [runway], show: [{when_known: [sid], show: [x]}]}]");
        assert!(parse_yaml::<CduFile>(&nested, Path::new("c.yaml")).is_err());
    }

    #[test]
    fn example_pack_uses_every_content_file_and_passes() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../packs/example-trainer");
        let pack = load_pack_dir(&dir).unwrap();
        let c = &pack.content;
        assert!(
            c.phases.is_some()
                && c.controls.is_some()
                && c.notes.is_some()
                && !c.panels.is_empty()
                && c.lights.is_some()
                && c.faults.is_some()
                && c.glossary.is_some()
                && c.briefings.is_some()
                && c.start_states.is_some()
                && c.cdu.is_some()
                && c.aliases.is_some()
        );
        assert_eq!(pack.other_files, Vec::<PathBuf>::new());
        assert_eq!(crate::validate_pack(&pack), Vec::new());
    }

    proptest::proptest! {
        /// The parser returns an error, never a panic, on arbitrary input.
        #[test]
        fn never_panics(input in ".{0,400}") {
            let _ = parse(&input);
        }
    }
}
