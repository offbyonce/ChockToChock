// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Links: banned domains and paths, the first-party allow-list, and the scan of every text
//! field.

use serde::Serialize;
use serde_json::Value;

use super::{Issue, error};
use crate::Pack;

/// Domains that must never be linked (non-public or redistributed manuals, and second-hand
/// study material). Matching covers subdomains too (`www.`, `en.` and so on).
pub const BLOCKED_DOMAINS: &[&str] = &[
    "brainscape.com",
    "scribd.com",
    "studylib.net",
    "manualshive.com",
    "allafricanairways.com",
    "slideshare.net",
    "archive.org",
    "aviationhunt.com",
];

/// `(domain, lower-case path prefix)` pairs that must never be linked.
pub const BLOCKED_PATHS: &[(&str, &str)] = &[("data.ntsb.gov", "/docket")];

/// First-party links the maintainer has approved, as `host` or `host/path-prefix`. Any other
/// link in a pack is an error, and a blocked link can never be listed here.
pub const ALLOWED_LINKS: &[&str] = &[];

/// The lower-case host and the rest of a URL (path, query and fragment, original case).
fn split_url(url: &str) -> Option<(String, &str)> {
    let rest = url.split_once("://").map_or(url, |(_, r)| r);
    let rest = rest.strip_prefix("//").unwrap_or(rest);
    let end = rest.find(['/', '?', '#']).unwrap_or(rest.len());
    let (authority, path) = rest.split_at(end);
    let host = authority.rsplit_once('@').map_or(authority, |(_, h)| h);
    let host = host.split(':').next()?;
    (!host.is_empty()).then(|| (host.to_ascii_lowercase(), path))
}

fn host_is(host: &str, domain: &str) -> bool {
    host == domain
        || host
            .strip_suffix(domain)
            .is_some_and(|sub| sub.ends_with('.'))
}

/// True when `path` is `prefix` or continues it at a path boundary. Ignores case.
fn path_has_prefix(path: &str, prefix: &str) -> bool {
    let path = path.to_ascii_lowercase();
    path.strip_prefix(&prefix.to_ascii_lowercase())
        .is_some_and(|rest| rest.is_empty() || rest.starts_with(['/', '?', '#']))
}

/// True when `url` points at a banned domain or path, including subdomains.
pub fn is_blocked_link(url: &str) -> bool {
    split_url(url).is_some_and(|(host, path)| {
        BLOCKED_DOMAINS.iter().any(|d| host_is(&host, d))
            || BLOCKED_PATHS
                .iter()
                .any(|(d, prefix)| host_is(&host, d) && path_has_prefix(path, prefix))
    })
}

fn is_allowed_link(url: &str, allow: &[&str]) -> bool {
    split_url(url).is_some_and(|(host, path)| {
        allow.iter().any(|entry| match entry.split_once('/') {
            Some((h, p)) => host == *h && path_has_prefix(path, &format!("/{p}")),
            None => host == *entry,
        })
    })
}

/// Everything in `text` that is a link: anything with a scheme, anything starting `www.` or
/// `//`, and bare mentions of a banned domain.
pub fn find_links(text: &str) -> Vec<&str> {
    text.split(|c: char| c.is_whitespace() || "()[]<>\"'`".contains(c))
        .map(|t| t.trim_end_matches(['.', ',', ';', ':', '!', '?']))
        .filter(|t| {
            !t.is_empty()
                && (t.contains("://")
                    || t.to_ascii_lowercase().starts_with("www.")
                    || t.starts_with("//")
                    || is_blocked_link(t))
        })
        .collect()
}

/// Check one link: banned links are always errors; others need to be on the allow-list.
fn link_issue(at: &str, link: &str, allow: &[&str]) -> Option<Issue> {
    if is_blocked_link(link) {
        Some(error(at, format!("link `{link}` is on the blocked list")))
    } else if is_allowed_link(link, allow) {
        None
    } else {
        Some(error(
            at,
            format!("link `{link}` is not allowed; packs carry no third-party URLs"),
        ))
    }
}

fn scan_value(at: &str, value: &Value, allow: &[&str], issues: &mut Vec<Issue>) {
    match value {
        Value::String(text) => {
            issues.extend(
                find_links(text)
                    .into_iter()
                    .filter_map(|l| link_issue(at, l, allow)),
            );
        }
        Value::Array(items) => {
            for (i, item) in items.iter().enumerate() {
                scan_value(&format!("{at}/{i}"), item, allow, issues);
            }
        }
        Value::Object(fields) => {
            for (key, item) in fields {
                scan_value(&format!("{at}/{key}"), item, allow, issues);
            }
        }
        _ => {}
    }
}

