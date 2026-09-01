import { mount } from "svelte";
import { createElement } from "react";
import { createRoot } from "react-dom/client";

import Harness from "./Harness.svelte";
import { Harness as ReactHarness } from "./Harness";

mount(Harness, { target: document.getElementById("svelte-mount") as HTMLElement });
createRoot(document.getElementById("react-mount") as HTMLElement).render(createElement(ReactHarness));
