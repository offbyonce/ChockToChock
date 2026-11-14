// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import { cleanup, fireEvent, render, screen } from "@testing-library/svelte";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import Toast from "./Toast.svelte";

beforeEach(() => vi.useFakeTimers());
afterEach(() => {
  cleanup();
  vi.useRealTimers();
});

describe("Toast", () => {
  it("announces itself, offers Undo as a button, and stays at least 8 seconds", async () => {
    let gone = false;
    let undone = false;
    render(Toast, {
      text: "Ticks cleared.",
      seconds: 3,
      onUndo: () => (undone = true),
      onExpire: () => (gone = true),
    });
    expect(screen.getByRole("status").textContent).toContain("Ticks cleared.");
    vi.advanceTimersByTime(7900);
    expect(gone).toBe(false);
    vi.advanceTimersByTime(200);
    expect(gone).toBe(true);
    await fireEvent.click(screen.getByRole("button", { name: "Undo" }));
    expect(undone).toBe(true);
  });

  it("pauses while pointed at or focused", async () => {
    let gone = false;
    render(Toast, {
      text: "Flight plan reset.",
      seconds: 8,
      onUndo: () => {},
      onExpire: () => (gone = true),
    });
    vi.advanceTimersByTime(5000);
    await fireEvent.pointerEnter(screen.getByRole("status"));
    vi.advanceTimersByTime(20000);
    expect(gone).toBe(false);
    await fireEvent.pointerLeave(screen.getByRole("status"));
    await fireEvent.focusIn(screen.getByRole("button", { name: "Undo" }));
    vi.advanceTimersByTime(20000);
    expect(gone).toBe(false);
    await fireEvent.focusOut(screen.getByRole("button", { name: "Undo" }));
    vi.advanceTimersByTime(2900);
    expect(gone).toBe(false);
    vi.advanceTimersByTime(200);
    expect(gone).toBe(true);
  });
});
