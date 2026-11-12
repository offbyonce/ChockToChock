// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! The compact plan (decision 0019): the parts of a SimBrief OFP the app uses now and a later
//! flight plan view needs, read from SimBrief's data. Personal fields (the pilot's name and ID,
//! the user ID, the request parameters and prefiling blocks) are never read. Missing values are
//! left out when saved, so the plan stays small.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use ts_rs::TS;

use crate::{OfpError, Units, int, list, num, text};

/// The plan's identity and times.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[ts(export, optional_fields)]
pub struct Plan {
    /// SimBrief's request ID and sequence ID: together they name this plan.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sequence_id: Option<String>,
    /// When SimBrief made the plan, in seconds since 1970 (UTC).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[ts(type = "number")]
    pub generated_at: Option<i64>,
    /// Fuel and weights are in these units.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub units: Option<Units>,
    #[serde(default)]
    pub general: General,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<Airport>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<Airport>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alternates: Vec<Airport>,
    /// The navlog, origin to destination.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fixes: Vec<Fix>,
    /// The navlog from the destination to the first alternate.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub alternate_fixes: Vec<Fix>,
    #[serde(default)]
    pub fuel: Fuel,
    #[serde(default)]
    pub weights: Weights,
    #[serde(default)]
    pub times: Times,
}

