// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import type { FlatPack, Step } from "@chocktochock/bindings";
import { cleanup, render, screen } from "@testing-library/svelte";
import { afterEach, describe, expect, it } from "vitest";
import { indexPack } from "../pack-index";
import StepDetail from "./StepDetail.svelte";

const pack = {
  manifest: { name: "Demo" },
  lists: [],
  content: {
    phases: null,
    controls: null,
    panels: [],
    notes: {
      how: [{ id: "baro", steps: ["Set the minimums.", "Check both sides agree."] }],
      explain: [
        { id: "fire", what: "A fire test.", why: "Detection must work.", success: "Bell and lights." },
      ],
    },
  },
} as unknown as FlatPack;

afterEach(cleanup);

describe("StepDetail", () => {
  it("shows a shared how-to note's steps, not its id, and the explanation", () => {
    const step = { id: "s", text: "MINIMUMS", expect: "SET", how: "baro", explain: "fire" } as Step;
    render(StepDetail, { index: indexPack(pack), step });

    expect(screen.getByText("How to do it")).toBeTruthy();
    expect(screen.getByText("Set the minimums.")).toBeTruthy();
    expect(screen.getByText("Check both sides agree.")).toBeTruthy();
    expect(screen.queryByText(/baro/)).toBeNull();
    expect(screen.getByText(/Bell and lights\./)).toBeTruthy();
  });
});
