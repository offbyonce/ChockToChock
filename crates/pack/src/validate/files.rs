// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! The manifest, and the files a pack holds: types and names.

use super::ids::{valid_id, valid_segment};
use super::{Issue, error};
use crate::{Pack, SCHEMA_VERSION};

/// File types a pack must not contain. Vendor images and screenshots never ship.
const BLOCKED_FILE_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "bmp", "webp", "tif", "tiff", "pdf",
];

/// Device names Windows reserves, besides `COM1`-`COM9` and `LPT1`-`LPT9`. A file with one of
/// these names cannot be checked out there, even with an extension.
const WINDOWS_RESERVED_NAMES: &[&str] = &["CON", "PRN", "AUX", "NUL"];

pub(super) fn check_manifest(pack: &Pack, issues: &mut Vec<Issue>) {
    let m = &pack.manifest;
    if !valid_id(&m.id) {
        issues.push(error(
            "pack.json",
            format!(
                "pack id `{}` must be lower-case letters, digits, dots and dashes",
                m.id
            ),
        ));
    }
    if m.schema_version > SCHEMA_VERSION {
        issues.push(error(
            "pack.json",
            format!(
                "schema version {} is newer than this app supports ({SCHEMA_VERSION})",
                m.schema_version
            ),
        ));
    }
    if m.kind == crate::PackKind::Aircraft {
        if m.checklists.is_empty() {
            issues.push(error(
                "pack.json",
                "an aircraft pack must list at least one checklist",
            ));
        }
        if pack.content.phases.is_none() {
            issues.push(error(
                "pack.json",
                "an aircraft pack must list a phases file",
            ));
        }
    }
    for tag in &m.basis_tags {
        if !valid_segment(tag) {
            issues.push(error(
                "pack.json",
                format!("basis tag `{tag}` must be lower-case letters, digits and dashes"),
            ));
        }
    }
}

/// Whether Windows reserves this file or folder name. Case and any extension do not matter.
fn is_windows_reserved(name: &str) -> bool {
    let stem = name.split('.').next().unwrap_or(name).to_ascii_uppercase();
    WINDOWS_RESERVED_NAMES.contains(&stem.as_str())
        || matches!(
            stem.as_bytes(),
            [b'C', b'O', b'M', b'1'..=b'9'] | [b'L', b'P', b'T', b'1'..=b'9']
        )
}

fn check_windows_name(path: &str, issues: &mut Vec<Issue>) {
    if path.split(['/', '\\']).any(is_windows_reserved) {
        issues.push(error(
            path,
            "this name is reserved on Windows, so the file cannot be checked out there",
        ));
    }
}

pub(super) fn check_files(pack: &Pack, issues: &mut Vec<Issue>) {
    for file in pack.manifest.data_files() {
        check_windows_name(file, issues);
    }
    for file in &pack.other_files {
        check_windows_name(&file.to_string_lossy(), issues);
        let ext = file
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        if BLOCKED_FILE_EXTENSIONS.contains(&ext.as_str()) {
            issues.push(error(
                file.to_string_lossy(),
                "packs must not contain images or PDFs; vendor artwork, screenshots and manuals never ship",
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::test_support::{assert_flags, messages, pack, step};
    use super::*;

    #[test]
    fn windows_reserved_names_are_recognized() {
        for name in [
            "CON",
            "con",
            "Con.yaml",
            "prn.txt",
            "AUX",
            "nul.tar.gz",
            "com1",
            "COM9.yaml",
            "lpt1",
            "Lpt9.md",
        ] {
            assert!(is_windows_reserved(name), "{name}");
        }
        for name in [
            "console.yaml",
            "center-console.yaml",
            "com",
            "com0",
            "com10",
            "lpt0",
            "lpt.yaml",
            "auxiliary",
            "null",
            "my-con.yaml",
        ] {
            assert!(!is_windows_reserved(name), "{name}");
        }
    }

    #[test]
    fn rejects_windows_reserved_names_listed_in_the_manifest() {
        let mut p = pack(vec![], vec![]);
        p.manifest.panels = vec!["panels/con.yaml".into(), "panels/oh.yaml".into()];
        p.manifest.checklists = vec!["aux/preflight.yaml".into()];
        let joined = messages(&p);
        assert!(joined.contains("panels/con.yaml"), "{joined}");
        assert!(joined.contains("aux/preflight.yaml"), "{joined}");
        assert!(!joined.contains("panels/oh.yaml"), "{joined}");
    }

    #[test]
    fn rejects_windows_reserved_names_among_other_files() {
        let p = pack(vec![], vec!["NUL", "extra/LPT3.md", "notes.md"]);
        let joined = messages(&p);
        assert!(joined.contains("NUL"), "{joined}");
        assert!(joined.contains("LPT3"), "{joined}");
        assert!(!joined.contains("notes.md"), "{joined}");
    }

    #[test]
    fn content_rules_are_enforced() {
        let joined = messages(&pack(
            vec![step("a"), step("a")],
            vec!["cockpit.png", "manual.pdf", "notes.md"],
        ));
        assert!(joined.contains("used twice"), "{joined}");
        assert!(joined.contains("cockpit.png"), "{joined}");
        assert!(joined.contains("manual.pdf"), "{joined}");
        assert!(!joined.contains("notes.md"), "{joined}");
    }

    #[test]
    fn newer_schema_is_rejected() {
        let mut p = pack(vec![step("a")], vec![]);
        p.manifest.schema_version = SCHEMA_VERSION + 1;
        assert_flags(&p, "newer than this app");
    }

    #[test]
    fn an_aircraft_pack_needs_phases() {
        let mut p = pack(vec![step("a")], vec![]);
        p.content.phases = None;
        assert_flags(&p, "phases file");
    }
}
