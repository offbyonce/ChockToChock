// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Loads the built WebAssembly module in Node and calls each export once, including on the
// flattened iFly pack. Run `just wasm web-packs` first (`just test` does).

import assert from "node:assert/strict";
import fs from "node:fs";
import path from "node:path";
import { test } from "node:test";
import * as wasm from "./src/wasm/web_host.js";

const here = import.meta.dirname;
wasm.initSync({ module: fs.readFileSync(path.join(here, "src/wasm/web_host_bg.wasm")) });

const fixture = (name) =>
  fs.readFileSync(path.join(here, "../../crates/simbrief/tests/fixtures", name), "utf8");
const ok = (answer) => {
  const reply = JSON.parse(answer);
  assert.ok("ok" in reply, answer);
  return reply.ok;
};

test("the module answers with app facts and the example checklist", () => {
  assert.equal(ok(wasm.app_info()).name, "Chock to Chock");
  assert.equal(ok(wasm.example_checklist()).steps.length, 3);
});

test("the flattened iFly pack loads", () => {
  const pack = ok(
    wasm.load_pack(fs.readFileSync(path.join(here, "public/packs/ifly-737-max8.json"), "utf8")),
  );
  assert.equal(pack.manifest.id, "ifly-737-max8");
  assert.equal(pack.lists.length, 65);
});

test("SimBrief plans read in both forms, and errors arrive tagged", () => {
  assert.equal(ok(wasm.parse_plan(fixture("ofp-sample.json"))).route, "SUMMA2 SUMMA J1 BTG");
  assert.equal(ok(wasm.parse_plan(fixture("ofp-sample.xml"))).origin, "EGLL");
  assert.deepEqual(JSON.parse(wasm.parse_plan(fixture("ofp-error.json"))), {
    error: { kind: "FlightPlan", message: "SimBrief says: Error: Unknown UserID" },
  });
  assert.match(ok(wasm.plan_url("123456")), /userid=123456&json=1$/);
});

test("the module keeps a loaded pack for flights, actions and the rail", () => {
  ok(wasm.load_pack(fs.readFileSync(path.join(here, "public/packs/ifly-737-max8.json"), "utf8")));
  const flight = ok(wasm.new_flight("ifly-737-max8"));
  assert.equal(flight.options.power, "gpu");
  const step = "ifly-737-max8.exterior.left-forward-fuselage";
  const skipped = ok(
    wasm.apply("ifly-737-max8", JSON.stringify(flight), JSON.stringify({ type: "skip", step, at: 1 })),
  );
  const settings = JSON.stringify(ok(wasm.default_settings()));
  const rail = ok(wasm.rail("ifly-737-max8", JSON.stringify(skipped), settings));
  const exterior = rail.stages[0].phases.find((p) => p.id === "exterior");
  assert.equal(exterior.skipped, 1);
  assert.equal(exterior.complete, false);
  assert.deepEqual(
    rail.reference.map((p) => p.id),
    ["memory", "quick-reference", "procedures"],
  );
  const open = ok(wasm.open_steps("ifly-737-max8", JSON.stringify(skipped), settings, ""));
  assert.equal(open.find((o) => o.step === step)?.skipped, true);
  const essentials = JSON.stringify({ ...JSON.parse(settings), essentials: true });
  const view = ok(wasm.phase_view("ifly-737-max8", JSON.stringify(skipped), essentials, "preflight"));
  assert.ok(view.lists.every((l) => l.steps.every((s) => s.essential)));
  assert.equal(JSON.parse(wasm.load_flight('{"format":99}')).ok.state, "newer");
});
