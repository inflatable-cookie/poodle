import "../../../tokens/artifacts/css/pug-tokens.css";
import "../../../tokens/artifacts/css/pug-theme-light.css";
import "../../../tokens/artifacts/css/pug-theme-dark.css";
import "../../../tokens/artifacts/css/pug-theme-loophole-studio.css";
import "../../../tokens/artifacts/css/pug-density-comfortable.css";
import "../../../tokens/artifacts/css/pug-density-compact.css";
import "../../../tokens/artifacts/css/pug-control-size-sm.css";
import "../../../tokens/artifacts/css/pug-control-size-md.css";
import "../../../tokens/artifacts/css/pug-control-size-lg.css";
import "./app.css";

import { mount } from "svelte";
import App from "./App.svelte";

const app = mount(App, {
  target: document.getElementById("app")!,
});

export default app;
