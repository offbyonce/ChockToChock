// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! `pack check <dir>...`  validate one or more pack folders (or a folder of packs).
//! `pack schema <dir>`    write the JSON Schemas into `<dir>`.
//! `pack overlap <dir> [--refs <folder>] [--allow <file>]`  compare pack text with private
//! reference files. The folder can also come from `CHOCKTOCHOCK_OVERLAP_REFS`.
//!
//! Output goes to stdout/stderr on purpose: this is a command-line tool.
#![allow(clippy::print_stdout, clippy::print_stderr)]

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, Result, bail};
use pack::{Severity, load_pack_dir, validate_pack};
use pack_overlap::{
    AllowList, REFS_VARIABLE, check_refs_location, find_overlaps, load_references, repo_roots,
};

const USAGE: &str = "usage:\n  pack check <pack-dir | packs-dir>...\n  pack schema <out-dir>\n  pack overlap <pack-dir | packs-dir> [--refs <folder>] [--allow <file>]";

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
        Some((cmd, rest)) if cmd == "overlap" && !rest.is_empty() => overlap(rest),
        _ => bail!("{USAGE}"),
    }
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
