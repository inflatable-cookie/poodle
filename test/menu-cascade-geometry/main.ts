import "../../packages/tokens/artifacts/css/poodle-tokens.css";
import "../../packages/tokens/artifacts/css/poodle-themes.css";
import "../../packages/tokens/artifacts/css/poodle-density-default.css";
import "../../packages/tokens/artifacts/css/poodle-control-size-md.css";

import { createElement } from "react";
import { createRoot } from "react-dom/client";
import { mount } from "svelte";

import Harness from "./Harness.svelte";
import { Harness as ReactHarness } from "./Harness";

document.documentElement.dataset.theme = "eclipse";
document.documentElement.dataset.density = "default";
document.documentElement.dataset.controlSize = "md";

mount(Harness, { target: document.getElementById("svelte-mount") as HTMLElement });
createRoot(document.getElementById("react-mount") as HTMLElement).render(createElement(ReactHarness));
