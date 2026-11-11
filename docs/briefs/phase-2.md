<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# Phase 2 brief: web beta with 3.0 parity

**Goal:** a public web beta on GitHub Pages that does everything the 3.0 checklist does, using
the iFly pack from Phase 1. It installs to a home screen, works offline after the first visit, and
follows the privacy rules in decision 0009.

**Gate:** every 3.0 feature works in the web app (checked against the Phase 1 parity report),
saved progress survives export and import, the app works offline once installed, and `just ci`
is green.

Work one step at a time (see `AGENTS.md`): finish a step, show the result, wait for the
maintainer's go-ahead. Steps marked **(maintainer)** are the maintainer's to do. That includes
every change to `.github/workflows/`. For those, prepare what the step needs and stop.

## Rules for this phase

- Decision 0009: no backend, no accounts, no analytics or tracking, no cookies, no third-party
  resources at runtime. SimBrief is called from the user's browser.
- Logic stays in `crates/`. The web host crate and the interface stay thin, as the Tauri shell
  is today. No aircraft names outside packs.
- The interface reaches storage, files and the network only through a small host adapter, so a
  desktop build can reuse it later (0009, later options).
- Every new dependency is proposed with a reason before it is added.

## Steps

1. **Layout and tooling proposal (no code).** Propose the `apps/` and `packages/` layout deferred
   from the structure clean-up: a shared interface package (for example `packages/ui`) used by
   `apps/web` now and `apps/desktop` later, `packages/bindings` for the generated types, and a
   thin web host crate in `crates/`. Also propose: the WebAssembly build tooling, how the offline
   service worker is built, which browser storage holds saved progress, and whether the parked
   Tauri shell keeps building in CI during Phase 2. Write choices that matter as a decision record
   and wait for approval.
2. **GitHub Pages deploy (maintainer).** Prepare a `just` recipe that builds the site into one
   folder, and say what the workflow must run. The maintainer writes the Pages workflow and turns
   on Pages for the repository.
3. **Prove SimBrief import from a hosted page.** Before any interface work, deploy a bare page
   that fetches a SimBrief plan for a username the pilot types in, directly from the browser, and
   parses it with the `simbrief` crate running as WebAssembly. Report whether it works from the
   Pages address. If it does not, propose a fallback that also needs no server (for example,
   importing a downloaded plan file) and wait for a decision.
   **Done 2026-11-11:** loading the latest plan by username worked from the maintainer's desktop
   browser on the live site (0009, risks), and so did loading a saved plan file, though the
   probe's plan loading is clunky (step 7 sets the product requirements). Tablets were not
   tested. The probe reads plans with a temporary JavaScript reader, so
   no workflow change was needed; the crate as WebAssembly comes in step 5.
