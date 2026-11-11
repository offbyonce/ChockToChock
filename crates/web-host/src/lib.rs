// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! The web host: thin WebAssembly glue between the web interface and `app-core`, as the Tauri
//! shell is for desktop (decision 0010). Each export calls one `app-core` function and holds no
//! logic of its own.
//!
//! Values cross the boundary as JSON strings, checked against the ts-rs types on the other side.
//! Every export answers with a [`Reply`], so errors arrive as a typed [`AppError`], not a thrown
//! string.

use std::cell::RefCell;
use std::collections::BTreeMap;

use app_core::{Action, AppError, Flight, Settings};
use pack::FlatPack;
use serde::Serialize;
use serde::de::DeserializeOwned;
use ts_rs::TS;
use wasm_bindgen::prelude::wasm_bindgen;

thread_local! {
    /// Packs loaded so far, by id. Kept here so a pack crosses the boundary once (decision 0012).
    static PACKS: RefCell<BTreeMap<String, FlatPack>> = const { RefCell::new(BTreeMap::new()) };
}

fn with_pack<T>(id: &str, f: impl FnOnce(&FlatPack) -> Result<T, AppError>) -> Result<T, AppError> {
    PACKS.with_borrow(|packs| match packs.get(id) {
        Some(pack) => f(pack),
        None => Err(AppError::PackInvalid(format!("pack {id} is not loaded"))),
    })
}

/// Reads JSON the interface sent; a failure is a bug on that side, so it is an invalid action.
fn read<T: DeserializeOwned>(what: &str, json: &str) -> Result<T, AppError> {
    serde_json::from_str(json)
        .map_err(|e| AppError::InvalidAction(format!("could not read the {what}: {e}")))
}

/// What every export answers: `{ "ok": value }` or `{ "error": AppError }`.
#[derive(Debug, Serialize, TS)]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub enum Reply<T> {
    Ok(T),
    Error(AppError),
}

/// Sent if an answer cannot be encoded, which only a bug could cause.
const UNENCODABLE: &str =
    r#"{"error":{"kind":"PackInvalid","message":"the answer could not be encoded"}}"#;

fn reply<T: Serialize>(result: Result<T, AppError>) -> String {
    let reply = match result {
        Ok(value) => Reply::Ok(value),
        Err(error) => Reply::Error(error),
    };
    serde_json::to_string(&reply).unwrap_or_else(|_| UNENCODABLE.to_owned())
}

/// Basic facts for the About box and the window title.
#[wasm_bindgen]
pub fn app_info() -> String {
    reply(Ok(app_core::app_info()))
}

/// The bundled format-demo checklist.
#[wasm_bindgen]
pub fn example_checklist() -> String {
    reply(app_core::example_checklist())
}

/// Read a flattened pack, as written by `pack build`, and keep it for the calls below.
#[wasm_bindgen]
pub fn load_pack(json: &str) -> String {
    reply(app_core::load_pack(json).inspect(|pack| {
        PACKS.with_borrow_mut(|packs| packs.insert(pack.manifest.id.clone(), pack.clone()));
    }))
}

/// A new flight for a loaded pack.
#[wasm_bindgen]
pub fn new_flight(pack: &str) -> String {
    reply(with_pack(pack, |p| Ok(app_core::new_flight(p))))
}

/// Apply an action to a flight and return the changed flight.
#[wasm_bindgen]
pub fn apply(pack: &str, flight: &str, action: &str) -> String {
    reply((|| {
        let flight: Flight = read("flight", flight)?;
        let action: Action = read("action", action)?;
        with_pack(pack, |p| app_core::apply(p, flight, action))
    })())
}

/// The phase rail for a flight.
#[wasm_bindgen]
pub fn rail(pack: &str, flight: &str) -> String {
    reply(
        read::<Flight>("flight", flight)
            .and_then(|flight| with_pack(pack, |p| Ok(app_core::rail(p, &flight)))),
    )
}

/// Read a saved flight: ready (migrated if older), newer, or unreadable.
#[wasm_bindgen]
pub fn load_flight(json: &str) -> String {
    reply(Ok(app_core::load_flight(json)))
}

/// Read saved settings: ready (migrated if older), newer, or unreadable.
#[wasm_bindgen]
pub fn load_settings(json: &str) -> String {
    reply(Ok(app_core::load_settings(json)))
}

/// Settings for a first visit.
#[wasm_bindgen]
pub fn default_settings() -> String {
    reply(Ok(Settings::default()))
}

/// The address of the latest SimBrief plan for a username or Pilot ID.
#[wasm_bindgen]
pub fn plan_url(user: &str) -> String {
    reply(app_core::plan_url(user))
}

/// Read a SimBrief plan: fetched JSON, or a saved JSON or XML file.
#[wasm_bindgen]
pub fn parse_plan(text: &str) -> String {
    reply(app_core::parse_plan(text))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn json(text: &str) -> serde_json::Value {
        serde_json::from_str(text).unwrap()
    }

    #[test]
    fn answers_are_ok_or_a_tagged_error() {
        assert_eq!(json(&app_info())["ok"]["name"], "Chock to Chock");
        assert_eq!(json(&example_checklist())["ok"]["id"], "before-start");
        assert_eq!(
            json(&plan_url(" ")),
            json(
                r#"{"error":{"kind":"FlightPlan","message":"enter a SimBrief username or Pilot ID"}}"#
            )
        );
        assert_eq!(json(&load_pack("{}"))["error"]["kind"], "PackInvalid");
        assert_eq!(json(&new_flight("nope"))["error"]["kind"], "PackInvalid");
        assert_eq!(
            json(&apply("x", "{}", "{}"))["error"]["kind"],
            "InvalidAction"
        );
        assert_eq!(json(&default_settings())["ok"]["theme"], "system");
        assert_eq!(
            json(&load_settings(r#"{"format":99}"#))["ok"]["state"],
            "newer"
        );
        assert_eq!(
            json(&parse_plan(
                "<OFP><origin><icao_code>EGLL</icao_code></origin></OFP>"
            ))["ok"]["origin"],
            "EGLL"
        );
    }

    #[test]
    fn a_loaded_pack_gives_flights_actions_and_a_rail() {
        let dir =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../packs/example-trainer");
        let pack = FlatPack::from(pack::load_pack_dir(&dir).unwrap());
        assert!(json(&load_pack(&serde_json::to_string(&pack).unwrap()))["ok"].is_object());
        let flight = json(&new_flight("example-trainer"))["ok"].to_string();
        let action = r#"{"type":"tick","step":"example-trainer.before-start.battery","at":7}"#;
        let ticked = json(&apply("example-trainer", &flight, action))["ok"].clone();
        assert_eq!(
            ticked["session"]["done"]["example-trainer.before-start.battery"],
            7
        );
        assert!(json(&rail("example-trainer", &ticked.to_string()))["ok"]["stages"].is_array());
    }
}
