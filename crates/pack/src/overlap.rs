// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! The text-overlap check (decision 0007): flags any run of 8 or more words that a pack shares
//! with private reference files kept outside the repo. Reference text is held in memory only.

use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::path::{Path, PathBuf};

use serde::Serialize;
use serde_json::Value;

use crate::Pack;

/// The shortest run of matching words that counts as an overlap.
pub const MIN_RUN: usize = 8;

/// The environment variable that names the reference folder.
pub const REFS_VARIABLE: &str = "CHOCKTOCHOCK_OVERLAP_REFS";

/// Keys whose values are ids or fixed vocabulary, never prose.
const NOT_PROSE: &[&str] = &[
    "basis",
    "color",
    "controls",
    "equals",
    "explain",
    "field",
    "fill",
    "from",
    "highlight",
    "how",
    "id",
    "kind",
    "lists",
    "locator",
    "one_of",
    "opens",
    "option",
    "part",
    "phase",
    "pick",
    "quick",
    "route_legs",
    "status",
    "style",
    "to",
    "when_known",
    "zone",
];

#[derive(Debug, thiserror::Error)]
pub enum OverlapError {
    #[error("cannot read {path}: {source}")]
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error(
        "the reference folder {0} is inside the repository; references must be kept outside it"
    )]
    InsideRepo(PathBuf),
    #[error(
        "the reference folder {0} contains the repository; choose a folder holding only references"
    )]
    ContainsRepo(PathBuf),
    #[error("no .txt files found in {0}; the check would pass without comparing anything")]
    NoReferences(PathBuf),
    #[error("allow-list line {line}: {message}")]
    BadAllowLine { line: usize, message: String },
}

/// Lower-cased words with punctuation removed. Apostrophes join; other punctuation separates.
pub fn normalize(text: &str) -> Vec<String> {
    let cleaned: String = text
        .chars()
        .filter(|c| !matches!(c, '\'' | '\u{2019}'))
        .map(|c| if c.is_alphanumeric() { c } else { ' ' })
        .collect();
    cleaned
        .to_lowercase()
        .split_whitespace()
        .map(str::to_owned)
        .collect()
}

fn window_key(words: &[String]) -> u64 {
    let mut hasher = DefaultHasher::new();
    words.hash(&mut hasher);
    hasher.finish()
}

/// Every run of `MIN_RUN` words in the reference files, held in memory only.
#[derive(Debug, Default)]
pub struct References {
    windows: HashMap<u64, usize>,
    names: Vec<String>,
}

impl References {
    /// Adds one reference file's text.
    pub fn add(&mut self, name: impl Into<String>, text: &str) {
        let index = self.names.len();
        self.names.push(name.into());
        for window in normalize(text).windows(MIN_RUN) {
            self.windows.entry(window_key(window)).or_insert(index);
        }
    }

    pub fn file_count(&self) -> usize {
        self.names.len()
    }

    /// `(start, end, reference index)` for each longest run of `words` found in the references.
    fn runs(&self, words: &[String]) -> Vec<(usize, usize, usize)> {
        let mut runs: Vec<(usize, usize, usize)> = Vec::new();
        for (start, window) in words.windows(MIN_RUN).enumerate() {
            let Some(reference) = self.windows.get(&window_key(window)) else {
                continue;
            };
            match runs.last_mut() {
                Some(run) if run.1 == start + MIN_RUN - 1 => run.1 = start + MIN_RUN,
                _ => runs.push((start, start + MIN_RUN, *reference)),
            }
        }
        runs
    }
}

fn collect_txt(
    dir: &Path,
    out: &mut Vec<PathBuf>,
    skipped: &mut usize,
) -> Result<(), OverlapError> {
    let io = |source| OverlapError::Io {
        path: dir.to_path_buf(),
        source,
    };
    for entry in std::fs::read_dir(dir).map_err(io)? {
        let path = entry.map_err(io)?.path();
        if path.is_dir() {
            collect_txt(&path, out, skipped)?;
        } else if path
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("txt"))
        {
            out.push(path);
        } else {
            *skipped += 1;
        }
    }
    Ok(())
}

