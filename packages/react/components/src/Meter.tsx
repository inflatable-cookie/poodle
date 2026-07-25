import "@poodle/styles/meter.css";

import type { CSSProperties } from "react";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlSize, SemanticControlSizeRole } from "./types";

export interface MeterProps {
  value?: number;
  min?: number;
  max?: number;
  low?: number | null;
  high?: number | null;
  optimum?: number | null;
  ariaLabel?: string | null;
  shape?: "linear" | "ring";
  tone?: "success" | "accent" | "warning" | "danger" | "neutral";
  showValue?: boolean;
  valueText?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
}

export function Meter({
  value = 0,
  min = 0,
  max = 100,
  low = null,
  high = null,
  optimum = null,
  ariaLabel = null,
  shape = "linear",
  tone = "success",
  showValue = false,
  valueText = null,
  size = null,
  sizeRole = "control",
}: MeterProps) {
  const uiPresentation = useUiPresentation();
  const safeMax = max <= min ? min + 1 : max;
  const safeValue = Math.min(Math.max(value, min), safeMax);
  const percentage = ((safeValue - min) / (safeMax - min)) * 100;
  // `high` wins over `low`, and drives the warning fill override in CSS.
  const level =
    high !== null && safeValue >= high
      ? "high"
      : low !== null && safeValue <= low
        ? "low"
        : "normal";
  const displayText = valueText ?? `${Math.round(percentage)}%`;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const rootStyle =
    shape === "ring"
      ? ({ "--poodle-meter-percentage": percentage } as CSSProperties)
      : undefined;

  return (
    <div
      className="poodle-meter"
      aria-label={ariaLabel ?? undefined}
      data-size={resolvedSize}
      data-shape={shape}
      data-tone={tone}
      data-level={level}
      style={rootStyle}
    >
      <meter
        className="poodle-meter__native"
        min={min}
        max={safeMax}
        low={low ?? undefined}
        high={high ?? undefined}
        optimum={optimum ?? undefined}
        value={safeValue}
      />
      <span className="poodle-meter__track" aria-hidden="true">
        <span
          className="poodle-meter__fill"
          style={shape === "ring" ? undefined : { width: `${percentage}%` }}
        />
      </span>
      {showValue ? (
        <span className="poodle-meter__value" aria-hidden="true">
          {displayText}
        </span>
      ) : null}
    </div>
  );
}
