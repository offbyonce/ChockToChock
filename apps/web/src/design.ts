// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import { DrawingGallery } from "@chocktochock/ui";
import "@chocktochock/ui/app.css";
import "./fonts/fonts.css";
import { mount } from "svelte";

const target = document.getElementById("gallery");
if (!target) throw new Error("missing #gallery element");

export default mount(DrawingGallery, { target, props: { packUrl: "../packs/ifly-737-max8.json" } });
