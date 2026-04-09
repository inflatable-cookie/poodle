import { getContext, setContext } from "svelte";
import { readable, writable, type Readable, type Writable } from "svelte/store";

import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface UiPresentationContextValue {
  density: ControlDensity;
  sizeScale: ControlSize;
}

type UiPresentationStore = Readable<UiPresentationContextValue> & UiPresentationContextValue;

const POODLE_UI_PRESENTATION = Symbol("poodle-ui-presentation");
const DEFAULT_UI_PRESENTATION: UiPresentationContextValue = {
  density: "default",
  sizeScale: "md",
};
const DEFAULT_UI_PRESENTATION_STORE = readable(DEFAULT_UI_PRESENTATION) as UiPresentationStore;

export function setUiPresentation(value: UiPresentationContextValue): Writable<UiPresentationContextValue> {
  const store = writable(value);
  setContext(POODLE_UI_PRESENTATION, store);
  return store;
}

export function getUiPresentation(): UiPresentationStore {
  return (
    getContext<Readable<UiPresentationContextValue>>(POODLE_UI_PRESENTATION) ?? DEFAULT_UI_PRESENTATION_STORE
  ) as UiPresentationStore;
}

export function resolveSemanticControlSize(
  sizeScale: ControlSize,
  role: SemanticControlSizeRole,
): ControlSize {
  if (sizeScale === "xs") {
    if (role === "prominent") return "sm";
    return "xs";
  }

  if (sizeScale === "sm") {
    if (role === "prominent") return "md";
    return "sm";
  }

  if (sizeScale === "md") {
    if (role === "chrome") return "sm";
    if (role === "prominent") return "lg";
    return "md";
  }

  if (sizeScale === "lg") {
    if (role === "chrome") return "md";
    if (role === "prominent") return "xl";
    return "lg";
  }

  if (role === "chrome") return "lg";
  return "xl";
}

export function controlHeightRem(size: ControlSize): number {
  if (size === "xs") return 1.5;
  if (size === "sm") return 1.75;
  if (size === "lg") return 2.75;
  if (size === "xl") return 3.25;
  return 2.25;
}

export function resolveSupportingVisualSize(size: ControlSize): ControlSize {
  if (size === "xl") return "lg";
  if (size === "lg") return "md";
  if (size === "md") return "sm";
  return size;
}

export function controlSpaceXRem(density: ControlDensity): number {
  if (density === "compact") return 0.5;
  if (density === "comfortable") return 1;
  return 0.75;
}

export function panelSpaceXRem(density: ControlDensity): number {
  if (density === "compact") return 0.75;
  if (density === "comfortable") return 1.25;
  return 1;
}

export function panelSpaceYRem(density: ControlDensity): number {
  if (density === "compact") return 0.5;
  if (density === "comfortable") return 1;
  return 0.75;
}
