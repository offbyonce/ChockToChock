// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Reading saved data (decision 0012). Each saved type carries a format number. An older format
//! is migrated forward step by step; a newer or unreadable one is reported, never discarded, so
//! the interface can keep it, tell the pilot and offer a copy.

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use ts_rs::TS;

use crate::flight::{FLIGHT_FORMAT, Flight};
#[cfg(test)]
use crate::settings::TextSize;
use crate::settings::{SETTINGS_FORMAT, Settings};

/// What reading saved data found.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, TS)]
#[serde(tag = "state", rename_all = "snake_case")]
#[ts(export)]
pub enum Loaded<T> {
    /// Read, and migrated forward if it was older (`migratedFrom` names the old format).
    Ready {
        value: T,
        #[serde(rename = "migratedFrom")]
        migrated_from: Option<u32>,
    },
    /// Written by a newer version of the app. Keep it; this version cannot read it.
    Newer { format: u32 },
    /// Not readable as this type. Keep it and tell the pilot why.
    Unreadable { reason: String },
}

/// Turns the saved shape of format `n` into the shape of format `n + 1`.
pub(crate) type Migration = fn(Value) -> Result<Value, String>;

/// Migrations for [`Flight`]: entry `i` turns format `i + 1` into format `i + 2`.
const FLIGHT_MIGRATIONS: &[Migration] = &[flight_1_to_2];

/// Format 2 adds the steps a panel state ticked or skipped (none yet in an older flight) and new
/// fill fields (destination QNH, arrival runway, STAR, alternate, tower frequency, thrust
/// rating), which are only new keys. Nothing older changes.
fn flight_1_to_2(value: Value) -> Result<Value, String> {
    value.as_object().ok_or("the flight is not an object")?;
    Ok(value)
}
/// Migrations for [`Settings`], indexed the same way.
const SETTINGS_MIGRATIONS: &[Migration] = &[
    settings_1_to_2,
    settings_2_to_3,
    settings_3_to_4,
    settings_4_to_5,
];

/// Format 5 adds the Flight pages' Descriptions On/Off, On to start with.
fn settings_4_to_5(mut value: Value) -> Result<Value, String> {
    let object = value.as_object_mut().ok_or("settings are not an object")?;
    object.insert("descriptions".into(), Value::from(true));
    Ok(value)
}

/// Format 4 replaces Map in steps with Panel preview in steps (round 6): Off stays Off, every
/// other choice becomes On.
fn settings_3_to_4(mut value: Value) -> Result<Value, String> {
    let object = value.as_object_mut().ok_or("settings are not an object")?;
    let mode = object.remove("mapInSteps");
    object.remove("mapBeforeHide");
    let off = mode.as_ref().and_then(Value::as_str) == Some("off");
    object.insert("panelPreview".into(), Value::from(!off));
    Ok(value)
}

/// Format 3 replaces the map On/Off with Map in steps: On becomes Always, Off stays Off.
fn settings_2_to_3(mut value: Value) -> Result<Value, String> {
    let object = value.as_object_mut().ok_or("settings are not an object")?;
    let on = object
        .remove("flightDeckMap")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let mode = if on { "always" } else { "off" };
    object.insert("mapInSteps".into(), Value::from(mode));
    object.insert("mapBeforeHide".into(), Value::from(mode));
    Ok(value)
}

/// Format 2 adds Text and controls size, standard for settings saved before it.
fn settings_1_to_2(mut value: Value) -> Result<Value, String> {
    let object = value.as_object_mut().ok_or("settings are not an object")?;
    object.insert("textSize".into(), Value::from("100"));
    Ok(value)
}

