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
  it("shows a step with controls in the board's order: value, Check, If not, Why, map, controls", () => {
    // The pane failed to render a step with drawings: binding an unset control card threw.
    render(StepPane, {
      index,
      step: step("before-start.fuel-pump-switches")!,
      targets: [],
      number: 6,
      total: 9,
      previous: step("before-start.terr-efis-control-panel"),
      mapMode: "new_panel",
      onMap: () => {},
    });
    expect(screen.getByText("Step 6 of 9 · Fuel pump switches")).toBeTruthy();
    const order = [...document.querySelectorAll(".pane-step > *")].map((e) =>
      ["head", "check", "ifnot", "why", "map", "controls"].find((c) => e.classList.contains(c)),
    );
    expect(order.filter(Boolean)).toEqual(["head", "check", "ifnot", "why", "map", "controls"]);
    // A new panel since the step before: the map figure says so.
    expect(screen.getByText("New panel")).toBeTruthy();
  });

  it("shows the one-line location instead of the map when the panel did not change", async () => {
    let asked = "";
    render(StepPane, {
      index,
      step: step("before-start.fuel-pump-switches")!,
      targets: [],
      number: 6,
      total: 9,
      previous: step("before-start.fuel-pump-switches"),
      mapMode: "on_request",
      onMap: (how: "hide" | "show") => (asked = how),
    });
    expect(document.querySelector(".map")).toBeNull();
    await fireEvent.click(screen.getByRole("button", { name: "Show map" }));
    expect(asked).toBe("show");
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
      mapMode: "off",
      onMap: () => {},
    });
    const button = screen.getByRole("button", { name: /^Terms:/ });
    await fireEvent.click(button);
    for (const t of withTerms.terms ?? []) expect(screen.getByText(t)).toBeTruthy();
  });

  it("names dashed and ◆ in a drawing's legend only when that drawing has them", () => {
    const t = (item: number, open: boolean, resolved: boolean, section = "fuel") => ({
      section,
      item,
      position: 1,
      lit: false,
      open,
      resolved,
    });
    const pane = (targets: ReturnType<typeof t>[]) =>
      render(StepPane, {
        index,
        step: step("before-start.fuel-pump-switches")!,
        targets,
        number: 6,
        total: 9,
        mapMode: "off",
        onMap: () => {},
      });
    const legend = () => document.querySelector(".controls figcaption")?.textContent ?? "";
    // A dashed control on another panel (the aft overhead) stays out of the fuel drawing's legend.
    const plain = pane([t(17, false, false), t(0, true, true, "elt")]);
    expect(document.querySelectorAll(".controls")).toHaveLength(2);
    expect(legend()).not.toMatch(/dashed|◆/);
    plain.unmount();
    pane([t(11, true, false), t(17, false, true)]);
    expect(legend()).toMatch(/dashed: only if the step's condition applies/);
    expect(legend()).toMatch(/◆ set by a decision step/);
  });
});
