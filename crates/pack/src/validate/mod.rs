// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Semantic checks that a schema cannot express, including the project's content rules
//! (decision 0007): no banned or unapproved links, no images, a basis on every list, and every
//! reference between content files resolving.

mod aliases;
mod cdu;
mod files;
mod ids;
mod index;
mod links;
mod panels;
mod phases;
mod references;
mod steps;
#[cfg(test)]
mod test_support;

use std::fmt;

use cdu::check_cdu;
use files::{check_files, check_manifest};
use index::Index;
use links::check_links;
use panels::check_panels_and_lights;
use phases::check_phases;
use references::check_other_references;
use steps::check_list;

use crate::Pack;

pub use ids::valid_step_id;
pub use links::{ALLOWED_LINKS, BLOCKED_DOMAINS, BLOCKED_PATHS, find_links, is_blocked_link};

/// How a blank is written in a step's expected response.
const BLANK: &str = "___";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Issue {
    pub severity: Severity,
    /// Where: a list id, a step id, a file name or a path into the content.
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

/// Run every semantic check. An empty list, or warnings only, means the pack may ship.
pub fn validate_pack(pack: &Pack) -> Vec<Issue> {
    validate_with_allow_list(pack, ALLOWED_LINKS)
}

fn validate_with_allow_list(pack: &Pack, allow: &[&str]) -> Vec<Issue> {
    let mut issues = Vec::new();
    check_manifest(pack, &mut issues);
    check_files(pack, &mut issues);
    let idx = Index::build(pack, &mut issues);
    for list in &pack.checklists {
        check_list(list, &idx, &mut issues);
    }
    check_phases(pack, &idx, &mut issues);
    check_panels_and_lights(pack, &idx, &mut issues);
    check_other_references(pack, &idx, &mut issues);
    check_cdu(pack, &mut issues);
    check_links(pack, allow, &mut issues);
    issues.sort_by(|a, b| b.severity.cmp(&a.severity).then_with(|| a.at.cmp(&b.at)));
    issues
}

#[cfg(test)]
mod tests {
    use super::test_support::{messages, pack, step};
    use super::*;

    #[test]
    fn clean_pack_has_no_issues() {
        assert_eq!(
            validate_pack(&pack(vec![step("a")], vec![])),
            Vec::<Issue>::new()
        );
    }

    #[test]
    fn a_step_needs_no_source() {
        // Decision 0007 dropped the per-step source rule.
        assert_eq!(messages(&pack(vec![step("a")], vec![])), "");
    }
}
