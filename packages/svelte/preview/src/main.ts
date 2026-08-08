import "@inflatable-cookie/poodle-svelte-tokens/styles.css";
// All theme layers in one aggregate — the selector offers every registered theme.
import "../../../tokens/artifacts/css/poodle-themes.css";
import "../../../tokens/artifacts/css/poodle-density-comfortable.css";
import "../../../tokens/artifacts/css/poodle-density-compact.css";
import "../../../tokens/artifacts/css/poodle-density-default.css";
import "../../../tokens/artifacts/css/poodle-control-size-xs.css";
import "../../../tokens/artifacts/css/poodle-control-size-sm.css";
import "../../../tokens/artifacts/css/poodle-control-size-md.css";
import "../../../tokens/artifacts/css/poodle-control-size-lg.css";
import "../../../tokens/artifacts/css/poodle-control-size-xl.css";
import "./app.css";

import { mount } from "svelte";
import App from "./App.svelte";

const app = mount(App, {
  target: document.getElementById("app")!,
});

export default app;
