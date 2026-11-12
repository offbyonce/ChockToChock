// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Lists and their steps.

use checklist_core::{Checklist, FillField, Step};

use super::confirmations::check_confirmations;
use super::index::Index;
use super::{BLANK, Issue, error, valid_step_id};

const ROUTE_LEG_ONLY_IN_CDU: &str =
    "`route_leg` is only valid in CDU text, with `index` and `part`";

fn check_step(step: &Step, idx: &Index, issues: &mut Vec<Issue>) {
    let at = step.id.as_str();
    if !valid_step_id(idx.pack_id, at) {
        issues.push(error(
            at,
            format!("step id must be `{}.<phase>.<short-name>`", idx.pack_id),
        ));
    }
    if step.text.trim().is_empty() || step.expect.trim().is_empty() {
        issues.push(error(
            at,
            "step text and expected response must not be empty",
        ));
    }
    for control in step.controls.iter().chain(step.panel_lights.keys()) {
        idx.require_control(at, control, issues);
    }
    check_confirmations(at, &step.confirmed_by, issues);
    if step.simulator_model && step.note.is_none() {
        issues.push(error(at, "`simulator_model` needs a `note` to apply to"));
    }
    let blanks = step.expect.matches(BLANK).count();
    if !step.fill.is_empty() && step.fill.len() != blanks {
        issues.push(error(
            at,
            format!(
                "`fill` has {} entries but `expect` has {blanks} blanks",
                step.fill.len()
            ),
        ));
    }
    if step.hint.iter().any(|h| h.field == FillField::Manual) {
        issues.push(error(at, "`manual` is only valid in a step's `fill`"));
    }
    let hint_fields = step.hint.iter().map(|h| &h.field);
    if step
        .fill
        .iter()
        .chain(hint_fields)
        .any(|f| *f == FillField::RouteLeg)
    {
        issues.push(error(at, ROUTE_LEG_ONLY_IN_CDU));
    }
    if let Some(id) = &step.opens
        && !idx.walkthroughs.contains(id.as_str())
    {
        issues.push(error(at, format!("opens unknown walkthrough `{id}`")));
    }
    if let Some(id) = &step.how
        && !idx.how.contains(id.as_str())
    {
        issues.push(error(at, format!("refers to unknown how note `{id}`")));
    }
    if let Some(id) = &step.explain
        && !idx.explain.contains(id.as_str())
    {
        issues.push(error(at, format!("refers to unknown explanation `{id}`")));
    }
    if let Some(show_if) = &step.show_if {
        idx.check_show_if(at, show_if, issues);
    }
    if let Some(auto) = step.timer.as_ref().and_then(|t| t.auto_start.as_ref()) {
        for id in &auto.from {
            idx.require_step(at, id, issues);
        }
    }
}

pub(super) fn check_list(list: &Checklist, idx: &Index, issues: &mut Vec<Issue>) {
    if list.steps.is_empty() {
        issues.push(error(&list.id, "checklist has no steps"));
    }
    if list.basis.is_empty() {
        issues.push(error(&list.id, "list has no `basis`"));
    }
    for tag in &list.basis {
        if !idx.tags.contains(tag.as_str()) {
            issues.push(error(
                &list.id,
                format!("basis `{tag}` is not declared in pack.json"),
            ));
        }
    }
    for step in &list.steps {
        check_step(step, idx, issues);
    }
}

/// Words too common to tell two sentences apart.
const COMMON: &[&str] = &[
    "a", "an", "the", "and", "or", "of", "to", "in", "on", "at", "for", "with", "is", "are", "be",
    "it", "its", "this", "that", "as", "by", "from", "then", "each", "one", "if", "not", "no",
    "up", "down", "so", "than", "out", "into",
];

fn words(text: &str) -> std::collections::BTreeSet<String> {
    text.to_lowercase()
        .split(|c: char| !c.is_alphanumeric() && c != '/')
        .filter(|w| !w.is_empty() && !COMMON.contains(w))
        .map(str::to_owned)
        .collect()
}

