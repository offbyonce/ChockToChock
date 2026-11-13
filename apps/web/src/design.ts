// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import { DrawingGallery, GateGallery } from "@chocktochock/ui";
import "@chocktochock/ui/app.css";
import "./fonts/fonts.css";
import { mount } from "svelte";

const target = document.getElementById("gallery");
if (!target) throw new Error("missing #gallery element");

// ?view=gate: round 6's dense-panel gate; otherwise every control kind and state.
const gate = new URLSearchParams(location.search).get("view") === "gate";
export default mount(gate ? GateGallery : DrawingGallery, {
  target,
  props: { packUrl: "../packs/ifly-737-max8.json" },
});
