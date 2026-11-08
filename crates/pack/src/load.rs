// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Reading packs from disk. YAML is parsed strictly: unknown fields, duplicate keys
//! and YAML 1.1 "yes/no" booleans are all rejected, with line and column in the error.

use std::path::{Path, PathBuf};

use checklist_core::Checklist;

use crate::Pack;
use crate::manifest::PackManifest;

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

/// Parse one checklist from YAML text. `origin` is only used in error messages.
pub fn parse_checklist_yaml(text: &str, origin: &Path) -> Result<Checklist, LoadError> {
    serde_saphyr::from_str_with_options(text, options()).map_err(|e| LoadError::Yaml {
        path: origin.to_path_buf(),
        message: e.to_string(),
    })
}

fn read(path: &Path) -> Result<String, LoadError> {
    std::fs::read_to_string(path).map_err(|source| LoadError::Io {
        path: path.to_path_buf(),
        source,
    })
}

/// Load `pack.json` and every checklist it lists from a pack folder.
pub fn load_pack_dir(dir: &Path) -> Result<Pack, LoadError> {
    let manifest_path = dir.join("pack.json");
    let manifest: PackManifest =
        serde_json::from_str(&read(&manifest_path)?).map_err(|source| LoadError::Json {
            path: manifest_path.clone(),
            source,
        })?;

    let mut checklists = Vec::with_capacity(manifest.checklists.len());
    for rel in &manifest.checklists {
        let path = dir.join(rel);
        checklists.push(parse_checklist_yaml(&read(&path)?, &path)?);
    }

    let mut other_files = Vec::new();
    collect_files(dir, dir, &mut other_files)?;
    other_files.retain(|p| {
        let s = p.to_string_lossy();
        s != "pack.json"
            && !manifest
                .checklists
                .iter()
                .any(|c| Path::new(c) == p.as_path())
    });
    other_files.sort();

    Ok(Pack {
        manifest,
        checklists,
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
steps:
  - id: demo.preflight.irs
    text: IRS mode selectors
    expect: NAV
    sources:
      - title: Example source
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

    proptest::proptest! {
        /// The parser returns an error, never a panic, on arbitrary input.
        #[test]
        fn never_panics(input in ".{0,400}") {
            let _ = parse(&input);
        }
    }
}
