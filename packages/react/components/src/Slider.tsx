import { useState, type CSSProperties, type FormEvent } from "react";
import { normalizeSliderValue, safeSliderMax, sliderTransition, type SliderContext } from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/slider.css";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";
import type { Orientation } from "./Separator";

export interface SliderProps {
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  value?: number;
  defaultValue?: number;
  min?: number;
  max?: number;
  step?: number;
  orientation?: Orientation;
  disabled?: boolean;
  ariaLabel?: string | null;
  valueText?: string | null;
  onValueChange?: (value: number) => void;
  onValueCommit?: (value: number) => void;
}

export function Slider({
  size = null,
  sizeRole = "control",
  density = null,
  value,
  defaultValue = 0,
  min = 0,
  max = 100,
  step = 1,
  orientation = "horizontal",
  disabled = false,
  ariaLabel = null,
  valueText = null,
  onValueChange,
  onValueCommit,
}: SliderProps) {
  const uiPresentation = useUiPresentation();
  const [uncontrolledValue, setUncontrolledValue] = useState(defaultValue);

  const isControlled = value !== undefined;
  const currentValue = isControlled ? value : uncontrolledValue;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const machineContext: SliderContext = { value: currentValue, min, max, step, disabled };
  const safeMax = safeSliderMax(min, max);
  const displayValue = normalizeSliderValue(machineContext, currentValue);
  const percentage = ((displayValue - min) / (safeMax - min)) * 100;
  const sliderStyle = { "--poodle-slider-percent": `${percentage}%` } as CSSProperties;

  function send(type: "INPUT" | "COMMIT", event: FormEvent<HTMLInputElement>): void {
    const raw = Number(event.currentTarget.value);
    const result = sliderTransition(machineContext, { type, raw });
    for (const effect of result.effects) {
      if (!isControlled) setUncontrolledValue(effect.value);
      if (effect.type === "emitValueChange") {
        onValueChange?.(effect.value);
      } else if (effect.type === "emitValueCommit") {
        onValueCommit?.(effect.value);
      }
    }
  }

  return (
    <div
      className="poodle-slider"
      data-orientation={orientation}
      data-disabled={disabled}
      style={sliderStyle}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      <span className="poodle-slider__track" aria-hidden="true">
        <span className="poodle-slider__fill" />
      </span>
      <input
        className="poodle-slider__control"
        type="range"
        min={min}
        max={safeMax}
        step={step}
        value={displayValue}
        disabled={disabled}
        aria-label={ariaLabel ?? undefined}
        aria-valuetext={valueText ?? undefined}
        onInput={(event) => send("INPUT", event)}
        onChange={(event) => send("COMMIT", event)}
      />
    </div>
  );
}
