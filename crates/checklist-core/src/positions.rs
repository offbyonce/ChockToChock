// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Which of a control's positions a step's value names: the step's target, for the control
//! drawings and for checking a pack's resting positions (decision 0014). Never the simulator's
//! state (decision 0006).

/// Whether `word` appears in `text` as a whole word or phrase (case-insensitive).
fn names(text: &str, word: &str) -> bool {
    let (text, word) = (text.to_uppercase(), word.to_uppercase());
    if word.is_empty() {
        return false;
    }
    let edge = |c: Option<char>| c.is_none_or(|c| !c.is_alphanumeric());
    text.match_indices(&word).any(|(at, _)| {
        edge(text[..at].chars().next_back()) && edge(text[at + word.len()..].chars().next())
    })
}

/// The position a value names, when it names exactly one of them. "ON, GUARD CLOSED" names ON
/// of "OFF, ON"; "ENG 1 AND 2 ON, ELEC 1 AND 2 OFF" names both, so neither is the target.
pub fn named_position(expect: &str, positions: &str) -> Option<usize> {
    let named: Vec<usize> = positions
        .split(',')
        .map(str::trim)
        .enumerate()
        .filter(|(_, p)| names(expect, p))
        .map(|(i, _)| i)
        .collect();
    match named[..] {
        [one] => Some(one),
        _ => None,
    }
}