macro_rules! compact {
    ($(#[$meta:meta])* pub struct $name:ident { $($(#[$fmeta:meta])* pub $field:ident: $ty:ty,)* }) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, TS)]
        #[serde(rename_all = "camelCase", deny_unknown_fields)]
        #[ts(export, optional_fields)]
        pub struct $name {
            $(
                $(#[$fmeta])*
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub $field: $ty,
            )*
        }
    };
}

compact! {
    /// What the flight is and how it is planned.
    pub struct General {
        pub airline: Option<String>,
        pub flight_number: Option<String>,
        /// The aircraft type code (`B38M`) and registration.
        pub aircraft: Option<String>,
        pub registration: Option<String>,
        pub cruise_altitude_ft: Option<i32>,
        pub cost_index: Option<i32>,
        pub route: Option<String>,
        pub sid: Option<String>,
        pub sid_transition: Option<String>,
        pub star: Option<String>,
        pub star_transition: Option<String>,
        /// Average wind along the route: direction (degrees true) and speed (knots).
        pub avg_wind_dir: Option<i32>,
        pub avg_wind_speed: Option<i32>,
        /// Average temperature deviation from ISA (degrees C).
        pub avg_isa_deviation: Option<i32>,
        pub air_distance_nm: Option<i32>,
        pub route_distance_nm: Option<i32>,
        /// The navigation database cycle (`2406`).
        pub airac: Option<String>,
    }
}

compact! {
    /// An airport of the plan: origin, destination or alternate.
    pub struct Airport {
        pub icao: Option<String>,
        pub iata: Option<String>,
        pub name: Option<String>,
        pub elevation_ft: Option<i32>,
        #[ts(type = "number")]
        pub lat: Option<f64>,
        #[ts(type = "number")]
        pub lon: Option<f64>,
        pub runway: Option<String>,
        pub transition_altitude_ft: Option<i32>,
        pub transition_level_ft: Option<i32>,
        pub metar: Option<String>,
        pub taf: Option<String>,
    }
}

compact! {
    /// One navlog fix.
    pub struct Fix {
        pub ident: Option<String>,
        /// SimBrief's kind of fix: `wpt`, `vor`, `apt`, `ltlg` and so on.
        pub kind: Option<String>,
        pub via: Option<String>,
        /// Part of a SID or STAR rather than the en-route legs.
        pub sid_star: Option<bool>,
        /// `CLB`, `CRZ` or `DSC`.
        pub stage: Option<String>,
        #[ts(type = "number")]
        pub lat: Option<f64>,
        #[ts(type = "number")]
        pub lon: Option<f64>,
        pub altitude_ft: Option<i32>,
        /// Distance of the leg to this fix (nautical miles).
        pub distance_nm: Option<i32>,
        pub wind_dir: Option<i32>,
        pub wind_speed: Option<i32>,
        /// Outside air temperature (degrees C).
        pub oat: Option<i32>,
        /// Planned fuel on board at the fix.
        pub fuel_on_board: Option<i32>,
        /// Time from takeoff to the fix (seconds).
        pub elapsed_s: Option<i32>,
    }
}

compact! {
    /// The fuel blocks.
    pub struct Fuel {
        pub taxi: Option<i32>,
        pub trip: Option<i32>,
        pub contingency: Option<i32>,
        pub alternate: Option<i32>,
        pub reserve: Option<i32>,
        pub etops: Option<i32>,
        pub extra: Option<i32>,
        pub min_takeoff: Option<i32>,
        pub plan_takeoff: Option<i32>,
        pub plan_ramp: Option<i32>,
        pub plan_landing: Option<i32>,
    }
}

compact! {
    /// Weights, planned and limits.
    pub struct Weights {
        pub oew: Option<i32>,
        pub passengers: Option<i32>,
        pub cargo: Option<i32>,
        pub payload: Option<i32>,
        pub zfw: Option<i32>,
        pub max_zfw: Option<i32>,
        pub takeoff: Option<i32>,
        pub max_takeoff: Option<i32>,
        pub landing: Option<i32>,
        pub max_landing: Option<i32>,
    }
}

compact! {
    /// Scheduled times (seconds since 1970, UTC) and durations (seconds).
    pub struct Times {
        #[ts(type = "number")]
        pub sched_out: Option<i64>,
        #[ts(type = "number")]
        pub sched_off: Option<i64>,
        #[ts(type = "number")]
        pub sched_on: Option<i64>,
        #[ts(type = "number")]
        pub sched_in: Option<i64>,
        pub enroute_s: Option<i32>,
        pub block_s: Option<i32>,
        pub taxi_out_s: Option<i32>,
        pub taxi_in_s: Option<i32>,
    }
}

fn long(root: &Value, path: &[&str]) -> Option<i64> {
    text(root, path)?.parse::<i64>().ok()
}

fn airport(v: &Value, weather: Option<(&Value, &str)>) -> Option<Airport> {
    let icao = text(v, &["icao_code"])?;
    let wx = |key: &str| weather.and_then(|(w, p)| text(w, &[&format!("{p}_{key}")]));
    Some(Airport {
        icao: Some(icao),
        iata: text(v, &["iata_code"]),
        name: text(v, &["name"]),
        elevation_ft: int(v, &["elevation"]),
        lat: num(v, &["pos_lat"]),
        lon: num(v, &["pos_long"]),
        runway: text(v, &["plan_rwy"]),
        transition_altitude_ft: int(v, &["trans_alt"]),
        transition_level_ft: int(v, &["trans_level"]),
        metar: text(v, &["metar"]).or_else(|| wx("metar")),
        taf: text(v, &["taf"]).or_else(|| wx("taf")),
    })
}

pub(crate) fn fix(v: &Value) -> Option<Fix> {
    Some(Fix {
        ident: Some(text(v, &["ident"])?),
        kind: text(v, &["type"]),
        via: text(v, &["via_airway"]),
        sid_star: text(v, &["is_sid_star"]).map(|s| s == "1"),
        stage: text(v, &["stage"]),
        lat: num(v, &["pos_lat"]),
        lon: num(v, &["pos_long"]),
        altitude_ft: int(v, &["altitude_feet"]),
        distance_nm: int(v, &["distance"]),
        wind_dir: int(v, &["wind_dir"]),
        wind_speed: int(v, &["wind_spd"]),
        oat: int(v, &["oat"]),
        fuel_on_board: int(v, &["fuel_plan_onboard"]),
        elapsed_s: int(v, &["time_total"]),
    })
}

/// The compact plan from SimBrief's data (JSON, or XML turned into the same shape).
pub(crate) fn plan_from_value(root: &Value) -> Result<Plan, OfpError> {
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
    let weather = root.get("weather");
    let at = |key: &str, wx: &'static str| {
        root.get(key)
            .and_then(|v| airport(v, weather.map(|w| (w, wx))))
    };
    let alternates = list(root.get("alternate"))
        .enumerate()
        .filter_map(|(i, v)| airport(v, weather.filter(|_| i == 0).map(|w| (w, "altn"))))
        .collect();
    Ok(Plan {
        request_id: text(root, &["params", "request_id"]),
        sequence_id: text(root, &["params", "sequence_id"]),
        generated_at: long(root, &["params", "time_generated"]),
        units,
        general: General {
            airline: text(root, &["general", "icao_airline"]),
            flight_number: text(root, &["general", "flight_number"]),
            aircraft: text(root, &["aircraft", "icaocode"]),
            registration: text(root, &["aircraft", "reg"]),
            cruise_altitude_ft: int(root, &["general", "initial_altitude"]),
            cost_index: int(root, &["general", "costindex"]),
            route: text(root, &["general", "route"]),
            sid: text(root, &["general", "sid_ident"]),
            sid_transition: text(root, &["general", "sid_trans"]),
            star: text(root, &["general", "star_ident"]),
            star_transition: text(root, &["general", "star_trans"]),
            avg_wind_dir: int(root, &["general", "avg_wind_dir"]),
            avg_wind_speed: int(root, &["general", "avg_wind_spd"]),
            avg_isa_deviation: int(root, &["general", "avg_temp_dev"]),
            air_distance_nm: int(root, &["general", "air_distance"]),
            route_distance_nm: int(root, &["general", "route_distance"]),
            airac: text(root, &["params", "airac"]),
        },
        origin: at("origin", "orig"),
        destination: at("destination", "dest"),
        alternates,
        fixes: list(root.pointer("/navlog/fix")).filter_map(fix).collect(),
        alternate_fixes: list(root.pointer("/alternate_navlog/fix"))
            .filter_map(fix)
            .collect(),
        fuel: Fuel {
            taxi: int(root, &["fuel", "taxi"]),
            trip: int(root, &["fuel", "enroute_burn"]),
            contingency: int(root, &["fuel", "contingency"]),
            alternate: int(root, &["fuel", "alternate_burn"]),
            reserve: int(root, &["fuel", "reserve"]),
            etops: int(root, &["fuel", "etops"]),
            extra: int(root, &["fuel", "extra"]),
            min_takeoff: int(root, &["fuel", "min_takeoff"]),
            plan_takeoff: int(root, &["fuel", "plan_takeoff"]),
            plan_ramp: int(root, &["fuel", "plan_ramp"]),
            plan_landing: int(root, &["fuel", "plan_landing"]),
        },
        weights: Weights {
            oew: int(root, &["weights", "oew"]),
            passengers: int(root, &["weights", "pax_count"]),
            cargo: int(root, &["weights", "cargo"]),
            payload: int(root, &["weights", "payload"]),
            zfw: int(root, &["weights", "est_zfw"]),
            max_zfw: int(root, &["weights", "max_zfw"]),
            takeoff: int(root, &["weights", "est_tow"]),
            max_takeoff: int(root, &["weights", "max_tow"]),
            landing: int(root, &["weights", "est_ldw"]),
            max_landing: int(root, &["weights", "max_ldw"]),
        },
        times: Times {
            sched_out: long(root, &["times", "sched_out"]),
            sched_off: long(root, &["times", "sched_off"]),
            sched_on: long(root, &["times", "sched_on"]),
            sched_in: long(root, &["times", "sched_in"]),
            enroute_s: int(root, &["times", "est_time_enroute"]),
            block_s: int(root, &["times", "est_block"]),
            taxi_out_s: int(root, &["times", "taxi_out"]),
            taxi_in_s: int(root, &["times", "taxi_in"]),
        },
    })
}
