// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import { DesignPage } from "@chocktochock/ui";
import "@chocktochock/ui/app.css";
import "./fonts/fonts.css";
import { mount } from "svelte";

const target = document.getElementById("design");
if (!target) throw new Error("missing #design element");

export default mount(DesignPage, { target, props: { packUrl: "../packs/ifly-737-max8.json" } });
