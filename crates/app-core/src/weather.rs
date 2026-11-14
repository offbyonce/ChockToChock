// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! METAR and TAF, decoded from one code table (owner, 2026-11-13). Each kind of group the table
//! knows names the section of the code it comes from: the WMO code forms FM 15 (METAR) and FM 51
//! (TAF), and US practice for visibility in statute miles, the A altimeter and remarks. The full
//! references are kept outside the repository (decisions 0007 and 0013). A group the table does
//! not cover is shown as printed and marked not decoded; nothing is guessed. The plain-language
//! descriptions the interface shows beside these rows are batch 7 content.

use serde::Serialize;
use ts_rs::TS;

/// What a group of a report says.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum Element {
    /// METAR or SPECI, TAF, AMD or COR: the report's kind.
    Kind,
    Station,
    Time,
    /// AUTO: no observer.
    Auto,
    /// The TAF's period, e.g. 1400/1506.
    Validity,
    Wind,
    /// The wind's direction varies between two bearings.
    WindVariation,
    Visibility,
    RunwayVisualRange,
    Weather,
    /// NSW: no significant weather (TAF).
    NoWeather,
    Clouds,
    /// CAVOK: ceiling and visibility OK.
    Cavok,
    Temperature,
    Altimeter,
    /// TX / TN: the forecast's highest and lowest temperatures (TAF).
    TemperatureExtreme,
    /// NOSIG: no significant change expected.
    NoChange,
    /// FM, BECMG, TEMPO, PROB: a change in a TAF or a trend.
    Change,
    /// RMK: remarks follow (US practice decodes some of them).
    Remarks,
    NotDecoded,
}

/// One row of a decoded report: the groups it reads and what they say.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct DecodedRow {
    pub element: Element,
    /// The groups as printed.
    pub raw: Vec<String>,
    /// What they say, one line each; empty for a group that is not decoded.
    pub lines: Vec<String>,
    /// The section of the code this comes from.
    pub source: String,
}

/// A report decoded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Decoded {
    pub station: Option<String>,
    /// The time of the report: day of the month, hour and minute in UTC.
    pub time: Option<(u8, u8, u8)>,
    /// Every group as printed, in order, with the row that reads it.
    pub groups: Vec<(String, usize)>,
    pub rows: Vec<DecodedRow>,
}

/// An altimeter setting read from a METAR.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Altimeter {
    /// "30.12" (inHg) or "1013" (hPa), as the flight's QNH fields take it.
    pub value: String,
    /// `inHg` or `hPa`.
    pub unit: String,
    /// The group as printed, e.g. "A3012".
    pub raw: String,
}

// The code table: where each kind of group is defined. Every entry was checked on 2026-11-14
// against the current editions of the WMO Manual on Codes, Volume I.1 (FM 15 METAR, FM 51 TAF)
// and the FAA's aviation weather services guidance (US practice); the full references are kept
// outside the repository (decisions 0007 and 0013). "FM 15 · 15.5" is regulation 15.5 of code form
// FM 15; "US · 3.1.5.6" is that section of the US guidance.
const SRC_KIND_METAR: &str = "FM 15 · 15.1";
const SRC_KIND_TAF: &str = "FM 51 · 51.1";
const SRC_COR_METAR: &str = "FM 15 · note 2";
const SRC_COR_TAF: &str = "FM 51 · note 5";
const SRC_AMD: &str = "FM 51 · 51.11";
const SRC_STATION_METAR: &str = "FM 15 · 15.2";
const SRC_STATION_TAF: &str = "FM 51 · 51.2";
const SRC_TIME_METAR: &str = "FM 15 · 15.3";
const SRC_TIME_TAF: &str = "FM 51 · 51.1.2";
const SRC_AUTO: &str = "FM 15 · 15.4";
const SRC_VALIDITY: &str = "FM 51 · 51.1.4";
const SRC_WIND_METAR: &str = "FM 15 · 15.5";
const SRC_WIND_TAF: &str = "FM 51 · 51.3";
const SRC_VIS_METAR: &str = "FM 15 · 15.6";
const SRC_VIS_TAF: &str = "FM 51 · 51.4";
const SRC_VIS_SM_METAR: &str = "US · 3.1.5.6";
const SRC_VIS_SM_TAF: &str = "US · 5.13.2.6";
const SRC_RVR: &str = "FM 15 · 15.7";
const SRC_RVR_FT: &str = "US · 3.1.5.7";
const SRC_WX_METAR: &str = "FM 15 · 15.8";
const SRC_WX_TAF: &str = "FM 51 · 51.5";
const SRC_RECENT: &str = "FM 15 · 15.13.2";
const SRC_CLOUD_METAR: &str = "FM 15 · 15.9.1";
const SRC_CLOUD_TAF: &str = "FM 51 · 51.6.1";
const SRC_VV_METAR: &str = "FM 15 · 15.9.2";
const SRC_VV_TAF: &str = "FM 51 · 51.6.2";
const SRC_CLEAR_US: &str = "US · 3.1.5.9";
const SRC_CAVOK_METAR: &str = "FM 15 · 15.10";
const SRC_CAVOK_TAF: &str = "FM 51 · 51.7";
const SRC_TEMP: &str = "FM 15 · 15.11";
const SRC_TAF_TX: &str = "FM 51 · 51.10";
const SRC_QNH: &str = "FM 15 · 15.12";
const SRC_ALT_A: &str = "US · 3.1.5.11";
const SRC_TREND: &str = "FM 15 · 15.14";
const SRC_NOSIG: &str = "FM 15 · 15.14.15";
const SRC_NSW_METAR: &str = "FM 15 · 15.14.13";
const SRC_NSW_TAF: &str = "FM 51 · 51.5.2";
const SRC_TAF_CHANGE: &str = "FM 51 · 51.8";
const SRC_PROB: &str = "FM 51 · 51.9";
const SRC_RMK: &str = "US · 3.1.5.12";
const SRC_RMK_STATION: &str = "US · 3.1.5.13.3";
const SRC_RMK_SLP: &str = "US · 3.1.5.13.25";
const SRC_RMK_TEMP: &str = "US · 3.1.5.13.31";
const SRC_RMK_MAINT: &str = "US · 3.1.5.14.1";

