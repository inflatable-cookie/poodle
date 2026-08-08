import { useId, useState } from "react";
import { singleSelectTransition } from "@inflatable-cookie/poodle-headless";

import "@inflatable-cookie/poodle-styles/segmented-control.css";

import { Icon } from "./Icon";
import { resolveSemanticControlSize, resolveSupportingVisualSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SegmentedControlOption, SemanticControlSizeRole } from "./types";

export interface SegmentedControlProps {
  value?: string | null;
  defaultValue?: string | null;
  options?: SegmentedControlOption[];
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  disabled?: boolean;
  ariaLabel?: string | null;
  name?: string;
  equalWidth?: boolean;
  onValueChange?: (value: string) => void;
}

export function SegmentedControl({
  value,
  defaultValue = null,
  options = [],
  size = null,
  sizeRole = "control",
  density = null,
  disabled = false,
  ariaLabel = null,
  name,
  equalWidth = true,
  onValueChange,
}: SegmentedControlProps) {
  const generatedName = useId();
  const uiPresentation = useUiPresentation();
  const [uncontrolledValue, setUncontrolledValue] = useState<string | null>(defaultValue);

  const isControlled = value !== undefined;
  const currentValue = isControlled ? value : uncontrolledValue;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedIconSize = resolveSupportingVisualSize(resolvedSize);
  const resolvedDensity = density ?? uiPresentation.density;

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
      className="poodle-segmented-control"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-equal-width={equalWidth}
      role="radiogroup"
      aria-label={ariaLabel ?? undefined}
    >
      {options.map((option) => (
        <label
          key={option.value}
          className="poodle-segmented-control__segment"
          data-selected={currentValue === option.value}
          data-icon-only={option.iconOnly === true && option.icon ? "true" : undefined}
          title={option.title ?? (option.iconOnly && option.icon ? option.label : undefined)}
        >
          <input
            className="poodle-segmented-control__control"
            type="radio"
            name={name ?? generatedName}
            value={option.value}
            checked={currentValue === option.value}
            disabled={disabled || option.disabled === true}
            aria-label={option.ariaLabel ?? (option.iconOnly && option.icon ? option.label : undefined)}
            onChange={() => handleChange(option.value)}
          />
          <span className="poodle-segmented-control__label">
            {option.icon ? <Icon icon={option.icon} size={resolvedIconSize} /> : null}
            {!option.iconOnly || !option.icon ? (
              <span className="poodle-segmented-control__label-text">{option.label}</span>
            ) : null}
          </span>
        </label>
      ))}
    </div>
  );
}
