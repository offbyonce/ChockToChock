// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

//! The desktop shell. It wires the checklist crates to the window and nothing more:
//! commands unpack arguments, call a crate, and convert errors. Logic belongs in `crates/`.
//!
//! `run()` lives here (not in `main.rs`) so a mobile entry point can reuse it later.

mod commands;
mod error;

pub use error::AppError;

pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let result = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::app_info,
            commands::example_checklist
        ])
        .run(tauri::generate_context!());

    if let Err(e) = result {
        tracing::error!("Chock to Chock stopped with an error: {e}");
        std::process::exit(1);
    }
}
