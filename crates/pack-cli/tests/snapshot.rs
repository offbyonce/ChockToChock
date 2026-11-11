// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Golden snapshots of the iFly pack as loaded for the app, so every content change shows up in
//! review. After an intended change, run `INSTA_UPDATE=always cargo nextest run -p pack-cli --test
//! snapshot` and read the diff. The snapshots hold pack content, so they carry its license
//! (CC BY-NC-SA 4.0, set in `REUSE.toml`).

#[cfg(test)]
mod tests {
    use std::path::Path;

    use pack::{FlatPack, PackContent, PackManifest};

    fn ifly() -> pack::Pack {
        let packs = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../packs");
        pack::load_pack_dir(&packs.join("ifly-737-max8")).unwrap()
    }

    /// Checks each part against the snapshot of the same name.
    fn assert_matches_snapshots(
        manifest: &PackManifest,
        lists: &[checklist_core::Checklist],
        content: &PackContent,
    ) {
        let mut settings = insta::Settings::clone_current();
        settings.set_snapshot_path(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/snapshots/ifly-737-max8"),
        );
        settings.set_prepend_module_to_snapshot(false);
        settings.set_omit_expression(true);
        settings.bind(|| {
            insta::assert_json_snapshot!("manifest", manifest);
            insta::assert_json_snapshot!("lists", lists);
            insta::assert_json_snapshot!("phases", content.phases);
            insta::assert_json_snapshot!("controls", content.controls);
            insta::assert_json_snapshot!("notes", content.notes);
            insta::assert_json_snapshot!("panels", content.panels);
            insta::assert_json_snapshot!("lights", content.lights);
            insta::assert_json_snapshot!("faults", content.faults);
            insta::assert_json_snapshot!("glossary", content.glossary);
            insta::assert_json_snapshot!("briefings", content.briefings);
            insta::assert_json_snapshot!("start-states", content.start_states);
            insta::assert_json_snapshot!("cdu", content.cdu);
            insta::assert_json_snapshot!("aliases", content.aliases);
        });
    }

    #[test]
    fn ifly_pack_matches_its_snapshots() {
        let pack = ifly();
        assert_matches_snapshots(&pack.manifest, &pack.checklists, &pack.content);
    }

    /// What `pack build` writes, read back, is the same pack the snapshots hold.
    #[test]
    fn the_flattened_ifly_pack_matches_its_snapshots() {
        let json = serde_json::to_string(&FlatPack::from(ifly())).unwrap();
        let flat: FlatPack = serde_json::from_str(&json).unwrap();
        assert_matches_snapshots(&flat.manifest, &flat.lists, &flat.content);
    }
}
