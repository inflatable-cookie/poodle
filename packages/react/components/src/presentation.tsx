import { createContext, useContext, type CSSProperties, type ReactNode } from "react";

// The shared stylesheet is the only definition of the root's `display: contents`,
// which is what keeps the wrapper layout- and accessibility-neutral. Without it
// the provider is a plain block and every descendant layout shifts.
import "@inflatable-cookie/poodle-core/styles/ui-presentation-provider.css";

import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

/**
 * UI presentation context: app-wide density and size-scale defaults.
 * Mirrors the Svelte package's `presentation.ts` — the resolver tables must
 * stay identical (the Svelte implementation is the reference).
 */

export interface UiPresentationContextValue {
  density: ControlDensity;
  sizeScale: ControlSize;
}

const DEFAULT_UI_PRESENTATION: UiPresentationContextValue = {
  density: "default",
  sizeScale: "md",
};

const UiPresentationContext = createContext<UiPresentationContextValue>(DEFAULT_UI_PRESENTATION);

export function UiPresentationProvider({
  density = "default",
  sizeScale = "md",
  children,
}: Partial<UiPresentationContextValue> & { children: ReactNode }) {
  const providerStyle: CSSProperties = {
    "--poodle-size-control-height": `${controlHeightRem(sizeScale)}rem`,
    "--poodle-space-control-x": `${controlSpaceXRem(density)}rem`,
    "--poodle-space-panel-x": `${panelSpaceXRem(density)}rem`,
    "--poodle-space-panel-y": `${panelSpaceYRem(density)}rem`,
  } as CSSProperties;

  return (
    <UiPresentationContext.Provider value={{ density, sizeScale }}>
      <div className="poodle-ui-presentation-provider" style={providerStyle}>
        {children}
      </div>
    </UiPresentationContext.Provider>
  );
}

export function useUiPresentation(): UiPresentationContextValue {
  return useContext(UiPresentationContext);
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

export function resolveSupportingVisualSize(size: ControlSize): ControlSize {
  if (size === "xl") return "lg";
  if (size === "lg") return "md";
  if (size === "md") return "sm";
  return size;
}
