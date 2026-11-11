<!--
SPDX-FileCopyrightText: 2026 offbyonce
SPDX-License-Identifier: MIT
-->
# import-3x

**Retired.** The iFly pack was imported once and is now edited by hand, so this converter is no
longer run. It is kept for reference, to show how the 3.0 data was mapped.

Converts the 3.0 checklist into pack YAML (Phase 1, step 4). It holds no 3.0 content: everything
is read from a local copy of the 3.0 source at run time, and its tests use invented data.

```sh
node tools/import-3x/convert.js --src <3.0 src folder> [--out <output folder>] [--report <file.json>]
```

The output defaults to a folder outside the repo. The converter refuses to write into `packs/`,
because the validated pack there is edited by hand and regenerating it would overwrite that work.
