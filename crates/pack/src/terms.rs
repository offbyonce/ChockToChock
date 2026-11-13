// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! The glossary terms a step's wording uses, by 3.0's rule: a term counts where it stands as a
//! whole word (not inside a longer word, number or slashed abbreviation), longest terms first, so
//! "HDG SEL" wins over "SEL". Used by `pack terms` to keep each step's `terms` list current; the
//! app shows the list, never a run-time match.

use checklist_core::Step;

/// A character that joins onto a term, so the term is not standing alone there.
const fn joins(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '/'
}

/// The glossary terms found in `text`, longest first and each once, in glossary spelling.
#[must_use]
pub fn terms_in(text: &str, glossary: &[&str]) -> Vec<String> {
    let mut sorted: Vec<&str> = glossary.to_vec();
    sorted.sort_by_key(|t| std::cmp::Reverse(t.len()));
    let mut taken = vec![false; text.len()];
    let mut found = Vec::new();
    for term in sorted {
        if term.is_empty() {
            continue;
        }
        let mut from = 0;
        while let Some(at) = text[from..].find(term).map(|i| i + from) {
            let end = at + term.len();
            let before = text[..at].chars().next_back();
            let after = text[end..].chars().next();
            let alone = !before.is_some_and(joins) && !after.is_some_and(joins);
            if alone && !taken[at..end].iter().any(|t| *t) {
                taken[at..end].iter_mut().for_each(|t| *t = true);
                if !found.iter().any(|f: &String| f == term) {
                    found.push(term.to_string());
                }
            }
            from = at + term.len().max(1);
        }
    }
    found
}

/// The terms a step's name and value use: what its `terms` list starts from (3.0 showed these).
#[must_use]
pub fn name_and_value_terms(step: &Step, glossary: &[&str]) -> Vec<String> {
    terms_in(&format!("{} {}", step.text, step.expect), glossary)
}

/// Terms the step's name, value, Check, If not or Why use that its `terms` list lacks.
#[must_use]
pub fn missing_terms(step: &Step, glossary: &[&str]) -> Vec<String> {
    let wording = [
        Some(step.text.as_str()),
        Some(step.expect.as_str()),
        step.check.as_deref(),
        step.if_not.as_deref(),
        step.why.as_deref(),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join(" | ");
    terms_in(&wording, glossary)
        .into_iter()
        .filter(|t| !step.terms.contains(t))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const GLOSSARY: &[&str] = &["APU", "N1", "HDG SEL", "SEL", "A/T", "CDU"];

    #[test]
    fn terms_count_only_as_whole_words_longest_first() {
        assert_eq!(terms_in("APU GEN SWITCHES ON", GLOSSARY), vec!["APU"]);
        // Inside a longer word, number or slashed abbreviation, a term does not count.
        assert_eq!(terms_in("APUS N12 A/THR", GLOSSARY), Vec::<String>::new());
        // The longer term wins where they overlap; the shorter still counts elsewhere.
        assert_eq!(
            terms_in("HDG SEL, CDU SEL", GLOSSARY),
            vec!["HDG SEL", "SEL", "CDU"]
        );
        assert_eq!(terms_in("A/T ARM", GLOSSARY), vec!["A/T"]);
    }

    #[test]
    fn missing_terms_are_those_the_wording_uses_and_the_list_lacks() {
        let mut step = Step::new("demo.x.y", "APU", "ON");
        step.check = Some("N1 rising".into());
        step.terms = vec!["APU".into()];
        assert_eq!(missing_terms(&step, GLOSSARY), vec!["N1"]);
        assert_eq!(name_and_value_terms(&step, GLOSSARY), vec!["APU"]);
    }
}
