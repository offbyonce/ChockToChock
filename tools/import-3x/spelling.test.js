// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import assert from "node:assert/strict";
import { test } from "node:test";

import { normalizeDeep, normalizeSpelling, SpellingLog } from "./spelling.js";

test("British words become American, keeping case", () => {
  assert.equal(normalizeSpelling("the colour of the Centre TYRE"), "the color of the Center TIRE");
});

test("changes are recorded with counts", () => {
  const log = new SpellingLog();
  normalizeSpelling("colour, Colour and pressurisation", log);
  assert.deepEqual(log.summary().changes, {
    "colour -> color": 2,
    "pressurisation -> pressurization": 1,
  });
});

test("only whole words change", () => {
  assert.equal(normalizeSpelling("recentre colourful"), "recentre colourful");
});

test("words not on the list but looking British are reported, ordinary words are not", () => {
  const log = new SpellingLog();
  normalizeSpelling("harmonise the hour, otherwise uncontrolled", log);
  assert.deepEqual(log.summary().suspects, { harmonise: 1 });
});

test("ids and fixed vocabulary are left alone, prose is changed", () => {
  const out = normalizeDeep({
    id: "x.colour",
    text: "colour",
    controls: ["colour"],
    steps: [{ note: "centre" }],
  });
  assert.deepEqual(out, { id: "x.colour", text: "color", controls: ["colour"], steps: [{ note: "center" }] });
});
