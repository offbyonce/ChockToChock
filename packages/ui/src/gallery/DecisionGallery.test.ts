// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import { cleanup, fireEvent, render, screen, within } from "@testing-library/svelte";
import { afterEach, describe, expect, it } from "vitest";
import DecisionGallery from "./DecisionGallery.svelte";

afterEach(cleanup);

const row = (name: string) => {
  const nm = [...document.querySelectorAll(".it .nm")].find((n) => n.textContent === name);
  const li = nm?.closest("li");
  if (!(li instanceof HTMLElement)) throw new Error(`no row ${name}`);
  return li;
};
const decision = () => row("Center tank fuel");
const fuel = () => row("Fuel pump switches");

describe("the decision step (Decision.dc.html, invented fixture)", () => {
  it("unanswered: a diamond, the question and two choices, only Skip, and it cannot be ticked", async () => {
    render(DecisionGallery, { state: "unanswered" });
    const d = decision();
    expect(d.classList.contains("current")).toBe(true);
    expect(d.querySelector(".diamond svg")).toBeTruthy();
    expect(within(d).getByText("CHECK QUANTITY")).toBeTruthy();
    expect(within(d).getByText("The center tank holds")).toBeTruthy();
    expect(
      within(d)
        .getAllByRole("button", { pressed: false })
        .map((b) => b.textContent?.trim()),
    ).toEqual(expect.arrayContaining(["More than 1,000 lb (450 kg)", "1,000 lb (450 kg) or less"]));
    expect(within(d).queryByRole("button", { name: "Done" })).toBeNull();
    expect(within(d).getByRole("button", { name: "Skip for now" })).toBeTruthy();
    expect(within(d).getByText("1 or 2 answer · S skip")).toBeTruthy();
    // The fuel step waits on the answer.
    expect(fuel().querySelector(".val")?.textContent).toBe("MAIN PUMPS ON\nCENTER PUMPS ON or OFF");
    expect(within(fuel()).getByRole("button", { name: "Depends on the center tank answer" })).toBeTruthy();
    // The status icon does not tick an unanswered decision.
    const tick = d.querySelector("button.tick");
    expect(tick?.getAttribute("aria-disabled")).toBe("true");
    await fireEvent.click(tick as Element);
    expect(decision().classList.contains("current")).toBe(true);
  });

  it("choosing answers and ticks it: Answered by you, and the fuel step resolves with a link back", async () => {
    render(DecisionGallery, { state: "unanswered" });
    await fireEvent.click(within(decision()).getByRole("button", { name: "More than 1,000 lb (450 kg)" }));
    expect(decision().classList.contains("done")).toBe(true);
    expect(within(decision()).getByText("Answered by you")).toBeTruthy();
    expect(within(decision()).getByText("More than 1,000 lb (450 kg)")).toBeTruthy();
    expect(fuel().classList.contains("current")).toBe(true);
    expect(fuel().querySelector(".val")?.textContent).toBe("MAIN PUMPS ON\nCENTER PUMPS ON");
    expect(within(fuel()).getByRole("button", { name: "Center tank more than 1,000 lb" })).toBeTruthy();
  });

  it("keys: 1 and 2 answer; Space never ticks an unanswered decision without a suggestion", async () => {
    render(DecisionGallery, { state: "unanswered" });
    await fireEvent.keyDown(window, { key: " " });
    expect(decision().classList.contains("current")).toBe(true);
    await fireEvent.keyDown(window, { key: "2" });
    expect(within(decision()).getByText("1,000 lb (450 kg) or less")).toBeTruthy();
    expect(decision().classList.contains("done")).toBe(true);
  });

  it("suggested: ringed and tagged, Confirm carries it, Space confirms; nothing ticks before", async () => {
    render(DecisionGallery, { state: "suggested" });
    const d = decision();
    expect(within(d).getByText("Suggested from your SimBrief plan: 0 lb")).toBeTruthy();
    const ringed = within(d).getByRole("button", { pressed: true });
    expect(ringed.textContent).toContain("1,000 lb (450 kg) or less");
    expect(within(ringed).getByText("Suggested")).toBeTruthy();
    expect(within(d).getByRole("button", { name: "Confirm: 1,000 lb or less" })).toBeTruthy();
    expect(within(d).getByText("Space confirm · 1 or 2 answer · S skip")).toBeTruthy();
    expect(d.classList.contains("current")).toBe(true);
    await fireEvent.keyDown(window, { key: " " });
    expect(within(decision()).getByText("Confirmed from your SimBrief plan")).toBeTruthy();
  });

  it("Change from the row menu reopens the choices in place; the current step does not move; Keep backs out", async () => {
    render(DecisionGallery, { state: "answered" });
    await fireEvent.click(
      within(decision()).getByRole("button", { name: "More actions for Center tank fuel" }),
    );
    await fireEvent.click(screen.getByRole("menuitem", { name: "Change answer" }));
    const d = decision();
    expect(d.classList.contains("changing")).toBe(true);
    expect(within(d).getByText("Change your answer")).toBeTruthy();
    expect(d.querySelector('.choice[aria-pressed="true"]')?.textContent).toContain(
      "1,000 lb (450 kg) or less",
    );
    expect(fuel().classList.contains("current")).toBe(true);
    await fireEvent.click(within(d).getByRole("button", { name: "Keep this answer" }));
    expect(decision().classList.contains("changing")).toBe(false);
    expect(within(decision()).getByText("Confirmed from your SimBrief plan")).toBeTruthy();
  });

  it("Change from the fuel step's link, a new answer: Changed by you, the fuel step updated, a toast with Undo", async () => {
    render(DecisionGallery, { state: "answered" });
    await fireEvent.click(within(fuel()).getByRole("button", { name: "Center tank 1,000 lb or less" }));
    await fireEvent.click(within(decision()).getByRole("button", { name: "More than 1,000 lb (450 kg)" }));
    expect(within(decision()).getByText("Changed by you")).toBeTruthy();
    expect(fuel().querySelector(".val")?.textContent).toBe("MAIN PUMPS ON\nCENTER PUMPS ON");
    const toast = screen.getByRole("status");
    expect(toast.textContent).toContain("Center tank: more than 1,000 lb. Fuel pump switches updated.");
    await fireEvent.click(within(toast).getByRole("button", { name: "Undo" }));
    expect(within(decision()).getByText("Confirmed from your SimBrief plan")).toBeTruthy();
    expect(within(fuel()).getByRole("button", { name: "Center tank 1,000 lb or less" })).toBeTruthy();
  });

  it("a done row never takes the Done button's style: they share no class", () => {
    render(DecisionGallery, { state: "answered" });
    const button = within(fuel()).getByRole("button", { name: "Done" });
    const shared = [...button.classList].filter((c) => !c.startsWith("svelte-"));
    const done = [decision(), decision().querySelector(".status")].filter((e): e is Element => !!e);
    for (const e of done) expect(shared.filter((c) => e.classList.contains(c))).toEqual([]);
  });
});
