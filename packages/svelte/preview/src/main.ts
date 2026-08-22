import "@inflatable-cookie/poodle-core/tokens/styles.css";
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
import "./catalogue.css";

import { mount } from "svelte";
import App from "./App.svelte";
import FixtureHost from "./fixture-host/FixtureHost.svelte";

// g15.047: `?fixture=…` swaps the catalogue for the capture-only fixture host.
const isFixtureCapture = new URLSearchParams(window.location.search).has("fixture");

const app = mount(isFixtureCapture ? FixtureHost : App, {
  target: document.getElementById("app")!,
});

export default app;
