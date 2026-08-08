import { useState, type ChangeEvent } from "react";

import "@inflatable-cookie/poodle-core/styles/time-input.css";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface TimeInputProps {
  id?: string | null;
  value?: string | null;
  defaultValue?: string | null;
  min?: string | null;
  max?: string | null;
  step?: number;
  disabled?: boolean;
  ariaLabel?: string | null;
  describedBy?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onValueChange?: (value: string | null) => void;
}

export function TimeInput({
  id = null,
  value,
  defaultValue = null,
  min = null,
  max = null,
  step = 60,
  disabled = false,
  ariaLabel = null,
  describedBy = null,
  size = null,
  sizeRole = "control",
  density = null,
  onValueChange,
}: TimeInputProps) {
  const uiPresentation = useUiPresentation();
  const [uncontrolledValue, setUncontrolledValue] = useState<string | null>(defaultValue);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const isControlled = value !== undefined;
  const currentValue = (isControlled ? value : uncontrolledValue) ?? "";

  function handleInput(event: ChangeEvent<HTMLInputElement>): void {
    const nextValue = event.currentTarget.value || null;
    if (!isControlled) setUncontrolledValue(nextValue);
    onValueChange?.(nextValue);
  }

  return (
    <input
      id={id ?? undefined}
      className="poodle-time-input"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      type="time"
      value={currentValue}
      min={min ?? undefined}
      max={max ?? undefined}
      step={step}
      disabled={disabled}
      aria-label={ariaLabel ?? undefined}
      aria-describedby={describedBy ?? undefined}
      onChange={handleInput}
    />
  );
}
