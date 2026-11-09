<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# import-3x

Converts the 3.0 checklist into pack YAML (Phase 1, step 4). It holds no 3.0 content: everything
is read from a local copy of the 3.0 source at run time, and its tests use invented data.

```sh
node tools/import-3x/convert.js --src <3.0 src folder> --out <output folder> [--report <file.json>]
```

The generated pack is not committed until the text-overlap check has run over it.
