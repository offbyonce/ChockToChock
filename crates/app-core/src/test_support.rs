// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Packs for tests, loaded from the repo's own packs.

use std::path::Path;

use pack::FlatPack;

fn load(id: &str) -> FlatPack {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../packs")
        .join(id);
    FlatPack::from(pack::load_pack_dir(&dir).unwrap())
}

pub(crate) fn example_pack() -> FlatPack {
    load("example-trainer")
}

pub(crate) fn ifly_pack() -> FlatPack {
    load("ifly-737-max8")
}