/// One of two sources: the METAR's, or the TAF's.
const fn by(taf: bool, metar: &'static str, forecast: &'static str) -> &'static str {
    if taf { forecast } else { metar }
}

const COVER: [(&str, &str); 4] = [
    ("FEW", "Few"),
    ("SCT", "Scattered"),
    ("BKN", "Broken"),
    ("OVC", "Overcast"),
];
const INTENSITY: [(&str, &str); 3] = [("+", "Heavy"), ("-", "Light"), ("VC", "In the vicinity:")];
const DESCRIPTOR: [(&str, &str); 8] = [
    ("MI", "shallow"),
    ("PR", "partial"),
    ("BC", "patches of"),
    ("DR", "low drifting"),
    ("BL", "blowing"),
    ("SH", "showers of"),
    ("TS", "thunderstorm with"),
    ("FZ", "freezing"),
];
const PHENOMENON: [(&str, &str); 22] = [
    ("DZ", "drizzle"),
    ("RA", "rain"),
    ("SN", "snow"),
    ("SG", "snow grains"),
    ("IC", "ice crystals"),
    ("PL", "ice pellets"),
    ("GR", "hail"),
    ("GS", "small hail"),
    ("UP", "unknown precipitation"),
    ("BR", "mist"),
    ("FG", "fog"),
    ("FU", "smoke"),
    ("VA", "volcanic ash"),
    ("DU", "dust"),
    ("SA", "sand"),
    ("HZ", "haze"),
    ("PY", "spray"),
    ("PO", "dust or sand whirls"),
    ("SQ", "squalls"),
    ("FC", "funnel cloud"),
    ("SS", "sandstorm"),
    ("DS", "duststorm"),
];

fn row(element: Element, raw: &[&str], lines: Vec<String>, source: &str) -> DecodedRow {
    DecodedRow {
        element,
        raw: raw.iter().map(|s| (*s).to_owned()).collect(),
        lines,
        source: source.to_owned(),
    }
}

fn digits(s: &str) -> bool {
    !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())
}

fn num(s: &str) -> Option<u32> {
    if digits(s) { s.parse().ok() } else { None }
}

/// "1,000" for 1000.
fn grouped(n: u32) -> String {
    let s = n.to_string();
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if i > 0 && (s.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(c);
    }
    out
}

/// DDHHMMZ.
fn time(group: &str) -> Option<(u8, u8, u8)> {
    let body = group.strip_suffix('Z')?;
    if body.len() != 6 || !digits(body) {
        return None;
    }
    let (day, hour, minute): (u8, u8, u8) = (
        body[0..2].parse().ok()?,
        body[2..4].parse().ok()?,
        body[4..6].parse().ok()?,
    );
    ((1..=31).contains(&day) && hour <= 24 && minute <= 59).then_some((day, hour, minute))
}

/// The wind group: dddff(Gfmfm)KT, MPS or KMH; VRBff; 00000.
fn wind(g: &str) -> Option<String> {
    let (body, unit) = [("KT", "kt"), ("MPS", "m/s"), ("KMH", "km/h")]
        .iter()
        .find_map(|(s, u)| g.strip_suffix(s).map(|b| (b, *u)))?;
    let (main, gust) = match body.split_once('G') {
        Some((m, g)) => (m, Some(num(g)?)),
        None => (body, None),
    };
    if main.len() < 5 {
        return None;
    }
    let (dir, speed) = main.split_at(3);
    let speed = num(speed)?;
    let gusts = gust
        .map(|g| format!(", gusting {g} {unit}"))
        .unwrap_or_default();
    if dir == "VRB" {
        return Some(format!("Variable at {speed} {unit}{gusts}"));
    }
    let d = num(dir)?;
    if d > 360 {
        return None;
    }
    if d == 0 && speed == 0 && gust.is_none() {
        return Some("Calm".into());
    }
    Some(format!("{d}° at {speed} {unit}{gusts}"))
}

/// dddVddd.
fn variation(g: &str) -> Option<String> {
    let (a, b) = g.split_once('V')?;
    let (a, b) = (num(a)?, num(b)?);
    (a.to_string().len() <= 3 && a <= 360 && b <= 360 && g.len() == 7)
        .then(|| format!("Varying between {a}° and {b}°"))
}

/// Metres: VVVV with an optional direction; 9999 is 10 km or more.
fn visibility_m(g: &str) -> Option<String> {
    let (v, dir) = if g.len() > 4 { g.split_at(4) } else { (g, "") };
    let v = num(v)?;
    let dirs = ["N", "NE", "E", "SE", "S", "SW", "W", "NW", "NDV"];
    if !dir.is_empty() && !dirs.contains(&dir) {
        return None;
    }
    let base = if v == 9999 {
        "10 km or more".to_owned()
    } else if v >= 5000 {
        format!("{} km", v / 1000)
    } else {
        format!("{} m", grouped(v))
    };
    Some(match dir {
        "" => base,
        "NDV" => format!("{base}, no directional variation"),
        d => format!("{base} toward {d}"),
    })
}