fn scan_links(at: &str, data: &impl Serialize, allow: &[&str], issues: &mut Vec<Issue>) {
    match serde_json::to_value(data) {
        Ok(value) => scan_value(at, &value, allow, issues),
        Err(e) => issues.push(error(at, format!("cannot scan for links: {e}"))),
    }
}

pub(super) fn check_links(pack: &Pack, allow: &[&str], issues: &mut Vec<Issue>) {
    scan_links("pack.json", &pack.manifest, allow, issues);
    for list in &pack.checklists {
        scan_links(&format!("list {}", list.id), list, allow, issues);
    }
    scan_links("content", &pack.content, allow, issues);
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::super::test_support::{assert_flags, pack, step};
    use super::super::validate_with_allow_list;
    use super::*;

    #[test]
    fn blocked_domains_match_subdomains_only() {
        assert!(is_blocked_link("https://www.brainscape.com/flashcards/x"));
        assert!(is_blocked_link("http://ARCHIVE.org/details/fcom"));
        assert!(is_blocked_link("https://aviationhunt.com/b737"));
        assert!(!is_blocked_link("https://notbrainscape.com/x"));
        assert!(!is_blocked_link("https://example.com/brainscape.com"));
    }

    #[test]
    fn docket_paths_are_blocked_by_prefix_ignoring_case() {
        assert!(is_blocked_link(
            "https://data.ntsb.gov/Docket/Document/docBLOB?ID=1"
        ));
        assert!(is_blocked_link("https://data.ntsb.gov/docket"));
        assert!(is_blocked_link("data.ntsb.gov/DOCKET?x=1"));
        assert!(!is_blocked_link("https://data.ntsb.gov/carol-main/"));
        assert!(!is_blocked_link("https://data.ntsb.gov/docketing"));
        assert!(!is_blocked_link("https://www.ntsb.gov/Docket"));
    }

    #[test]
    fn links_are_found_in_prose() {
        assert_eq!(
            find_links(
                "See (https://a.example/x), www.b.example. Or //c.example and scribd.com/doc."
            ),
            vec![
                "https://a.example/x",
                "www.b.example",
                "//c.example",
                "scribd.com/doc"
            ]
        );
        assert_eq!(
            find_links("Set FL350/M.78, V1 142. N1 85.5 and A/T ON"),
            Vec::<&str>::new()
        );
    }

    #[test]
    fn links_are_found_in_any_text_field() {
        let mut banned_note = step("a");
        banned_note.note = Some("From https://www.scribd.com/doc/1".into());
        assert_flags(&pack(vec![banned_note], vec![]), "blocked list");

        let mut other_link = step("a");
        other_link.check = Some("Details at https://example.com/page".into());
        assert_flags(&pack(vec![other_link], vec![]), "not allowed");

        let mut p = pack(vec![step("a")], vec![]);
        p.manifest.attribution = vec!["See brainscape.com/x".into()];
        assert_flags(&p, "pack.json/attribution/0");

        let mut p = pack(vec![step("a")], vec![]);
        if let Some(file) = &mut p.content.phases {
            file.stages[0].title = "See https://example.com".into();
        }
        assert_flags(&p, "content/phases/stages/0/title");
    }

    #[test]
    fn the_allow_list_starts_empty_and_cannot_override_a_ban() {
        assert_eq!(ALLOWED_LINKS, Vec::<&str>::new());
        let mut s = step("a");
        s.note = Some("https://first.example/docs/x and https://first.example/other".into());
        let p = pack(vec![s], vec![]);
        let issues = validate_with_allow_list(&p, &["first.example/docs"]);
        assert_eq!(issues.len(), 1, "{issues:?}");
        assert!(issues[0].message.contains("first.example/other"));

        let mut s = step("a");
        s.note = Some("https://scribd.com/x".into());
        let issues = validate_with_allow_list(&pack(vec![s], vec![]), &["scribd.com"]);
        assert!(issues[0].message.contains("blocked list"), "{issues:?}");
    }

    proptest! {
        /// Link scanning returns a result, never a panic, on arbitrary text.
        #[test]
        fn find_links_never_panics(input in ".{0,300}") {
            let _ = find_links(&input);
        }

        /// A banned link is caught whatever subdomain or path it uses, and however it is
        /// embedded in a sentence.
        #[test]
        fn banned_links_are_always_found(
            sub in "[a-z]{1,8}",
            path in "[a-zA-Z0-9/_-]{0,20}",
            idx in 0..BLOCKED_DOMAINS.len(),
        ) {
            let url = format!("https://{sub}.{}/{path}", BLOCKED_DOMAINS[idx]);
            let text = format!("See ({url}), then continue.");
            let links = find_links(&text);
            prop_assert_eq!(links.len(), 1);
            prop_assert!(is_blocked_link(links[0]));
        }
    }
}