pub(crate) fn load_with<T: DeserializeOwned>(
    json: &str,
    current: u32,
    migrations: &[Migration],
) -> Loaded<T> {
    let mut value: Value = match serde_json::from_str(json) {
        Ok(value) => value,
        Err(e) => {
            return Loaded::Unreadable {
                reason: format!("not JSON: {e}"),
            };
        }
    };
    let Some(format) = value
        .get("format")
        .and_then(Value::as_u64)
        .and_then(|f| u32::try_from(f).ok())
    else {
        return Loaded::Unreadable {
            reason: "no format number".into(),
        };
    };
    if format > current {
        return Loaded::Newer { format };
    }
    let first = usize::try_from(format.saturating_sub(1)).unwrap_or(usize::MAX);
    let needed = usize::try_from(current - format).unwrap_or(usize::MAX);
    if format == 0 || migrations.len() < first.saturating_add(needed) {
        return Loaded::Unreadable {
            reason: format!("format {format} is not one this app knows"),
        };
    }
    for (step, migrate) in migrations.iter().enumerate().skip(first).take(needed) {
        value = match migrate(value) {
            Ok(next) => next,
            Err(reason) => {
                return Loaded::Unreadable {
                    reason: format!("migrating from format {}: {reason}", step + 1),
                };
            }
        };
        if let Some(object) = value.as_object_mut() {
            object.insert("format".into(), Value::from(step + 2));
        }
    }
    match serde_json::from_value(value) {
        Ok(value) => Loaded::Ready {
            value,
            migrated_from: (format < current).then_some(format),
        },
        Err(e) => Loaded::Unreadable {
            reason: e.to_string(),
        },
    }
}

/// Read a saved flight.
pub fn load_flight(json: &str) -> Loaded<Flight> {
    load_with(json, FLIGHT_FORMAT, FLIGHT_MIGRATIONS)
}

/// Read the saved SimBrief plan (decision 0019).
pub fn load_plan(json: &str) -> Loaded<crate::SavedPlan> {
    load_with(json, crate::PLAN_FORMAT, &[])
}

