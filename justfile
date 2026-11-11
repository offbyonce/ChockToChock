# SPDX-FileCopyrightText: 2026 offbyonce
# SPDX-License-Identifier: MIT
#
# Every task lives here. Git hooks and GitHub Actions call these recipes, so a
# check that passes locally passes everywhere. Run `just` to list them.

set windows-shell := ["powershell.exe", "-NoLogo", "-NoProfile", "-Command"]

# List the recipes.
default:
    @just --list

# Install JS packages and fetch Rust crates (run once after cloning).
setup:
    pnpm install
    cargo fetch

# Run the web app's dev server (the app is at /app/).
dev: wasm web-packs
    pnpm --filter @chocktochock/web dev

# Format everything in place.
fmt:
    cargo fmt --all
    pnpm exec biome check --write .

# Fail if anything is not formatted.
fmt-check:
    cargo fmt --all --check
    pnpm exec biome format .

# Lints and type checks (warnings fail).
lint: wasm
    cargo clippy --workspace --all-targets -- -D warnings
    pnpm exec biome check .
    pnpm --filter @chocktochock/ui check
    pnpm --filter @chocktochock/web check

# All tests: Rust (nextest + doctests), the interface (Vitest), the 3.0 converter and the web site.
test: wasm web-packs
    cargo nextest run --workspace
    cargo test --doc --workspace
    pnpm --filter @chocktochock/ui test
    node --test "tools/import-3x/*.test.js" "apps/web/*.test.js" "apps/web/build/*.test.js"

# Build the web host as WebAssembly into apps/web/src/wasm. Needs the wasm-bindgen CLI at the
# same version as the crate (decision 0010).
wasm:
    cargo build -q -p web-host --target wasm32-unknown-unknown --profile wasm
    wasm-bindgen --target web --remove-name-section --remove-producers-section --out-dir apps/web/src/wasm target/wasm32-unknown-unknown/wasm/web_host.wasm

# Check every pack and write it as flattened JSON for the web app to fetch.
web-packs:
    cargo run -q -p pack-cli -- build packs apps/web/public/packs

# Build the deployable web site into apps/web/dist (published to GitHub Pages): the placeholder
# front page, the work-in-progress app at app/ and the SimBrief probe page, with the WebAssembly
# module and the flattened packs.
site: wasm web-packs
    pnpm --filter @chocktochock/web build

# The service worker's version-to-version update test in headless Firefox (needs Firefox; not
# part of `just ci`). See apps/web/build/update-test.mjs.
update-test: site
    node apps/web/build/update-test.mjs

# Regenerate the pack JSON Schemas from the Rust types.
schemas:
    cargo run -q -p pack-cli -- schema schemas

# Regenerate the TypeScript types from the Rust types.
bindings:
    cargo test -q --workspace export_bindings

# Fail if generated schemas or bindings differ from what is committed.
generated-check: schemas bindings
    git diff --exit-code -- schemas packages/bindings/src/generated

# Validate every pack, including the content rules.
packs:
    cargo run -q -p pack-cli -- check packs

# Compare pack text with private reference files named by CHOCKTOCHOCK_OVERLAP_REFS.
overlap:
    cargo run -q -p pack-cli -- overlap packs --allow crates/pack-overlap/allow.txt

# Licenses, advisories, banned crates and sources (Rust), then JS advisories.
deny:
    cargo deny check
    pnpm audit --prod

# Everything a change must pass. Same command locally and in CI.
ci: fmt-check lint test generated-check packs site deny
    @echo "All checks passed."