/// A step's note must add something to its "How to do it" steps: a note whose words are mostly
/// already there says the same thing twice, so the pilot reads it twice.
pub(super) fn check_notes_repeat_how(pack: &crate::Pack, issues: &mut Vec<Issue>) {
    let Some(notes) = &pack.content.notes else {
        return;
    };
    for step in pack.checklists.iter().flat_map(|l| &l.steps) {
        let (Some(note), Some(how)) = (&step.note, &step.how) else {
            continue;
        };
        let Some(how) = notes.how.iter().find(|h| &h.id == how) else {
            continue;
        };
        let note_words = words(note);
        let how_words = words(&how.steps.join(" "));
        let shared = note_words.intersection(&how_words).count();
        if !note_words.is_empty() && shared * 10 >= note_words.len() * 7 {
            issues.push(error(
                step.id.as_str(),
                "the note repeats its \"How to do it\" steps; say it once",
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use checklist_core::{
        CduField, CduFile, CduPage, CduPart, CduScreen, CduWalkthrough, FillField, HintPart,
    };

    use super::super::test_support::{assert_flags, messages, pack, step};

    #[test]
    fn a_note_must_not_repeat_its_how_to_steps() {
        let mut s = step("ifly-737-max8.preflight.n1-bugs");
        s.how = Some("n1".into());
        s.note = Some(
            "Compare the takeoff thrust bugs with the green N1 reference on the engine display."
                .into(),
        );
        let mut p = pack(vec![s], vec![]);
        p.content.notes = Some(checklist_core::NotesFile {
            how: vec![checklist_core::HowNote {
                id: "n1".into(),
                steps: vec![
                    "Compare the N1 bugs with the green reference N1 readouts on the engine display.".into(),
                    "Check that they agree with the CDU value.".into(),
                ],
            }],
            explain: vec![],
        });
        assert_flags(&p, "the note repeats its \"How to do it\" steps");
        p.checklists[0].steps[0].note = Some("Set the bugs before the engines start.".into());
        assert!(!messages(&p).contains("repeats"));
    }

    #[test]
    fn lists_need_a_declared_basis() {
        let mut p = pack(vec![step("a")], vec![]);
        p.checklists[0].basis.clear();
        assert_flags(&p, "no `basis`");

        let mut p = pack(vec![step("a")], vec![]);
        p.checklists[0].basis = vec!["some-manual".into()];
        assert_flags(&p, "not declared");

        let mut p = pack(vec![step("a")], vec![]);
        p.manifest.basis_tags = vec!["Some Manual".into()];
        assert_flags(&p, "must be lower-case");
    }

    #[test]
    fn fill_must_match_the_blanks() {
        let mut s = step("a");
        s.expect = "V1 ___ VR ___".into();
        s.fill = vec![FillField::V1];
        assert_flags(
            &pack(vec![s.clone()], vec![]),
            "1 entries but `expect` has 2 blanks",
        );
        s.fill = vec![FillField::V1, FillField::Manual];
        assert_eq!(messages(&pack(vec![s], vec![])), "");
    }

    #[test]
    fn simulator_model_applies_to_a_note() {
        let mut s = step("a");
        s.simulator_model = true;
        assert_flags(&pack(vec![s.clone()], vec![]), "needs a `note`");
        s.note = Some("The guard closes by itself.".into());
        assert_eq!(messages(&pack(vec![s], vec![])), "");
    }

    #[test]
    fn manual_is_only_for_step_blanks() {
        let mut s = step("a");
        s.hint = vec![HintPart {
            prefix: "X ".into(),
            field: FillField::Manual,
        }];
        assert_flags(&pack(vec![s], vec![]), "only valid in a step's `fill`");

        let mut p = pack(vec![step("a")], vec![]);
        p.content.cdu = Some(CduFile {
            walkthroughs: vec![CduWalkthrough {
                id: "preflight".into(),
                title: "Preflight".into(),
                pages: vec![CduPage {
                    page: "IDENT".into(),
                    key: None,
                    title: "Check".into(),
                    action: vec![CduPart::Field(CduField {
                        field: FillField::Manual,
                        style: None,
                        before: None,
                        after: None,
                        empty: None,
                        empty_style: None,
                        index: None,
                        part: None,
                    })],
                    check: vec![],
                    note: vec![],
                    screen: CduScreen {
                        title: vec![],
                        page: vec![],
                        rows: vec![],
                        highlight: vec![],
                        scratchpad: vec![],
                    },
                }],
            }],
        });
        assert_flags(&p, "only valid in a step's `fill`");
    }

    #[test]
    fn step_references_must_resolve() {
        let mut s = step("a");
        s.controls = vec!["nope".into()];
        assert_flags(&pack(vec![s], vec![]), "unknown control");

        let mut s = step("a");
        s.opens = Some("nope".into());
        assert_flags(&pack(vec![s], vec![]), "unknown walkthrough");

        let mut s = step("a");
        s.how = Some("nope".into());
        assert_flags(&pack(vec![s], vec![]), "unknown how note");

        let mut s = step("a");
        s.explain = Some("nope".into());
        assert_flags(&pack(vec![s], vec![]), "unknown explanation");
    }
}
