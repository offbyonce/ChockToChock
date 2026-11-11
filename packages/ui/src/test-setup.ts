// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

// jsdom has no ResizeObserver, which Svelte's bind:clientWidth uses; drawings then use their
// default scale. Tests only.
globalThis.ResizeObserver ??= class {
  observe(): void {}
  unobserve(): void {}
  disconnect(): void {}
};
