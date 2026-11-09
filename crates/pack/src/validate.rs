// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Semantic checks that a schema cannot express, including the project's content rules:
//! no blocked sources, no images, every step sourced, ids unique.

use std::collections::BTreeSet;
use std::fmt;

use crate::{Pack, SCHEMA_VERSION};

/// Domains whose content we must not use (non-public or redistributed manuals and decks).
/// Matching covers subdomains too (`www.`, `en.` and so on).
pub const BLOCKED_SOURCE_DOMAINS: &[&str] = &[
    "brainscape.com",
    "scribd.com",
    "studylib.net",
    "manualshive.com",
    "allafricanairways.com",
    "slideshare.net",
    "archive.org",
];

/// File types a pack must not contain. Vendor images and screenshots never ship.
const BLOCKED_FILE_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "bmp", "webp", "tif", "tiff", "pdf",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Issue {
    pub severity: Severity,
    /// Where: a checklist id, a step id, or a file name.
    pub at: String,
    pub message: String,
}

impl fmt::Display for Issue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let level = match self.severity {
            Severity::Warning => "warning",
            Severity::Error => "error",
        };
        write!(f, "{level}: {}: {}", self.at, self.message)
    }
}

fn error(at: impl Into<String>, message: impl Into<String>) -> Issue {
    Issue {
        severity: Severity::Error,
        at: at.into(),
        message: message.into(),
    }
}

/// The host part of a URL, lower-cased.
fn host_of(url: &str) -> Option<String> {
    let rest = url.split_once("://").map_or(url, |(_, r)| r);
    let host = rest.split(['/', '?', '#']).next()?;
    let host = host.rsplit_once('@').map_or(host, |(_, h)| h);
    let host = host.split(':').next()?;
    (!host.is_empty()).then(|| host.to_ascii_lowercase())
}

/// True when `url` points at a blocked domain or one of its subdomains.
pub fn is_blocked_source(url: &str) -> bool {
    host_of(url).is_some_and(|host| {
        BLOCKED_SOURCE_DOMAINS
            .iter()
            .any(|d| host == *d || host.ends_with(&format!(".{d}")))
    })
}

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '.' | '-'))
        && !id.starts_with(['.', '-'])
        && !id.ends_with(['.', '-'])
}

/// Run every semantic check. An empty list, or warnings only, means the pack may ship.
pub fn validate_pack(pack: &Pack) -> Vec<Issue> {
    let mut issues = Vec::new();
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
    if m.checklists.is_empty() && m.kind == crate::PackKind::Aircraft {
        issues.push(error(
            "pack.json",
            "an aircraft pack must list at least one checklist",
        ));
    }

    for file in &pack.other_files {
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

    let mut list_ids = BTreeSet::new();
    let mut step_ids = BTreeSet::new();
    for list in &pack.checklists {
        if !list_ids.insert(list.id.as_str()) {
            issues.push(error(&list.id, "checklist id is used twice"));
        }
        if list.steps.is_empty() {
            issues.push(error(&list.id, "checklist has no steps"));
        }
        for step in &list.steps {
            let at = step.id.as_str();
            if !valid_id(at) {
                issues.push(error(
                    at,
                    "step id must be lower-case letters, digits, dots and dashes",
                ));
            }
            if !step_ids.insert(at) {
                issues.push(error(at, "step id is used twice in this pack"));
            }
            if step.text.trim().is_empty() || step.expect.trim().is_empty() {
                issues.push(error(
                    at,
                    "step text and expected response must not be empty",
                ));
            }
            if step.sources.is_empty() {
                issues.push(error(
                    at,
                    "step has no source; every step must say where it came from",
                ));
            }
            for source in &step.sources {
                if let Some(url) = &source.url
                    && is_blocked_source(url)
                {
                    issues.push(error(at, format!("source `{url}` is on the blocked list")));
                }
            }
        }
    }

    issues.sort_by(|a, b| b.severity.cmp(&a.severity).then_with(|| a.at.cmp(&b.at)));
    issues
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use checklist_core::{Checklist, SourceRef, Step, StepId};

    use super::*;
    use crate::manifest::{PackKind, PackManifest};

    fn manifest() -> PackManifest {
        PackManifest {
            json_schema: None,
            id: "demo-aircraft".into(),
            name: "Demo".into(),
            version: semver::Version::new(0, 1, 0),
            schema_version: 1,
            kind: PackKind::Aircraft,
            engine: semver::VersionReq::parse(">=0.1").unwrap(),
            dependencies: std::collections::BTreeMap::default(),
            license: "CC-BY-NC-SA-4.0".into(),
            attribution: vec![],
            checklists: vec!["preflight.yaml".into()],
        }
    }

    fn step(id: &str, url: Option<&str>) -> Step {
        Step {
            id: StepId::new(id),
            text: "Battery".into(),
            expect: "ON".into(),
            note: None,
            sources: vec![SourceRef {
                title: "Source".into(),
                url: url.map(Into::into),
            }],
        }
    }

    fn pack(steps: Vec<Step>, other_files: Vec<&str>) -> Pack {
        Pack {
            manifest: manifest(),
            checklists: vec![Checklist {
                id: "preflight".into(),
                title: "Preflight".into(),
                steps,
            }],
            other_files: other_files.into_iter().map(PathBuf::from).collect(),
        }
    }

    #[test]
    fn clean_pack_has_no_issues() {
        let issues = validate_pack(&pack(
            vec![step("demo.a", Some("https://example.com/doc"))],
            vec![],
        ));
        assert_eq!(issues, Vec::<Issue>::new());
    }

    #[test]
    fn blocked_domains_match_subdomains_only() {
        assert!(is_blocked_source("https://www.brainscape.com/flashcards/x"));
        assert!(is_blocked_source("http://ARCHIVE.org/details/fcom"));
        assert!(!is_blocked_source("https://notbrainscape.com/x"));
        assert!(!is_blocked_source("https://example.com/brainscape.com"));
    }

    #[test]
    fn content_rules_are_enforced() {
        let mut unsourced = step("demo.b", None);
        unsourced.sources.clear();
        let issues = validate_pack(&pack(
            vec![
                step("demo.a", Some("https://www.scribd.com/doc/1")),
                unsourced,
                step("demo.a", None),
            ],
            vec!["cockpit.png", "notes.md"],
        ));
        let messages: Vec<String> = issues.iter().map(ToString::to_string).collect();
        let joined = messages.join("\n");
        assert!(joined.contains("blocked list"), "{joined}");
        assert!(joined.contains("no source"), "{joined}");
        assert!(joined.contains("used twice"), "{joined}");
        assert!(joined.contains("cockpit.png"), "{joined}");
        assert!(!joined.contains("notes.md"), "{joined}");
    }

    #[test]
    fn newer_schema_is_rejected() {
        let mut p = pack(vec![step("demo.a", None)], vec![]);
        p.manifest.schema_version = SCHEMA_VERSION + 1;
        assert!(
            validate_pack(&p)
                .iter()
                .any(|i| i.message.contains("newer than this app"))
        );
    }
}