/// Statute miles: 10SM, 1/2SM, M1/4SM, P6SM; `whole` is a preceding whole number ("1 1/2SM").
fn visibility_sm(group: &str, whole: Option<u32>) -> Option<String> {
    let miles = group.strip_suffix("SM")?;
    let (prefix, miles) = if let Some(rest) = miles.strip_prefix('M') {
        ("less than ", rest)
    } else if let Some(rest) = miles.strip_prefix('P') {
        ("more than ", rest)
    } else {
        ("", miles)
    };
    let amount = if let Some((top, bottom)) = miles.split_once('/') {
        let (top, bottom) = (num(top)?, num(bottom)?);
        if bottom == 0 {
            return None;
        }
        match whole {
            Some(units) => format!("{units} {top}/{bottom}"),
            None => format!("{top}/{bottom}"),
        }
    } else {
        if whole.is_some() {
            return None;
        }
        num(miles)?.to_string()
    };
    // One mile or less reads "mile": "1 statute mile", "1/4 statute mile".
    let one = amount == "1" || (whole.is_none() && amount.contains('/'));
    Some(format!(
        "{prefix}{amount} statute mile{}",
        if one { "" } else { "s" }
    ))
}

/// R16L/0600(V0800)(U|D|N) or R28R/2400FT: runway visual range.
fn rvr(g: &str) -> Option<String> {
    let rest = g.strip_prefix('R')?;
    let (rwy, val) = rest.split_once('/')?;
    if rwy.is_empty()
        || rwy.len() > 3
        || !rwy[..2.min(rwy.len())].bytes().all(|b| b.is_ascii_digit())
    {
        return None;
    }
    let (val, feet) = match val.strip_suffix("FT") {
        Some(v) => (v, true),
        None => (val, false),
    };
    let (val, trend) = match val.chars().last() {
        Some('U') => (&val[..val.len() - 1], ", rising"),
        Some('D') => (&val[..val.len() - 1], ", falling"),
        Some('N') => (&val[..val.len() - 1], ", no change"),
        _ => (val, ""),
    };
    let unit = if feet { "ft" } else { "m" };
    // Four digits each, with P (more than) or M (less than): anything else, such as a runway
    // state group, is not a visual range.
    let four = |v: &str| (v.len() == 4).then(|| num(v)).flatten();
    let one = |v: &str| -> Option<String> {
        if let Some(r) = v.strip_prefix('P') {
            Some(format!("more than {} {unit}", grouped(four(r)?)))
        } else if let Some(r) = v.strip_prefix('M') {
            Some(format!("less than {} {unit}", grouped(four(r)?)))
        } else {
            Some(format!("{} {unit}", grouped(four(v)?)))
        }
    };
    let range = match val.split_once('V') {
        Some((a, b)) => format!("{} to {}", one(a)?, one(b)?),
        None => one(val)?,
    };
    Some(format!("Runway {rwy}: {range}{trend}"))
}

/// Present weather: intensity or proximity, a descriptor, phenomena; RE for recent weather.
fn weather(g: &str) -> Option<String> {
    let (recent, mut rest) = match g.strip_prefix("RE") {
        Some(r) => (true, r),
        None => (false, g),
    };
    let mut words = vec![];
    for (code, word) in INTENSITY {
        if let Some(r) = rest.strip_prefix(code) {
            words.push(word.to_owned());
            rest = r;
            break;
        }
    }
    for (code, word) in DESCRIPTOR {
        if let Some(r) = rest.strip_prefix(code) {
            words.push(word.to_owned());
            rest = r;
            break;
        }
    }
    let mut found = false;
    while !rest.is_empty() {
        let (code, word) = PHENOMENON.iter().find(|(c, _)| rest.starts_with(c))?;
        words.push((*word).to_owned());
        rest = &rest[code.len()..];
        found = true;
    }
    // A descriptor alone (TS, SH) is weather too: a thunderstorm, or showers.
    if !found {
        match g.trim_start_matches(['+', '-']).trim_start_matches("VC") {
            "TS" => {
                words = vec![
                    words.first().cloned().unwrap_or_default(),
                    "thunderstorm".into(),
                ];
            }
            "SH" => words = vec![words.first().cloned().unwrap_or_default(), "showers".into()],
            _ => return None,
        }
        words.retain(|w| !w.is_empty());
    }
    let mut text = words.join(" ");
    if let Some(first) = text.get(0..1) {
        text = first.to_uppercase() + &text[1..];
    }
    Some(if recent {
        format!("Recent: {}", lower_first(&text))
    } else {
        text
    })
}

fn lower_first(s: &str) -> String {
    let mut c = s.chars();
    c.next()
        .map(|f| f.to_lowercase().collect::<String>() + c.as_str())
        .unwrap_or_default()
}

/// One cloud group: cover and height in hundreds of feet, with CB or TCU; VV; NSC, NCD, SKC, CLR.
fn cloud(g: &str) -> Option<String> {
    match g {
        "NSC" => return Some("No significant cloud".into()),
        "NCD" => return Some("No cloud detected".into()),
        "SKC" => return Some("Sky clear".into()),
        "CLR" => return Some("No cloud detected at or below 12,000 ft".into()),
        _ => {}
    }
    if let Some(h) = g.strip_prefix("VV") {
        return if h == "///" {
            Some("Sky hidden, vertical visibility not reported".into())
        } else {
            Some(format!(
                "Sky hidden, vertical visibility {} ft",
                grouped(num(h)? * 100)
            ))
        };
    }
    let (word, rest) = COVER
        .iter()
        .find_map(|(c, w)| g.strip_prefix(c).map(|r| (*w, r)))?;
    if rest.len() < 3 {
        return None;
    }
    let (h, kind) = rest.split_at(3);
    let kind = match kind {
        "" => "",
        "CB" => " cumulonimbus",
        "TCU" => " towering cumulus",
        "///" => " (type not reported)",
        _ => return None,
    };
    if h == "///" {
        return Some(format!("{word}{kind}, height not reported"));
    }
    Some(format!("{word} {} ft{kind}", grouped(num(h)? * 100)))
}

