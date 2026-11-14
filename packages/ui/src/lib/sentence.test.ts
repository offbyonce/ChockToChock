// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import type { FlatPack } from "@chocktochock/bindings";
import { describe, expect, it } from "vitest";
import { indexPack } from "../pack-index";
import { stepTitle } from "./sentence";

const packs = import.meta.glob<FlatPack>("../../../../apps/web/public/packs/ifly-737-max8.json", {
  eager: true,
  import: "default",
});
const pack = Object.values(packs)[0] as FlatPack;
const index = indexPack(pack);
const step = (id: string) => index.step(`ifly-737-max8.${id}`);

describe("stepTitle", () => {
  it("keeps cockpit text in capitals and the rest as a sentence", () => {
    // "STAB out of trim light": the light's label lost its capitals.
    expect(stepTitle(index, step("preflight.stab-out-of-trim-light")!)).toBe("STAB OUT OF TRIM light");
    expect(stepTitle(index, step("before-start.fuel-pump-switches")!)).toBe("Fuel pump switches");
    expect(stepTitle(index, step("preflight.crossfeed-selector")!)).toBe("CROSSFEED selector");
  });
});
