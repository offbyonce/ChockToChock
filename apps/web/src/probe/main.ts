// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// The SimBrief probe page (Phase 2, step 3), now reading plans with the simbrief crate as
// WebAssembly. The username goes from this browser straight to SimBrief; it is not stored, and
// nothing is sent anywhere else. Kept as a test page; the app's own plan loading comes in step 7.

import { type AppError, isAppError, type Ofp } from "@chocktochock/bindings";
import { parsePlan, planUrl } from "../core";

function element<T extends HTMLElement>(id: string): T {
  const found = document.getElementById(id);
  if (!found) throw new Error(`missing #${id}`);
  return found as T;
}

/** The message of a tagged error from Rust, or of anything else that was thrown. */
function message(e: unknown): string {
  if (isAppError(e)) return (e as AppError).message;
  return e instanceof Error ? `${e.name}: ${e.message}` : String(e);
}

/** A few fields from the plan, as the page shows them. */
function fields(plan: Ofp): Record<string, string> {
  return {
    Flight: `${plan.airline ?? ""}${plan.flightNumber ?? ""}`,
    From: [plan.origin, plan.originRunway && `runway ${plan.originRunway}`].filter(Boolean).join(", "),
    To: plan.destination ?? "",
    "Cruise altitude": plan.cruiseAltitudeFt == null ? "" : `${plan.cruiseAltitudeFt} ft`,
    Route: plan.route ?? "",
    "Ramp fuel": plan.rampFuel == null ? "" : `${plan.rampFuel} ${plan.units ?? ""}`.trim(),
    AIRAC: plan.airac ?? "",
    "Navlog fixes": String(plan.fixes.length),
  };
}

/** Shows the outcome: a heading, then either the plan fields or the details of what failed. */
function show(target: string, ok: boolean, heading: string, rows: Record<string, string>): void {
  const box = element(target);
  box.hidden = false;
  box.className = ok ? "result ok" : "result fail";
  const title = document.createElement("p");
  title.className = "verdict";
  title.textContent = heading;
  const list = document.createElement("dl");
  for (const [name, value] of Object.entries(rows)) {
    const dt = document.createElement("dt");
    dt.textContent = name;
    const dd = document.createElement("dd");
    dd.textContent = value || "(not in the plan)";
    list.append(dt, dd);
  }
  box.replaceChildren(title, list);
}

async function fetchPlan(event: SubmitEvent): Promise<void> {
  event.preventDefault();
  const button = element<HTMLButtonElement>("fetch-button");
  let url: string;
  try {
    url = await planUrl(element<HTMLInputElement>("user").value);
  } catch (e) {
    show("fetch-result", false, "Not sent", { Error: message(e) });
    return;
  }
  button.disabled = true;
  const started = performance.now();
  const abort = new AbortController();
  const timer = setTimeout(() => abort.abort(), 20000);
  const details: Record<string, string> = { Address: url };
  try {
    const response = await fetch(url, { signal: abort.signal, cache: "no-store", credentials: "omit" });
    details["HTTP status"] = `${response.status} ${response.statusText}`.trim();
    details.Time = `${Math.round(performance.now() - started)} ms`;
    const body = await response.text();
    try {
      show("fetch-result", true, "It worked: SimBrief answered this page directly.", {
        ...fields(await parsePlan(body)),
        ...details,
      });
    } catch (e) {
      // SimBrief answered (so the browser allowed the request), but with no usable plan.
      show("fetch-result", false, "SimBrief answered, but sent no usable plan.", {
        ...details,
        Error: message(e),
      });
    }
  } catch (e) {
    details.Time = `${Math.round(performance.now() - started)} ms`;
    const reason =
      e instanceof DOMException && e.name === "AbortError"
        ? "No answer within 20 seconds."
        : `${message(e)}. The browser blocked or could not make the request; the console may say why.`;
    show("fetch-result", false, "The request failed.", { ...details, Error: reason });
  } finally {
    clearTimeout(timer);
    button.disabled = false;
  }
}

async function loadFile(): Promise<void> {
  const input = element<HTMLInputElement>("file");
  const file = input.files?.[0];
  if (!file) return;
  try {
    show("file-result", true, `Read ${file.name}.`, fields(await parsePlan(await file.text())));
  } catch (e) {
    show("file-result", false, `Could not read ${file.name}.`, { Error: message(e) });
  }
  input.value = "";
}

element<HTMLFormElement>("fetch-form").addEventListener("submit", (event) => void fetchPlan(event));
element<HTMLInputElement>("file").addEventListener("change", () => void loadFile());
