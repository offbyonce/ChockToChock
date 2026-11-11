// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// When to offer "A new version is ready" (decision 0010). Plain JavaScript, so Node's test runner
// covers it with fake service worker objects (update-watch.test.js).
//
// On a first visit no worker controls the page, and the worker that installs then is the first
// install: there is no older version running, so there is nothing to offer. That worker has to be
// recognized by identity. Checking for a controller when its events arrive is not enough: a
// browser may finish installing, activate the worker and hand it the page before it delivers the
// queued "updatefound" and "statechange" events, which then look just like an update.

/**
 * Calls `offer(worker)` once a new version is installed and waiting behind the running one.
 * @param {ServiceWorkerContainer} workers
 * @param {ServiceWorkerRegistration} registration
 * @param {(worker: ServiceWorker) => void} offer
 */
export function watchForUpdates(workers, registration, offer) {
  if (registration.waiting && workers.controller) offer(registration.waiting);
  // The first install on an uncontrolled page: the worker installing now, or else the next one.
  /** @type {ServiceWorker | "next" | null} */
  let first = workers.controller ? null : (registration.installing ?? "next");
  registration.addEventListener("updatefound", () => {
    const worker = registration.installing;
    if (!worker) return;
    if (first === "next") first = worker;
    if (worker === first) return;
    worker.addEventListener("statechange", () => {
      if (worker.state === "installed" && workers.controller) offer(worker);
    });
  });
}
