// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Id formats and duplicate ids.

use std::collections::BTreeSet;

use super::{Issue, error};

pub(super) fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '.' | '-'))
        && !id.starts_with(['.', '-'])
        && !id.ends_with(['.', '-'])
}

/// One part of a step id: lower-case letters, digits and inner dashes.
pub(super) fn valid_segment(segment: &str) -> bool {
    !segment.is_empty()
        && segment
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !segment.starts_with('-')
        && !segment.ends_with('-')
}

/// True when `id` is `<pack-id>.<phase>.<short-name>`. The phase part records where the step
/// was first published, so it is not required to match the step's current phase.
pub fn valid_step_id(pack_id: &str, id: &str) -> bool {
    id.strip_prefix(pack_id)
        .and_then(|rest| rest.strip_prefix('.'))
        .and_then(|rest| rest.split_once('.'))
        .is_some_and(|(phase, short)| valid_segment(phase) && valid_segment(short))
}

/// Collect ids, reporting any used twice.
pub(super) fn unique<'a>(
    what: &str,
    ids: impl IntoIterator<Item = &'a str>,
    issues: &mut Vec<Issue>,
) -> BTreeSet<&'a str> {
    let mut seen = BTreeSet::new();
    for id in ids {
        if !seen.insert(id) {
            issues.push(error(id, format!("{what} id is used twice in this pack")));
        }
    }
    seen
}

#[cfg(test)]
mod tests {
    use checklist_core::Step;
    use proptest::prelude::*;

    use super::super::test_support::{PACK, assert_flags, messages, pack};
    use super::*;

    #[test]
    fn step_ids_follow_the_format() {
        for bad in [
            "demo-aircraft.a",
            "other.preflight.a",
            "demo-aircraft.preflight.a.b",
            "demo-aircraft.preflight.A",
            "demo-aircraft..a",
        ] {
            let p = pack(vec![Step::new(bad, "T", "E")], vec![]);
            assert_flags(&p, "step id must be");
        }
    }

    #[test]
    fn a_moved_step_keeps_its_id() {
        // The phase part records first publication, so it need not name a current phase.
        let p = pack(
            vec![Step::new(format!("{PACK}.old-phase.a"), "T", "E")],
            vec![],
        );
        assert_eq!(messages(&p), "");
    }

    #[test]
    fn library_pack_ids_may_contain_dots() {
        assert!(valid_step_id("lib.shared-cdu", "lib.shared-cdu.perf.fuel"));
    }

    proptest! {
        /// Well-formed ids pass, and adding a level or changing the pack fails.
        #[test]
        fn step_id_format(phase in "[a-z0-9]{1,10}", short in "[a-z0-9]{1,10}") {
            let id = format!("demo.{phase}.{short}");
            prop_assert!(valid_step_id("demo", &id));
            let extra = format!("{id}.x");
            prop_assert!(!valid_step_id("demo", &extra));
            prop_assert!(!valid_step_id("other", &id));
        }
    }
}
