// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Shared by the local browser tools (update-test.mjs, screenshots.mjs): a small file server for a
// built site, and headless Firefox driven over WebDriver BiDi. Needs Firefox on the PATH (or
// FIREFOX=/path/to/firefox). Not part of `just ci`.

import { spawn } from "node:child_process";
import fs from "node:fs";
import http from "node:http";
import path from "node:path";

export const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

const TYPES = {
  ".html": "text/html",
  ".js": "text/javascript",
  ".css": "text/css",
  ".wasm": "application/wasm",
  ".json": "application/json",
  ".webmanifest": "application/manifest+json",
  ".png": "image/png",
  ".svg": "image/svg+xml",
  ".ttf": "font/ttf",
  ".woff2": "font/woff2",
};

/** Serves one built site on localhost until closed. */
export function serve(root, port) {
  const server = http.createServer((req, res) => {
    let file = path.join(root, decodeURIComponent(new URL(req.url, "http://x").pathname));
    if (fs.existsSync(file) && fs.statSync(file).isDirectory()) file = path.join(file, "index.html");
    if (!file.startsWith(root) || !fs.existsSync(file)) {
      res.writeHead(404).end();
      return;
    }
    res.writeHead(200, { "content-type": TYPES[path.extname(file)] ?? "application/octet-stream" });
    fs.createReadStream(file).pipe(res);
  });
  return new Promise((resolve, reject) => {
    server.once("error", reject);
    server.listen(port, "127.0.0.1", () => resolve(server));
  });
}

/**
 * Starts headless Firefox with a fresh profile, optional preferences and window size (in headless
 * mode, the viewport), and BiDi on `port`.
 */
export async function startFirefox(profile, port, prefs = {}, size = null) {
  fs.rmSync(profile, { recursive: true, force: true });
  fs.mkdirSync(profile, { recursive: true });
  const lines = Object.entries(prefs).map(([k, v]) => `user_pref(${JSON.stringify(k)}, ${JSON.stringify(v)});`);
  fs.writeFileSync(path.join(profile, "user.js"), `${lines.join("\n")}\n`);
  const firefox = spawn(
    process.env.FIREFOX ?? "firefox",
    [
      "--headless",
      "--no-remote",
      "--profile",
      profile,
      `--remote-debugging-port=${port}`,
      ...(size ? ["--width", String(size.width), "--height", String(size.height)] : []),
    ],
    { stdio: ["ignore", "ignore", "pipe"] },
  );
  let ready = false;
  firefox.stderr.on("data", (data) => {
    if (String(data).includes("WebDriver BiDi listening")) ready = true;
  });
  for (let i = 0; i < 150 && !ready; i++) await sleep(200);
  if (!ready) {
    firefox.kill();
    throw new Error("Firefox did not start WebDriver BiDi");
  }
  return firefox;
}

/** A minimal WebDriver BiDi session: `send(method, params)`, `js(expression)`, `until(...)`. */
export async function session(port) {
  const ws = new WebSocket(`ws://127.0.0.1:${port}/session`);
  await new Promise((resolve, reject) => {
    ws.onopen = resolve;
    ws.onerror = reject;
  });
  let nextId = 1;
  const pending = new Map();
  ws.onmessage = (message) => {
    const reply = JSON.parse(message.data);
    pending.get(reply.id)?.(reply);
    pending.delete(reply.id);
  };
  // Every command gives up after `ms`, so a stuck page fails the run instead of hanging it.
  const send = (method, params, ms = 30000) =>
    new Promise((resolve, reject) => {
      const id = nextId++;
      const timer = setTimeout(() => {
        pending.delete(id);
        reject(new Error(`${method}: no answer in ${ms} ms`));
      }, ms);
      pending.set(id, (reply) => {
        clearTimeout(timer);
        if (reply.type === "error") reject(new Error(`${method}: ${reply.message}`));
        else resolve(reply.result);
      });
      ws.send(JSON.stringify({ id, method, params }));
    });
  await send("session.new", { capabilities: {} });
  const context = (await send("browsingContext.getTree", {})).contexts[0].context;
  const js = async (expression) => {
    const r = await send("script.evaluate", { expression, target: { context }, awaitPromise: true, resultOwnership: "none" });
    if (r.type === "exception") throw new Error(r.exceptionDetails.text);
    return r.result.value;
  };
  const until = async (expression, what, ms = 20000) => {
    for (let t = 0; t < ms; t += 250) {
      if (await js(expression).catch(() => false)) return;
      await sleep(250);
    }
    throw new Error(`timed out waiting for ${what}`);
  };
  return { send, js, until, context };
}
