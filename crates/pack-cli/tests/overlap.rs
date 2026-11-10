// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Runs the real `pack overlap` command.

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};
    use std::process::{Command, Output};

    const PHRASE: &str = "alpha bravo charlie delta echo foxtrot golf hotel";

    fn example_pack() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../packs/example-trainer")
    }

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("pack-cli-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn run(args: &[&str], refs_variable: Option<&Path>) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_pack"));
        command.args(args).env_remove("CHOCKTOCHOCK_OVERLAP_REFS");
        if let Some(refs) = refs_variable {
            command.env("CHOCKTOCHOCK_OVERLAP_REFS", refs);
        }
        command.output().unwrap()
    }

    fn text(output: &Output) -> String {
        format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    }

    /// A one-list copy of the example pack whose flow holds `line` as a note.
    fn pack_with_note(dir: &Path, line: &str) -> PathBuf {
        let pack = dir.join("pack");
        std::fs::create_dir_all(&pack).unwrap();
        std::fs::write(
            pack.join("pack.json"),
            r#"{"id":"demo","name":"Demo","version":"0.1.0","schemaVersion":2,"kind":"library",
               "engine":">=0.1.0","license":"CC-BY-NC-SA-4.0","checklists":["a.yaml"]}"#,
        )
        .unwrap();
        std::fs::write(
            pack.join("a.yaml"),
            format!(
                "id: a\ntitle: A\nkind: flow\nbasis: [x]\nsteps:\n  - id: demo.p.a\n    text: T\n    expect: E\n    note: {line}\n"
            ),
        )
        .unwrap();
        pack
    }

    fn pack_arg(pack: &Path) -> String {
        pack.to_string_lossy().into_owned()
    }

    #[test]
    fn without_a_folder_it_says_not_run_and_exits_cleanly() {
        let output = run(&["overlap", &pack_arg(&example_pack())], None);
        assert!(output.status.success(), "{}", text(&output));
        assert!(text(&output).contains("not run"), "{}", text(&output));
    }

    #[test]
    fn a_folder_inside_the_repo_is_refused() {
        let inside = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let output = run(&["overlap", &pack_arg(&example_pack())], Some(&inside));
        assert_eq!(output.status.code(), Some(2), "{}", text(&output));
        assert!(
            text(&output).contains("inside the repository"),
            "{}",
            text(&output)
        );
    }

    #[test]
    fn a_match_fails_and_prints_where_it_is() {
        let dir = temp_dir("match");
        let refs = dir.join("refs");
        std::fs::create_dir_all(&refs).unwrap();
        std::fs::write(refs.join("manual.txt"), PHRASE).unwrap();
        let pack = pack_with_note(&dir, PHRASE);
        let output = run(&["overlap", &pack_arg(&pack)], Some(&refs));
        let printed = text(&output);
        assert_eq!(output.status.code(), Some(1), "{printed}");
        assert!(printed.contains("a.yaml"), "{printed}");
        assert!(printed.contains("demo.p.a/note"), "{printed}");
        assert!(printed.contains("manual.txt"), "{printed}");
        assert!(printed.contains(PHRASE), "{printed}");
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn the_refs_flag_and_allow_list_work_and_clean_text_passes() {
        let dir = temp_dir("allow");
        let refs = dir.join("refs");
        std::fs::create_dir_all(&refs).unwrap();
        std::fs::write(refs.join("manual.txt"), PHRASE).unwrap();
        let pack = pack_with_note(&dir, PHRASE);
        let allow = dir.join("allow.txt");
        std::fs::write(&allow, format!("{PHRASE} | Standard phrase\n")).unwrap();
        let args = [
            "overlap",
            &pack_arg(&pack),
            "--refs",
            refs.to_str().unwrap(),
            "--allow",
            allow.to_str().unwrap(),
        ];
        let output = run(&args, None);
        assert!(output.status.success(), "{}", text(&output));
        assert!(text(&output).contains("1 allowed"), "{}", text(&output));

        let clean = pack_with_note(&dir.join("clean"), "Nothing in common here");
        let output = run(
            &[
                "overlap",
                &pack_arg(&clean),
                "--refs",
                refs.to_str().unwrap(),
            ],
            None,
        );
        assert!(output.status.success(), "{}", text(&output));
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn an_empty_reference_folder_is_an_error_not_a_pass() {
        let dir = temp_dir("empty");
        let output = run(&["overlap", &pack_arg(&example_pack())], Some(&dir));
        assert_eq!(output.status.code(), Some(2), "{}", text(&output));
        assert!(text(&output).contains("no .txt files"), "{}", text(&output));
        std::fs::remove_dir_all(dir).unwrap();
    }
}
