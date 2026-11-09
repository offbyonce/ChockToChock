// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import assert from "node:assert/strict";
import { test } from "node:test";

import { toYaml } from "./yaml.js";

test("words a strict reader could misread are quoted", () => {
  const text = toYaml({ a: "ON", b: "OFF", c: "NO", d: "null", e: "Battery switch" });
  assert.equal(text, 'a: "ON"\nb: "OFF"\nc: "NO"\nd: "null"\ne: Battery switch\n');
});

test("punctuation and non-ASCII text are quoted", () => {
  assert.equal(toYaml({ a: "x: y", b: "□□", c: "1/2" }), 'a: "x: y"\nb: "□□"\nc: "1/2"\n');
});

test("nested data uses block style, with a header", () => {
  const text = toYaml({ list: [{ id: "a", tags: ["p", "q"] }, "plain"], empty: [] }, ["# note"]);
  assert.equal(text, "# note\nlist:\n  - id: a\n    tags:\n      - p\n      - q\n  - plain\nempty: []\n");
});

test("numbers and booleans stay unquoted", () => {
  assert.equal(toYaml({ a: 12.5, b: true }), "a: 12.5\nb: true\n");
});
