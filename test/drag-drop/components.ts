/**
 * Mounted Svelte and React component fixtures for the drag substrate.
 *
 * `g16.028` migrated ModelCatalogueEditor, OrderBy, and BlockEditor onto the
 * common substrate. The custom-surface fixture next door proves the substrate;
 * this page proves the *components* on it, in both web frameworks, including
 * two instances that deliberately share item ids under one provider.
 */
import "@inflatable-cookie/poodle-core/styles/drag-drop.css";

import { mount } from "svelte";
import { createRoot } from "react-dom/client";
import { createElement } from "react";

import ComponentsHarness from "./ComponentsHarness.svelte";
import { ComponentsHarness as ReactComponentsHarness } from "./components-react";

mount(ComponentsHarness, { target: document.getElementById("svelte-mount") as HTMLElement });
createRoot(document.getElementById("react-mount") as HTMLElement).render(
  createElement(ReactComponentsHarness),
);
