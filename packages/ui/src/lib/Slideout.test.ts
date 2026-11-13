// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import type { FlatPack } from "@chocktochock/bindings";
import { cleanup, fireEvent, render, screen } from "@testing-library/svelte";
import { afterEach, describe, expect, it } from "vitest";
import { indexPack } from "../pack-index";
import Slideout from "./Slideout.svelte";

const packs = import.meta.glob<FlatPack>("../../../../apps/web/public/packs/ifly-737-max8.json", {
  eager: true,
  import: "default",
});
const pack = Object.values(packs)[0] as FlatPack;
const index = indexPack(pack);
const step = index.step("ifly-737-max8.before-start.fuel-pump-switches");
const fuel = pack.content.panels.find((p) => p.id === "oh")?.sections.find((s) => s.id === "fuel");
const ctrL = fuel?.items.findIndex((i) => i.label === "CTR L") ?? -1;
const aft1 = fuel?.items.findIndex((i) => i.label === "AFT 1") ?? -1;
const targets = [
  { section: "fuel", item: aft1, position: 1, lit: false, open: false, resolved: false },
  { section: "fuel", item: ctrL, position: null, lit: false, open: true, resolved: false },
];

afterEach(cleanup);

describe("Slideout", () => {
  const props = (entry: "panel" | "where" | "control") => ({
    index,
    step: step!,
    number: 6,
    targets,
    opening: entry === "control" ? { entry, control: { section: "fuel", item: ctrL } } : { entry },
    onClose: () => {},
    onGo: () => {},
  });

  it("opens on the panel with Where it is folded to one line, and closes on Esc", async () => {
    let closed = false;
    render(Slideout, { ...props("panel"), onClose: () => (closed = true) });
    expect(screen.getByRole("heading", { level: 2, name: "Fuel panel" })).toBeTruthy();
    expect(screen.getByText("Step 6 · Fuel pump switches")).toBeTruthy();
    expect(screen.getByRole("button", { name: /^Where it is/ }).getAttribute("aria-expanded")).toBe("false");
    await fireEvent.keyDown(screen.getByRole("dialog"), { key: "Escape" });
    expect(closed).toBe(true);
  });

  it("opens from Where is this? with Where it is expanded: the views and the area map", () => {
    render(Slideout, props("where"));
    expect(screen.getByRole("button", { name: "Where it is" }).getAttribute("aria-expanded")).toBe("true");
    expect(screen.getByText("From the side: how high, how far")).toBeTruthy();
    expect(screen.getByText("From the captain seat: which side")).toBeTruthy();
  });

  it("opens on a control's card, its control ringed, Where it is folded", () => {
    render(Slideout, props("control"));
    expect(screen.getByRole("heading", { level: 3, name: /^CTR L · / })).toBeTruthy();
    expect(screen.getByText("Positions")).toBeTruthy();
    expect(screen.getByText("OFF (up) · ON (down)")).toBeTruthy();
    expect(screen.getByText("In this step")).toBeTruthy();
    expect(document.querySelectorAll(".selection")).toHaveLength(1);
    expect(screen.getByRole("button", { name: /^Where it is/ }).getAttribute("aria-expanded")).toBe("false");
  });
});