/// Reads every `.txt` file under `dir`. Returns the references and how many other files were
/// skipped. A folder with no text files is an error, so the check can never pass by accident.
pub fn load_references(dir: &Path) -> Result<(References, usize), OverlapError> {
    let mut files = Vec::new();
    let mut skipped = 0;
    collect_txt(dir, &mut files, &mut skipped)?;
    files.sort();
    if files.is_empty() {
        return Err(OverlapError::NoReferences(dir.to_path_buf()));
    }
    let mut references = References::default();
    for path in files {
        let bytes = std::fs::read(&path).map_err(|source| OverlapError::Io {
            path: path.clone(),
            source,
        })?;
        let name = path
            .strip_prefix(dir)
            .unwrap_or(&path)
            .to_string_lossy()
            .into_owned();
        references.add(name, &String::from_utf8_lossy(&bytes));
    }
    Ok((references, skipped))
}

/// The repositories that `paths` sit in, found by looking for `.git` in each path's ancestors.
pub fn repo_roots(paths: &[&Path]) -> Vec<PathBuf> {
    let mut roots = Vec::new();
    for path in paths {
        let Ok(path) = path.canonicalize() else {
            continue;
        };
        if let Some(root) = path.ancestors().find(|a| a.join(".git").exists())
            && !roots.iter().any(|r| r == root)
        {
            roots.push(root.to_path_buf());
        }
    }
    roots
}

/// Refuses a reference folder inside a repository, or one that contains a repository.
pub fn check_refs_location(refs: &Path, roots: &[PathBuf]) -> Result<PathBuf, OverlapError> {
    let refs = refs.canonicalize().map_err(|source| OverlapError::Io {
        path: refs.to_path_buf(),
        source,
    })?;
    for root in roots {
        if refs.starts_with(root) {
            return Err(OverlapError::InsideRepo(refs));
        }
        if root.starts_with(&refs) {
            return Err(OverlapError::ContainsRepo(refs));
        }
    }
    Ok(refs)
}

/// Runs that are allowed to match, such as standard phrases and switch names.
#[derive(Debug, Default)]
pub struct AllowList {
    runs: Vec<Vec<String>>,
}

impl AllowList {
    /// Reads lines of `run of words | reason`. Blank lines and `#` comments are ignored.
    pub fn parse(text: &str) -> Result<Self, OverlapError> {
        let mut runs = Vec::new();
        for (index, line) in text.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            let bad = |message: &str| OverlapError::BadAllowLine {
                line: index + 1,
                message: message.to_owned(),
            };
            let (run, reason) = trimmed
                .rsplit_once('|')
                .ok_or_else(|| bad("expected `run of words | reason`"))?;
            if reason.trim().is_empty() {
                return Err(bad("the reason is empty"));
            }
            let words = normalize(run);
            if words.len() < MIN_RUN {
                return Err(bad("the run is shorter than the check's 8 words"));
            }
            runs.push(words);
        }
        Ok(Self { runs })
    }

    /// True when `words` is an allowed run or part of one.
    pub fn allows(&self, words: &[String]) -> bool {
        self.runs
            .iter()
            .any(|run| run.windows(words.len().max(1)).any(|w| w == words))
    }
}

/// One run of words shared with a reference file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Overlap {
    /// The pack file, relative to the pack folder.
    pub file: String,
    /// The step or list id and field.
    pub at: String,
    /// The matching words. These are the pack's own text.
    pub words: Vec<String>,
    /// The name of a reference file holding the run.
    pub reference: String,
    /// True when the run spans several steps of a list rather than one field.
    pub across_steps: bool,
}

fn add_document(docs: &mut Vec<(String, Value)>, file: Option<&String>, data: &impl Serialize) {
    if let (Some(file), Ok(value)) = (file, serde_json::to_value(data)) {
        docs.push((file.clone(), value));
    }
}

