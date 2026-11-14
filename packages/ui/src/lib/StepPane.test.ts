// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import type { FlatPack } from "@chocktochock/bindings";
import { cleanup, fireEvent, render, screen } from "@testing-library/svelte";
import { afterEach, describe, expect, it } from "vitest";
import { indexPack } from "../pack-index";
import StepPane from "./StepPane.svelte";

const packs = import.meta.glob<FlatPack>("../../../../apps/web/public/packs/ifly-737-max8.json", {
  eager: true,
  import: "default",
});
const pack = Object.values(packs)[0] as FlatPack;
const index = indexPack(pack);
const step = (id: string) => index.step(`ifly-737-max8.${id}`);

afterEach(cleanup);

describe("StepPane", () => {
  const fuel = () => ({
    index,
    step: step("before-start.fuel-pump-switches")!,
    targets: [{ section: "fuel", item: 17, position: 1, lit: false, open: false, resolved: false }],
    number: 6,
    total: 9,
  });

  it("shows the round 6 pane in order: value, Check, If not, Why, location line, panel preview", () => {
    render(StepPane, { ...fuel(), onOpen: () => {} });
    expect(screen.getByText("Step 6 of 9 · Fuel pump switches")).toBeTruthy();
    const order = [...document.querySelectorAll(".pane-step > *")].map((e) =>
      ["head", "check", "ifnot", "why", "line", "preview"].find((c) => e.classList.contains(c)),
    );
    expect(order.filter(Boolean)).toEqual(["head", "check", "ifnot", "why", "line", "preview"]);
    // No map figure, no labeled drawing, no "New panel" tag (round 6).
    expect(document.querySelector(".map, .controls")).toBeNull();
    expect(screen.queryByText(/New panel/)).toBeNull();
    expect(document.querySelectorAll(".preview text").length).toBeGreaterThan(0);
    expect(document.querySelector(".preview g")?.classList.contains("textless")).toBe(true);
  });

  it("opens the slide-out on Where it is from the location line and on the panel from the preview", async () => {
    const opened: unknown[] = [];
    render(StepPane, { ...fuel(), onOpen: (how: unknown) => opened.push(how) });
    await fireEvent.click(screen.getByRole("button", { name: /Where is this\?/ }));
    await fireEvent.click(screen.getByRole("button", { name: "Show the whole Fuel panel" }));
    expect(opened).toEqual([{ entry: "where" }, { entry: "panel", panel: "oh" }]);
  });

  it("shows Show panel instead of the preview when Panel preview in steps is off", async () => {
    const opened: unknown[] = [];
    render(StepPane, { ...fuel(), preview: false, onOpen: (how: unknown) => opened.push(how) });
    expect(document.querySelector("button.preview")).toBeNull();
    await fireEvent.click(screen.getByRole("button", { name: /Show panel/ }));
    expect(opened).toEqual([{ entry: "panel" }]);
  });

  it("lists the step's terms with their glossary meanings", async () => {
    const withTerms = pack.lists.flatMap((l) => l.steps).find((s) => (s.terms ?? []).length > 1);
    if (!withTerms) throw new Error("no step with terms");
    render(StepPane, {
      index,
      step: withTerms,
      targets: [],
      number: 1,
      total: 1,
      onOpen: () => {},
    });
    const button = screen.getByRole("button", { name: /^Terms:/ });
    await fireEvent.click(button);
    for (const t of withTerms.terms ?? []) expect(screen.getByText(t)).toBeTruthy();
  });
});