4. **Restructure (paths only).** Move the interface from `apps/desktop/ui` into the shared package
   and create `apps/web` around it, as approved in step 1. No behavior change. Update the pnpm
   workspace, the `justfile`, the `AGENTS.md` repository map and any docs naming moved paths.
   `just ci` green.
   **Done 2026-11-11** (PR #10): `packages/ui` with a host interface, `apps/web`, `crates/app-core`
   and `crates/web-host`; the Tauri shell and `crates/store` parked.
5. **Packs as flattened JSON, and the web host.** Add a `pack-cli` command that writes one
   flattened JSON file per aircraft, checked by `pack check` first, with a test that it matches
   the pack snapshots. Build the web host crate that exposes the core to the interface. Types
   still come from `ts-rs`. Once the `simbrief` crate runs as WebAssembly, remove the probe's
   temporary JavaScript plan reader (`ofp-probe.js` and its test).
   **Done 2026-11-11:** `pack build` writes `<id>.json`; the web host runs as WebAssembly and the
   app uses it; the `simbrief` crate reads fetched JSON and saved JSON or XML files, and the
   JavaScript reader is gone. `just site` builds the site with a strict content security policy
   injected into every page: the placeholder stays the front page, the work-in-progress app is at
   the unlinked `app/` (noindex, with the "simulator training only" line) and the probe page stays
   where it was. The app does not use the iFly pack yet (step 7).
6. **Installable and offline.** Web app manifest, service worker and bundled fonts and icons, under
   the content security policy that step 5 added. A new version never replaces content in the
   middle of a session; the app offers a reload instead. Show it working offline in a browser.
   **Done 2026-11-11:** the app at `app/` has a web app manifest and our own icons, and B612 is
   bundled. A hand-written service worker precaches the app, the WebAssembly module, the fonts and
   the packs, under a cache named from a hash of the built files; it never takes over mid-session
   and offers a reload instead. `just update-test` runs the version-to-version test in headless
   Firefox (first visit, offline, new version waiting with the reload offer, reload); it passes.
   Still to do by hand on the live site, once per release while the beta lasts: install the app,
   deploy a new version, and check the reload offer and the new version on a desktop browser and
   a tablet.
7. **Interface parity.** Phases, checklists and flows, panel states, timers, the CDU walkthrough,
   SimBrief import and the guided tour, matching 3.0. Report one feature at a time. Include the
   carried-over checks from the Phase 1 brief: the LEGS page renders correctly with no departure
   set (with a test), and CDU comparison tests with partly filled flight data.
   SimBrief import must be easier than the step 3 probe, which does not carry into the product:
   - Remember the SimBrief username on the device, with an option to forget it.
   - One-tap "Refresh plan", and optionally load the plan automatically when the app opens.
   - Show which plan is loaded, its route and its age.
   - Keep the last plan for offline use.
   - Friendly errors, such as "Username not found", "No plan filed yet" and "Offline, using your
     last plan".
   - The saved-file option is a secondary route: drag and drop anywhere, with short instructions
     shown only when the pilot chooses it.

   Interface requirements for every list:
   - **One presentation:** every step looks the same; there is no status or verification marker
     (decision 0013). Practice and the reference screens use every step.
   - **Item states:** pending, current, done and skipped, each shown by shape and text as well as
     color, so they read in grayscale and for color-blind pilots. A skipped item stays visibly
     open, and a list with skipped items never looks complete.
   - **Current-item box:** the challenge on the left, the value on the right, and a large
     Check/Next button within thumb reach on a tablet.
   - **Detail levels:** a quick level (item and value) and an expanded level (where it is, why,
     and links such as the CDU walkthrough).
   - **Wording:** make clear that the pilot confirms each item. The app has no simulator link
     (decision 0006) and never implies it knows a switch position.
   - **Touch targets:** at least 44 to 48 px.
   - **Theme:** dark, with a dimming control, and text contrast of at least 4.5:1 in every theme.
   - **Typeface:** Geist for interface text, as in 3.0, and B612 Mono for values (both SIL Open
     Font License 1.1), bundled and precached with the site. B612, bundled in step 6, was replaced
     on the batch 0 review so the app matches 3.0's type.
   - **Non-normal lists:** larger type than normal lists.
   - **Completion:** a clear completion call at the end of each list (for example "Before start
     checklist complete"), shown only when no item is still open.
   - **Icons:** one permissive general icon set, Tabler Icons (MIT), plus our own small family of
     schematic control glyphs on the same 24 px grid: toggle switch, guarded switch, rotary
     selector, push button with annunciator, lever and light. They are drawn from scratch and
     never traced from any real panel; lit or selected states use fill as well as color, and
     amber and red always come with the words CAUTION and WARNING. Only the Tabler icons we use
     are copied in as SVG files, not installed as a package, so decision 0010's rule of no new
     JavaScript dependencies holds; MIT is already in `LICENSES/`.
   - Adding any new license file to `LICENSES/` needs the maintainer's go-ahead first (B612's
     `OFL-1.1.txt` is approved and added with the font in step 6).
   Batches (one pull request each, screenshots for review saved outside the repo). 3.0 is the
   design baseline: the layout and look match 3.0, and the requirements above are improvements on
   top of it, not a different design. Each batch's review screenshots include 3.0 and the new app
   side by side, at the same size and theme (`just screenshots` with `CHOCKTOCHOCK_BASELINE`).
   0. Foundations: decision 0012, app state in Rust and the WebAssembly calls, themes and
      dimming with a contrast test, the icon pipeline, the app frame (top bar, phase rail,
      Emergency button, addresses) and the screenshot tool. Reworked on review to the 3.0
      layout: stage tabs and phase chips, the flight strip, step rows, Flow view with the
      current-item box, and the context panel with the flight deck map and control schematic.
   1. Lists: phase screen, Checklist and Flow, item states, current-item box, quick and expanded,
      item detail, completion call, open items, options, Full and Essentials, keyboard, and
      saving progress on the device (brought forward from step 8).
   2. Flight data: flight setup, flight strip, values in steps, SimBrief plan loading.
   3. Panel states, timers and briefings in the flow.
   4. CDU walkthrough.
   5. Flight deck: overview map, panel views with our own glyphs, light meanings, exterior
      inspection on our own airplane outline.
   6. Reference and learning: emergency reference, glossary, how procedures work, practice,
      search, guided tour.
   7. Settings (including Essentials changes and print) and the flight log with the logbook.
8. **Saved progress, export and import.** Keep progress in browser storage behind the host
   adapter, and ask the browser for persistent storage. Export writes a file in a versioned format
   defined in Rust. Import validates the file, applies aliases for old ids, and never silently
   overwrites: it says what will be replaced first. Remind pilots to export from time to time.
9. **3.0 import.** The web app cannot read 3.0's browser storage, because it is held under a
   different address. 3.0 can export its saved state as a JSON file (its "Export JSON" button,
   marked with the 3.0 app name and version 3). Import that file through the alias table, with the
   same validation and replace warning as step 8, and test it with an invented 3.0 export.
10. **Notices.** The notice "Simulator training only. Not for real-world use." once at first
    launch, covering the whole app. Every step is presented the same way, with no status or
    verification marker (decision 0013).
    A short privacy note in the app that says what is stored (progress, settings and the SimBrief
    username), where (in this browser, on this device), that nothing is sent to us, that SimBrief
    is contacted directly from the browser, and that GitHub Pages logs visitor addresses under
    GitHub's privacy statement. When the app is ready for users (here or in step 11, as the
    maintainer decides), it replaces the placeholder as the front page (`index.html`), and the
    temporary `app/` address and its interim notice line go.
    **Report a problem with a step:** each step offers a link that opens a prefilled GitHub issue
    form (`.github/ISSUE_TEMPLATE/step-problem.yml`, added in this step) with the pack, list, step
    id and app version filled in. There is no backend, and the form asks for nothing personal
    beyond what GitHub requires.
11. **Beta check.** Run through the parity list on desktop and tablet browsers, installed and
    offline, and report what passes and what does not. Any workflow change still needed, such as
    adding the web build to the CI checks, is listed for the maintainer.
12. **Hand back.** Branch ready for review, with the parity check, the browsers tested, the open
    issues and the remaining maintainer steps.

## Out of scope for Phase 2

- Phase 3 training features (decision 0008).
- A desktop release, a simulator link or an in-sim panel (0009, later options; 0006).
- Accounts, sync between devices, or anything that needs a server.
- An MSFS 2024 EFB app.

## Notes for later phases

**Phase 3:**

- Aliases for practice scores saved per cockpit area, along with the practice text (carried over
  from the Phase 1 brief).