/// Every prose-bearing file in the pack, as JSON. The alias table holds no prose.
fn documents(pack: &Pack) -> Vec<(String, Value)> {
    let m = &pack.manifest;
    let c = &pack.content;
    let mut docs = Vec::new();
    for (file, list) in m.checklists.iter().zip(&pack.checklists) {
        add_document(&mut docs, Some(file), list);
    }
    for (file, panel) in m.panels.iter().zip(&c.panels) {
        add_document(&mut docs, Some(file), panel);
    }
    add_document(&mut docs, m.phases.as_ref(), &c.phases);
    add_document(&mut docs, m.controls.as_ref(), &c.controls);
    add_document(&mut docs, m.notes.as_ref(), &c.notes);
    add_document(&mut docs, m.lights.as_ref(), &c.lights);
    add_document(&mut docs, m.faults.as_ref(), &c.faults);
    add_document(&mut docs, m.glossary.as_ref(), &c.glossary);
    add_document(&mut docs, m.briefings.as_ref(), &c.briefings);
    add_document(&mut docs, m.start_states.as_ref(), &c.start_states);
    add_document(&mut docs, m.cdu.as_ref(), &c.cdu);
    docs
}

fn holds_objects(value: &Value) -> bool {
    value
        .as_array()
        .is_some_and(|items| items.iter().any(Value::is_object))
}

fn walk(value: &Value, at: &str, visit: &mut impl FnMut(&str, &str)) {
    match value {
        Value::String(text) => visit(at, text),
        Value::Array(items) => {
            for (i, item) in items.iter().enumerate() {
                walk(item, &format!("{at}/{i}"), visit);
            }
        }
        Value::Object(fields) => {
            let here = fields.get("id").and_then(Value::as_str).unwrap_or(at);
            for (key, item) in fields {
                // `how` is an id on a step but a list of notes in the notes file.
                if !NOT_PROSE.contains(&key.as_str()) || holds_objects(item) {
                    walk(item, &format!("{here}/{key}"), visit);
                }
            }
        }
        _ => {}
    }
}

