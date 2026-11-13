// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! `pack check <dir>...`  validate one or more pack folders (or a folder of packs).
//! `pack schema <dir>`    write the JSON Schemas into `<dir>`.
//! `pack build <dir> <out-dir>`  check each pack, then write it as one flattened `<id>.json`,
//!                        plus `index.json` listing the packs written.
//! `pack overlap <dir> [--refs <folder>] [--allow <file>]`  compare pack text with private
//! reference files. The folder can also come from `CHOCKTOCHOCK_OVERLAP_REFS`.
//! `pack suggest-targets <pack-dir>`  draft each step's targets from its wording, as JSON, with
//! every place the reading was not certain (decision 0018). An author checks and keeps them.
//! `pack terms <pack-dir>`  list steps whose name, value, Check, If not or Why use a glossary term
//! their `terms` list lacks (3.0's whole-word rule), so the lists stay current; fails if any.
//! With `--json`, print every step's full list as JSON instead, for an author to write in.
//!
//! Output goes to stdout/stderr on purpose: this is a command-line tool.
#![allow(clippy::print_stdout, clippy::print_stderr)]

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, Result, bail};
use pack::{FlatPack, PackSummary, Severity, load_pack_dir, validate_pack};
use pack_overlap::{
    AllowList, REFS_VARIABLE, check_refs_location, find_overlaps, load_references, repo_roots,
};

const USAGE: &str = "usage:\n  pack check <pack-dir | packs-dir>...\n  pack schema <out-dir>\n  pack build <pack-dir | packs-dir> <out-dir>\n  pack overlap <pack-dir | packs-dir> [--refs <folder>] [--allow <file>]\n  pack suggest-targets <pack-dir>\n  pack terms [--json] <pack-dir>";

fn main() -> ExitCode {
    match run() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("error: {e:#}");
            ExitCode::from(2)
        }
    }
}

fn run() -> Result<bool> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.split_first() {
        Some((cmd, rest)) if cmd == "check" && !rest.is_empty() => check(rest),
        Some((cmd, [out])) if cmd == "schema" => schema(Path::new(out)).map(|()| true),
        Some((cmd, [packs, out])) if cmd == "build" => build(packs, Path::new(out)),
        Some((cmd, rest)) if cmd == "overlap" && !rest.is_empty() => overlap(rest),
        Some((cmd, [dir])) if cmd == "suggest-targets" => suggest_targets(Path::new(dir)),
        Some((cmd, [dir])) if cmd == "terms" => terms(Path::new(dir)),
        Some((cmd, [flag, dir])) if cmd == "terms" && flag == "--json" => {
            terms_json(Path::new(dir))
        }
        _ => bail!("{USAGE}"),
    }
}

/// Prints each step's drafted targets and flags, by step id.
fn suggest_targets(dir: &Path) -> Result<bool> {
    let pack = load_pack_dir(dir).map_err(|e| anyhow::anyhow!("{e}"))?;
    let mut out = serde_json::Map::new();
    for step in pack.checklists.iter().flat_map(|l| &l.steps) {
        if step.controls.is_empty() {
            continue;
        }
        let s = pack::suggest::suggest(&pack.content.panels, step);
        out.insert(step.id.to_string(), serde_json::to_value(s)?);
    }
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(true)
}

fn terms(dir: &Path) -> Result<bool> {
    let pack = load_pack_dir(dir).map_err(|e| anyhow::anyhow!("{e}"))?;
    let glossary: Vec<&str> = pack
        .content
        .glossary
        .iter()
        .flat_map(|g| &g.terms)
        .map(|t| t.term.as_str())
        .collect();
    let mut clean = true;
    for step in pack.checklists.iter().flat_map(|l| &l.steps) {
        let missing = pack::terms::missing_terms(step, &glossary);
        if !missing.is_empty() {
            clean = false;
            println!(
                "{}: uses {} but its terms list lacks it",
                step.id,
                missing.join(", ")
            );
        }
    }
    if clean {
        println!("every step's terms list covers its wording");
    }
    Ok(clean)
}

fn terms_json(dir: &Path) -> Result<bool> {
    let pack = load_pack_dir(dir).map_err(|e| anyhow::anyhow!("{e}"))?;
    let glossary: Vec<&str> = pack
        .content
        .glossary
        .iter()
        .flat_map(|g| &g.terms)
        .map(|t| t.term.as_str())
        .collect();
    let mut out = serde_json::Map::new();
    for step in pack.checklists.iter().flat_map(|l| &l.steps) {
        let mut all = step.terms.clone();
        all.extend(pack::terms::missing_terms(step, &glossary));
        if !all.is_empty() {
            out.insert(step.id.to_string(), serde_json::to_value(all)?);
        }
    }
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(true)
}

/// A folder with `pack.json` is one pack; otherwise each child folder with one is a pack.
fn pack_dirs(arg: &str) -> Result<Vec<PathBuf>> {
    let dir = PathBuf::from(arg);
    if dir.join("pack.json").is_file() {
        return Ok(vec![dir]);
    }
    let mut found = Vec::new();
    for entry in
        std::fs::read_dir(&dir).with_context(|| format!("cannot read {}", dir.display()))?
    {
        let path = entry?.path();
        if path.join("pack.json").is_file() {
            found.push(path);
        }
    }
    found.sort();
    Ok(found)
}