/// TT/TdTd with M for minus.
/// Two digits with M for minus, in whole degrees.
fn celsius(text: &str) -> Option<i32> {
    let (minus, digits_part) = match text.strip_prefix('M') {
        Some(rest) => (true, rest),
        None => (false, text),
    };
    if digits_part.len() != 2 {
        return None;
    }
    let value = i32::try_from(num(digits_part)?).ok()?;
    Some(if minus { -value } else { value })
}

fn temperature(group: &str) -> Option<String> {
    let (air, dew) = group.split_once('/')?;
    let air = celsius(air)?;
    if dew.is_empty() || dew == "//" {
        return Some(format!("{air} °C"));
    }
    Some(format!("{air} °C · dew point {} °C", celsius(dew)?))
}

/// Q (hPa) or A (inches of mercury, US practice).
fn altimeter_group(g: &str) -> Option<(Altimeter, String, &'static str)> {
    if g.len() != 5 {
        return None;
    }
    let (kind, v) = g.split_at(1);
    let n = num(v)?;
    match kind {
        "Q" if (850..=1100).contains(&n) => Some((
            Altimeter {
                value: n.to_string(),
                unit: "hPa".into(),
                raw: g.into(),
            },
            format!("{n} hPa"),
            SRC_QNH,
        )),
        "A" if (2500..=3200).contains(&n) => {
            let v = format!("{}.{:02}", n / 100, n % 100);
            Some((
                Altimeter {
                    value: v.clone(),
                    unit: "inHg".into(),
                    raw: g.into(),
                },
                format!("{v} inHg"),
                SRC_ALT_A,
            ))
        }
        _ => None,
    }
}

/// DDHH/DDHH.
fn period(g: &str) -> Option<String> {
    let (a, b) = g.split_once('/')?;
    let p = |v: &str| -> Option<(u32, u32)> {
        if v.len() == 4 {
            Some((num(&v[..2])?, num(&v[2..])?))
        } else {
            None
        }
    };
    let ((d1, h1), (d2, h2)) = (p(a)?, p(b)?);
    Some(format!("Day {d1} {h1:02}:00 to day {d2} {h2:02}:00 UTC"))
}

/// TX12/1418Z or TNM03/1421Z.
fn extreme(g: &str) -> Option<String> {
    let (word, rest) = if let Some(r) = g.strip_prefix("TX") {
        ("Highest", r)
    } else {
        ("Lowest", g.strip_prefix("TN")?)
    };
    let (value, at) = rest.split_once('/')?;
    let at = at.strip_suffix('Z')?;
    let t = celsius(value)?;
    if at.len() != 4 {
        return None;
    }
    Some(format!(
        "{word} {t} °C, day {} {}:00 UTC",
        num(&at[..2])?,
        &at[2..]
    ))
}

/// FM DDHHMM, BECMG, TEMPO, PROB30/40: what a change group says; `next` is the period after it.
fn change(g: &str) -> Option<String> {
    if let Some(t) = g.strip_prefix("FM")
        && t.len() == 6
        && digits(t)
    {
        return Some(format!(
            "From day {} {}:{} UTC",
            num(&t[..2])?,
            &t[2..4],
            &t[4..]
        ));
    }
    match g {
        "BECMG" => Some("Becoming".into()),
        "TEMPO" => Some("Temporarily".into()),
        "PROB30" => Some("30% probability".into()),
        "PROB40" => Some("40% probability".into()),
        _ => None,
    }
}

/// What a US remark says, for the remarks the table covers; the rest stay as printed.
fn remark(g: &str) -> Option<String> {
    match g {
        "AO1" => return Some("Automated station without a precipitation discriminator".into()),
        "AO2" => return Some("Automated station with a precipitation discriminator".into()),
        "$" => return Some("The station needs maintenance".into()),
        _ => {}
    }
    if let Some(p) = g.strip_prefix("SLP")
        && p.len() == 3
    {
        let n = num(p)?;
        let hpa = if n >= 500 {
            900.0 + f64::from(n) / 10.0
        } else {
            1000.0 + f64::from(n) / 10.0
        };
        return Some(format!("Sea level pressure {hpa:.1} hPa"));
    }
    if let Some(t) = g.strip_prefix('T')
        && t.len() == 8
        && digits(t)
    {
        let c = |s: &str| -> Option<f64> {
            let v = f64::from(num(&s[1..])?) / 10.0;
            Some(if &s[..1] == "1" { -v } else { v })
        };
        return Some(format!(
            "Temperature {:.1} °C · dew point {:.1} °C",
            c(&t[..4])?,
            c(&t[4..])?
        ));
    }
    None
}

/// Decodes a METAR or SPECI.
pub fn decode_metar(text: &str) -> Decoded {
    decode(text, false)
}

/// Decodes a TAF.
pub fn decode_taf(text: &str) -> Decoded {
    decode(text, true)
}

/// One group read: what it says, the section it comes from, how many tokens it took, and
/// whether it joins the row before it (a second cloud layer, a wind variation).
struct Read {
    element: Element,
    lines: Vec<String>,
    source: &'static str,
    taken: usize,
    joins: bool,
}

