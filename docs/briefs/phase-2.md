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
   Checked by hand on the live site after batch 0's deploy: the reload offer appeared and Reload
   brought in the new version. The step is done. While the beta lasts, repeat the check once per
   release: install the app, deploy a new version, and check the reload offer and the new version
   on a desktop browser and a tablet.
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
     (decision 0006), so it never claims to know the simulator's state. That does not mean hiding
     what a step asks for: control drawings show the step's target position, highlighted and
     labeled as the target ("set to ON"), never as live state.
   - **Touch targets:** at least 44 px for every control and link (decision 0021).
   - **Theme:** dark and light on every screen, drawings included, with the dimming control kept,
     and contrast checked in CI for both themes (decision 0021).
   - **Typeface:** Geist for interface text and Geist Mono for values (both SIL Open Font License
     1.1), bundled and precached with the site (decision 0021; Geist Mono replaces B612 Mono).
   - **Non-normal lists:** larger type than normal lists.
   - **Completion:** a clear completion call at the end of each list (for example "Before start
     checklist complete"), shown only when no item is still open.
   - **Icons:** one permissive general icon set, Tabler Icons (MIT), plus our own small family of
     schematic control glyphs on the same 24 px grid: toggle switch, guarded switch, rotary
     selector, push button with annunciator, lever and light. They are drawn from scratch and
     never traced from any real panel. As in 3.0, color marks the controls a step uses, with no
     boxes or heavier outlines; without color they still read, because only they show a position
     (the knob at the target). Lit lights use fill as well as color, and amber and red always come
     with the words CAUTION and WARNING. Only the Tabler icons we use
     are copied in as SVG files, not installed as a package, so decision 0010's rule of no new
     JavaScript dependencies holds; MIT is already in `LICENSES/`.
   - Adding any new license file to `LICENSES/` needs the maintainer's go-ahead first (B612's
     `OFL-1.1.txt` is approved and added with the font in step 6).
   Batches (one pull request each, screenshots for review saved in `~/private/screenshots/`, outside
   the repo). The approved design in [docs/design/](../design/README.md) is the baseline, recorded
   in decisions 0021 and 0022; 3.0 is the reference for features, not for layout. Every 3.0
   feature keeps a reachable home at every size ([coverage.md](../design/coverage.md)). Each
   batch's independent reviewer, with fresh context, checks: both themes; all five sizes (phone,
   tablet portrait, tablet landscape, 1440, 2560 by 1440); the coverage rows the batch touches;
   the token lint; 44 px targets; contrast; that nothing was removed; and a matched 3.0 compare
   (`just screenshots` with `CHOCKTOCHOCK_BASELINE`). Its findings are included unedited, with the
   fixes. A batch is ready only when nothing is worse. The new app replaces the front page only
   when it clearly exceeds 3.0. Tablet and phone support is claimed only after the qualification
   in decision 0021.
   0. Foundations (done): decision 0012, app state in Rust and the WebAssembly calls, themes and
      dimming, the icon pipeline, the app frame and the screenshot tool.
   1. Lists (done): phase screen, Checklist and Flow, item states, open items, options, Full and
      Essentials, keyboard, saving progress on the device. Flight data was then begun: flight
      values, the compact SimBrief plan (decision 0019), Flight setup and the flight strip (PR #17).
   The boards in docs/design/boards are the specification, not inspiration (maintainer,
   2026-11-12): every screen, overlay, view, label, caption, legend, color, spacing and order
   matches its board exactly, in both themes and at every size the boards show, with values from
   tokens.json. The only exception is the panel control drawings, whose layout comes from the pack
   and whose drawing quality may exceed the board's. "Drawn from pack data" means the pack stores
   what the board draws, never a redrawing. Where a board cannot be matched exactly, stop and ask
   the maintainer before building anything different. Every reviewer brief adds a side-by-side of
   each changed screen against its board, both themes, with every difference listed; any
   difference the maintainer has not approved is a finding.
   The remaining interface work follows the approved design, in these batches:
   1. Tokens, themes and scaling: `tokens.json` to CSS variables, light and dark, the scale and
      breakpoints, Settings text size, the raw-value lint, a contrast check for both themes in CI,
      and every screen moved onto the tokens.
   2. Shared drawing renderer: maps, panels, side and seat views from pack data; the states
      changed, resting, conditional, resolved, lit and unlit (unlit is outline only); labels at
      11 px or more and light legends at 10 px or more; both themes; drawings that cannot fit
      their labels open as a full-width sheet. The panel icon and drawing-quality work
      (content-to-validate.md, last section), with before and after for every panel type and a
      side-by-side for each control type (current drawing, board, result). The existing control
      drawings win where they are better (owner's addendum in docs/design/README.md). Templates
      for displays, gauges, readouts, levers, handles and continuous knobs. Decision 0022's
      conditional and resolved drawing.
      Carried from batch 1's review: the "SET TO" caption (10 px, under the 11 px label rule)
      gives way to the changed-state drawing, and the orientation caption sits with its drawing,
      not below the "See the whole panel larger" button.
      Pack format 4 lands here (decisions 0022 and 0023): decision steps and targets that depend
      on them, and the side and seat frames for Where is this?; test fixtures are invented and
      stay out of the shipped pack, and existing packs migrate with their content unchanged.
   3. Core components and the desktop checklist: header, stage and phase tracks with open-step
      dots, flight strip, step rows (to do, current, done, skipped, decision), row menu with Add
      to and Remove from Essentials, Check, If not, Why with How to do it and Terms, side pane
      order, map hidden state, the open-steps list across all stages, the Where is this?
      pop-over, wide scaling.
      Carried from batch 1's review: step rows are at least 44 px tap targets, with the whole row
      (title, value and location line) as the target, never titles below 44 px; the controls
      drawing takes the side pane's remaining height so a whole panel no longer pushes the
      step's switches below the fold (the fuel pump switches today); term definitions and the
      Add to / Remove from Essentials action on the row.
      The phase progress-bar track reaches 3:1 using the `todo-ring` color (owner, 2026-11-12).
   4. Flight strip panel and Flight pages: clearance, frequencies, notes, Import latest from
      SimBrief, Reset flight plan, All flight data; Flight sections with field descriptions,
      destination chips and the Descriptions toggle; Weather with Find it in your OFP and a plain
      link to SimBrief that sends no user data; SimBrief and the offline OFP file (the SimBrief
      requirements above); Start options, including the panel-state picker; Approach.
      Aircraft wording (field descriptions, destination chips, option titles) comes from the
      pack. Carried from batch 1's review: SimBrief username and Import latest in the strip.
   5. Tablet and phone layouts and overlays, per screens.md.
      Carried from batch 1's review: the phone stage and phase picker (today one cut-off chip,
      a bare dot in Essentials), the progress bar on tablet and phone, and the inline controls
      drawing in an expanded step on tablet portrait and phone.
   6. The other views: Flow, Emergency reference, step timers, CDU walkthrough, briefings,
      exterior inspection on our own airplane outline, Flight deck reference, menu, Flight log,
      Settings.
      Carried from batch 1's review: Settings rows for Start, Map in steps, Your Essentials and
      Keep screen awake; the step timer chip (IRS); "Used in" links to the steps that use a
      control; every menu item in coverage.md (Flight deck, Practice, Glossary, Flight log,
      Tour, References, How procedures work). The coverage.md rows added for review apply.
      Find (Ctrl K) in the header, built from a results board the owner will provide; hidden
      in the header until then (owner, 2026-11-13).
   7. Content. First on the sourcing list (owner, 2026-11-12): the resting position of FLT
      CONTROL A and B, which the pack rests at STBY RUD where they are normally guarded ON; and
      the placards of the 11 positions not in capitals (`pack check` warns on each): "OFF to
      BRIGHT" on eight lighting knobs, "Guarded" on generator DISCONNECT 1 and 2, and the air
      temperature SOURCE selector's "Supply duct, passenger cabin, pack". On the validation list
      under decision 0007: the 17 glossary entries written in batch 2 (P1, P2, P3, P5, P7, P9,
      center tank, main tank, crossfeed, autobrake, start lever, probe heat, window heat, outflow
      valve, standby power, yaw damper, trim air, anti-ice). Then
      the center tank decision step (the pilot answers until the pack has a sourced
      wing-tank capacity; then the app may suggest an answer for the pilot to confirm) and every
      mockup wording in content-to-validate.md, sourced and validated under decision 0007 before
      it ships.
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
    A short privacy note in the app that says what is stored (progress, settings, the remembered
    SimBrief username and the compact flight plan, with no pilot name or ID; decision 0019), where
    (in this browser, on this device only), that nothing is sent to us, that SimBrief
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
