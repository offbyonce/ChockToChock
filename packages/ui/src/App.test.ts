// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import type {
  Action,
  FlatPack,
  Flight,
  PhaseView,
  Rail,
  RailPhase,
  Settings,
  StepState,
} from "@chocktochock/bindings";
import { cleanup, fireEvent, render, screen, within } from "@testing-library/svelte";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
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
      phases: [phase("power-up", "Power up", 0, 1, 2), phase("exterior", "Exterior", 18, 0, 18)],
    },
    { title: "Departure", phases: [phase("taxi", "Taxi", 0, 0, 9)] },
  ],
  reference: [phase("memory", "Memory items", 0, 0, 12)],
};

const pack = {
  manifest: { name: "Demo aircraft" },
  lists: [
    {
      id: "power-up-flow",
      title: "Power up flow",
      kind: "flow",
      steps: [
        {
          id: "battery",
          number: 1,
          text: "BATTERY",
          expect: "ON",
          why: "First power.",
          check: "Voltage shows",
        },
        // No number in the pack, as for some real steps; the shown number comes from Rust.
        { id: "standby", text: "STANDBY POWER", expect: "AUTO" },
      ],
    },
  ],
  content: {
    phases: {
      options: [
        {
          id: "power",
          title: "Power source",
          values: ["gpu", "apu"],
          default: "gpu",
          choices: [
            { value: "gpu", label: "Ground power at the gate", description: "Start the APU later." },
            { value: "apu", label: "No ground power", description: "Start the APU now." },
          ],
          ask_in: ["power-up"],
        },
      ],
      stages: [],
      phases: [],
    },
    controls: null,
    panels: [],
  },
} as unknown as FlatPack;

function view(states: [StepState, StepState]): PhaseView {
  const done = states.filter((s) => s === "done").length;
  const skipped = states.filter((s) => s === "skipped").length;
  const progress = { done, skipped, toDo: 2 - done - skipped, total: 2 };
  const ids = ["battery", "standby"];
  const current = ids[states.indexOf("current")] ?? null;
  return {
    id: "power-up",
    title: "Power up",
    reference: false,
    stage: "Preflight",
    position: [1, 2],
    previous: null,
    next: { id: "exterior", title: "Exterior" },
    lists: [
      {
        id: "power-up-flow",
        title: "Power up flow",
        kind: "flow",
        steps: ids.map((id, i) => ({
          id,
          number: i + 1,
          state: states[i] as StepState,
          counts: true,
          targets: [],
        })),
        progress,
        complete: done === 2,
        current,
      },
    ],
    progress,
    complete: done === 2,
  };
}

function host(overrides: Partial<Host> = {}): Host {
  return {
    appInfo: async () => ({ name: "Chock to Chock", version: "0.1.0", packSchemaVersion: 2 }),
    packs: async () => [{ id: "demo", name: "Demo", version: "0.1.0", kind: "aircraft" }],
    loadPack: async () => pack,
    newFlight: async () =>
      ({ pack: "demo", options: { power: "gpu" }, session: { done: {}, skipped: {} } }) as unknown as Flight,
    apply: async (_pack, flight) => flight,
    rail: async () => rail,
    phaseView: async () => view(["skipped", "current"]),
    defaultSettings: async () =>
      ({ theme: "dark", dimming: 0, detail: "expanded", listView: "checklist" }) as Settings,
    ...overrides,
  };
}

beforeEach(() => {
  window.location.hash = "";
});
afterEach(cleanup);