impl Read {
    fn one(element: Element, line: impl Into<String>, source: &'static str) -> Self {
        Self {
            element,
            lines: vec![line.into()],
            source,
            taken: 1,
            joins: false,
        }
    }
    fn joining(mut self) -> Self {
        self.joins = true;
        self
    }
}

/// Where the reader is in a report.
#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Stage {
    /// The kind and station.
    #[default]
    Header,
    /// The station is read; the time comes next.
    Station,
    /// The weather groups.
    Body,
    /// After RMK.
    Remarks,
}

struct Place {
    stage: Stage,
    station: bool,
}

/// The report's kind and station, which come first.
fn read_header(group: &str, taf: bool, place: &mut Place) -> Option<Read> {
    if place.stage == Stage::Header
        && matches!(group, "METAR" | "SPECI" | "TAF" | "AMD" | "COR" | "CCA")
    {
        let what = match group {
            "METAR" => "Routine report",
            "SPECI" => "Special report",
            "TAF" => "Forecast",
            "AMD" => "Amended",
            _ => "Corrected",
        };
        return Some(Read::one(
            Element::Kind,
            what,
            match group {
                "AMD" => SRC_AMD,
                "COR" | "CCA" => by(taf, SRC_COR_METAR, SRC_COR_TAF),
                _ => by(taf, SRC_KIND_METAR, SRC_KIND_TAF),
            },
        ));
    }
    if !place.station && group.len() == 4 && group.bytes().all(|b| b.is_ascii_uppercase()) {
        place.station = true;
        place.stage = Stage::Station;
        return Some(Read::one(
            Element::Station,
            group,
            by(taf, SRC_STATION_METAR, SRC_STATION_TAF),
        ));
    }
    let before = place.stage;
    place.stage = place.stage.max(Stage::Body);
    if before < Stage::Body
        && let Some((day, hour, minute)) = time(group)
    {
        return Some(Read::one(
            Element::Time,
            format!("Day {day}, {hour:02}:{minute:02} UTC"),
            by(taf, SRC_TIME_METAR, SRC_TIME_TAF),
        ));
    }
    None
}

/// Words that stand alone: AUTO, COR, RMK, NOSIG, NSW, CAVOK.
fn read_word(group: &str, taf: bool, place: &mut Place) -> Option<Read> {
    Some(match group {
        "AUTO" => Read::one(Element::Auto, "Automated report, no observer", SRC_AUTO),
        "COR" => Read::one(
            Element::Kind,
            "Corrected",
            by(taf, SRC_COR_METAR, SRC_COR_TAF),
        ),
        "RMK" => {
            place.stage = Stage::Remarks;
            Read::one(Element::Remarks, "Remarks follow", SRC_RMK)
        }
        "NOSIG" => Read::one(
            Element::NoChange,
            "No significant change expected",
            SRC_NOSIG,
        ),
        "NSW" => Read::one(
            Element::NoWeather,
            "No significant weather",
            by(taf, SRC_NSW_METAR, SRC_NSW_TAF),
        ),
        "CAVOK" => Read::one(
            Element::Cavok,
            "Visibility 10 km or more, no cloud below 5,000 ft or CB, no significant weather",
            by(taf, SRC_CAVOK_METAR, SRC_CAVOK_TAF),
        ),
        _ => return None,
    })
}

/// A change group, with PROB's TEMPO and the period that follows.
fn read_change(tokens: &[&str], at: usize, taf: bool) -> Option<Read> {
    let group = tokens[at];
    let first = change(group)?;
    let mut lines = vec![first];
    let mut taken = 1;
    if group.starts_with("PROB") && tokens.get(at + 1) == Some(&"TEMPO") {
        lines[0] = format!("{}, temporarily", lines[0]);
        taken += 1;
    }
    if let Some(p) = tokens.get(at + taken).and_then(|t| period(t)) {
        lines.push(p);
        taken += 1;
    }
    let source = if !taf {
        SRC_TREND
    } else if group.starts_with("PROB") {
        SRC_PROB
    } else {
        SRC_TAF_CHANGE
    };
    Some(Read {
        element: Element::Change,
        lines,
        source,
        taken,
        joins: false,
    })
}

