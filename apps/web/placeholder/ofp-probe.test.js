// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Uses the simbrief crate's invented fixtures, so the probe and the crate read the same data.

import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { test } from "node:test";
import { fetchUrl, readPlan } from "./ofp-probe.js";

const fixtures = path.join(import.meta.dirname, "../../../crates/simbrief/tests/fixtures");
const fixture = (name) => fs.readFileSync(path.join(fixtures, name), "utf8");

test("a Pilot ID and a username make the same addresses as the simbrief crate", () => {
  assert.equal(fetchUrl(" 123456 "), "https://www.simbrief.com/api/xml.fetcher.php?userid=123456&json=1");
  assert.equal(
    fetchUrl("Pilot One"),
    "https://www.simbrief.com/api/xml.fetcher.php?username=Pilot%20One&json=1",
  );
  assert.throws(() => fetchUrl("  "), /Enter a SimBrief username/);
});

test("the sample plan gives the fields the page shows", () => {
  const fields = readPlan(fixture("ofp-sample.json"));
  assert.equal(fields.Flight, "TSTX4567");
  assert.equal(fields.Route, "SUMMA2 SUMMA J1 BTG");
  assert.equal(fields["Cruise altitude"], "37000 ft");
  assert.equal(fields.From, "EGLL, runway 09R");
  assert.equal(fields.To, "LFPG");
  assert.equal(fields["Ramp fuel"], "6400 kgs");
  assert.equal(fields.AIRAC, "2406");
  assert.match(fields["Navlog fixes"], /^\d+$/);
});

test("SimBrief's error status and text that is not a plan give the exact reason", () => {
  assert.throws(() => readPlan(fixture("ofp-error.json")), /SimBrief says: Error: Unknown UserID/);
  assert.throws(() => readPlan("{not json"), /not JSON/);
  assert.throws(() => readPlan("Service unavailable"), /not a SimBrief plan/);
  assert.throws(() => readPlan("<OFP></OFP>"), /cannot read XML/);
});
