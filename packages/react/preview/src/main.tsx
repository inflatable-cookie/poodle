import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

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
import "./gallery/gallery.css";

import { App } from "./gallery/App";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
