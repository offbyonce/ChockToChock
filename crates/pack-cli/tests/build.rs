// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Runs the real `pack build` command.

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::process::Command;

    use pack::FlatPack;

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("pack-build-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    fn build(packs: &Path, out: &Path) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_pack"))
            .arg("build")
            .arg(packs)
            .arg(out)
            .output()
            .unwrap()
    }

    #[test]
    fn writes_one_flattened_file_per_pack() {
        let out = temp_dir("ok");
        let packs = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../packs");
        let output = build(&packs, &out);
        assert!(output.status.success(), "{output:?}");
        let text = std::fs::read_to_string(out.join("example-trainer.json")).unwrap();
        let flat: FlatPack = serde_json::from_str(&text).unwrap();
        assert_eq!(flat.manifest.id, "example-trainer");
        assert_eq!(flat.lists.len(), 3);
        assert!(out.join("ifly-737-max8.json").is_file());
        let index: Vec<pack::PackSummary> =
            serde_json::from_str(&std::fs::read_to_string(out.join("index.json")).unwrap())
                .unwrap();
        let ids: Vec<&str> = index.iter().map(|p| p.id.as_str()).collect();
        assert_eq!(ids, ["example-trainer", "ifly-737-max8"]);
        std::fs::remove_dir_all(out).unwrap();
    }

    #[test]
    fn a_pack_with_errors_is_not_written() {
        let dir = temp_dir("bad");
        let pack = dir.join("pack");
        std::fs::create_dir_all(&pack).unwrap();
        std::fs::write(
            pack.join("pack.json"),
            r#"{"id":"demo","name":"Demo","version":"0.1.0","schemaVersion":2,"kind":"library",
               "engine":">=0.1.0","license":"CC-BY-NC-SA-4.0","checklists":["a.yaml"]}"#,
        )
        .unwrap();
        // No basis and a step id outside the pack: two errors.
        std::fs::write(
            pack.join("a.yaml"),
            "id: a\ntitle: A\nkind: flow\nbasis: []\nsteps:\n  - id: other.p.a\n    text: T\n    expect: E\n",
        )
        .unwrap();
        let out = dir.join("out");
        let output = build(&pack, &out);
        assert_eq!(output.status.code(), Some(1), "{output:?}");
        assert!(!out.join("demo.json").exists());
        std::fs::remove_dir_all(dir).unwrap();
    }
}
