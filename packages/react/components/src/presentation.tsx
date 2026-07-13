import { createContext, useContext, type ReactNode } from "react";

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
  return (
    <UiPresentationContext.Provider value={{ density, sizeScale }}>
      {children}
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

export function resolveSupportingVisualSize(size: ControlSize): ControlSize {
  if (size === "xl") return "lg";
  if (size === "lg") return "md";
  if (size === "md") return "sm";
  return size;
}
