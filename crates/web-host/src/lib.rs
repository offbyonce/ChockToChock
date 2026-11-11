// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! The web host: thin WebAssembly glue between the web interface and `app-core`, as the Tauri
//! shell is for desktop (decision 0010). Each export calls one `app-core` function and holds no
//! logic of its own.
//!
//! Values cross the boundary as JSON strings, checked against the ts-rs types on the other side.
//! Every export answers with a [`Reply`], so errors arrive as a typed [`AppError`], not a thrown
//! string.

use app_core::AppError;
use serde::Serialize;
use ts_rs::TS;
use wasm_bindgen::prelude::wasm_bindgen;

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

/// Read a flattened pack, as written by `pack build`.
#[wasm_bindgen]
pub fn load_pack(json: &str) -> String {
    reply(app_core::load_pack(json))
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
        assert_eq!(
            json(&parse_plan(
                "<OFP><origin><icao_code>EGLL</icao_code></origin></OFP>"
            ))["ok"]["origin"],
            "EGLL"
        );
    }
}
