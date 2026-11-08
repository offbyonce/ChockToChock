<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0005: Notify-only updates via GitHub Releases; unsigned beta

Date: 2026-11-08. Status: accepted.

## Decision
- The app checks the latest GitHub Release and, if newer, shows the notes and a link.
  No automatic install yet.
- The Tauri updater key pair is generated before the first release and backed up, so
  in-app installs can be switched on later without stranding users.
- Beta builds ship unsigned. Signing (SignPath Foundation if eligible, otherwise Azure
  Artifact Signing or an OV certificate) is revisited before a wider release.

## Consequences
- Unsigned builds show a SmartScreen warning and are blocked where Windows 11 Smart App
  Control is on.
