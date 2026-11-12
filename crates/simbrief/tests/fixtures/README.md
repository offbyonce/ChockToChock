<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# Test fixtures

Every fixture here is invented for testing. None contains a real person's name,
a real SimBrief account, or text copied from a manual.

- `ofp-*.json`: made-up SimBrief OFP responses (JSON), including the `{}` that SimBrief
  sends for empty values.
- `ofp-sample.xml`: the same made-up plan as `ofp-sample.json`, in SimBrief's XML form.

The sample plan carries invented personal fields (a pilot "Pilot", the ID 123456) only so a test
can check that the compact plan never reads them.
