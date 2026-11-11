<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# Parked: SQLite saved data

The web app keeps saved progress in browser storage (decisions 0009 and 0010), and `rusqlite`'s
bundled SQLite does not build for WebAssembly. This crate is kept for a later desktop build but
not built: it is excluded from the Cargo workspace, so its five tests do not run in CI. Its
`rusqlite` version stays pinned in the root `Cargo.toml`.

It last compiled and passed its tests on 2026-11-11. To revive it, move `crates/store` from
`exclude` back to `members` in the root `Cargo.toml`.
