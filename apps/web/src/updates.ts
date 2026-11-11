// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// Registers the service worker (./sw.js, built by the site build) so the app works offline, and
// offers a reload when a new version is ready. Never swap mid-session (decision 0010): the new
// version runs only after the pilot presses Reload, or on the next launch.

import { watchForUpdates } from "./update-watch.js";

const CHECK_EVERY_MS = 60 * 60 * 1000;

function offerReload(worker: ServiceWorker, onReload: () => void): void {
  if (document.getElementById("update-ready")) return;
  const box = document.createElement("div");
  box.id = "update-ready";
  box.className = "update-ready";
  box.setAttribute("role", "status");
  const text = document.createElement("span");
  text.textContent = "A new version is ready.";
  const button = document.createElement("button");
  button.type = "button";
  button.textContent = "Reload";
  button.addEventListener("click", () => {
    onReload();
    worker.postMessage({ type: "reload-to-update" });
  });
  box.append(text, button);
  document.body.append(box);
}

export function registerServiceWorker(): void {
  if (!import.meta.env.PROD || !("serviceWorker" in navigator)) return;
  const workers = navigator.serviceWorker;
  let reloadRequested = false;
  // The new version took over because the pilot asked; load it. The first install also takes
  // control of the page, but then nothing was asked, so nothing reloads.
  workers.addEventListener("controllerchange", () => {
    if (reloadRequested) location.reload();
  });
  const onReload = () => {
    reloadRequested = true;
  };
  window.addEventListener("load", async () => {
    const registration = await workers.register("./sw.js");
    watchForUpdates(workers, registration, (worker) => offerReload(worker, onReload));
    // Long sessions: check for a new version now and then, and when the app comes back into view.
    setInterval(() => void registration.update(), CHECK_EVERY_MS);
    document.addEventListener("visibilitychange", () => {
      if (document.visibilityState === "visible") void registration.update();
    });
  });
}
