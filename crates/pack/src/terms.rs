// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! The glossary terms a step's wording uses, by 3.0's rule: a term counts where it stands as a
//! whole word (not inside a longer word, number or slashed abbreviation), longest terms first, so
//! "HDG SEL" wins over "SEL". A term written in capitals (an abbreviation or placard word) matches
//! only in capitals; a term with small letters ("center tank") matches in any case. The step's
//! wording includes where it is ("Forward overhead (P5) · Fuel panel"). Units and plain
//! abbreviations (glossary `step_term: false`) are never a step's terms. Used by `pack terms` to
//! keep each step's `terms` list current; the app shows the list, never a run-time match.

use checklist_core::Step;

use crate::Pack;

/// A character that joins onto a term, so the term is not standing alone there.
const fn joins(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '/'
}

/// The glossary terms found in `text`, each once, in glossary spelling, in the order they first
/// appear.
#[must_use]
pub fn terms_in(text: &str, glossary: &[&str]) -> Vec<String> {
    let mut sorted: Vec<&str> = glossary.to_vec();
    sorted.sort_by_key(|t| std::cmp::Reverse(t.len()));
    let lower = text.to_ascii_lowercase();
    let mut taken = vec![false; text.len()];
    let mut found: Vec<(usize, String)> = Vec::new();
    for term in sorted {
        if term.is_empty() {
            continue;
        }
        let any_case = term.chars().any(|c| c.is_ascii_lowercase());
        let (hay, needle) = if any_case {
            (lower.as_str(), term.to_ascii_lowercase())
        } else {
            (text, term.to_string())
        };
        let mut from = 0;
        while let Some(at) = hay[from..].find(needle.as_str()).map(|i| i + from) {
            let end = at + needle.len();
            let before = text[..at].chars().next_back();
            let after = text[end..].chars().next();
            let alone = !before.is_some_and(joins) && !after.is_some_and(joins);
            if alone && !taken[at..end].iter().any(|t| *t) {
                taken[at..end].iter_mut().for_each(|t| *t = true);
                match found.iter_mut().find(|(_, f)| f == term) {
                    Some(first) => first.0 = first.0.min(at),
                    None => found.push((at, term.to_string())),
                }
            }
            from = at + needle.len().max(1);
        }
    }
    found.sort_by_key(|(at, _)| *at);
    found.into_iter().map(|(_, t)| t).collect()
}

/// The glossary terms that may be a step's terms: all but units and plain abbreviations.
#[must_use]
pub fn step_glossary(pack: &Pack) -> Vec<&str> {
    pack.content
        .glossary
        .iter()
        .flat_map(|g| &g.terms)
        .filter(|t| t.step_term != Some(false))
        .map(|t| t.term.as_str())
        .collect()
}

/// Where the step is, as the app's location line shows it: the area and panel of its first
/// control that has a place ("Forward overhead (P5) · Fuel panel"), or nothing.
#[must_use]
pub fn step_location(pack: &Pack, step: &Step) -> String {
    let Some(controls) = &pack.content.controls else {
        return String::new();
    };
    step.controls
        .iter()
        .filter_map(|id| controls.controls.iter().find(|c| &c.id == id))
        .filter_map(|c| c.zone.as_ref())
        .find_map(|z| controls.zones.iter().find(|zone| &zone.id == z))
        .map(|zone| format!("{} · {}", zone.area, zone.title))
        .unwrap_or_default()
}

/// The terms a step's name and value use: what its `terms` list starts from (3.0 showed these).
#[must_use]
pub fn name_and_value_terms(step: &Step, glossary: &[&str]) -> Vec<String> {
    terms_in(&format!("{} {}", step.text, step.expect), glossary)
}

/// The terms the step's name, value, Check, If not, Why and location use, in that order: what
/// its `terms` list should hold.
#[must_use]
pub fn step_terms(step: &Step, glossary: &[&str], location: &str) -> Vec<String> {
    let wording = [
        Some(step.text.as_str()),
        Some(step.expect.as_str()),
        step.check.as_deref(),
        step.if_not.as_deref(),
        step.why.as_deref(),
        Some(location),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join(" | ");
    terms_in(&wording, glossary)
}

/// Terms the step's wording or location uses that its `terms` list lacks.
#[must_use]
pub fn missing_terms(step: &Step, glossary: &[&str], location: &str) -> Vec<String> {
    step_terms(step, glossary, location)
        .into_iter()
        .filter(|t| !step.terms.contains(t))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const GLOSSARY: &[&str] = &[
        "APU",
        "N1",
        "HDG SEL",
        "SEL",
        "A/T",
        "CDU",
        "center tank",
        "P5",
    ];

    #[test]
    fn terms_count_only_as_whole_words_longest_first() {
        assert_eq!(terms_in("APU GEN SWITCHES ON", GLOSSARY), vec!["APU"]);
        // Inside a longer word, number or slashed abbreviation, a term does not count.
        assert_eq!(terms_in("APUS N12 A/THR", GLOSSARY), Vec::<String>::new());
        // The longer term wins where they overlap; the shorter still counts elsewhere.
        assert_eq!(
            terms_in("HDG SEL, CDU SEL", GLOSSARY),
            vec!["HDG SEL", "CDU", "SEL"]
        );
        assert_eq!(terms_in("A/T ARM", GLOSSARY), vec!["A/T"]);
    }

    #[test]
    fn missing_terms_are_those_the_wording_uses_and_the_list_lacks() {
        let mut step = Step::new("demo.x.y", "APU", "ON");
        step.check = Some("N1 rising".into());
        step.terms = vec!["APU".into()];
        assert_eq!(missing_terms(&step, GLOSSARY, ""), vec!["N1"]);
        assert_eq!(name_and_value_terms(&step, GLOSSARY), vec!["APU"]);
    }

    #[test]
    fn word_terms_match_in_any_case_and_the_location_counts() {
        // The fuel pump step listed "LB, KG"; the board lists "center tank, P5".
        let step = Step::new(
            "demo.x.y",
            "FUEL PUMP SWITCHES",
            "ON IF FUEL IN THE CENTER TANK",
        );
        assert_eq!(
            step_terms(&step, GLOSSARY, "Forward overhead (P5) · Fuel panel"),
            vec!["center tank", "P5"]
        );
        // Capitals only match capitals: "apu" in prose is not the APU.
        assert_eq!(terms_in("an apu", GLOSSARY), Vec::<String>::new());
    }
}
