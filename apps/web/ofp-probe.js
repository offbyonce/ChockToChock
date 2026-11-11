// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Reads a SimBrief plan for the SimBrief probe page (Phase 2, step 3). Throwaway: the real app
// parses plans with the `simbrief` crate as WebAssembly. The paths and rules here follow that
// crate, so the probe shows what the app will see. No DOM access, so Node can test it.

const FETCH_BASE = "https://www.simbrief.com/api/xml.fetcher.php";

/** The address of the latest plan. All digits means a Pilot ID; anything else is a username. */
export function fetchUrl(user) {
  const value = String(user ?? "").trim();
  if (!value) throw new Error("Enter a SimBrief username or Pilot ID.");
  const key = /^\d+$/.test(value) ? "userid" : "username";
  return `${FETCH_BASE}?${key}=${encodeURIComponent(value)}&json=1`;
}

/** Text of a value, or "" when it is missing, empty, or SimBrief's `{}` placeholder. */
function text(value) {
  return typeof value === "string" || typeof value === "number" ? String(value).trim() : "";
}

/** A reader over parsed JSON: `get("general/route")` and the navlog fixes. */
function jsonReader(source) {
  let root;
  try {
    root = JSON.parse(source);
  } catch (e) {
    throw new Error(`SimBrief returned something that is not JSON: ${e.message}`);
  }
  const get = (path) => {
    let value = root;
    for (const key of path.split("/")) {
      if (value === null || typeof value !== "object") return "";
      value = value[key];
    }
    return text(value);
  };
  const raw = root?.navlog?.fix;
  const fixes = Array.isArray(raw) ? raw : raw && typeof raw === "object" ? [raw] : [];
  return { get, fixCount: fixes.filter((fix) => text(fix?.ident)).length };
}

/** A reader over an XML document, given a function that parses XML text into a DOM document. */
function xmlReader(source, parseXml) {
  if (!parseXml) throw new Error("This browser cannot read XML files.");
  const doc = parseXml(source);
  if (!doc?.documentElement || doc.getElementsByTagName("parsererror").length > 0) {
    throw new Error("That file is not valid XML.");
  }
  const child = (node, name) => [...node.children].find((c) => c.tagName === name);
  const get = (path) => {
    let node = doc.documentElement;
    for (const key of path.split("/")) {
      node = node && child(node, key);
    }
    return node ? text(node.textContent) : "";
  };
  const navlog = child(doc.documentElement, "navlog");
  const fixes = navlog ? [...navlog.children].filter((c) => c.tagName === "fix") : [];
  return { get, fixCount: fixes.filter((fix) => child(fix, "ident")?.textContent.trim()).length };
}

/**
 * Reads plan text (JSON from the API, or a saved JSON or XML file) into a few fields.
 * Throws an Error with the exact reason when the text is not a usable plan.
 */
export function readPlan(source, parseXml) {
  const body = String(source ?? "")
    .replace(/^﻿/, "")
    .trim();
  let reader;
  if (body.startsWith("{")) reader = jsonReader(body);
  else if (body.startsWith("<")) reader = xmlReader(body, parseXml);
  else throw new Error("That is not a SimBrief plan (expected JSON or XML).");

  const status = reader.get("fetch/status");
  if (status && status.toLowerCase() !== "success") throw new Error(`SimBrief says: ${status}`);

  const { get } = reader;
  const flight = `${get("general/icao_airline")}${get("general/flight_number")}`;
  const units = get("params/units");
  const fuel = get("fuel/plan_ramp");
  return {
    Flight: flight,
    From: [get("origin/icao_code"), get("origin/plan_rwy") && `runway ${get("origin/plan_rwy")}`]
      .filter(Boolean)
      .join(", "),
    To: get("destination/icao_code"),
    "Cruise altitude": get("general/initial_altitude") && `${get("general/initial_altitude")} ft`,
    Route: get("general/route"),
    "Ramp fuel": fuel && `${fuel} ${units}`.trim(),
    AIRAC: get("params/airac"),
    "Navlog fixes": String(reader.fixCount),
  };
}
