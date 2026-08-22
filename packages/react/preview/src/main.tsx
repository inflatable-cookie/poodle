import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

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
import "./gallery/gallery.css";
import "../../../svelte/preview/src/catalogue.css";

import { App } from "./gallery/App";
import { FixtureHost } from "./fixture-host/FixtureHost";

// g15.047: `?fixture=…` swaps the gallery for the capture-only fixture host.
const isFixtureCapture = new URLSearchParams(window.location.search).has("fixture");

createRoot(document.getElementById("root")!).render(
  isFixtureCapture ? (
    <FixtureHost />
  ) : (
    <StrictMode>
      <App />
    </StrictMode>
  ),
);
