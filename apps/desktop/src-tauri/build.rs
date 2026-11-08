// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! Lists every app command so Tauri generates a permission for each one.
//! A command not granted in `capabilities/` cannot be called from the interface.

fn main() {
    let commands = &["app_info", "example_checklist"];
    let attributes = tauri_build::Attributes::new()
        .app_manifest(tauri_build::AppManifest::new().commands(commands));
    if let Err(e) = tauri_build::try_build(attributes) {
        // Build scripts report failures by panicking; this is the conventional exit.
        #[allow(clippy::panic)]
        {
            panic!("tauri build failed: {e:#}");
        }
    }
}
