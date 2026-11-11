<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0001: Tauri 2, a Rust core and a Svelte 5 interface

Date: 2026-11-08. Status: accepted. Amended by [0009](0009-web-first-delivery.md) (delivery).

## Context
The 3.0 checklist is a single HTML file. We want a desktop app for Windows (where MSFS
runs), later macOS and Linux, possibly mobile, with a sim link and many aircraft.

## Decision
- Tauri 2 (stable 2.12) for the shell; not the Tauri 3 alpha.
- All logic in plain Rust crates with no Tauri dependency (`checklist-core`, `pack`,
  `simbrief`, `sim-link`, `store`). The Tauri crate only wires commands.
- Svelte 5 + Vite + TypeScript (strict) for the interface. TypeScript 6, because
  svelte-check does not yet support TypeScript 7.
- Types cross the boundary through `ts-rs`; errors cross as a tagged `AppError`.

## Consequences
- Most tests run on any machine without a window or a sim.
- A later move to Tauri 3 touches only `apps/desktop/src-tauri`.
- React's larger ecosystem is given up; we build our own cockpit components anyway.