/// The weather itself: wind, visibility, runway visual range, clouds, temperatures, pressure.
fn read_body(tokens: &[&str], at: usize, taf: bool) -> Option<Read> {
    let group = tokens[at];
    if let Some(w) = wind(group) {
        return Some(Read::one(
            Element::Wind,
            w,
            by(taf, SRC_WIND_METAR, SRC_WIND_TAF),
        ));
    }
    if let Some(v) = variation(group) {
        return Some(Read::one(Element::Wind, v, by(taf, SRC_WIND_METAR, SRC_WIND_TAF)).joining());
    }
    // "1 1/2SM": a whole number, then a fraction in statute miles.
    if group.len() == 1
        && let Some(next) = tokens.get(at + 1)
        && let Some(v) = visibility_sm(next, num(group))
    {
        return Some(Read {
            element: Element::Visibility,
            lines: vec![v],
            source: by(taf, SRC_VIS_SM_METAR, SRC_VIS_SM_TAF),
            taken: 2,
            joins: false,
        });
    }
    if let Some(v) = visibility_sm(group, None) {
        return Some(Read::one(
            Element::Visibility,
            v,
            by(taf, SRC_VIS_SM_METAR, SRC_VIS_SM_TAF),
        ));
    }
    if (4..=7).contains(&group.len())
        && let Some(v) = visibility_m(group)
    {
        return Some(
            Read::one(Element::Visibility, v, by(taf, SRC_VIS_METAR, SRC_VIS_TAF)).joining(),
        );
    }
    if let Some(v) = rvr(group) {
        let source = if group.ends_with("FT") {
            SRC_RVR_FT
        } else {
            SRC_RVR
        };
        return Some(Read::one(Element::RunwayVisualRange, v, source).joining());
    }
    if let Some(c) = cloud(group) {
        let source = if matches!(group, "CLR" | "SKC") {
            SRC_CLEAR_US
        } else if group.starts_with("VV") {
            by(taf, SRC_VV_METAR, SRC_VV_TAF)
        } else {
            by(taf, SRC_CLOUD_METAR, SRC_CLOUD_TAF)
        };
        return Some(Read::one(Element::Clouds, c, source).joining());
    }
    if let Some(t) = extreme(group) {
        return Some(Read::one(Element::TemperatureExtreme, t, SRC_TAF_TX).joining());
    }
    if !taf && let Some(t) = temperature(group) {
        return Some(Read::one(Element::Temperature, t, SRC_TEMP));
    }
    if let Some((_, line, source)) = altimeter_group(group) {
        return Some(Read::one(Element::Altimeter, line, source));
    }
    let source = if group.starts_with("RE") {
        SRC_RECENT
    } else {
        by(taf, SRC_WX_METAR, SRC_WX_TAF)
    };
    weather(group).map(|w| Read::one(Element::Weather, w, source).joining())
}

/// Adds a read to the report: a new row, or the row before when it joins one of its kind.
fn add(out: &mut Decoded, raws: &[&str], read: Read) {
    let index = match out.rows.last_mut() {
        Some(last) if read.joins && last.element == read.element => {
            last.raw.extend(raws.iter().map(|s| (*s).to_owned()));
            last.lines.extend(read.lines);
            out.rows.len() - 1
        }
        _ => {
            out.rows
                .push(row(read.element, raws, read.lines, read.source));
            out.rows.len() - 1
        }
    };
    out.groups
        .extend(raws.iter().map(|g| ((*g).to_owned(), index)));
}

fn decode(text: &str, taf: bool) -> Decoded {
    let tokens: Vec<&str> = text
        .split_whitespace()
        .filter(|t| *t != "=")
        .map(|t| t.trim_end_matches('='))
        .collect();
    let mut out = Decoded {
        station: None,
        time: None,
        groups: vec![],
        rows: vec![],
    };
    let mut place = Place {
        stage: Stage::Header,
        station: false,
    };
    let mut at = 0;
    while at < tokens.len() {
        let group = tokens[at];
        let read = if place.stage == Stage::Remarks {
            remark(group).map(|line| {
                let source = match group {
                    "AO1" | "AO2" => SRC_RMK_STATION,
                    "$" => SRC_RMK_MAINT,
                    g if g.starts_with("SLP") => SRC_RMK_SLP,
                    _ => SRC_RMK_TEMP,
                };
                Read::one(Element::Remarks, line, source)
            })
        } else {
            read_header(group, taf, &mut place)
                .or_else(|| read_word(group, taf, &mut place))
                .or_else(|| {
                    if taf {
                        period(group).map(|p| Read::one(Element::Validity, p, SRC_VALIDITY))
                    } else {
                        None
                    }
                })
                .or_else(|| read_change(&tokens, at, taf))
                .or_else(|| read_body(&tokens, at, taf))
        };
        let read = read.unwrap_or(Read {
            element: Element::NotDecoded,
            lines: vec![],
            source: "",
            taken: 1,
            joins: false,
        });
        match read.element {
            Element::Station => out.station = Some(group.to_owned()),
            Element::Time => out.time = time(group),
            _ => {}
        }
        let taken = read.taken;
        add(&mut out, &tokens[at..at + taken], read);
        at += taken;
    }
    out
}

