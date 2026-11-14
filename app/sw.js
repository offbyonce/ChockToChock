// Retires the old app's service worker at /ChockToChock/app/ (published beside 404.html, at
// app/sw.js, in the repository with the old name).
//
// The old worker serves the app from its cache first, so an installed copy would keep showing
// the old app and never reach the redirect page. Browsers re-fetch the worker script when the app
// is opened (at least daily); this replacement installs at once, removes the old caches,
// unregisters itself and reloads its pages, which then reach 404.html and move to /involo/.
self.addEventListener("install", () => self.skipWaiting());

self.addEventListener("activate", (event) => {
  event.waitUntil(
    (async () => {
      for (const key of await caches.keys()) {
        if (key.startsWith("chocktochock-")) await caches.delete(key);
      }
      await self.registration.unregister();
      const pages = await self.clients.matchAll({ type: "window" });
      for (const page of pages) page.navigate(page.url);
    })(),
  );
});
