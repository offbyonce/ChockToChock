// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// The service worker (decision 0010). This is a template: the build (apps/web/build/precache.js)
// fills in VERSION, derived from the built files, and FILES, the list to precache, and writes it
// to app/sw.js. Each version gets its own cache, so a deploy never mixes old and new files.
//
// Never swap mid-session: a new version installs in the background and then waits. It takes over
// only when the pilot presses Reload (the page sends "reload-to-update"), or on the next launch
// once every window of the old version is closed.

const VERSION = "__CHOCK_VERSION__";
const FILES = ["__CHOCK_FILES__"];
const PREFIX = "chocktochock-";
const CACHE = PREFIX + VERSION;

self.addEventListener("install", (event) => {
  // `cache: "reload"` skips the browser's HTTP cache, so a new version never stores old files.
  const requests = FILES.map((file) => new Request(new URL(file, self.location), { cache: "reload" }));
  event.waitUntil(caches.open(CACHE).then((cache) => cache.addAll(requests)));
});

self.addEventListener("activate", (event) => {
  event.waitUntil(
    (async () => {
      for (const key of await caches.keys()) {
        if (key.startsWith(PREFIX) && key !== CACHE) await caches.delete(key);
      }
      await self.clients.claim();
    })(),
  );
});

self.addEventListener("message", (event) => {
  if (event.data?.type === "reload-to-update") self.skipWaiting();
});

self.addEventListener("fetch", (event) => {
  const request = event.request;
  // Only this site's files; SimBrief requests go straight to the network.
  if (request.method !== "GET" || new URL(request.url).origin !== self.location.origin) return;
  event.respondWith(
    (async () => {
      const cache = await caches.open(CACHE);
      const navigate = request.mode === "navigate";
      const hit =
        (await cache.match(request, { ignoreSearch: navigate })) ??
        (navigate ? await cache.match(new URL("index.html", self.location)) : undefined);
      return hit ?? fetch(request);
    })(),
  );
});