/// Compares the whole pack with the references: each prose field alone, then each list's step
/// text and expected responses joined in order, because checklist rows are short enough that a
/// copied list would pass a field-only check.
pub fn find_overlaps(pack: &Pack, references: &References) -> Vec<Overlap> {
    let mut found = Vec::new();
    for (file, doc) in documents(pack) {
        walk(&doc, "", &mut |at, text| {
            let words = normalize(text);
            for (start, end, reference) in references.runs(&words) {
                found.push(Overlap {
                    file: file.clone(),
                    at: at.trim_start_matches('/').to_owned(),
                    words: words[start..end].to_vec(),
                    reference: references.names[reference].clone(),
                    across_steps: false,
                });
            }
        });
    }
    for (file, list) in pack.manifest.checklists.iter().zip(&pack.checklists) {
        let mut words = Vec::new();
        let mut spans = Vec::new();
        for step in &list.steps {
            for text in [&step.text, &step.expect] {
                let start = words.len();
                words.extend(normalize(text));
                spans.push(start..words.len());
            }
        }
        for (start, end, reference) in references.runs(&words) {
            // A run inside one field has already been reported above.
            if !spans.iter().any(|s| s.start <= start && end <= s.end) {
                found.push(Overlap {
                    file: file.clone(),
                    at: format!("list {}", list.id),
                    words: words[start..end].to_vec(),
                    reference: references.names[reference].clone(),
                    across_steps: true,
                });
            }
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use checklist_core::{Checklist, ListKind, NotesFile, Step};
    use proptest::prelude::*;

    use super::*;
    use crate::manifest::{PackKind, PackManifest};
    use crate::{PackContent, SCHEMA_VERSION};

    const PHRASE: &str = "alpha bravo charlie delta echo foxtrot golf hotel";

    fn pack(steps: Vec<Step>) -> Pack {
        Pack {
            manifest: PackManifest {
                json_schema: None,
                id: "demo".into(),
                name: "Demo".into(),
                version: semver::Version::new(0, 1, 0),
                schema_version: SCHEMA_VERSION,
                kind: PackKind::Aircraft,
                engine: semver::VersionReq::parse(">=0.1").unwrap(),
                dependencies: std::collections::BTreeMap::default(),
                license: "CC-BY-NC-SA-4.0".into(),
                attribution: vec![],
                basis_tags: vec![],
                checklists: vec!["lists/a.yaml".into()],
                phases: None,
                controls: None,
                notes: Some("notes.yaml".into()),
                panels: vec![],
                lights: None,
                faults: None,
                glossary: None,
                briefings: None,
                start_states: None,
                cdu: None,
                aliases: None,
            },
            checklists: vec![Checklist {
                id: "list-a".into(),
                title: "List".into(),
                kind: ListKind::Checklist,
                basis: vec![],
                memory: false,
                when: None,
                why: None,
                steps,
            }],
            content: PackContent::default(),
            other_files: vec![],
        }
    }

    fn references(text: &str) -> References {
        let mut refs = References::default();
        refs.add("manual.txt", text);
        refs
    }

    fn step(short: &str, text: &str, expect: &str) -> Step {
        Step::new(format!("demo.p.{short}"), text, expect)
    }

    fn with_note(note: &str) -> Pack {
        let mut s = step("a", "Unrelated", "ON");
        s.note = Some(note.into());
        pack(vec![s])
    }

    #[test]
    fn normalizing_drops_case_and_punctuation() {
        assert_eq!(
            normalize("The Pilot's A/P -- ON,\n  now!"),
            ["the", "pilots", "a", "p", "on", "now"]
        );
    }

    #[test]
    fn eight_shared_words_are_flagged_and_seven_are_not() {
        let refs = references(PHRASE);
        let found = find_overlaps(&with_note(&format!("Intro {PHRASE} outro")), &refs);
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].file, "lists/a.yaml");
        assert_eq!(found[0].at, "demo.p.a/note");
        assert_eq!(found[0].reference, "manual.txt");
        assert_eq!(found[0].words.join(" "), PHRASE);
        assert!(!found[0].across_steps);

        let seven = "alpha bravo charlie delta echo foxtrot golf";
        assert_eq!(find_overlaps(&with_note(seven), &refs), vec![]);
    }

    #[test]
    fn case_punctuation_and_line_breaks_do_not_hide_a_match() {
        let refs = references("ALPHA, bravo;\ncharlie\n\n  delta. Echo-foxtrot golf (hotel)");
        assert_eq!(find_overlaps(&with_note(PHRASE), &refs).len(), 1);
    }

    #[test]
    fn a_longer_run_is_reported_once_at_full_length() {
        let long = format!("{PHRASE} india juliet");
        let found = find_overlaps(&with_note(&long), &references(&long));
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].words.len(), 10);
    }

    #[test]
    fn a_list_copied_across_short_rows_is_caught_only_by_the_joined_check() {
        let steps = vec![
            step("one", "Alpha bravo", "CHARLIE"),
            step("two", "Delta echo", "FOXTROT"),
            step("three", "Golf hotel", "INDIA"),
        ];
        let p = pack(steps);
        let refs = references(PHRASE);

        // No single field reaches 8 words, so a field-by-field check alone finds nothing.
        for step in &p.checklists[0].steps {
            assert!(normalize(&step.text).len() + normalize(&step.expect).len() < MIN_RUN);
        }

        let found = find_overlaps(&p, &refs);
        assert_eq!(found.len(), 1, "{found:?}");
        assert!(found[0].across_steps);
        assert_eq!(found[0].at, "list list-a");
        assert_eq!(found[0].words.join(" "), PHRASE);
    }

    #[test]
    fn a_run_inside_one_field_is_not_reported_twice() {
        let found = find_overlaps(&pack(vec![step("a", PHRASE, "ON")]), &references(PHRASE));
        assert_eq!(found.len(), 1);
        assert!(!found[0].across_steps);
    }

    #[test]
    fn other_content_files_are_checked_and_ids_are_not() {
        let mut p = pack(vec![step("a", "T", "E")]);
        p.content.notes = Some(NotesFile {
            how: vec![checklist_core::HowNote {
                id: "alpha-bravo-charlie-delta-echo-foxtrot-golf-hotel".into(),
                steps: vec![format!("First {PHRASE}")],
            }],
            explain: vec![],
        });
        let found = find_overlaps(&p, &references(PHRASE));
        assert_eq!(found.len(), 1, "{found:?}");
        assert_eq!(found[0].file, "notes.yaml");
        assert!(found[0].at.ends_with("/steps/0"), "{}", found[0].at);
    }

    #[test]
    fn the_allow_list_needs_a_reason_and_a_full_run() {
        let list =
            AllowList::parse(&format!("# comment\n\n{PHRASE} india | Standard phrase\n")).unwrap();
        assert!(list.allows(&normalize(PHRASE)));
        assert!(list.allows(&normalize(&format!("{PHRASE} india"))));
        assert!(!list.allows(&normalize(&format!("zulu {PHRASE}"))));

        assert!(AllowList::parse(PHRASE).is_err());
        assert!(AllowList::parse(&format!("{PHRASE} |  ")).is_err());
        assert!(AllowList::parse("too short | reason").is_err());
    }

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("overlap-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn references_load_from_text_files_only() {
        let dir = temp_dir("load");
        std::fs::create_dir(dir.join("sub")).unwrap();
        std::fs::write(dir.join("sub/a.TXT"), PHRASE).unwrap();
        std::fs::write(dir.join("skip.pdf"), "binary").unwrap();
        let (refs, skipped) = load_references(&dir).unwrap();
        assert_eq!(refs.file_count(), 1);
        assert_eq!(skipped, 1);
        let found = find_overlaps(&with_note(PHRASE), &refs);
        assert_eq!(
            found[0].reference,
            format!("sub{}a.TXT", std::path::MAIN_SEPARATOR)
        );
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn a_folder_with_no_text_files_is_an_error() {
        let dir = temp_dir("empty");
        std::fs::write(dir.join("only.pdf"), "x").unwrap();
        assert!(matches!(
            load_references(&dir),
            Err(OverlapError::NoReferences(_))
        ));
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn reference_folders_in_or_around_a_repo_are_refused() {
        let root = temp_dir("repo").canonicalize().unwrap();
        let repo = root.join("repo");
        std::fs::create_dir_all(repo.join(".git")).unwrap();
        std::fs::create_dir_all(repo.join("packs/a")).unwrap();
        std::fs::create_dir_all(repo.join("refs")).unwrap();
        std::fs::create_dir_all(root.join("private")).unwrap();

        let roots = repo_roots(&[&repo.join("packs/a")]);
        assert_eq!(roots, vec![repo.clone()]);
        assert!(matches!(
            check_refs_location(&repo.join("refs"), &roots),
            Err(OverlapError::InsideRepo(_))
        ));
        assert!(matches!(
            check_refs_location(&root, &roots),
            Err(OverlapError::ContainsRepo(_))
        ));
        assert!(check_refs_location(&root.join("private"), &roots).is_ok());
        std::fs::remove_dir_all(root).unwrap();
    }

    proptest! {
        /// Comparing never panics, whatever the text.
        #[test]
        fn never_panics(reference in ".{0,200}", note in ".{0,200}") {
            let _ = find_overlaps(&with_note(&note), &references(&reference));
        }

        /// A planted run is always found, whatever surrounds it.
        #[test]
        fn planted_runs_are_always_found(
            run in proptest::collection::vec("[a-z]{1,8}", MIN_RUN..14),
            before in "[a-z ]{0,30}",
            after in " [a-z ]{0,30}",
        ) {
            let run = run.join(" ");
            let note = format!("{before} {run}{after}");
            let found = find_overlaps(&with_note(&note), &references(&run));
            prop_assert!(!found.is_empty());
        }
    }
}