/// The altimeter setting a METAR gives, if it has one the table reads.
pub fn altimeter(metar: &str) -> Option<Altimeter> {
    let mut seen_rmk = false;
    metar.split_whitespace().find_map(|g| {
        if g == "RMK" {
            seen_rmk = true;
        }
        if seen_rmk {
            None
        } else {
            altimeter_group(g).map(|(a, _, _)| a)
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lines(d: &Decoded, e: Element) -> Vec<String> {
        d.rows
            .iter()
            .filter(|r| r.element == e)
            .flat_map(|r| r.lines.clone())
            .collect()
    }

    #[test]
    fn decodes_a_us_metar_as_the_board_shows_it() {
        let d = decode_metar("KSEA 121753Z 18008KT 10SM FEW040 BKN250 09/04 A3012");
        assert_eq!(d.station.as_deref(), Some("KSEA"));
        assert_eq!(d.time, Some((12, 17, 53)));
        assert_eq!(lines(&d, Element::Wind), ["180° at 8 kt"]);
        assert_eq!(lines(&d, Element::Visibility), ["10 statute miles"]);
        assert_eq!(
            lines(&d, Element::Clouds),
            ["Few 4,000 ft", "Broken 25,000 ft"]
        );
        assert_eq!(lines(&d, Element::Temperature), ["9 °C · dew point 4 °C"]);
        assert_eq!(lines(&d, Element::Altimeter), ["30.12 inHg"]);
        // Both cloud groups belong to one row, and every group points at its row.
        let clouds = d
            .rows
            .iter()
            .position(|r| r.element == Element::Clouds)
            .unwrap();
        assert_eq!(d.groups.iter().filter(|(_, r)| *r == clouds).count(), 2);
        assert_eq!(d.groups.len(), 8);
    }

    #[test]
    fn decodes_international_style() {
        let d = decode_metar(
            "METAR EHAM 132325Z 22004KT 200V270 9999 -SHRA FEW018 SCT035TCU 04/03 Q1018 NOSIG",
        );
        assert_eq!(
            lines(&d, Element::Wind),
            ["220° at 4 kt", "Varying between 200° and 270°"]
        );
        assert_eq!(lines(&d, Element::Visibility), ["10 km or more"]);
        assert_eq!(lines(&d, Element::Weather), ["Light showers of rain"]);
        assert_eq!(
            lines(&d, Element::Clouds),
            ["Few 1,800 ft", "Scattered 3,500 ft towering cumulus"]
        );
        assert_eq!(lines(&d, Element::Altimeter), ["1018 hPa"]);
        assert_eq!(
            lines(&d, Element::NoChange),
            ["No significant change expected"]
        );
        assert_eq!(lines(&d, Element::NotDecoded).len(), 0);
    }

    #[test]
    fn decodes_cavok_vrb_gusts_minus_temperatures_mps_and_auto() {
        let d = decode_metar("METAR ENGM 140120Z VRB02KT CAVOK M04/M07 Q1003");
        assert_eq!(lines(&d, Element::Wind), ["Variable at 2 kt"]);
        assert_eq!(lines(&d, Element::Temperature), ["-4 °C · dew point -7 °C"]);
        assert_eq!(
            d.rows
                .iter()
                .filter(|r| r.element == Element::Cavok)
                .count(),
            1
        );
        let d = decode_metar("METAR BIKF 140130Z 36027G38KT 9999 BKN033 00/M05 Q1003");
        assert_eq!(lines(&d, Element::Wind), ["360° at 27 kt, gusting 38 kt"]);
        let d = decode_metar("METAR ZBAA 140130Z 35002MPS 320V030 5000 BR NSC 07/03 Q1021 NOSIG");
        assert_eq!(
            lines(&d, Element::Wind),
            ["350° at 2 m/s", "Varying between 320° and 30°"]
        );
        assert_eq!(lines(&d, Element::Visibility), ["5 km"]);
        assert_eq!(lines(&d, Element::Weather), ["Mist"]);
        assert_eq!(lines(&d, Element::Clouds), ["No significant cloud"]);
        let d = decode_metar("METAR EGLL 140150Z COR AUTO 00000KT 9999 NCD M01/M01 Q1019 NOSIG");
        assert_eq!(lines(&d, Element::Wind), ["Calm"]);
        assert_eq!(lines(&d, Element::Auto), ["Automated report, no observer"]);
        assert_eq!(lines(&d, Element::Clouds), ["No cloud detected"]);
    }

    #[test]
    fn decodes_us_fractions_vertical_visibility_and_remarks() {
        let d = decode_metar(
            "KXYZ 141153Z 00000KT 1 1/2SM BR VV004 M01/M02 A2992 RMK AO2 SLP132 T10111017",
        );
        assert_eq!(lines(&d, Element::Visibility), ["1 1/2 statute miles"]);
        assert_eq!(
            lines(&d, Element::Clouds),
            ["Sky hidden, vertical visibility 400 ft"]
        );
        assert_eq!(
            lines(&d, Element::Remarks),
            [
                "Remarks follow",
                "Automated station with a precipitation discriminator",
                "Sea level pressure 1013.2 hPa",
                "Temperature -1.1 °C · dew point -1.7 °C"
            ]
        );
        assert_eq!(
            decode_metar("KXYZ 141153Z 00000KT M1/4SM FG 00/00 A2992").rows[3].lines,
            ["less than 1/4 statute mile"]
        );
        assert_eq!(
            decode_metar("KXYZ 141153Z 00000KT P6SM SKC 00/00 A2992").rows[3].lines,
            ["more than 6 statute miles"]
        );
    }

    #[test]
    fn shows_what_the_table_does_not_cover_as_printed_never_a_guess() {
        let d = decode_metar(
            "METAR UUEE 140130Z 27007MPS 9999 -RA OVC009 04/02 Q0993 R24L/290345 NOSIG",
        );
        let not: Vec<_> = d
            .rows
            .iter()
            .filter(|r| r.element == Element::NotDecoded)
            .collect();
        assert_eq!(not.len(), 1);
        assert_eq!(not[0].raw, ["R24L/290345"]);
        assert_eq!(not[0].lines.len(), 0);
        // US remarks the table does not cover stay as printed too.
        let d =
            decode_metar("KSFO 140156Z 24004KT 10SM FEW017 14/11 A2968 RMK AO2 RAB25E48 SLP049");
        let not: Vec<_> = d
            .rows
            .iter()
            .filter(|r| r.element == Element::NotDecoded)
            .flat_map(|r| r.raw.clone())
            .collect();
        assert_eq!(not, ["RAB25E48"]);
    }

    #[test]
    fn decodes_taf_change_groups() {
        let d = decode_taf(
            "TAF EDDF 132300Z 1400/1506 VRB03KT CAVOK TEMPO 1402/1409 3000 BR BKN006 PROB30 TEMPO 1403/1408 0800 FZFG BKN001 BECMG 1409/1411 09005KT",
        );
        assert_eq!(
            lines(&d, Element::Validity),
            ["Day 14 00:00 to day 15 06:00 UTC"]
        );
        let changes: Vec<_> = d
            .rows
            .iter()
            .filter(|r| r.element == Element::Change)
            .collect();
        assert_eq!(changes.len(), 3);
        assert_eq!(
            changes[0].lines,
            ["Temporarily", "Day 14 02:00 to day 14 09:00 UTC"]
        );
        assert_eq!(changes[1].raw, ["PROB30", "TEMPO", "1403/1408"]);
        assert_eq!(changes[1].lines[0], "30% probability, temporarily");
        assert_eq!(lines(&d, Element::Weather), ["Mist", "Freezing fog"]);
        let d = decode_taf(
            "TAF BIKF 140113Z 1403/1503 01020G30KT 9999 -SHSN SCT015 BKN025 TX01/1403Z TNM03/1421Z BECMG 1405/1407 01020KT NSW FM140800 30008KT P6SM",
        );
        assert_eq!(
            lines(&d, Element::TemperatureExtreme),
            [
                "Highest 1 °C, day 14 03:00 UTC",
                "Lowest -3 °C, day 14 21:00 UTC"
            ]
        );
        assert_eq!(lines(&d, Element::NoWeather), ["No significant weather"]);
        assert!(lines(&d, Element::Change).contains(&"From day 14 08:00 UTC".to_owned()));
    }

    #[test]
    fn each_row_names_where_its_code_is_defined_for_its_report() {
        let source = |d: &Decoded, e: Element| {
            d.rows
                .iter()
                .find(|r| r.element == e)
                .map(|r| r.source.clone())
        };
        let m = decode_metar("KSFO 140156Z 24004KT 1 1/2SM RERA CLR 14/11 A2968 RMK AO2 SLP049");
        assert_eq!(source(&m, Element::Wind).as_deref(), Some("FM 15 · 15.5"));
        assert_eq!(
            source(&m, Element::Visibility).as_deref(),
            Some("US · 3.1.5.6")
        );
        assert_eq!(
            source(&m, Element::Weather).as_deref(),
            Some("FM 15 · 15.13.2")
        );
        assert_eq!(source(&m, Element::Clouds).as_deref(), Some("US · 3.1.5.9"));
        assert_eq!(
            source(&m, Element::Altimeter).as_deref(),
            Some("US · 3.1.5.11")
        );
        let t = decode_taf(
            "TAF AMD EDDF 132300Z 1400/1506 VRB03KT 9999 NSW BKN006 PROB30 1403/1408 0800 FZFG TX09/1414Z",
        );
        assert_eq!(source(&t, Element::Kind).as_deref(), Some("FM 51 · 51.1"));
        assert_eq!(
            source(&t, Element::Validity).as_deref(),
            Some("FM 51 · 51.1.4")
        );
        assert_eq!(source(&t, Element::Wind).as_deref(), Some("FM 51 · 51.3"));
        assert_eq!(
            source(&t, Element::NoWeather).as_deref(),
            Some("FM 51 · 51.5.2")
        );
        assert_eq!(source(&t, Element::Change).as_deref(), Some("FM 51 · 51.9"));
        assert_eq!(
            source(&t, Element::TemperatureExtreme).as_deref(),
            Some("FM 51 · 51.10")
        );
        assert!(t.rows.iter().any(|r| r.source == "FM 51 · 51.11"), "AMD");
    }

    #[test]
    fn reads_the_altimeter_in_either_style_and_ignores_remarks() {
        assert_eq!(
            altimeter("KSEA 121753Z 18008KT 10SM 09/04 A3012")
                .unwrap()
                .value,
            "30.12"
        );
        let q = altimeter("EHAM 132325Z 22004KT 9999 04/03 Q1018 NOSIG").unwrap();
        assert_eq!((q.value.as_str(), q.unit.as_str()), ("1018", "hPa"));
        assert_eq!(altimeter("KXYZ 141153Z 00000KT 10SM 00/00 RMK A3012"), None);
    }

    #[test]
    fn every_real_report_in_the_fixtures_decodes_without_a_guess() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/weather");
        let metars = std::fs::read_to_string(dir.join("metar.txt")).unwrap();
        let mut not = vec![];
        for line in metars.lines().filter(|l| !l.trim().is_empty()) {
            let d = decode_metar(line);
            assert!(d.station.is_some() && d.time.is_some(), "{line}");
            // Every group is accounted for, decoded or shown as printed.
            assert_eq!(d.groups.len(), line.split_whitespace().count(), "{line}");
            assert!(d.rows.iter().any(|r| r.element == Element::Wind), "{line}");
            assert!(altimeter(line).is_some(), "{line}");
            let rmk = d
                .rows
                .iter()
                .position(|r| r.element == Element::Remarks)
                .unwrap_or(d.rows.len());
            not.extend(
                d.rows[..rmk]
                    .iter()
                    .filter(|r| r.element == Element::NotDecoded)
                    .flat_map(|r| r.raw.clone()),
            );
        }
        // Before the remarks, only runway state groups (outside the table) stay not decoded.
        assert!(not.iter().all(|g| g.starts_with("R24")), "{not:?}");
        // TAFs: one per record, continuation lines indented.
        let tafs = std::fs::read_to_string(dir.join("taf.txt")).unwrap();
        let mut reports: Vec<String> = vec![];
        for line in tafs.lines() {
            if line.starts_with("TAF") {
                reports.push(line.to_owned());
            } else if let Some(last) = reports.last_mut() {
                last.push(' ');
                last.push_str(line.trim());
            }
        }
        assert_eq!(reports.len(), 12);
        for r in &reports {
            let d = decode_taf(r);
            assert!(d.station.is_some(), "{r}");
            assert!(d.rows.iter().any(|x| x.element == Element::Validity), "{r}");
            assert_eq!(d.groups.len(), r.split_whitespace().count(), "{r}");
        }
    }
}
