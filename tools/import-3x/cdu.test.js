// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import assert from "node:assert/strict";
import { test } from "node:test";

import { convertCdu, findHits, flatten, leakedMarkers, verifyCdu } from "./cdu.js";

const table = {
  values: [
    { value: "ZZAA", target: { field: "origin" } },
    { value: "99X", target: { field: "runway" } },
    { value: "15", target: { field: "takeoff_flaps" } },
    { value: "WPA01", target: { field: "route_leg", index: 1, part: "to" } },
    { value: "1234z", target: { field: "utc_time" }, always: true },
  ],
  whole: [{ value: "+1F +2C", target: { field: "isa_deviation" } }],
  leg1Via: { value: "DIRECT", target: { field: "route_leg", index: 1, part: "via" } },
  overflow: { value: "7", target: { route_legs: "overflow" } },
};

const row = (...cells) => cells;
const sep = [{ sep: 1 }, "<BACK", "", "NEXT>"];

function page(overrides, rows, screen = {}) {
  return {
    pg: "DEMO",
    t: "Demo page",
    a: "Do it.",
    c: "Done.",
    s: { t: "DEMO", p: "1/1", hl: [], r: [...rows, sep], ...screen },
    ...overrides,
  };
}

function convertOne(blank, filled) {
  const gaps = [];
  const pages = convertCdu([blank], [filled], table, gaps);
  assert.deepEqual(gaps, []);
  assert.deepEqual(verifyCdu(pages, [blank], [filled], table, { pages: 1 }), []);
  return pages[0];
}

test("cells flatten into text and styles", () => {
  assert.deepEqual(flatten(["a", { u: "b" }, { g: "c" }, { a: "d" }, ""]), [
    { text: "a", style: null },
    { text: "b", style: "user" },
    { text: "c", style: "data" },
    { text: "d", style: "accent" },
  ]);
});

test("numbers only match on digit boundaries", () => {
  assert.equal(findHits("1500AGL", table.values).length, 0);
  assert.equal(findHits("15 degrees", table.values).length, 1);
});

test("a live value keeps its style, literal text and placeholder", () => {
  const blank = page({}, [row("RWY", "-----", "", "")]);
  const filled = page({}, [row("RWY", { u: "RW99X" }, "", "")]);
  assert.deepEqual(convertOne(blank, filled).screen.rows[0].left, [
    { field: "runway", style: "user", before: "RW", empty: "-----" },
  ]);
});

test("a value mixed with other text becomes a group", () => {
  const blank = page({}, [row("", { g: "(none)" }, "", "")]);
  const filled = page({}, [row("", ["<SEL> ", { u: "99X" }], "", "")]);
  assert.deepEqual(convertOne(blank, filled).screen.rows[0].left, [
    {
      when_known: ["runway"],
      show: ["<SEL> ", { field: "runway", style: "user" }],
      otherwise: [{ text: "(none)", style: "data" }],
    },
  ]);
});

test("prose splits around values when the rest also appears when blank", () => {
  const blank = page({ a: "Type the airport and press 1L." }, []);
  const filled = page({ a: "Type ZZAA and press 1L." }, []);
  assert.deepEqual(convertOne(blank, filled).action, [
    "Type ",
    { field: "origin", empty: "the airport" },
    " and press 1L.",
  ]);
});

test("values that are always live are found even in the blank run", () => {
  const blank = page({}, [row("UTC", { g: "1234z" }, "", "")]);
  const converted = convertOne(blank, blank);
  assert.deepEqual(converted.screen.rows[0].left, [{ field: "utc_time", style: "data" }]);
});

test("route rows use the indexed field, and the first row's via is recognized", () => {
  const blank = page({}, [row("VIA", "------", "TO", "------")]);
  const filled = page({}, [row("VIA", { u: "DIRECT" }, "TO", { u: "WPA01" })]);
  const [first] = convertOne(blank, filled).screen.rows;
  assert.equal(first.left[0].field, "route_leg");
  assert.equal(first.left[0].part, "via");
  assert.deepEqual(first.right, [
    { field: "route_leg", index: 1, part: "to", style: "user", empty: "------" },
  ]);
});

test("a combined value is treated as one field", () => {
  const blank = page({}, [row("ISA", "---", "", "")]);
  const filled = page({}, [row("ISA", [{ u: "+1F" }, " ", { u: "+2C" }], "", "")]);
  assert.equal(convertOne(blank, filled).screen.rows[0].left[0].field, "isa_deviation");
});

test("route counts use the page's row count", () => {
  const blank = page({ n: "Leave it." }, [row("A", "b", "", "")], { p: "1/1" });
  const filled = page({ n: "7 more follow." }, [row("A", { u: "WPA01" }, "", "")], { p: "1/3" });
  const gaps = [];
  const [converted] = convertCdu([blank], [filled], table, gaps);
  assert.deepEqual(converted.screen.page, ["1/", { route_legs: "pages", per_page: 1 }]);
  assert.deepEqual(converted.note, [
    { route_legs: "overflow", per_page: 1, after: " more follow.", empty: "Leave it." },
  ]);
  assert.deepEqual(verifyCdu([converted], [blank], [filled], table, { pages: 3 }), []);
});

test("text that changes with no recognizable value is reported", () => {
  const blank = page({ a: "One." }, []);
  const filled = page({ a: "Two." }, []);
  const gaps = [];
  const [converted] = convertCdu([blank], [filled], table, gaps);
  assert.equal(gaps.length, 1);
  assert.deepEqual(converted.action, ["One."]);
});

test("the round trip catches a wrong conversion", () => {
  const blank = page({}, [row("RWY", "-----", "", "")]);
  const filled = page({}, [row("RWY", { u: "99X" }, "", "")]);
  const gaps = [];
  const pages = convertCdu([blank], [filled], table, gaps);
  pages[0].screen.rows[0].left = ["wrong"];
  assert.equal(verifyCdu(pages, [blank], [filled], table, { pages: 1 }).length, 2);
});

test("leaked markers are found", () => {
  assert.deepEqual(leakedMarkers([{ a: "fine", b: ["x ZZAA"] }], table), ["ZZAA"]);
  assert.deepEqual(leakedMarkers([{ a: "fine" }], table), []);
});
