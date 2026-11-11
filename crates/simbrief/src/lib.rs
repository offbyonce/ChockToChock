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

use serde::Serialize;
use serde_json::Value;
use ts_rs::TS;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub enum Units {
    Kg,
    Lb,
}

/// One navlog fix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[ts(export)]
pub struct Fix {
    pub ident: String,
    pub via: Option<String>,
    /// True when the fix is part of a SID or STAR rather than the en-route legs.
    pub sid_star: bool,
    pub stage: Option<String>,
}

/// The fields the checklist and CDU walkthrough use. Every field is optional because
/// SimBrief omits what the dispatcher did not plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Ofp {
    pub airline: Option<String>,
    pub flight_number: Option<String>,
    pub origin: Option<String>,
    pub origin_runway: Option<String>,
    pub origin_elevation_ft: Option<i32>,
    pub transition_altitude_ft: Option<i32>,
    pub destination: Option<String>,
    pub destination_elevation_ft: Option<i32>,
    pub transition_level_ft: Option<i32>,
    pub cruise_altitude_ft: Option<i32>,
    pub route: Option<String>,
    pub cost_index: Option<i32>,
    pub units: Option<Units>,
    pub ramp_fuel: Option<i32>,
    pub reserve_fuel: Option<i32>,
    pub zero_fuel_weight: Option<i32>,
    pub airac: Option<String>,
    pub registration: Option<String>,
    pub fixes: Vec<Fix>,
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
pub fn parse_ofp(body: &str) -> Result<Ofp, OfpError> {
    let body = body.trim_start_matches('\u{feff}').trim_start();
    if body.starts_with('<') {
        parse_ofp_xml(body)
    } else {
        parse_ofp_json(body)
    }
}

/// Parse the JSON text of a SimBrief fetch (`&json=1`).
pub fn parse_ofp_json(body: &str) -> Result<Ofp, OfpError> {
    let root: Value = serde_json::from_str(body).map_err(|e| OfpError::NotJson(e.to_string()))?;
    ofp_from_value(&root)
}

/// Parse a saved SimBrief XML plan. It is turned into the JSON shape, then read the same way.
pub fn parse_ofp_xml(body: &str) -> Result<Ofp, OfpError> {
    let doc = roxmltree::Document::parse(body).map_err(|e| OfpError::NotXml(e.to_string()))?;
    ofp_from_value(&xml_to_value(doc.root_element()))
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

fn ofp_from_value(root: &Value) -> Result<Ofp, OfpError> {
    if let Some(status) = text(root, &["fetch", "status"])
        && !status.eq_ignore_ascii_case("success")
    {
        return Err(OfpError::Fetch(status));
    }

    let units =
        text(root, &["params", "units"]).and_then(|u| match u.to_ascii_lowercase().as_str() {
            "kgs" | "kg" => Some(Units::Kg),
            "lbs" | "lb" => Some(Units::Lb),
            _ => None,
        });

    let fixes = match root.pointer("/navlog/fix") {
        Some(Value::Array(items)) => items.iter().filter_map(fix).collect(),
        Some(obj @ Value::Object(_)) => fix(obj).into_iter().collect(),
        _ => Vec::new(),
    };

    Ok(Ofp {
        airline: text(root, &["general", "icao_airline"]),
        flight_number: text(root, &["general", "flight_number"]),
        origin: text(root, &["origin", "icao_code"]),
        origin_runway: text(root, &["origin", "plan_rwy"]),
        origin_elevation_ft: int(root, &["origin", "elevation"]),
        transition_altitude_ft: int(root, &["origin", "trans_alt"]),
        destination: text(root, &["destination", "icao_code"]),
        destination_elevation_ft: int(root, &["destination", "elevation"]),
        transition_level_ft: int(root, &["destination", "trans_level"]),
        cruise_altitude_ft: int(root, &["general", "initial_altitude"]),
        route: text(root, &["general", "route"]),
        cost_index: int(root, &["general", "costindex"]),
        units,
        ramp_fuel: int(root, &["fuel", "plan_ramp"]),
        reserve_fuel: int(root, &["fuel", "reserve"]),
        zero_fuel_weight: int(root, &["weights", "est_zfw"]),
        airac: text(root, &["params", "airac"]),
        registration: text(root, &["aircraft", "reg"]),
        fixes,
    })
}

fn fix(v: &Value) -> Option<Fix> {
    Some(Fix {
        ident: text(v, &["ident"])?,
        via: text(v, &["via_airway"]),
        sid_star: text(v, &["is_sid_star"]).is_some_and(|s| s == "1"),
        stage: text(v, &["stage"]),
    })
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
        assert_eq!(ofp.registration, None, "`{{}}` is SimBrief's empty value");
        assert_eq!(ofp.origin_runway.as_deref(), Some("09R"));
    }

    #[test]
    fn single_fix_object_is_accepted() {
        let ofp = parse_ofp_json(
            r#"{"navlog":{"fix":{"ident":"ABC","via_airway":"DCT","is_sid_star":"0"}}}"#,
        )
        .unwrap();
        assert_eq!(ofp.fixes.len(), 1);
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
