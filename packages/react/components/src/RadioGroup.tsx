import { useId, useState, type CSSProperties } from "react";
import { singleSelectTransition } from "@inflatable-cookie/poodle-headless";

import "@inflatable-cookie/poodle-styles/radio-group.css";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, RadioGroupOption, SemanticControlSizeRole } from "./types";
import type { Orientation } from "./Separator";

export interface RadioGroupProps {
  value?: string | null;
  defaultValue?: string | null;
  options?: RadioGroupOption[];
  orientation?: Orientation;
  disabled?: boolean;
  ariaLabel?: string | null;
  describedBy?: string | null;
  name?: string;
  selectedColor?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onValueChange?: (value: string) => void;
}

export function RadioGroup({
  value,
  defaultValue = null,
  options = [],
  orientation = "vertical",
  disabled = false,
  ariaLabel = null,
  describedBy = null,
  name,
  selectedColor = null,
  size = null,
  sizeRole = "control",
  density = null,
  onValueChange,
}: RadioGroupProps) {
  const generatedName = useId();
  const uiPresentation = useUiPresentation();
  const [uncontrolledValue, setUncontrolledValue] = useState<string | null>(defaultValue);

  const isControlled = value !== undefined;
  const currentValue = isControlled ? value : uncontrolledValue;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const groupStyles = selectedColor
    ? ({ "--poodle-radio-selected-color": selectedColor } as CSSProperties)
    : undefined;

  function handleChange(nextValue: string): void {
    const result = singleSelectTransition(
      {
        value: currentValue ?? null,
        options: options.map((option) => ({ value: option.value, disabled: disabled || option.disabled === true })),
        disabled,
      },
      { type: "SELECT", value: nextValue },
    );
    for (const effect of result.effects) {
      if (effect.type === "emitValueChange") {
        if (!isControlled) setUncontrolledValue(effect.value);
        onValueChange?.(effect.value);
      }
    }
  }

  return (
    <div
      className="poodle-radio-group"
      data-orientation={orientation}
      data-disabled={disabled}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      role="radiogroup"
      aria-label={ariaLabel ?? undefined}
      aria-describedby={describedBy ?? undefined}
      style={groupStyles}
    >
      {options.map((option) => (
        <label
          key={option.value}
          className="poodle-radio-group__option"
          data-disabled={disabled || option.disabled === true}
        >
          <input
            className="poodle-radio-group__control"
            type="radio"
            name={name ?? generatedName}
            value={option.value}
            checked={currentValue === option.value}
            disabled={disabled || option.disabled === true}
            onChange={() => handleChange(option.value)}
          />
          <span className="poodle-radio-group__indicator" aria-hidden="true">
            <span className="poodle-radio-group__dot" />
          </span>
          <span className="poodle-radio-group__label">{option.label}</span>
        </label>
      ))}
    </div>
  );
}
