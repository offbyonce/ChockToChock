// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Which of a control's positions a step's value names: the step's target, for the control
//! drawings and for checking a pack's resting positions (decision 0014). Never the simulator's
//! state (decision 0006).

/// Whether `word` appears in `text` as a whole word or phrase (case-insensitive).
pub fn names(text: &str, word: &str) -> bool {
    let (text, word) = (text.to_uppercase(), word.to_uppercase());
    if word.is_empty() {
        return false;
    }
    let edge = |c: Option<char>| c.is_none_or(|c| !c.is_alphanumeric());
    // A number with a thousands separator is one number: "1,000" does not name "1" or "000".
    let in_number = |before: &str, after: &str| {
        let mut a = after.chars();
        let mut b = before.chars().rev();
        (word.ends_with(|c: char| c.is_ascii_digit())
            && a.next() == Some(',')
            && a.next().is_some_and(|c| c.is_ascii_digit()))
            || (word.starts_with(|c: char| c.is_ascii_digit())
                && b.next() == Some(',')
                && b.next().is_some_and(|c| c.is_ascii_digit()))
    };
    text.match_indices(&word).any(|(at, _)| {
        let (before, after) = (&text[..at], &text[at + word.len()..]);
        edge(before.chars().next_back()) && edge(after.chars().next()) && !in_number(before, after)
    })
}

/// Other words a step's wording uses for a position: "DOWN" for a lever's DN, "ARM" for ARMED.
const POSITION_WORDS: &[(&str, &[&str])] = &[
    ("DN", &["DOWN"]),
    ("ARMED", &["ARM"]),
    ("RELEASED", &["RELEASE"]),
    ("CUT OUT", &["CUTOUT"]),
    ("MAN", &["MANUAL"]),
];

/// Whether a value names a position, by its name or another word for it.
pub fn names_position(text: &str, position: &str) -> bool {
    names(text, position)
        || POSITION_WORDS
            .iter()
            .filter(|(p, _)| *p == position)
            .flat_map(|(_, words)| words.iter())
            .any(|w| names(text, w))
}

/// The position a value names, when it names exactly one of them. "ON, GUARD CLOSED" names ON
/// of "OFF, ON"; "ENG 1 AND 2 ON, ELEC 1 AND 2 OFF" names both, so neither is the target. A
/// position named inside a longer one also named ("STEADY" in "STROBE & STEADY") gives way to it.
pub fn named_position(expect: &str, positions: &str) -> Option<usize> {
    let all: Vec<&str> = positions.split(',').map(str::trim).collect();
    let named: Vec<usize> = (0..all.len())
        .filter(|&i| names_position(expect, all[i]))
        .collect();
    let longest: Vec<usize> = named
        .iter()
        .copied()
        .filter(|&i| {
            !named
                .iter()
                .any(|&j| j != i && all[j].len() > all[i].len() && names(all[j], all[i]))
        })
        .collect();
    match longest[..] {
        [one] => Some(one),
        _ => None,
    }
}

/// A value's clauses, split at commas (not inside numbers), semicolons and full stops.
pub fn clauses(expect: &str) -> Vec<&str> {
    let mut out = vec![];
    let mut start = 0;
    let b = expect.as_bytes();
    for (i, c) in expect.char_indices() {
        let digit_comma = c == ','
            && i > 0
            && b[i - 1].is_ascii_digit()
            && b.get(i + 1).is_some_and(u8::is_ascii_digit);
        if matches!(c, ',' | ';' | '.') && !digit_comma {
            out.push(expect[start..i].trim());
            start = i + 1;
        }
    }
    out.push(expect[start..].trim());
    out.retain(|c| !c.is_empty());
    out
}

/// Whether a value says a light is on, as 3.0 read it: ON, LIT, ILLUMINATED and the like, and
/// not OUT, OFF or DARK.
pub fn says_on(text: &str) -> bool {
    let on = ["ON", "ILLUMINATED", "LIT", "COMES ON"];
    let out = ["OUT", "OFF", "EXTINGUISHED", "DARK"];
    on.iter().any(|w| names(text, w)) && !out.iter().any(|w| names(text, w))
}

