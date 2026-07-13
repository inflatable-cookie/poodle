import type { CSSProperties } from "react";

import "@poodle/styles/spinner.css";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, SemanticControlSizeRole, SpinnerSize, SpinnerTone, SpinnerVariant } from "./types";

export interface SpinnerProps {
  variant?: SpinnerVariant;
  size?: SpinnerSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  tone?: SpinnerTone;
  ariaLabel?: string | null;
  className?: string;
  style?: CSSProperties;
}

const GRID_PHASES = ["tl", "tr", "ml", "mr", "bl", "br"] as const;

export function Spinner({
  variant = "ring",
  size = null,
  sizeRole = "control",
  density = null,
  tone = "current",
  ariaLabel = null,
  className = "",
  style,
}: SpinnerProps) {
  const uiPresentation = useUiPresentation();
  const resolvedSize = (size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole)) as SpinnerSize;
  const resolvedDensity = density ?? uiPresentation.density;

  return (
    <span
      className={`poodle-spinner ${className}`.trim()}
      data-variant={variant}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-tone={tone}
      style={style}
      role={ariaLabel ? "status" : undefined}
      aria-label={ariaLabel ?? undefined}
      aria-live={ariaLabel ? "polite" : undefined}
      aria-hidden={ariaLabel ? undefined : "true"}
    >
      {variant === "ring" ? (
        <span className="poodle-spinner__ring" aria-hidden="true" />
      ) : (
        <span className="poodle-spinner__grid" aria-hidden="true">
          {GRID_PHASES.map((phase) => (
            <span key={phase} className="poodle-spinner__cell" data-phase={phase} />
          ))}
        </span>
      )}
    </span>
  );
}
