// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

export function watchForUpdates(
  workers: ServiceWorkerContainer,
  registration: ServiceWorkerRegistration,
  offer: (worker: ServiceWorker) => void,
): void;