fn check(args: &[String]) -> Result<bool> {
    let mut ok = true;
    let mut count = 0;
    for arg in args {
        for dir in pack_dirs(arg)? {
            count += 1;
            match load_pack_dir(&dir) {
                Err(e) => {
                    ok = false;
                    eprintln!("FAIL {}\n  {e}", dir.display());
                }
                Ok(pack) => {
                    let issues = validate_pack(&pack);
                    let errors = issues
                        .iter()
                        .filter(|i| i.severity == Severity::Error)
                        .count();
                    let steps: usize = pack.checklists.iter().map(|c| c.steps.len()).sum();
                    if errors > 0 {
                        ok = false;
                    }
                    let status = if errors > 0 { "FAIL" } else { "ok  " };
                    println!(
                        "{status} {} {} ({} checklists, {steps} steps)",
                        pack.manifest.id,
                        pack.manifest.version,
                        pack.checklists.len()
                    );
                    for issue in issues {
                        println!("  {issue}");
                    }
                }
            }
        }
    }
    if count == 0 {
        bail!("no packs found in {}", args.join(", "));
    }
    Ok(ok)
}

struct OverlapArgs {
    target: String,
    refs: Option<PathBuf>,
    allow: Option<PathBuf>,
}

fn overlap_args(args: &[String]) -> Result<OverlapArgs> {
    let mut parsed = OverlapArgs {
        target: String::new(),
        refs: None,
        allow: None,
    };
    let mut rest = args.iter();
    while let Some(arg) = rest.next() {
        match arg.as_str() {
            "--refs" => parsed.refs = rest.next().map(PathBuf::from),
            "--allow" => parsed.allow = rest.next().map(PathBuf::from),
            other if parsed.target.is_empty() && !other.starts_with("--") => {
                other.clone_into(&mut parsed.target);
            }
            _ => bail!("{USAGE}"),
        }
    }
    if parsed.target.is_empty() {
        bail!("{USAGE}");
    }
    if parsed.refs.is_none() {
        parsed.refs = std::env::var_os(REFS_VARIABLE)
            .filter(|v| !v.is_empty())
            .map(PathBuf::from);
    }
    Ok(parsed)
}

fn overlap(args: &[String]) -> Result<bool> {
    let parsed = overlap_args(args)?;
    let Some(refs) = parsed.refs else {
        println!("overlap check not run: {REFS_VARIABLE} is not set");
        return Ok(true);
    };
    let dirs = pack_dirs(&parsed.target)?;
    if dirs.is_empty() {
        bail!("no packs found in {}", parsed.target);
    }
    let cwd = std::env::current_dir()?;
    let mut places: Vec<&Path> = dirs.iter().map(PathBuf::as_path).collect();
    places.push(&cwd);
    let refs = check_refs_location(&refs, &repo_roots(&places))?;
    let (references, skipped) = load_references(&refs)?;
    let allow = match parsed.allow {
        Some(path) => {
            let text = std::fs::read_to_string(&path)
                .with_context(|| format!("cannot read {}", path.display()))?;
            AllowList::parse(&text)?
        }
        None => AllowList::default(),
    };
    let mut flagged = 0;
    let mut allowed = 0;
    for dir in &dirs {
        let pack = load_pack_dir(dir)?;
        for found in find_overlaps(&pack, &references) {
            if allow.allows(&found.words) {
                allowed += 1;
                continue;
            }
            flagged += 1;
            let scope = if found.across_steps {
                " (across steps)"
            } else {
                ""
            };
            println!(
                "OVERLAP {} {}{scope}\n  in reference: {}\n  words ({}): {}",
                found.file,
                found.at,
                found.reference,
                found.words.len(),
                found.words.join(" ")
            );
        }
    }
    println!(
        "overlap: {} reference files, {skipped} other files skipped, {flagged} flagged, {allowed} allowed",
        references.file_count()
    );
    Ok(flagged == 0)
}

fn schema(out: &Path) -> Result<()> {
    std::fs::create_dir_all(out)?;
    for (name, json) in pack::schema::all() {
        let path = out.join(name);
        std::fs::write(&path, json).with_context(|| format!("cannot write {}", path.display()))?;
        println!("wrote {}", path.display());
    }
    Ok(())
}

/// Check every pack, then write each one as `<out>/<id>.json`. A pack with errors is not written.
fn build(packs: &str, out: &Path) -> Result<bool> {
    let dirs = pack_dirs(packs)?;
    if dirs.is_empty() {
        bail!("no packs found in {packs}");
    }
    std::fs::create_dir_all(out).with_context(|| format!("cannot create {}", out.display()))?;
    let mut ok = true;
    let mut index = Vec::new();
    for dir in dirs {
        let pack = load_pack_dir(&dir).with_context(|| format!("cannot load {}", dir.display()))?;
        let errors: Vec<_> = validate_pack(&pack)
            .into_iter()
            .filter(|i| i.severity == Severity::Error)
            .collect();
        if !errors.is_empty() {
            ok = false;
            eprintln!("FAIL {}: not built, run `pack check`", pack.manifest.id);
            for issue in errors {
                eprintln!("  {issue}");
            }
            continue;
        }
        let path = out.join(format!("{}.json", pack.manifest.id));
        index.push(PackSummary::from(&pack.manifest));
        let json = serde_json::to_string(&FlatPack::from(pack))?;
        std::fs::write(&path, &json).with_context(|| format!("cannot write {}", path.display()))?;
        println!("built {} ({} bytes)", path.display(), json.len());
    }
    let path = out.join("index.json");
    std::fs::write(&path, serde_json::to_string(&index)?)
        .with_context(|| format!("cannot write {}", path.display()))?;
    println!("wrote {} ({} packs)", path.display(), index.len());
    Ok(ok)
}
