// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! SimBrief plans. The host fetches the text from the address given here, or reads a saved
//! file, and hands it back to be read.

use simbrief::Ofp;

use crate::AppError;

/// The address of the latest plan for a SimBrief username or Pilot ID.
pub fn plan_url(user: &str) -> Result<String, AppError> {
    Ok(simbrief::fetch_url(user)?)
}

/// Read a plan fetched from SimBrief, or a saved plan file in JSON or XML.
pub fn parse_plan(text: &str) -> Result<Ofp, AppError> {
    Ok(simbrief::parse_ofp(text)?)
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
    fn plans_read_and_errors_become_flight_plan_errors() {
        let plan = parse_plan(&fixture("ofp-sample.xml")).unwrap();
        assert_eq!(plan.origin.as_deref(), Some("EGLL"));
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
