// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! `pack check <dir>...`  validate one or more pack folders (or a folder of packs).
//! `pack schema <dir>`    write the JSON Schemas into `<dir>`.
//!
//! Output goes to stdout/stderr on purpose: this is a command-line tool.
#![allow(clippy::print_stdout, clippy::print_stderr)]

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use anyhow::{Context, Result, bail};
use pack::{Severity, load_pack_dir, validate_pack};

const USAGE: &str = "usage:\n  pack check <pack-dir | packs-dir>...\n  pack schema <out-dir>";

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

fn schema(out: &Path) -> Result<()> {
    std::fs::create_dir_all(out)?;
    for (name, json) in pack::schema::all() {
        let path = out.join(name);
        std::fs::write(&path, json).with_context(|| format!("cannot write {}", path.display()))?;
        println!("wrote {}", path.display());
    }
    Ok(())
}
