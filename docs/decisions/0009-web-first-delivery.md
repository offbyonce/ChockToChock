<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0009: Web first: a hosted app that installs and works offline

Date: 2026-11-10. Status: accepted. Amends [0001](0001-app-stack.md).

## Context
0001 chose a Tauri desktop app, Windows first. Since then, decision 0006 made the app a training
tool with no simulator connection, so it no longer needs to run on the simulator's machine. Many
pilots fly on Xbox or PlayStation, or want the app on a tablet beside the screen; a Windows
installer serves none of them. Beta installers would also ship unsigned (0005) and show a
SmartScreen warning, or be blocked where Smart App Control is on.

## Decision
- Phase 2 ships as a hosted web app on GitHub Pages. It can be installed to a home screen or
  desktop, and works offline after the first visit.
- The Rust crates stay the single home of the logic and run in the browser as WebAssembly. A
  thin web host crate wires them up, as the Tauri shell does today. The interface stays Svelte 5.
- Packs ship as flattened JSON, one file per aircraft, built and checked at build time. The
  browser loads finished data; it does not read YAML.
- Saved progress lives in browser storage, with export to a file and import from one. The file
  format is a versioned Rust type, so a later desktop build can read the same files.
- What 0001 says about Tauri, `crates/store` and the desktop shell now applies to the later
  desktop option below, not to Phase 2. 0005 (updates and signing) likewise applies only to a
  desktop build; on the web, each deploy is the update.

**Reasons:**
- **Tablet and console users.** Pilots on Xbox, PlayStation or a Mac can use the app on any
  device with a modern browser.
- **No installer warning.** Nothing to download, sign or approve.
- **Instant updates.** A fix reaches every pilot on their next visit.

**Risks:**
- **Browser storage can be cleared**, by the user, by storage pressure, or by browser policy
  (Safari can remove the data of sites not used for seven days unless the app is installed).
  Mitigations: ask the browser for persistent storage, make export easy to find, and remind pilots
  to export.
- **SimBrief import must work from a hosted page.** The browser fetches the plan directly, which
  only works if SimBrief allows requests from our address. This is tested early in Phase 2; if it
  fails, the fallback must also need no server (for example, importing a downloaded plan file).
  **Resolved 2026-11-11** by the Phase 2, step 3 probe: on the live Pages site, the maintainer
  loaded their latest plan by username directly from their desktop browser. SimBrief's API also
  answers with `access-control-allow-origin: *`. Loading a saved plan file, the no-server
  fallback, also worked. Tablets were not tested then.
- **The site is public.** Anyone can open it, unfinished work is visible, and copies can be
  hosted elsewhere. Beta labeling, the first-launch notice and the content rules in 0007 matter
  more. GitHub Pages cannot set custom response headers, so the content security policy goes in a
  `<meta>` tag, which cannot cover everything a header can.

## Privacy and availability
- **No backend.** The site is static files only. Nothing is sent to us, because there is nowhere
  to send it.
- **No accounts.**
- **No analytics or tracking** of any kind.
- **No cookies.** The only stored data is strictly necessary local storage of the user's own
  progress and settings, on their own device.
- **No third-party resources at runtime.** All fonts, icons and assets are bundled with the site.
- **SimBrief is called directly from the user's browser**, with the username they enter. We never
  see the request or the plan.
- **Hosting logs.** GitHub Pages logs visitor IP addresses, under GitHub's own privacy statement.
  We do not receive or use those logs.
- **No availability promise.** The site may be down or change at any time. Installed copies keep
  working offline.

## Later options (not decisions)
- **A Tauri desktop build** around the same interface. For PC pilots it could later add a
  SimConnect link and an in-sim toolbar panel. That would need a new decision revisiting 0006.
- **An MSFS 2024 EFB app** reusing the packs.

The web app's design must not block either: the interface reaches storage, files and network only
through small adapters, the Rust core stays free of browser-only code, and packs stay plain JSON.

## Consequences
- One codebase serves PC, Mac, Xbox and PlayStation pilots, on desktop, tablet and phone.
- `crates/store` (SQLite) is not used in Phase 2. It stays in the repo for a desktop build.
- The desktop shell in `apps/desktop` is parked: kept, but not released in Phase 2.
- New build tooling is needed for WebAssembly and the offline service worker; each new
  dependency is justified when it is added.
- A deploy workflow for GitHub Pages is needed; workflow changes are the maintainer's.
