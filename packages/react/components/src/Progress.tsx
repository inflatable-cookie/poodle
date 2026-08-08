import "@inflatable-cookie/poodle-styles/progress.css";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlSize, SemanticControlSizeRole } from "./types";

export interface ProgressProps {
  value?: number | null;
  max?: number;
  indeterminate?: boolean;
  ariaLabel?: string | null;
  valueText?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
}

export function Progress({
  value = null,
  max = 100,
  indeterminate = false,
  ariaLabel = null,
  valueText = null,
  size = null,
  sizeRole = "control",
}: ProgressProps) {
  const uiPresentation = useUiPresentation();
  const safeMax = max <= 0 ? 100 : max;
  const safeValue = value === null ? null : Math.min(Math.max(value, 0), safeMax);
  const percentage = safeValue === null ? 0 : safeValue / safeMax;
  const computedValueText = !indeterminate && safeValue !== null ? `${Math.round(percentage * 100)}%` : null;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);

  return (
    <div
      className="poodle-progress"
      data-size={resolvedSize}
      data-indeterminate={indeterminate}
      role="progressbar"
      aria-label={ariaLabel ?? undefined}
      aria-valuemin={indeterminate ? undefined : 0}
      aria-valuemax={indeterminate ? undefined : safeMax}
      aria-valuenow={indeterminate || safeValue === null ? undefined : safeValue}
      aria-valuetext={valueText ?? computedValueText ?? undefined}
    >
      <span
        className="poodle-progress__indicator"
        style={indeterminate ? undefined : { transform: `scaleX(${percentage})` }}
      />
    </div>
  );
}