/// Whether a value says this light is on. A value about several lights ("VALVE LIGHTS ON,
/// FILTER BYPASS OUT") is read clause by clause: the clause naming the light, or else a word of
/// its name, or else the whole value. Also says which: `true` when a clause names the light.
pub fn says_light_on(expect: &str, label: &str) -> (bool, bool) {
    let clauses: Vec<&str> = clauses(expect);
    if let Some(c) = clauses.iter().find(|c| names(c, label)) {
        return (says_on(c), true);
    }
    let words: Vec<&str> = label.split_whitespace().filter(|w| w.len() > 3).collect();
    let clause = clauses.iter().find(|c| words.iter().any(|w| names(c, w)));
    (says_on(clause.copied().unwrap_or(expect)), false)
}

/// A value with the names of some lights taken out, so a switch's position is read from what is
/// left: "PASS OXY ON LIGHT OUT" does not name ON.
pub fn switch_value<'a>(
    expect: &str,
    light_labels: impl IntoIterator<Item = &'a str>,
    switch_positions: &[&str],
) -> String {
    let mut value = expect.to_owned();
    // A light named like a position (window heat ON) is not taken out: the word is the switch's.
    let position_word = |l: &str| {
        switch_positions.contains(&l)
            || POSITION_WORDS
                .iter()
                .any(|(p, words)| switch_positions.contains(p) && words.contains(&l))
    };
    for label in light_labels.into_iter().filter(|l| !position_word(l)) {
        value = value.replace(label, " ");
    }
    value
}

/// A panel item's locator, `panel/section/LABEL/n`: the n-th item (from 1) with that label in
/// the section.
pub fn locator(panel: &crate::Panel, section: &crate::PanelSection, item: usize) -> String {
    let label = section.items.get(item).map_or("", |i| i.label.as_str());
    let n = section.items[..item.min(section.items.len())]
        .iter()
        .filter(|i| i.label == label)
        .count()
        + 1;
    format!("{}/{}/{label}/{n}", panel.id, section.id)
}

/// The section and item index a locator names, if it names one.
pub fn locate<'a>(
    panels: &'a [crate::Panel],
    at: &str,
) -> Option<(&'a crate::PanelSection, usize)> {
    let mut front = at.splitn(3, '/');
    let (panel, section, rest) = (front.next()?, front.next()?, front.next()?);
    let (label, n) = rest.rsplit_once('/')?;
    let n: usize = n.parse().ok()?;
    let section = panels
        .iter()
        .find(|p| p.id == panel)?
        .sections
        .iter()
        .find(|s| s.id == section)?;
    let item = section
        .items
        .iter()
        .enumerate()
        .filter(|(_, i)| i.label == label)
        .nth(n.checked_sub(1)?)?
        .0;
    Some((section, item))
}

#[cfg(test)]
mod reading_tests {
    use super::{clauses, named_position, names, switch_value};

    #[test]
    fn a_number_with_a_thousands_separator_is_one_number() {
        // "1,000 FT" named flap position 1.
        assert!(!names("BE STABILIZED BY 1,000 FT", "1"));
        assert!(names("FLAPS 1, THEN 5", "1"));
    }

    #[test]
    fn clauses_split_at_full_stops_but_not_inside_numbers() {
        assert_eq!(
            clauses("STAY IN MANUAL. MASKS CAN COME OFF ABOVE 10,000 FT"),
            ["STAY IN MANUAL", "MASKS CAN COME OFF ABOVE 10,000 FT"]
        );
    }

    #[test]
    fn positions_are_read_by_other_words_and_the_longer_of_nested_names() {
        assert_eq!(named_position("DOWN WHEN SPEED ALLOWS", "UP, DN"), Some(1));
        assert_eq!(
            named_position("STROBE & STEADY", "STROBE & STEADY, OFF, STEADY"),
            Some(0)
        );
    }

    #[test]
    fn a_light_named_like_a_position_does_not_hide_the_switch_position() {
        // The window heat lights are named ON; the MANUAL light is a word for MAN.
        assert!(names(&switch_value("ON", ["ON"], &["OFF", "ON"]), "ON"));
        assert!(names(
            &switch_value("STAY IN MANUAL", ["MANUAL"], &["AUTO", "MAN"]),
            "MANUAL"
        ));
        assert!(!names(
            &switch_value("PASS OXY ON LIGHT OUT", ["PASS OXY ON"], &["NORMAL", "ON"]),
            "ON"
        ));
    }
}
