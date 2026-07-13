import type { ReactNode } from "react";

import "@poodle/styles/status-indicator.css";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole, StatusTone } from "./types";

export interface StatusIndicatorProps {
  status?: StatusTone;
  label?: string | null;
  ariaLabel?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  typography?: "label" | "inherit";
  children?: ReactNode;
}

export function StatusIndicator({
  status = "neutral",
  label = null,
  ariaLabel = null,
  size = null,
  sizeRole = "control",
  density = null,
  typography = "label",
  children,
}: StatusIndicatorProps) {
  const uiPresentation = useUiPresentation();
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;

  return (
    <span
      className="poodle-status-indicator"
      data-status={status}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-typography={typography}
      aria-label={ariaLabel ?? undefined}
    >
      <span className="poodle-status-indicator__dot" aria-hidden="true" />
      {label ? <span className="poodle-status-indicator__label">{label}</span> : children}
    </span>
  );
}
