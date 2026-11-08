<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# 0003: MIT code, CC BY-NC-SA 4.0 content, DCO sign-off

Date: 2026-11-08. Status: accepted.

## Decision
- Code: MIT. Checklist content in `packs/`: CC BY-NC-SA 4.0, so it can be shared and
  adapted with credit but not sold. Recorded per file with REUSE (`REUSE.toml`).
- The repository is public on GitHub under offbyonce and starts with fresh history
  (no 3.0 history is imported).
- Contributors sign off each commit (Developer Certificate of Origin). No CLA.

## Consequences
- Contributed content stays under CC BY-NC-SA; relicensing it later would need each
  contributor's permission.
- A commit-msg hook and review enforce the sign-off.
