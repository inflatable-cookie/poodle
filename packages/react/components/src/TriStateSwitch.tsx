import { useId, useState, type CSSProperties } from "react";
import { singleSelectTransition } from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/tri-state-switch.css";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole, TriStateValue } from "./types";

export interface TriStateSwitchProps {
  value?: TriStateValue;
  defaultValue?: TriStateValue;
  options?: Record<TriStateValue, string>;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  disabled?: boolean;
  ariaLabel: string;
  excludedColor?: string | null;
  defaultColor?: string | null;
  includedColor?: string | null;
  onValueChange?: (value: TriStateValue) => void;
}

const ORDERED_VALUES: TriStateValue[] = ["excluded", "default", "included"];

export function TriStateSwitch({
  value,
  defaultValue = "default",
  options = { excluded: "Exclude", default: "Default", included: "Include" },
  size = null,
  sizeRole = "control",
  density = null,
  disabled = false,
  ariaLabel,
  excludedColor = null,
  defaultColor = null,
  includedColor = null,
  onValueChange,
}: TriStateSwitchProps) {
  const groupName = useId();
  const uiPresentation = useUiPresentation();
  const [uncontrolledValue, setUncontrolledValue] = useState<TriStateValue>(defaultValue);

  const isControlled = value !== undefined;
  const currentValue = isControlled ? value : uncontrolledValue;
  const selectedIndex = Math.max(0, ORDERED_VALUES.indexOf(currentValue));
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;

  const style: CSSProperties & Record<string, string> = {
    "--poodle-tri-state-active-index": String(selectedIndex),
    ...(excludedColor ? { "--poodle-tri-state-excluded-color": excludedColor } : null),
    ...(defaultColor ? { "--poodle-tri-state-default-color": defaultColor } : null),
    ...(includedColor ? { "--poodle-tri-state-included-color": includedColor } : null),
  };

  function handleSelect(nextValue: TriStateValue): void {
    const result = singleSelectTransition(
      { value: currentValue, options: ORDERED_VALUES.map((candidate) => ({ value: candidate })), disabled },
      { type: "SELECT", value: nextValue },
    );
    for (const effect of result.effects) {
      if (effect.type === "emitValueChange") {
        if (!isControlled) setUncontrolledValue(effect.value as TriStateValue);
        onValueChange?.(effect.value as TriStateValue);
      }
    }
  }

  return (
    <div
      className="poodle-tri-state-switch"
      role="radiogroup"
      aria-label={ariaLabel}
      aria-disabled={disabled ? "true" : undefined}
      data-state={currentValue}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-disabled={disabled}
      style={style}
    >
      <span className="poodle-tri-state-switch__selection" aria-hidden="true" />
      {ORDERED_VALUES.map((optionValue) => (
        <label
          key={optionValue}
          className="poodle-tri-state-switch__option"
          data-state={optionValue}
          data-selected={currentValue === optionValue}
        >
          <input
            className="poodle-tri-state-switch__control"
            type="radio"
            name={groupName}
            checked={currentValue === optionValue}
            disabled={disabled}
            aria-label={options[optionValue]}
            onChange={() => handleSelect(optionValue)}
          />
          <span className="poodle-tri-state-switch__segment">{options[optionValue]}</span>
        </label>
      ))}
    </div>
  );
}
