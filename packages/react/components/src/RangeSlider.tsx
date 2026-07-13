import { useState, type CSSProperties, type FormEvent } from "react";
import {
  normalizeRangeValue,
  rangeSliderTransition,
  safeSliderMax,
  type RangeSliderContext,
} from "@poodle/headless";

import "@poodle/styles/range-slider.css";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";
import type { Orientation } from "./Separator";

export interface RangeSliderProps {
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  value?: [number, number];
  defaultValue?: [number, number];
  min?: number;
  max?: number;
  step?: number;
  orientation?: Orientation;
  disabled?: boolean;
  ariaLabel?: string | null;
  lowerValueText?: string | null;
  upperValueText?: string | null;
  onValueChange?: (value: [number, number]) => void;
  onValueCommit?: (value: [number, number]) => void;
}

export function RangeSlider({
  size = null,
  sizeRole = "control",
  density = null,
  value,
  defaultValue = [0, 100],
  min = 0,
  max = 100,
  step = 1,
  orientation = "horizontal",
  disabled = false,
  ariaLabel = null,
  lowerValueText = null,
  upperValueText = null,
  onValueChange,
  onValueCommit,
}: RangeSliderProps) {
  const uiPresentation = useUiPresentation();
  const [uncontrolledValue, setUncontrolledValue] = useState<[number, number]>(defaultValue);

  const isControlled = value !== undefined;
  const currentValue = isControlled ? value : uncontrolledValue;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const machineContext: RangeSliderContext = { value: currentValue, min, max, step, disabled };
  const safeMax = safeSliderMax(min, max);
  const [displayLower, displayUpper] = normalizeRangeValue(machineContext);
  const lowerPercent = ((displayLower - min) / (safeMax - min)) * 100;
  const upperPercent = ((displayUpper - min) / (safeMax - min)) * 100;
  const rangeStyle = {
    "--poodle-range-start": `${lowerPercent}%`,
    "--poodle-range-end": `${upperPercent}%`,
  } as CSSProperties;

  function send(type: "INPUT" | "COMMIT", thumb: "lower" | "upper", event: FormEvent<HTMLInputElement>): void {
    const raw = Number(event.currentTarget.value);
    const result = rangeSliderTransition(machineContext, { type, thumb, raw });
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
      className="poodle-range-slider"
      data-orientation={orientation}
      data-disabled={disabled}
      style={rangeStyle}
      data-size={resolvedSize}
      data-density={resolvedDensity}
    >
      <span className="poodle-range-slider__track" aria-hidden="true">
        <span className="poodle-range-slider__fill" />
      </span>

      <input
        className="poodle-range-slider__control poodle-range-slider__control--lower"
        type="range"
        min={min}
        max={safeMax}
        step={step}
        value={displayLower}
        disabled={disabled}
        aria-label={ariaLabel ? `${ariaLabel} minimum` : "Minimum value"}
        aria-valuetext={lowerValueText ?? undefined}
        onInput={(event) => send("INPUT", "lower", event)}
        onChange={(event) => send("COMMIT", "lower", event)}
      />

      <input
        className="poodle-range-slider__control poodle-range-slider__control--upper"
        type="range"
        min={min}
        max={safeMax}
        step={step}
        value={displayUpper}
        disabled={disabled}
        aria-label={ariaLabel ? `${ariaLabel} maximum` : "Maximum value"}
        aria-valuetext={upperValueText ?? undefined}
        onInput={(event) => send("INPUT", "upper", event)}
        onChange={(event) => send("COMMIT", "upper", event)}
      />
    </div>
  );
}
