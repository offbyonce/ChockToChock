// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Typed calls into the WebAssembly module (crates/web-host). Every export answers with a JSON
// Reply; a reply with an error is thrown as the tagged AppError, so callers can branch on `kind`.
// Run `just wasm` to build the module into ./wasm.

import type { AppInfo, Checklist, FlatPack, Ofp, Reply } from "@chocktochock/bindings";
import init, * as wasm from "./wasm/web_host.js";

let ready: Promise<unknown> | undefined;

/** Loads the module once; later calls wait for the same load. */
function load(): Promise<unknown> {
  ready ??= init();
  return ready;
}

function unwrap<T>(answer: string): T {
  const reply = JSON.parse(answer) as Reply<T>;
  if ("error" in reply) throw reply.error;
  return reply.ok;
}

export async function appInfo(): Promise<AppInfo> {
  await load();
  return unwrap<AppInfo>(wasm.app_info());
}

export async function exampleChecklist(): Promise<Checklist> {
  await load();
  return unwrap<Checklist>(wasm.example_checklist());
}

/** Reads a flattened pack's JSON text, as written by `pack build`. */
export async function loadPack(json: string): Promise<FlatPack> {
  await load();
  return unwrap<FlatPack>(wasm.load_pack(json));
}

/** The address of the latest SimBrief plan for a username or Pilot ID. */
export async function planUrl(user: string): Promise<string> {
  await load();
  return unwrap<string>(wasm.plan_url(user));
}

/** Reads a SimBrief plan: fetched JSON, or a saved JSON or XML file. */
export async function parsePlan(text: string): Promise<Ofp> {
  await load();
  return unwrap<Ofp>(wasm.parse_plan(text));
}
