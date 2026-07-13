import "@poodle/styles/meter.css";

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
  size = null,
  sizeRole = "control",
}: MeterProps) {
  const uiPresentation = useUiPresentation();
  const safeMax = max <= min ? min + 1 : max;
  const safeValue = Math.min(Math.max(value, min), safeMax);
  const percentage = ((safeValue - min) / (safeMax - min)) * 100;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);

  return (
    <div className="poodle-meter" aria-label={ariaLabel ?? undefined} data-size={resolvedSize}>
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
        <span className="poodle-meter__fill" style={{ width: `${percentage}%` }} />
      </span>
    </div>
  );
}
