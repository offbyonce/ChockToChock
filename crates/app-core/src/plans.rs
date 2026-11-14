// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! SimBrief plans. The host fetches the text from the address given here, or reads a saved
//! file, and hands it back to be read into the compact plan (decision 0019), whose values then go
//! into the flight as 3.0 put them there.

use std::collections::BTreeMap;

use checklist_core::FillField;
use serde::{Deserialize, Serialize};
use simbrief::{Plan, Units};
use ts_rs::TS;

use crate::AppError;

/// The saved format of a [`SavedPlan`]. Raise it, with a migration and tests, when the shape changes.
pub const PLAN_FORMAT: u32 = 1;

/// The last plan, kept on the device for offline use and its age (decision 0019).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export)]
pub struct SavedPlan {
    pub format: u32,
    pub plan: Plan,
    /// When the app loaded it, in milliseconds since 1970 (UTC).
    #[ts(type = "number")]
    pub loaded_at: i64,
}

/// The flight values a plan fills in, and how many of the pilot's fields that is.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct PlanValues {
    pub values: BTreeMap<FillField, String>,
    /// The fields the pilot sees filled (3.0 counted these in its notice).
    pub filled: usize,
    pub origin: String,
    pub destination: String,
    /// The plan sets a runway or departure, which the pilot checks against the clearance.
    pub check_clearance: bool,
}

/// The address of the latest plan for a SimBrief username or Pilot ID.
pub fn plan_url(user: &str) -> Result<String, AppError> {
    Ok(simbrief::fetch_url(user)?)
}

/// Read a plan fetched from SimBrief, or a saved plan file in JSON or XML.
pub fn parse_plan(text: &str) -> Result<Plan, AppError> {
    Ok(simbrief::parse_ofp(text)?)
}

/// Thousands, rounded up to the next tenth (what the CDU takes for ZFW and reserves).
fn thousands_up(v: i32) -> String {
    format!("{:.1}", (f64::from(v) / 100.0 - 1e-9).ceil() / 10.0)
}

/// Thousands, rounded to the nearest tenth.
fn thousands(v: i32) -> String {
    format!("{:.1}", (f64::from(v) / 100.0).round() / 10.0)
}

fn icao(code: Option<&String>) -> Option<String> {
    let c = code?.to_uppercase();
    (c.len() == 4 && c.chars().all(|ch| ch.is_ascii_alphanumeric())).then_some(c)
}

/// A runway as the pilot types it: "9R" and "RW09R" are "09R".
fn runway(r: &str) -> Option<String> {
    let r = r.to_uppercase();
    let r = r.strip_prefix("RW").unwrap_or(&r);
    let (num, side) = r.split_at(r.find(|c: char| !c.is_ascii_digit()).unwrap_or(r.len()));
    let n: u8 = num.parse().ok()?;
    ((1..=36).contains(&n) && num.len() <= 2 && matches!(side, "" | "L" | "C" | "R"))
        .then(|| format!("{n:02}{side}"))
}

/// The departure and its transition: the plan's own SID fields, else 3.0's reading of the
/// navlog (the first SID fix in the first six, and the route's next word or the SID's last fix).
fn departure(plan: &Plan) -> (Option<String>, Option<String>) {
    let word = |w: &str, max: usize| {
        let ok = (2..=max).contains(&w.len()) && w.chars().all(|c| c.is_ascii_alphanumeric());
        ok.then(|| w.to_owned())
    };
    if let Some(sid) = plan
        .general
        .sid
        .as_deref()
        .and_then(|s| word(&s.to_uppercase(), 8))
    {
        let trans = plan
            .general
            .sid_transition
            .as_deref()
            .and_then(|t| word(&t.to_uppercase(), 5));
        return (Some(sid), trans);
    }
    let fixes = &plan.fixes;
    let Some(i) = fixes
        .iter()
        .position(|f| f.sid_star == Some(true) && f.stage.as_deref() != Some("DSC"))
        .filter(|i| *i < 6)
    else {
        return (None, None);
    };
    let Some(sid) = fixes[i]
        .via
        .as_deref()
        .map(str::to_uppercase)
        .filter(|v| v != "DCT")
        .and_then(|v| word(&v, 8))
    else {
        return (None, None);
    };
    let mut k = i;
    while fixes.get(k + 1).is_some_and(|f| {
        f.sid_star == Some(true)
            && f.via.as_deref().map(str::to_uppercase).as_deref() == Some(sid.as_str())
    }) {
        k += 1;
    }
    let route = plan
        .general
        .route
        .clone()
        .unwrap_or_default()
        .to_uppercase();
    let mut words = route.split_whitespace();
    let letters = |w: &str| (2..=5).contains(&w.len()) && w.chars().all(|c| c.is_ascii_uppercase());
    let trans = match (words.next(), words.next()) {
        (Some(first), Some(next)) if first == sid && letters(next) => Some(next.to_owned()),
        _ => fixes[k]
            .ident
            .as_deref()
            .map(str::to_uppercase)
            .filter(|f| letters(f)),
    };
    (Some(sid), trans)
}

