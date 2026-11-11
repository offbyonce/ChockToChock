// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Wires the SimBrief probe page. The username goes from this browser straight to SimBrief; it
// is not stored, and nothing is sent anywhere else.

import { fetchUrl, readPlan } from "./ofp-probe.js";

const $ = (id) => document.getElementById(id);
const parseXml = (source) => new DOMParser().parseFromString(source, "application/xml");

/** Shows the outcome: a heading, then either the plan fields or the details of what failed. */
function show(target, ok, heading, rows) {
  const box = $(target);
  box.hidden = false;
  box.className = ok ? "result ok" : "result fail";
  box.replaceChildren();
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
  box.append(title, list);
}

async function fetchPlan(event) {
  event.preventDefault();
  const button = $("fetch-button");
  let url;
  try {
    url = fetchUrl($("user").value);
  } catch (e) {
    show("fetch-result", false, "Not sent", { Error: e.message });
    return;
  }
  button.disabled = true;
  const started = performance.now();
  const abort = new AbortController();
  const timer = setTimeout(() => abort.abort(), 20000);
  const details = { Address: url };
  try {
    const response = await fetch(url, {
      signal: abort.signal,
      cache: "no-store",
      credentials: "omit",
    });
    details["HTTP status"] = `${response.status} ${response.statusText}`.trim();
    details.Time = `${Math.round(performance.now() - started)} ms`;
    const body = await response.text();
    try {
      const fields = readPlan(body, parseXml);
      show("fetch-result", true, "It worked: SimBrief answered this page directly.", {
        ...fields,
        ...details,
      });
    } catch (e) {
      // SimBrief answered (so the browser allowed the request), but with no usable plan.
      show("fetch-result", false, "SimBrief answered, but sent no usable plan.", {
        ...details,
        Error: e.message,
      });
    }
  } catch (e) {
    details.Time = `${Math.round(performance.now() - started)} ms`;
    const reason =
      e.name === "AbortError"
        ? "No answer within 20 seconds."
        : `${e.name}: ${e.message}. The browser blocked or could not make the request; the console may say why.`;
    show("fetch-result", false, "The request failed.", { ...details, Error: reason });
  } finally {
    clearTimeout(timer);
    button.disabled = false;
  }
}

async function loadFile() {
  const file = $("file").files[0];
  if (!file) return;
  try {
    const fields = readPlan(await file.text(), parseXml);
    show("file-result", true, `Read ${file.name}.`, fields);
  } catch (e) {
    show("file-result", false, `Could not read ${file.name}.`, { Error: e.message });
  }
  $("file").value = "";
}

$("fetch-form").addEventListener("submit", fetchPlan);
$("file").addEventListener("change", loadFile);
