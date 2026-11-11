// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Internal confirmation records on steps and light notes (decision 0013). A missing record is
//! never an issue; a record that is present must be well formed.

use std::collections::BTreeSet;

use checklist_core::Confirmation;

use super::{Issue, error};

fn leap(year: u32) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

/// True for a real calendar date written `YYYY-MM-DD`.
pub(super) fn valid_date(date: &str) -> bool {
    let parts: Vec<&str> = date.split('-').collect();
    let [y, m, d] = parts.as_slice() else {
        return false;
    };
    if y.len() != 4 || m.len() != 2 || d.len() != 2 {
        return false;
    }
    let (Ok(year), Ok(month), Ok(day)) = (y.parse::<u32>(), m.parse::<u32>(), d.parse::<u32>())
    else {
        return false;
    };
    let days = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if leap(year) => 29,
        2 => 28,
        _ => return false,
    };
    (1..=days).contains(&day)
}

pub(super) fn check_confirmations(at: &str, records: &[Confirmation], issues: &mut Vec<Issue>) {
    let mut seen = BTreeSet::new();
    for record in records {
        if !valid_date(&record.date) {
            issues.push(error(
                at,
                format!(
                    "confirmation date `{}` must be a real date written YYYY-MM-DD",
                    record.date
                ),
            ));
        }
        if !seen.insert((record.kind, record.date.as_str())) {
            issues.push(error(at, "the same confirmation is recorded twice"));
        }
    }
}

#[cfg(test)]
mod tests {
    use checklist_core::ConfirmationKind;

    use super::super::test_support::{assert_flags, messages, pack, step};
    use super::*;

    fn record(kind: ConfirmationKind, date: &str) -> Confirmation {
        Confirmation {
            kind,
            date: date.into(),
        }
    }

    #[test]
    fn dates_must_be_real_and_written_in_full() {
        for good in ["2026-11-09", "2024-02-29", "2026-12-31"] {
            assert!(valid_date(good), "{good}");
        }
        for bad in [
            "2026-02-29",
            "2026-13-01",
            "2026-11-31",
            "2026-1-09",
            "09-11-2026",
            "",
            "2026-11-09T00",
        ] {
            assert!(!valid_date(bad), "{bad}");
        }
    }

    #[test]
    fn a_step_without_a_record_is_fine_and_records_are_checked() {
        assert_eq!(messages(&pack(vec![step("a")], vec![])), "");

        let mut s = step("a");
        s.confirmed_by = vec![
            record(ConfirmationKind::VendorDocs, "2026-11-09"),
            record(ConfirmationKind::RegulatorDocs, "2026-11-09"),
        ];
        assert_eq!(messages(&pack(vec![s.clone()], vec![])), "");

        s.confirmed_by
            .push(record(ConfirmationKind::VendorDocs, "2026-11-09"));
        assert_flags(&pack(vec![s.clone()], vec![]), "recorded twice");

        s.confirmed_by = vec![record(ConfirmationKind::VendorDocs, "11/09/2026")];
        assert_flags(&pack(vec![s], vec![]), "must be a real date");
    }
}