fn put(v: &mut BTreeMap<FillField, String>, f: FillField, s: String) {
    v.insert(f, s);
}

/// Fuel, weights, performance and the route, read as 3.0 read them.
fn performance(plan: &Plan, v: &mut BTreeMap<FillField, String>) {
    let g = &plan.general;
    if let Some(f) = plan.fuel.plan_ramp.filter(|f| *f > 0) {
        put(v, FillField::Fuel, f.to_string());
        if let Some(u) = plan.units {
            put(
                v,
                FillField::WeightUnit,
                if u == Units::Kg { "KG" } else { "LB" }.into(),
            );
        }
    }
    if let Some(e) = plan
        .destination
        .as_ref()
        .and_then(|d| d.elevation_ft)
        .filter(|e| (-1000..=14000).contains(e))
    {
        put(v, FillField::LandingAltitude, e.to_string());
    }
    if let Some(z) = plan.weights.zfw.filter(|z| *z > 0) {
        put(v, FillField::Zfw, thousands_up(z));
    }
    if let Some(r) = plan.fuel.reserve.filter(|r| *r > 0) {
        put(
            v,
            FillField::Reserves,
            thousands_up(r + plan.fuel.alternate.filter(|a| *a > 0).unwrap_or(0)),
        );
    }
    if let Some(m) = plan.fuel.min_takeoff.filter(|m| *m > 0) {
        put(v, FillField::MinimumTakeoffFuel, m.to_string());
    }
    if let Some(ci) = g.cost_index.filter(|c| (0..=9999).contains(c)) {
        put(v, FillField::CostIndex, ci.to_string());
    }
    if let (Some(d), Some(s)) = (g.avg_wind_dir, g.avg_wind_speed)
        && (0..=360).contains(&d)
        && (0..400).contains(&s)
    {
        let d = if d % 360 == 0 { 360 } else { d % 360 };
        put(v, FillField::CruiseWind, format!("{d:03}/{s:03}"));
    }
    if let Some(i) = g.avg_isa_deviation.filter(|i| i.abs() <= 60) {
        put(v, FillField::IsaDeviation, i.to_string());
    }
    if let Some(t) = plan
        .origin
        .as_ref()
        .and_then(|o| o.transition_altitude_ft)
        .filter(|t| (1000..=20000).contains(t))
    {
        put(v, FillField::TransitionAltitude, t.to_string());
    }
    if let Some(t) = plan
        .destination
        .as_ref()
        .and_then(|d| d.transition_level_ft)
        .filter(|t| (1000..=60000).contains(t))
    {
        put(v, FillField::TransitionLevel, ((t + 50) / 100).to_string());
    }
    if let Some(w) = plan.weights.takeoff.filter(|w| *w > 0) {
        put(v, FillField::TakeoffWeight, thousands(w));
    }
    if let Some(p) = plan.weights.passengers.filter(|p| (0..=500).contains(p)) {
        put(v, FillField::Passengers, p.to_string());
    }
    if let Some(c) = plan.weights.cargo.filter(|c| *c >= 0) {
        put(v, FillField::Cargo, thousands(c));
    }
    if let Some(r) = g.route.as_deref() {
        let r = r
            .to_uppercase()
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ");
        if !r.is_empty() && r.len() <= 400 {
            put(v, FillField::Route, r);
        }
    }
    if let Some(o) = plan.origin.as_ref()
        && let (Some(lat), Some(lon)) = (o.lat, o.lon)
        && lat.abs() <= 90.0
        && lon.abs() <= 180.0
    {
        put(v, FillField::OriginPosition, format!("{lat} {lon}"));
    }
    if let Some(a) = g
        .airac
        .as_deref()
        .filter(|a| a.len() == 4 && a.chars().all(|c| c.is_ascii_digit()))
    {
        put(v, FillField::NavDatabase, a.to_owned());
    }
}