/// Read saved settings.
pub fn load_settings(json: &str) -> Loaded<Settings> {
    load_with(json, SETTINGS_FORMAT, SETTINGS_MIGRATIONS)
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;
    use serde_json::json;

    use super::*;

    #[test]
    fn a_format_1_flight_loads_as_format_2_unchanged() {
        let pack = crate::test_support::ifly_pack();
        let mut flight = crate::new_flight(&pack);
        flight
            .values
            .insert(checklist_core::FillField::Qnh, "29.92".into());
        let mut old = serde_json::to_value(&flight).unwrap();
        old["format"] = json!(1);
        let Loaded::Ready {
            value,
            migrated_from,
        } = load_flight(&old.to_string())
        else {
            panic!("did not load");
        };
        assert_eq!(migrated_from, Some(1));
        assert_eq!(value.format, 2);
        assert_eq!(
            value.values, flight.values,
            "the departure QNH stays; the destination's is new"
        );
        assert!(value.state_ticks.is_empty());
    }

    #[test]
    fn current_settings_read_back() {
        let settings = Settings::default();
        let json = serde_json::to_string(&settings).unwrap();
        assert_eq!(
            load_settings(&json),
            Loaded::Ready {
                value: settings,
                migrated_from: None
            }
        );
    }

    #[test]
    fn settings_from_format_1_get_the_standard_text_size() {
        let mut old = serde_json::to_value(Settings::default()).unwrap();
        let object = old.as_object_mut().unwrap();
        object.remove("textSize");
        object.insert("format".into(), json!(1));
        object.insert("dimming".into(), json!(2));
        let Loaded::Ready {
            value,
            migrated_from,
        } = load_settings(&old.to_string())
        else {
            panic!("format 1 settings did not load");
        };
        assert_eq!(migrated_from, Some(1));
        assert_eq!(value.text_size, TextSize::Standard);
        assert_eq!(value.dimming, 2);
        assert_eq!(value.format, SETTINGS_FORMAT);
    }

    #[test]
    fn settings_from_format_2_turn_the_map_switch_into_the_panel_preview() {
        for (on, preview) in [(true, true), (false, false)] {
            let mut old = serde_json::to_value(Settings::default()).unwrap();
            let object = old.as_object_mut().unwrap();
            object.remove("panelPreview");
            object.insert("flightDeckMap".into(), json!(on));
            object.insert("format".into(), json!(2));
            let Loaded::Ready {
                value,
                migrated_from,
            } = load_settings(&old.to_string())
            else {
                panic!("format 2 settings did not load");
            };
            assert_eq!(migrated_from, Some(2));
            assert_eq!(value.panel_preview, preview);
        }
    }

    #[test]
    fn settings_from_format_3_keep_off_and_turn_every_other_map_choice_on() {
        for (mode, preview) in [
            ("always", true),
            ("new_panel", true),
            ("on_request", true),
            ("off", false),
        ] {
            let mut old = serde_json::to_value(Settings::default()).unwrap();
            let object = old.as_object_mut().unwrap();
            object.remove("panelPreview");
            object.insert("mapInSteps".into(), json!(mode));
            object.insert("mapBeforeHide".into(), json!("new_panel"));
            object.insert("format".into(), json!(3));
            let Loaded::Ready {
                value,
                migrated_from,
            } = load_settings(&old.to_string())
            else {
                panic!("format 3 settings did not load");
            };
            assert_eq!(migrated_from, Some(3));
            assert_eq!(value.panel_preview, preview, "{mode}");
        }
    }

    #[test]
    fn settings_from_format_4_get_descriptions_on() {
        let mut old = serde_json::to_value(Settings::default()).unwrap();
        old["format"] = json!(4);
        old.as_object_mut().unwrap().remove("descriptions");
        let Loaded::Ready {
            value,
            migrated_from,
        } = load_settings(&old.to_string())
        else {
            panic!("not ready");
        };
        assert_eq!(migrated_from, Some(4));
        assert!(value.descriptions);
    }

    #[test]
    fn newer_and_unreadable_data_are_reported_not_dropped() {
        assert_eq!(
            load_settings(r#"{"format": 99}"#),
            Loaded::Newer { format: 99 }
        );
        assert!(matches!(
            load_settings("not json"),
            Loaded::Unreadable { .. }
        ));
        assert!(matches!(
            load_settings(r#"{"theme": "dark"}"#),
            Loaded::Unreadable { .. }
        ));
        assert!(matches!(
            load_flight(r#"{"format": 0}"#),
            Loaded::Unreadable { .. }
        ));
        // The current format with a missing field cannot be read, so it is kept, not reset.
        assert!(matches!(
            load_flight(r#"{"format": 1}"#),
            Loaded::Unreadable { .. }
        ));
    }

    /// A made-up type at format 3, to test the migration chain before any real migration exists.
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Demo {
        format: u32,
        name: String,
        lights: bool,
    }

    fn rename_title(mut v: Value) -> Result<Value, String> {
        let title = v["title"].take();
        let object = v.as_object_mut().ok_or("not an object")?;
        object.remove("title");
        object.insert("name".into(), title);
        Ok(v)
    }

    fn add_lights(mut v: Value) -> Result<Value, String> {
        v.as_object_mut()
            .ok_or("not an object")?
            .insert("lights".into(), json!(true));
        Ok(v)
    }

    #[test]
    fn older_formats_migrate_forward_step_by_step() {
        let chain: &[Migration] = &[rename_title, add_lights];
        let from_one = load_with::<Demo>(r#"{"format": 1, "title": "A"}"#, 3, chain);
        let expected = Demo {
            format: 3,
            name: "A".into(),
            lights: true,
        };
        assert_eq!(
            from_one,
            Loaded::Ready {
                value: expected,
                migrated_from: Some(1)
            }
        );
        let from_two = load_with::<Demo>(r#"{"format": 2, "name": "B"}"#, 3, chain);
        assert!(matches!(
            from_two,
            Loaded::Ready {
                migrated_from: Some(2),
                ..
            }
        ));
        let broken = load_with::<Demo>(
            r#"{"format": 1, "title": "A", "name": "x"}"#,
            3,
            &[|_| Err("bad".into()), add_lights],
        );
        assert!(matches!(broken, Loaded::Unreadable { reason } if reason.contains("format 1")));
    }
}
