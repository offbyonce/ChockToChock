// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! JSON Schemas generated from the Rust types. The committed copies in `schemas/`
//! give pack authors editor autocomplete; a test fails if they drift from the code.

use checklist_core::{
    AliasesFile, BriefingsFile, CduFile, Checklist, ControlsFile, FaultsFile, GlossaryFile,
    LightsFile, NotesFile, Panel, PhasesFile, StartStatesFile,
};

use crate::manifest::PackManifest;

/// `(file name, pretty JSON)` for every schema we publish.
pub fn all() -> Vec<(&'static str, String)> {
    vec![
        ("aliases.schema.json", render::<AliasesFile>()),
        ("briefings.schema.json", render::<BriefingsFile>()),
        ("cdu.schema.json", render::<CduFile>()),
        ("checklist.schema.json", render::<Checklist>()),
        ("controls.schema.json", render::<ControlsFile>()),
        ("faults.schema.json", render::<FaultsFile>()),
        ("glossary.schema.json", render::<GlossaryFile>()),
        ("lights.schema.json", render::<LightsFile>()),
        ("notes.schema.json", render::<NotesFile>()),
        ("pack.schema.json", render::<PackManifest>()),
        ("panel.schema.json", render::<Panel>()),
        ("phases.schema.json", render::<PhasesFile>()),
        ("start-states.schema.json", render::<StartStatesFile>()),
    ]
}

fn render<T: schemars::JsonSchema>() -> String {
    let schema = schemars::schema_for!(T);
    // Serialising a schema built by schemars cannot fail.
    let mut json = serde_json::to_string_pretty(&schema).unwrap_or_default();
    json.push('\n');
    json
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    /// Fails when the committed schema files no longer match the Rust types.
    /// Run `just schemas` to regenerate them.
    #[test]
    fn committed_schemas_match_the_code() {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../schemas");
        for (name, expected) in super::all() {
            let committed = std::fs::read_to_string(dir.join(name)).unwrap_or_default();
            assert!(
                committed == expected,
                "schemas/{name} is out of date; run `just schemas`"
            );
        }
    }
}