/// The alternate, the arrival and each airport's altimeter setting (batch 4, owner 2026-11-13).
/// The destination's setting takes the departure's unit, converted when its METAR uses the other.
fn arrival(plan: &Plan, v: &mut BTreeMap<FillField, String>) {
    if let Some(a) = icao(plan.alternates.first().and_then(|a| a.icao.as_ref())) {
        put(v, FillField::Alternate, a);
    }
    if let Some(r) = plan
        .destination
        .as_ref()
        .and_then(|d| d.runway.as_deref())
        .and_then(runway)
    {
        put(v, FillField::ArrivalRunway, r);
    }
    if let Some(star) = plan
        .general
        .star
        .as_deref()
        .map(str::to_uppercase)
        .filter(|s| (2..=8).contains(&s.len()) && s.chars().all(|c| c.is_ascii_alphanumeric()))
    {
        put(v, FillField::Star, star);
    }
    let metar = |a: Option<&simbrief::Airport>| {
        a.and_then(|a| a.metar.as_deref())
            .and_then(crate::weather::altimeter)
    };
    let dep = metar(plan.origin.as_ref());
    let dest = metar(plan.destination.as_ref());
    let unit = dep.as_ref().or(dest.as_ref()).map(|a| a.unit.clone());
    if let Some(u) = &unit {
        put(v, FillField::QnhUnit, u.clone());
    }
    if let Some(a) = dep {
        put(v, FillField::Qnh, a.value);
    }
    if let (Some(a), Some(u)) = (dest, unit) {
        put(v, FillField::DestinationQnh, in_unit(&a, &u));
    }
}

/// An altimeter setting in the flight's unit: 1 inHg is 33.8639 hPa.
fn in_unit(a: &crate::weather::Altimeter, unit: &str) -> String {
    match (a.unit.as_str(), unit) {
        ("inHg", "hPa") => a
            .value
            .parse::<f64>()
            .map_or_else(|_| a.value.clone(), |v| format!("{:.0}", v * 33.8639)),
        ("hPa", "inHg") => a
            .value
            .parse::<f64>()
            .map_or_else(|_| a.value.clone(), |v| format!("{:.2}", v / 33.8639)),
        _ => a.value.clone(),
    }
}

