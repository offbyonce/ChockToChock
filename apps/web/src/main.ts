// SPDX-FileCopyrightText: 2026 offbyonce
// SPDX-License-Identifier: MIT

import { App, hostContext } from "@chocktochock/ui";
import "@chocktochock/ui/app.css";
import "./app-page.css";
import { mount } from "svelte";
import { webHost } from "./web-host";

const target = document.getElementById("app");
if (!target) throw new Error("missing #app element");

export default mount(App, { target, context: hostContext(webHost) });
