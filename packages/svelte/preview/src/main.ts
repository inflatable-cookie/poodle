import "@flint/svelte-tokens/styles.css";
import "../../../tokens/artifacts/css/flint-theme-light.css";
import "../../../tokens/artifacts/css/flint-theme-dark.css";
import "../../../tokens/artifacts/css/flint-theme-loophole-studio.css";
import "../../../tokens/artifacts/css/flint-density-comfortable.css";
import "../../../tokens/artifacts/css/flint-density-compact.css";
import "../../../tokens/artifacts/css/flint-control-size-sm.css";
import "../../../tokens/artifacts/css/flint-control-size-md.css";
import "../../../tokens/artifacts/css/flint-control-size-lg.css";
import "./app.css";

import { mount } from "svelte";
import App from "./App.svelte";

const app = mount(App, {
  target: document.getElementById("app")!,
});

export default app;