describe("App", () => {
  it("lays out stages, phase chips and steps, with states in words", async () => {
    render(App, { context: hostContext(host()) });

    expect(await screen.findByRole("heading", { level: 1, name: "Power up" })).toBeTruthy();
    const stages = screen.getByRole("navigation", { name: "Stages" });
    expect(within(stages).getByRole("link", { name: "Preflight" }).getAttribute("aria-current")).toBe("true");
    expect(screen.getByRole("link", { name: "Exterior, complete" })).toBeTruthy();
    expect(screen.getByRole("link", { name: "Power up, has skipped steps" })).toBeTruthy();
    expect(screen.getByRole("button", { name: "1BATTERY , skipped, still open ON" })).toBeTruthy();
    expect(screen.getByRole("button", { name: "2STANDBY POWER , current AUTO" })).toBeTruthy();
    const top = document.querySelector<HTMLElement>("header.top");
    expect(
      top &&
        within(top)
          .getByRole("link", { name: /Emergency/ })
          .getAttribute("href"),
    ).toBe("#/reference");
  });

  it("ticks through the host and gives the completion call only when nothing is open", async () => {
    const actions: Action[] = [];
    const after: [StepState, StepState][] = [
      ["skipped", "done"],
      ["done", "done"],
    ];
    let states: [StepState, StepState] = ["skipped", "current"];
    const h = host({
      apply: async (_pack, flight, action) => {
        actions.push(action);
        states = after.shift() ?? states;
        return { ...flight };
      },
      phaseView: async () => view(states),
    });
    render(App, { context: hostContext(h) });

    await fireEvent.click(await screen.findByRole("button", { name: "Confirm STANDBY POWER: AUTO" }));
    expect(await screen.findByText("1 skipped step still open. Power up flow is not complete.")).toBeTruthy();
    expect(screen.queryByText("Power up flow complete")).toBeNull();

    await fireEvent.click(screen.getByRole("button", { name: "Confirm BATTERY: ON" }));
    expect(await screen.findByText("Power up flow complete")).toBeTruthy();
    expect(actions.map((a) => a.type === "tick" && a.step)).toEqual(["standby", "battery"]);
  });

  it("opens a step with its details once and a quiet Skip for now at the bottom", async () => {
    let states: [StepState, StepState] = ["current", "to_do"];
    render(App, { context: hostContext(host({ phaseView: async () => view(states) })) });

    await fireEvent.click(await screen.findByRole("button", { name: "Show details for BATTERY" }));
    expect(screen.getAllByText(/First power\./)).toHaveLength(1);
    expect(screen.getByText("CHECK")).toBeTruthy();
    expect(screen.getByText("Find it later in Open steps.")).toBeTruthy();
    const skip = screen.getByRole("button", { name: "Skip for now" });
    expect(skip.classList.contains("quiet")).toBe(true);
    states = ["skipped", "current"];
  });

  it("shows the options the pack asks for on a phase as cards, and applies a choice", async () => {
    const actions: Action[] = [];
    const h = host({
      apply: async (_pack, flight, action) => {
        actions.push(action);
        return { ...flight, options: { power: "apu" } };
      },
    });
    render(App, { context: hostContext(h) });

    const gpu = await screen.findByRole("button", { name: /Ground power at the gate/ });
    expect(gpu.getAttribute("aria-pressed")).toBe("true");
    await fireEvent.click(screen.getByRole("button", { name: /No ground power/ }));
    expect(actions[0]).toEqual({ type: "setOption", option: "power", value: "apu" });
    expect(
      (await screen.findByRole("button", { name: /No ground power/ })).getAttribute("aria-pressed"),
    ).toBe("true");
  });

  it("shows the current step in a large box in Flow view", async () => {
    render(App, { context: hostContext(host()) });
    await screen.findByRole("heading", { level: 1, name: "Power up" });
    await fireEvent.click(screen.getByRole("button", { name: "Flow view" }));

    expect(await screen.findByRole("heading", { level: 1, name: "STANDBY POWER" })).toBeTruthy();
    expect(screen.getByText("Power up flow · Step 2 of 2")).toBeTruthy();
    expect(screen.getByRole("button", { name: "Confirmed, next step" })).toBeTruthy();
    expect((screen.getByRole("button", { name: "Back" }) as HTMLButtonElement).disabled).toBe(false);
  });

  it("shows a tagged error from Rust", async () => {
    const failing = host({
      loadPack: vi.fn(() => Promise.reject({ kind: "PackInvalid", message: "pack format 3 is newer" })),
    });
    render(App, { context: hostContext(failing) });

    expect((await screen.findByRole("alert")).textContent).toBe("PackInvalid: pack format 3 is newer");
  });
});
