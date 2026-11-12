// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! SimBrief operational flight plan (OFP) parsing.
//!
//! SimBrief's JSON is converted from XML, so empty values arrive as `{}` and a
//! single list item can arrive as an object instead of an array. Everything is read
//! through small helpers that treat those cases as "missing" rather than failing.
//! A saved XML plan is turned into the same shape first, so both forms read alike.
//!
//! Fetching is deliberately not here: the app makes the request (with no cookies)
//! and hands the text to [`parse_ofp`].

use std::fmt::Write as _;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;

mod plan;

pub use plan::{Airport, Fix, Fuel, General, Plan, Times, Weights};

const FETCH_BASE: &str = "https://www.simbrief.com/api/xml.fetcher.php";

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum OfpError {
    #[error("SimBrief returned something that is not JSON: {0}")]
    NotJson(String),
    #[error("that file is not a valid SimBrief XML plan: {0}")]
    NotXml(String),
    #[error("SimBrief says: {0}")]
    Fetch(String),
    #[error("enter a SimBrief username or Pilot ID")]
    EmptyUser,
}

/// Weight units the plan uses. Fuel and weights are in these units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub enum Units {
    Kg,
    Lb,
}

/// The URL for the latest OFP. All digits means a Pilot ID; anything else is a username.
pub fn fetch_url(user: &str) -> Result<String, OfpError> {
    let user = user.trim();
    if user.is_empty() {
        return Err(OfpError::EmptyUser);
    }
    let key = if user.chars().all(|c| c.is_ascii_digit()) {
        "userid"
    } else {
        "username"
    };
    Ok(format!("{FETCH_BASE}?{key}={}&json=1", encode(user)))
}

/// Percent-encode everything except unreserved URL characters.
fn encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'~') {
            out.push(char::from(b));
        } else {
            let _ = write!(out, "%{b:02X}");
        }
    }
    out
}

