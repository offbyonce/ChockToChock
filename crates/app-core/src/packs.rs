// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Reading a flattened pack, the one JSON file per pack that `pack build` writes.

use pack::{FlatPack, SCHEMA_VERSION};

use crate::AppError;

/// Read a flattened pack. A pack newer than this app understands is refused.
pub fn load_pack(json: &str) -> Result<FlatPack, AppError> {
    let flat: FlatPack = serde_json::from_str(json)
        .map_err(|e| AppError::PackInvalid(format!("the pack file could not be read: {e}")))?;
    let version = flat.manifest.schema_version;
    if version > SCHEMA_VERSION {
        return Err(AppError::PackInvalid(format!(
            "pack {} uses pack format {version}; this app understands up to {SCHEMA_VERSION}",
            flat.manifest.id
        )));
    }
    Ok(flat)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    fn example_json() -> String {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../packs/example-trainer");
        serde_json::to_string(&FlatPack::from(pack::load_pack_dir(&dir).unwrap())).unwrap()
    }

    #[test]
    fn a_flattened_pack_loads() {
        let flat = load_pack(&example_json()).unwrap();
        assert_eq!(flat.manifest.id, "example-trainer");
        assert_eq!(flat.lists.len(), 3);
    }

    #[test]
    fn a_newer_or_broken_pack_is_refused() {
        let mut json: serde_json::Value = serde_json::from_str(&example_json()).unwrap();
        json["manifest"]["schemaVersion"] = (SCHEMA_VERSION + 1).into();
        let newer = json.to_string();
        let Err(AppError::PackInvalid(message)) = load_pack(&newer) else {
            panic!("a newer pack must be refused");
        };
        assert!(message.contains("understands up to"), "{message}");
        assert!(matches!(load_pack("{}"), Err(AppError::PackInvalid(_))));
    }
}
