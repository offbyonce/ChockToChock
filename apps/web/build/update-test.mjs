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

import fs from "node:fs";
import path from "node:path";
import { serve as serveOn, session, sleep, startFirefox } from "./browser.mjs";
import { writeServiceWorker } from "./precache.js";

const web = path.resolve(import.meta.dirname, "..");
const dist = path.join(web, "dist");
// Under the repo's target folder, not /tmp: a sandboxed Firefox may not reach /tmp.
const work = path.resolve(web, "../../target/update-test");
const PORT = 8771;
const BIDI = 9233;
const serve = (root) => serveOn(root, PORT);

function prepare() {
  if (!fs.existsSync(path.join(dist, "app/sw.js"))) throw new Error("build the site first: just site");
  fs.rmSync(work, { recursive: true, force: true });
  fs.mkdirSync(work, { recursive: true });
  const a = path.join(work, "A");
  const b = path.join(work, "B");
  fs.cpSync(dist, a, { recursive: true });
  fs.cpSync(dist, b, { recursive: true });
  const page = path.join(b, "app/index.html");
  fs.writeFileSync(page, fs.readFileSync(page, "utf8").replace("</title>", " B</title>"));
  writeServiceWorker(web, b);
  return { a, b };
}

const STATE = `(async () => JSON.stringify({
  title: document.title,
  controlled: !!navigator.serviceWorker.controller,
  caches: (await caches.keys()).sort(),
  waiting: !!(await navigator.serviceWorker.getRegistration())?.waiting,
  offer: !!document.getElementById("update-ready"),
  app: !!document.querySelector("[data-app-ready]"),
}))()`;

async function main() {
  const { a, b } = prepare();
  const firefox = await startFirefox(path.join(work, "profile"), BIDI);
  const results = [];
  const check = (name, ok, detail) => {
    results.push(ok);
    process.stdout.write(`${ok ? "PASS" : "FAIL"} ${name}\n${ok ? "" : `  ${detail}\n`}`);
  };
  let server;
  try {
    const { send, js, until, context } = await session(BIDI);
    const state = async () => JSON.parse(await js(STATE));
    const appShown = `!!document.querySelector("[data-app-ready]")`;

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