/// Text at `path`, or `None` when it is missing, empty, or SimBrief's `{}` placeholder.
fn text(root: &Value, path: &[&str]) -> Option<String> {
    let mut v = root;
    for key in path {
        v = v.get(key)?;
    }
    match v {
        Value::String(s) if !s.trim().is_empty() => Some(s.trim().to_owned()),
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

/// A decimal number at `path`, if it is one.
fn num(root: &Value, path: &[&str]) -> Option<f64> {
    text(root, path)?
        .parse::<f64>()
        .ok()
        .filter(|v| v.is_finite())
}

/// The items at a place SimBrief may hold a list: an array, one object, or nothing.
fn list(v: Option<&Value>) -> impl Iterator<Item = &Value> {
    let items: &[Value] = match v {
        Some(Value::Array(items)) => items,
        Some(one @ Value::Object(_)) => std::slice::from_ref(one),
        _ => &[],
    };
    items.iter()
}

fn int(root: &Value, path: &[&str]) -> Option<i32> {
    text(root, path)?
        .parse::<f64>()
        .ok()
        .filter(|v| v.is_finite())
        .map(|v| {
            // SimBrief values are well inside i32; round rather than truncate.
            #[allow(clippy::cast_possible_truncation)]
            let rounded = v.round() as i32;
            rounded
        })
}

/// Parse a SimBrief plan in either form: XML when it starts with `<`, JSON otherwise.
pub fn parse_ofp(body: &str) -> Result<Plan, OfpError> {
    let body = body.trim_start_matches('\u{feff}').trim_start();
    if body.starts_with('<') {
        parse_ofp_xml(body)
    } else {
        parse_ofp_json(body)
    }
}

/// Parse the JSON text of a SimBrief fetch (`&json=1`).
pub fn parse_ofp_json(body: &str) -> Result<Plan, OfpError> {
    let root: Value = serde_json::from_str(body).map_err(|e| OfpError::NotJson(e.to_string()))?;
    plan::plan_from_value(&root)
}

/// Parse a saved SimBrief XML plan. It is turned into the JSON shape, then read the same way.
pub fn parse_ofp_xml(body: &str) -> Result<Plan, OfpError> {
    let doc = roxmltree::Document::parse(body).map_err(|e| OfpError::NotXml(e.to_string()))?;
    plan::plan_from_value(&xml_to_value(doc.root_element()))
}

/// An element as SimBrief's JSON would hold it: text when it has no child elements,
/// otherwise an object whose repeated child names become arrays.
fn xml_to_value(node: roxmltree::Node<'_, '_>) -> Value {
    let mut children = node
        .children()
        .filter(roxmltree::Node::is_element)
        .peekable();
    if children.peek().is_none() {
        let text: String = node.children().filter_map(|c| c.text()).collect();
        return Value::String(text.trim().to_owned());
    }
    let mut fields = serde_json::Map::new();
    for child in children {
        let value = xml_to_value(child);
        match fields.get_mut(child.tag_name().name()) {
            Some(Value::Array(items)) => items.push(value),
            Some(existing) => *existing = Value::Array(vec![existing.take(), value]),
            None => {
                fields.insert(child.tag_name().name().to_owned(), value);
            }
        }
    }
    Value::Object(fields)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> String {
        let path = format!("{}/tests/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
        std::fs::read_to_string(path).unwrap()
    }

    #[test]
    fn sample_plan_parses() {
        insta::assert_json_snapshot!(parse_ofp_json(&fixture("ofp-sample.json")).unwrap());
    }

    #[test]
    fn placeholders_become_missing() {
        let ofp = parse_ofp_json(&fixture("ofp-sample.json")).unwrap();
        assert_eq!(
            ofp.general.registration, None,
            "`{{}}` is SimBrief's empty value"
        );
        assert_eq!(ofp.origin.and_then(|o| o.runway).as_deref(), Some("09R"));
    }

    #[test]
    fn the_plan_never_reads_personal_fields() {
        // The fixture's pilot is "Pilot" with the ID 123456, in crew, fetch, params and api_params.
        let saved =
            serde_json::to_string(&parse_ofp(&fixture("ofp-sample.json")).unwrap()).unwrap();
        assert!(!saved.contains("123456"), "{saved}");
        assert!(!saved.contains("\"Pilot\""), "{saved}");
    }

    #[test]
    fn a_saved_plan_reads_back_the_same_and_leaves_out_what_is_missing() {
        let plan = parse_ofp(&fixture("ofp-sample.json")).unwrap();
        let saved = serde_json::to_string(&plan).unwrap();
        assert!(
            !saved.contains("null"),
            "missing values are left out: {saved}"
        );
        let back: Plan = serde_json::from_str(&saved).unwrap();
        assert_eq!(back, plan);
        assert_eq!(plan.alternates.len(), 1);
        assert_eq!(
            plan.alternates[0].metar.as_deref(),
            Some("LFPO 010900Z 25009KT CAVOK 16/06 Q1016"),
            "the alternate's weather comes from the weather section"
        );
        assert_eq!(
            plan.alternate_fixes.len(),
            1,
            "one fix arrives as an object"
        );
    }

    /// Reads a real plan the maintainer keeps outside the repository and reports the compact
    /// plan's size: `CHOCKTOCHOCK_PLAN=<file> cargo test -p simbrief -- --ignored --nocapture`.
    #[test]
    #[ignore = "needs a real SimBrief plan file outside the repository"]
    fn measures_a_real_plan() {
        let path = std::env::var("CHOCKTOCHOCK_PLAN").expect("CHOCKTOCHOCK_PLAN");
        let raw = std::fs::read_to_string(path).unwrap();
        let plan = parse_ofp(&raw).unwrap();
        let saved = serde_json::to_string(&plan).unwrap();
        let fixes = plan.fixes.len() + plan.alternate_fixes.len();
        let fix_bytes: usize = plan
            .fixes
            .iter()
            .chain(&plan.alternate_fixes)
            .map(|f| serde_json::to_string(f).unwrap().len() + 1)
            .sum();
        println!(
            "raw {} bytes; compact {} bytes ({} fixes, {} bytes each on average)",
            raw.len(),
            saved.len(),
            fixes,
            fix_bytes / fixes.max(1)
        );
        assert_eq!(serde_json::from_str::<Plan>(&saved).unwrap(), plan);
        if let Ok(out) = std::env::var("CHOCKTOCHOCK_PLAN_OUT") {
            std::fs::write(out, &saved).unwrap();
        }
    }

    #[test]
    fn single_fix_object_is_accepted() {
        let ofp = parse_ofp_json(
            r#"{"navlog":{"fix":{"ident":"ABC","via_airway":"DCT","is_sid_star":"0"}}}"#,
        )
        .unwrap();
        assert_eq!(ofp.fixes.len(), 1);
        assert_eq!(ofp.fixes[0].sid_star, Some(false));
    }

    #[test]
    fn simbrief_errors_are_reported() {
        let err = parse_ofp_json(&fixture("ofp-error.json")).unwrap_err();
        assert_eq!(err, OfpError::Fetch("Error: Unknown UserID".into()));
        assert!(matches!(
            parse_ofp_json("<html>"),
            Err(OfpError::NotJson(_))
        ));
    }

    #[test]
    fn fetch_url_picks_userid_or_username() {
        assert_eq!(
            fetch_url(" 123456 ").unwrap(),
            format!("{FETCH_BASE}?userid=123456&json=1")
        );
        assert_eq!(
            fetch_url("sim pilot&x").unwrap(),
            format!("{FETCH_BASE}?username=sim%20pilot%26x&json=1")
        );
        assert_eq!(fetch_url("  "), Err(OfpError::EmptyUser));
    }

    #[test]
    fn a_saved_xml_plan_reads_the_same_as_json() {
        let xml = parse_ofp_xml(&fixture("ofp-sample.xml")).unwrap();
        assert_eq!(xml, parse_ofp_json(&fixture("ofp-sample.json")).unwrap());
    }

    #[test]
    fn parse_ofp_picks_the_form_from_the_text() {
        let json = parse_ofp(&fixture("ofp-sample.json")).unwrap();
        assert_eq!(parse_ofp(&fixture("ofp-sample.xml")).unwrap(), json);
        assert_eq!(
            parse_ofp(&format!("\u{feff}\n{}", fixture("ofp-sample.json"))).unwrap(),
            json
        );
        assert_eq!(
            parse_ofp("<OFP><fetch><status>Error: Unknown UserID</status></fetch></OFP>"),
            Err(OfpError::Fetch("Error: Unknown UserID".into()))
        );
        assert!(matches!(
            parse_ofp("<OFP><general>"),
            Err(OfpError::NotXml(_))
        ));
        assert!(matches!(
            parse_ofp("Service unavailable"),
            Err(OfpError::NotJson(_))
        ));
    }

    proptest::proptest! {
        #[test]
        fn never_panics_on_arbitrary_json(input in ".{0,300}") {
            let _ = parse_ofp_json(&input);
        }

        #[test]
        fn never_panics_on_arbitrary_text(input in ".{0,300}") {
            let _ = parse_ofp(&input);
            let _ = parse_ofp(&format!("<{input}"));
        }
    }
}
