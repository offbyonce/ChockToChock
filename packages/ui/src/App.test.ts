// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import type { FlatPack, Flight, Rail, RailPhase, Settings } from "@chocktochock/bindings";
import { render, screen } from "@testing-library/svelte";
import { beforeEach, describe, expect, it } from "vitest";
import App from "./App.svelte";
import { type Host, hostContext } from "./host";

function phase(id: string, title: string, done: number, skipped: number, total: number): RailPhase {
  const progress = { done, skipped, toDo: total - done - skipped, total };
  return { id, title, progress, complete: done === total, open: total - done, skipped };
}

const rail: Rail = {
  stages: [
    {
      title: "Preflight",
      phases: [phase("power-up", "Power up", 2, 1, 6), phase("exterior", "Exterior", 18, 0, 18)],
    },
    { title: "Departure", phases: [phase("taxi", "Taxi", 0, 0, 9)] },
  ],
  reference: [phase("memory", "Memory items", 0, 0, 12)],
};

function host(overrides: Partial<Host> = {}): Host {
  return {
    appInfo: async () => ({ name: "Chock to Chock", version: "0.1.0", packSchemaVersion: 2 }),
    packs: async () => [{ id: "demo", name: "Demo", version: "0.1.0", kind: "aircraft" }],
    loadPack: async () => ({}) as FlatPack,
    newFlight: async () => ({ pack: "demo" }) as Flight,
    apply: async (_pack, flight) => flight,
    rail: async () => rail,
    defaultSettings: async () => ({ theme: "dark", dimming: 0 }) as Settings,
    ...overrides,
  };
}

beforeEach(() => {
  window.location.hash = "";
});

describe("App", () => {
  it("shows the stages and phases from the rail, with progress in words", async () => {
    render(App, { context: hostContext(host()) });

    expect(await screen.findByRole("heading", { level: 1, name: "Power up" })).toBeTruthy();
    expect(screen.getByText("2 of 6 done · 1 skipped, still open · 3 to do")).toBeTruthy();
    expect(screen.getByText("1 skipped")).toBeTruthy();
    expect(screen.getByRole("link", { name: /Exterior, Complete/ })).toBeTruthy();
    expect(screen.getByRole("link", { name: /Emergency/ }).getAttribute("href")).toBe("#/reference");
  });

  it("shows a tagged error from Rust", async () => {
    const failing = host({
      loadPack: () => Promise.reject({ kind: "PackInvalid", message: "pack format 3 is newer" }),
    });
    render(App, { context: hostContext(failing) });

    expect((await screen.findByRole("alert")).textContent).toBe("PackInvalid: pack format 3 is newer");
  });
});
