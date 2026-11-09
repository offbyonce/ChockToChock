<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# Development setup

One-time installs, then `just setup`. Rust's exact version comes from
`rust-toolchain.toml`; rustup downloads it automatically the first time you build.

## macOS

```sh
xcode-select --install                       # Apple's compiler tools
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh   # Rust (rustup)
brew install node just                       # Node 22+ and the task runner
corepack enable                              # pinned pnpm (Node 25+ has no corepack: npm i -g pnpm@12.18.0)
cargo install --locked cargo-nextest cargo-deny
```

## Windows

1. Install the **Microsoft C++ Build Tools** with the "Desktop development with C++" workload.
2. Install Rust from <https://rustup.rs>.
3. In PowerShell:

   ```powershell
   winget install OpenJS.NodeJS.LTS Casey.Just
   corepack enable
   cargo install --locked cargo-nextest cargo-deny
   ```

WebView2 is already part of Windows 10 and 11.

## Linux (Ubuntu 22.04 or newer)

```sh
sudo apt install libwebkit2gtk-4.1-dev libglib2.0-dev pkg-config build-essential file \
  libxdo-dev libssl-dev libayatana-appindicator3-dev librsvg2-dev
```

Then Rust, Node 22+, pnpm and the tools:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
# Node 22-24 include corepack; Node 25 and later do not, so install pnpm with npm instead:
corepack enable || npm install -g --prefix ~/.local pnpm@12.18.0   # ~/.local/bin must be on PATH
cargo install --locked just cargo-nextest cargo-deny
```

## Every machine

```sh
just setup    # pnpm install (also installs git hooks) + cargo fetch
just dev      # run the app with live reload
just ci       # every check; run before pushing
```

Optional: `pipx install reuse`, then `reuse lint` checks every file has a licence.
