<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0004: Sim link behind a trait; pure-Rust SimConnect client first

Date: 2026-11-08. Status: superseded by [0006](0006-instruction-not-sim-control.md).

## Context
Microsoft's SDK licence does not clearly allow shipping `SimConnect.dll`. MSFS 2024
scoped `L:1:` variables reportedly cannot be read over SimConnect.

## Decision
- The engine only sees the `SimSource` trait (`crates/sim-link`). `FakeSim` replays
  recorded timelines for tests.
- First real backend: `flybywireless-simconnect` 0.2.1 (MIT), a pure-Rust client that
  needs no DLL and connects by named pipe (Windows) or TCP.
- No vendor SDK headers, DLLs or other add-ons' code are copied in.

## Spike result (2026-11-08)
- The crate builds on Linux and macOS (TCP) and Windows (named pipe).
- It has the calls we need: data definitions, system events, client data (for PMDG-style
  broadcasts) and MSFS 2024 input events.
- Its only `unsafe` code looks up the Windows Documents folder for `SimConnect.cfg`.
- A connection attempt with no sim running fails cleanly.
- Not yet tested against a running sim: that happens on the Windows PC in Phase 3.

## Consequences
- Risk: young crate, one maintainer. The trait keeps a swap to `simconnect-sdk` or raw
  bindings cheap.
