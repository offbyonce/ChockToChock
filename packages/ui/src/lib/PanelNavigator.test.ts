// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import type { Panel } from "@chocktochock/bindings";
import { cleanup, fireEvent, render, screen } from "@testing-library/svelte";
import { afterEach, describe, expect, it, vi } from "vitest";
import PanelNavigator from "./PanelNavigator.svelte";

afterEach(cleanup);

const panel = {
  id: "mn",
  title: "Main",
  width: 1000,
  height: 120,
  sections: [{ id: "cl", title: "Captain lights", x: 0, y: 0, width: 918, height: 109, items: [] }],
} as unknown as Panel;

describe("PanelNavigator", () => {
  it("shows the whole panel with a frame, and moves the labeled view with the arrow keys", async () => {
    const scroller = document.createElement("div");
    scroller.innerHTML = '<svg viewBox="0 0 918 109"></svg>';
    Object.defineProperty(scroller, "clientWidth", { value: 400 });
    Object.defineProperty(scroller, "clientHeight", { value: 200 });
    const scrollBy = vi.fn();
    scroller.scrollBy = scrollBy as unknown as typeof scroller.scrollBy;
    document.body.append(scroller);
    render(PanelNavigator, { panel, focus: ["cl"], targets: [], scroller, title: "Captain lights panel" });
    const nav = screen.getByRole("slider");
    expect(nav.querySelector(".marker")).not.toBeNull();
    await fireEvent.keyDown(nav, { key: "ArrowRight" });
    expect(scrollBy).toHaveBeenCalledWith({ left: 100, top: 0 });
    scroller.remove();
  });
});
