import "@poodle/svelte-tokens/styles.css";
import "../../../tokens/artifacts/css/poodle-theme-light.css";
import "../../../tokens/artifacts/css/poodle-theme-dark.css";
import "../../../tokens/artifacts/css/poodle-theme-loophole-studio.css";
import "../../../tokens/artifacts/css/poodle-density-comfortable.css";
import "../../../tokens/artifacts/css/poodle-density-compact.css";
import "../../../tokens/artifacts/css/poodle-control-size-sm.css";
import "../../../tokens/artifacts/css/poodle-control-size-md.css";
import "../../../tokens/artifacts/css/poodle-control-size-lg.css";
import "./app.css";

import { mount } from "svelte";
import App from "./App.svelte";

const app = mount(App, {
  target: document.getElementById("app")!,
});

export default app;
