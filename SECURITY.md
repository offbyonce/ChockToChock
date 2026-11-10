<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# Security

## Reporting a vulnerability

Please report security problems privately through GitHub's
**Security → Report a vulnerability** form on this repository, not in a public issue.
This is a hobby project; reports are answered on a best-effort basis.

## What the app does and does not do

- The interface can call only the app's own commands. It has no direct file, network or
  shell access, and a strict content security policy blocks remote scripts.
- Network requests go only to SimBrief (when you import a flight plan, with no cookies
  sent) and, in later versions, to this repository's releases to check for updates.
- Your SimBrief username and checklist progress are stored only on your computer.
- The app collects no telemetry.

## For maintainers

- Signing and update keys never live in this repository. They are passed to release
  builds only, never to builds of pull requests.
- GitHub secret scanning and push protection are enabled.
- `just deny` checks dependency licenses and known advisories.
