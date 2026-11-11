// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// The version-to-version update test (decision 0010), run with `just update-test` after
// `just site`. It drives a real headless Firefox over WebDriver BiDi through four checks:
//   1. version A: the first visit installs the service worker and caches A;
//   2. offline: with the server stopped, the installed app still loads, in B612;
//   3. version B deployed while A is open: A keeps running, B waits, the reload offer shows;
//   4. Reload: B runs and A's cache is gone.
// Version B is the built site with a changed page title and its own service worker version.
// Needs Firefox on the PATH (or FIREFOX=/path/to/firefox). Not part of `just ci`.

import { spawn } from "node:child_process";
import fs from "node:fs";
import http from "node:http";
import path from "node:path";
import { writeServiceWorker } from "./precache.js";

const web = path.resolve(import.meta.dirname, "..");
const dist = path.join(web, "dist");
// Under the repo's target folder, not /tmp: a sandboxed Firefox may not reach /tmp.
const work = path.resolve(web, "../../target/update-test");
const PORT = 8771;
const BIDI = 9233;
const sleep = (ms) => new Promise((resolve) => setTimeout(resolve, ms));

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
};

/** Serves one built site on localhost until closed. */
function serve(root) {
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
    server.listen(PORT, "127.0.0.1", () => resolve(server));
  });
}

function prepare() {
  if (!fs.existsSync(path.join(dist, "app/sw.js"))) throw new Error("build the site first: just site");
  fs.rmSync(work, { recursive: true, force: true });
  fs.mkdirSync(path.join(work, "profile"), { recursive: true });
  const a = path.join(work, "A");
  const b = path.join(work, "B");
  fs.cpSync(dist, a, { recursive: true });
  fs.cpSync(dist, b, { recursive: true });
  const page = path.join(b, "app/index.html");
  fs.writeFileSync(page, fs.readFileSync(page, "utf8").replace("</title>", " B</title>"));
  writeServiceWorker(web, b);
  return { a, b };
}

async function startFirefox() {
  const firefox = spawn(
    process.env.FIREFOX ?? "firefox",
    ["--headless", "--no-remote", "--profile", path.join(work, "profile"), `--remote-debugging-port=${BIDI}`],
    { stdio: ["ignore", "ignore", "pipe"] },
  );
  let ready = false;
  firefox.stderr.on("data", (data) => {
    if (String(data).includes("WebDriver BiDi listening")) ready = true;
  });
  for (let i = 0; i < 150 && !ready; i++) await sleep(200);
  if (!ready) throw new Error("Firefox did not start WebDriver BiDi");
  return firefox;
}

/** A minimal WebDriver BiDi client: send a command, get its result. */
async function connect() {
  const ws = new WebSocket(`ws://127.0.0.1:${BIDI}/session`);
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
  return (method, params) =>
    new Promise((resolve, reject) => {
      const id = nextId++;
      pending.set(id, (reply) =>
        reply.type === "error" ? reject(new Error(`${method}: ${reply.message}`)) : resolve(reply.result),
      );
      ws.send(JSON.stringify({ id, method, params }));
    });
}

const STATE = `(async () => JSON.stringify({
  title: document.title,
  controlled: !!navigator.serviceWorker.controller,
  caches: (await caches.keys()).sort(),
  waiting: !!(await navigator.serviceWorker.getRegistration())?.waiting,
  offer: !!document.getElementById("update-ready"),
  app: document.body.innerText.includes("Battery switch"),
}))()`;

async function main() {
  const { a, b } = prepare();
  const firefox = await startFirefox();
  const results = [];
  const check = (name, ok, detail) => {
    results.push(ok);
    process.stdout.write(`${ok ? "PASS" : "FAIL"} ${name}\n${ok ? "" : `  ${detail}\n`}`);
  };
  let server;
  try {
    const send = await connect();
    await send("session.new", { capabilities: {} });
    const context = (await send("browsingContext.getTree", {})).contexts[0].context;
    const js = async (expression) => {
      const r = await send("script.evaluate", { expression, target: { context }, awaitPromise: true, resultOwnership: "none" });
      if (r.type === "exception") throw new Error(r.exceptionDetails.text);
      return r.result.value;
    };
    const until = async (expression, what) => {
      for (let t = 0; t < 20000; t += 250) {
        if (await js(expression).catch(() => false)) return;
        await sleep(250);
      }
      throw new Error(`timed out waiting for ${what}`);
    };
    const state = async () => JSON.parse(await js(STATE));
    const appShown = `document.body.innerText.includes("Battery switch")`;

    server = await serve(a);
    await send("browsingContext.navigate", { context, url: `http://localhost:${PORT}/app/`, wait: "complete" });
    await until("!!navigator.serviceWorker.controller", "the worker to control the page");
    await until(appShown, "the app to render");
    let s = await state();
    const cacheA = s.caches[0];
    check("A: the first visit installs the worker and caches version A", s.controlled && s.app && s.caches.length === 1, JSON.stringify(s));

    await new Promise((resolve) => server.close(resolve));
    server = undefined;
    await send("browsingContext.reload", { context, wait: "complete" });
    await until(appShown, "the app to render offline");
    s = await state();
    const font = await js(`(async () => { await document.fonts.ready; return document.fonts.check("16px B612"); })()`);
    check("offline: the installed app loads with no server, in B612", s.app && s.caches[0] === cacheA && font, JSON.stringify({ ...s, font }));

    server = await serve(b);
    await js("(async () => { await (await navigator.serviceWorker.getRegistration()).update(); return true; })()");
    await until(`!!document.getElementById("update-ready")`, "the reload offer");
    s = await state();
    check(
      "B deployed: A keeps running unchanged, B waits, the reload offer shows",
      !s.title.endsWith(" B") && s.waiting && s.offer && s.caches.length === 2 && s.caches.includes(cacheA),
      JSON.stringify(s),
    );

    await js(`document.querySelector("#update-ready button").click(); true`);
    await sleep(1000);
    await until(`document.title.endsWith(" B")`, "version B to load");
    await until(appShown, "the app to render on version B");
    await until("(async () => (await caches.keys()).length === 1)()", "A's cache to be deleted");
    s = await state();
    check("Reload: B runs and A's cache is gone", s.controlled && !s.offer && s.caches[0] !== cacheA, JSON.stringify(s));
  } catch (e) {
    check("the test ran to the end", false, e.message);
  } finally {
    server?.close();
    firefox.kill();
  }
  const failed = results.filter((ok) => !ok).length;
  process.stdout.write(`update test: ${results.length - failed} passed, ${failed} failed\n`);
  process.exitCode = failed ? 1 : 0;
}

await main();
