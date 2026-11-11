<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# Parked: the desktop app

Phase 2 ships as a web app (decisions 0009 and 0010), so the Tauri shell in `src-tauri` is kept
but not built: it is excluded from the Cargo workspace, and CI does not compile it. Its Tauri
versions stay pinned in the root `Cargo.toml`.

It last compiled on 2026-11-11 against `app-core`, with Tauri 2.12. Reviving it takes:

1. Moving `apps/desktop/src-tauri` from `exclude` back to `members` in the root `Cargo.toml`.
2. A desktop entry around the shared interface in `packages/ui`, with a Tauri host that implements
   its host interface, and pointing `tauri.conf.json` (`beforeBuildCommand`, `frontendDist`) at it.
3. The Tauri JavaScript packages (`@tauri-apps/api`, `@tauri-apps/cli`) back in the pnpm catalog.
4. The Linux WebKit libraries back in CI, and the `RUSTSEC-2024-0429` advisory reviewed again
   (glib 0.18 comes from Tauri's GTK stack).
5. A new decision before adding any simulator connection (0006, 0009).
