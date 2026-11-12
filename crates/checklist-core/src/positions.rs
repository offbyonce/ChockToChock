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
    let clauses: Vec<&str> = expect.split([',', ';']).map(str::trim).collect();
    if let Some(c) = clauses.iter().find(|c| names(c, label)) {
        return (says_on(c), true);
    }
    let words: Vec<&str> = label.split_whitespace().filter(|w| w.len() > 3).collect();
    let clause = clauses.iter().find(|c| words.iter().any(|w| names(c, w)));
    (says_on(clause.copied().unwrap_or(expect)), false)
}

/// A value with the names of some lights taken out, so a switch's position is read from what is
/// left: "PASS OXY ON LIGHT OUT" does not name ON.
pub fn switch_value<'a>(expect: &str, light_labels: impl IntoIterator<Item = &'a str>) -> String {
    let mut value = expect.to_owned();
    for label in light_labels {
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