/// The flight values a plan fills in, read as 3.0 read them. A plan without both airports is
/// refused, as 3.0 refused it.
pub fn plan_values(plan: &Plan) -> Result<PlanValues, AppError> {
    let (Some(origin), Some(destination)) = (
        icao(plan.origin.as_ref().and_then(|a| a.icao.as_ref())),
        icao(plan.destination.as_ref().and_then(|a| a.icao.as_ref())),
    ) else {
        return Err(AppError::FlightPlan(
            "that plan has no departure and destination airports".into(),
        ));
    };
    let g = &plan.general;
    let mut v = BTreeMap::new();
    let v = &mut v;
    put(v, FillField::Origin, origin.clone());
    put(v, FillField::Destination, destination.clone());
    if let Some(n) = g.flight_number.as_deref() {
        let n: String = n.to_uppercase().split_whitespace().collect();
        let full = format!(
            "{}{n}",
            g.airline.as_deref().unwrap_or_default().to_uppercase()
        );
        put(
            v,
            FillField::FlightNumber,
            if full.len() <= 7 {
                full
            } else {
                n.chars().take(7).collect()
            },
        );
    }
    if let Some(a) = g.cruise_altitude_ft {
        let fl = if a >= 1000 { (a + 50) / 100 } else { a };
        if (100..=410).contains(&fl) {
            put(v, FillField::CruiseLevel, fl.to_string());
        }
    }
    if let Some(r) = plan
        .origin
        .as_ref()
        .and_then(|o| o.runway.as_deref())
        .and_then(runway)
    {
        put(v, FillField::Runway, r);
    }
    let (sid, trans) = departure(plan);
    if let Some(s) = sid {
        put(v, FillField::Sid, s);
    }
    if let Some(t) = trans {
        put(v, FillField::Transition, t);
    }
    performance(plan, v);
    arrival(plan, v);
    // The pilot's fields: not the position, the database or the weight unit, as 3.0 counted.
    let filled = v
        .keys()
        .filter(|f| {
            !matches!(
                f,
                FillField::OriginPosition | FillField::NavDatabase | FillField::WeightUnit
            )
        })
        .count();
    let check_clearance = v.contains_key(&FillField::Runway) || v.contains_key(&FillField::Sid);
    Ok(PlanValues {
        values: std::mem::take(v),
        filled,
        origin,
        destination,
        check_clearance,
    })
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    fn fixture(name: &str) -> String {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../simbrief/tests/fixtures");
        std::fs::read_to_string(dir.join(name)).unwrap()
    }

    #[test]
    fn a_plan_fills_flight_values_as_3_0_did() {
        let plan = parse_plan(&fixture("ofp-sample.json")).unwrap();
        let pv = plan_values(&plan).unwrap();
        let get = |f: FillField| pv.values.get(&f).map(String::as_str);
        // TSTX and 4567 make eight characters, over 3.0's seven: the number alone.
        assert_eq!(get(FillField::FlightNumber), Some("4567"));
        assert_eq!(get(FillField::Origin), Some("EGLL"));
        assert_eq!(get(FillField::CruiseLevel), Some("370"));
        assert_eq!(get(FillField::Runway), Some("09R"));
        assert_eq!(get(FillField::Sid), Some("SUMMA2"));
        assert_eq!(get(FillField::Fuel), Some("6400"));
        assert_eq!(get(FillField::WeightUnit), Some("KG"));
        assert_eq!(get(FillField::Zfw), Some("52.3"));
        // Reserve plus alternate, in thousands rounded up: 1,200 + 700 is 1.9.
        assert_eq!(get(FillField::Reserves), Some("1.9"));
        assert_eq!(get(FillField::CruiseWind), Some("265/042"));
        assert_eq!(get(FillField::TransitionLevel), Some("70"));
        assert_eq!(get(FillField::TakeoffWeight), Some("58.5"));
        assert_eq!(get(FillField::Cargo), Some("2.1"));
        assert_eq!(get(FillField::NavDatabase), Some("2406"));
        assert!(pv.check_clearance);
        assert!(
            !pv.values.contains_key(&FillField::Transition),
            "the plan's SID has no transition"
        );
    }

    #[test]
    fn a_plan_fills_the_alternate_arrival_and_both_altimeter_settings() {
        let mut plan = parse_plan(&fixture("ofp-sample.json")).unwrap();
        let pv = plan_values(&plan).unwrap();
        assert_eq!(
            pv.values.get(&FillField::Alternate).map(String::as_str),
            Some("LFPO")
        );
        assert_eq!(
            pv.values
                .get(&FillField::LandingAltitude)
                .map(String::as_str),
            Some("392")
        );
        let o = plan.origin.as_mut().unwrap();
        o.metar = Some("EGLL 010850Z 24008KT 9999 FEW030 15/08 Q1015".into());
        let d = plan.destination.as_mut().unwrap();
        d.metar = Some("KSFO 010856Z 28010KT 10SM FEW017 15/11 A2992".into());
        d.runway = Some("RW28R".into());
        plan.general.star = Some("bdega4".into());
        let pv = plan_values(&plan).unwrap();
        let get = |f: FillField| pv.values.get(&f).map(String::as_str);
        assert_eq!(get(FillField::QnhUnit), Some("hPa"));
        assert_eq!(get(FillField::Qnh), Some("1015"));
        // The destination's 29.92 inHg in the departure's unit.
        assert_eq!(get(FillField::DestinationQnh), Some("1013"));
        assert_eq!(get(FillField::ArrivalRunway), Some("28R"));
        assert_eq!(get(FillField::Star), Some("BDEGA4"));
    }

    #[test]
    fn a_plan_goes_into_the_flight_in_one_action() {
        let pack = crate::test_support::ifly_pack();
        let plan = parse_plan(&fixture("ofp-sample.json")).unwrap();
        let values = plan_values(&plan).unwrap().values;
        let flight = crate::apply(
            &pack,
            crate::new_flight(&pack),
            crate::Action::SetValues {
                values: values.clone(),
            },
        )
        .unwrap();
        assert_eq!(flight.values, values);
    }

    #[test]
    fn a_long_flight_number_and_odd_runways_read_as_3_0_read_them() {
        assert_eq!(runway("RW9R").as_deref(), Some("09R"));
        assert_eq!(runway("27").as_deref(), Some("27"));
        assert_eq!(runway("37"), None);
        assert_eq!(runway("09X"), None);
        let mut plan = parse_plan(&fixture("ofp-sample.json")).unwrap();
        plan.general.airline = Some("TSTX".into());
        plan.general.flight_number = Some("45678".into());
        let pv = plan_values(&plan).unwrap();
        assert_eq!(
            pv.values[&FillField::FlightNumber],
            "45678",
            "airline and number exceed 7"
        );
        plan.destination = None;
        assert!(plan_values(&plan).is_err());
    }

    #[test]
    fn plans_read_and_errors_become_flight_plan_errors() {
        let plan = parse_plan(&fixture("ofp-sample.xml")).unwrap();
        assert_eq!(plan.origin.and_then(|o| o.icao).as_deref(), Some("EGLL"));
        assert_eq!(
            parse_plan(&fixture("ofp-error.json")),
            Err(AppError::FlightPlan(
                "SimBrief says: Error: Unknown UserID".into()
            ))
        );
        assert!(
            plan_url("Pilot")
                .unwrap()
                .ends_with("username=Pilot&json=1")
        );
        assert!(matches!(plan_url(" "), Err(AppError::FlightPlan(_))));
    }
}
