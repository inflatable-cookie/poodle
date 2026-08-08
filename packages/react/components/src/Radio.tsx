import { useState, type ChangeEvent, type CSSProperties } from "react";
import { switchTransition } from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/radio.css";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface RadioProps {
  id?: string;
  name?: string;
  value?: string;
  checked?: boolean;
  defaultChecked?: boolean;
  disabled?: boolean;
  readOnly?: boolean;
  label?: string | null;
  ariaLabel?: string | null;
  describedBy?: string | null;
  selectedColor?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onCheckedChange?: (checked: boolean) => void;
}

export function Radio({
  id,
  name,
  value,
  checked,
  defaultChecked = false,
  disabled = false,
  readOnly = false,
  label = null,
  ariaLabel = null,
  describedBy = null,
  selectedColor = null,
  size = null,
  sizeRole = "control",
  density = null,
  onCheckedChange,
}: RadioProps) {
  const uiPresentation = useUiPresentation();
  const [uncontrolledChecked, setUncontrolledChecked] = useState(defaultChecked);

  const isControlled = checked !== undefined;
  const currentChecked = isControlled ? checked === true : uncontrolledChecked;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const radioStyles = selectedColor
    ? ({ "--poodle-radio-selected-color": selectedColor } as CSSProperties)
    : undefined;

  function handleChange(event: ChangeEvent<HTMLInputElement>): void {
    const control = event.currentTarget;
    const result = switchTransition(
      { checked: currentChecked, disabled, readOnly },
      { type: "TOGGLE", nextChecked: control.checked },
    );
    for (const effect of result.effects) {
      if (effect.type === "revertNativeChecked") {
        control.checked = currentChecked;
      } else if (effect.type === "emitCheckedChange") {
        if (!isControlled) setUncontrolledChecked(effect.checked);
        onCheckedChange?.(effect.checked);
      }
    }
  }

  return (
    <label
      className="poodle-radio"
      data-disabled={disabled}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      style={radioStyles}
    >
      <input
        id={id}
        name={name}
        value={value}
        className="poodle-radio__control"
        type="radio"
        checked={currentChecked}
        disabled={disabled}
        aria-label={label ? undefined : (ariaLabel ?? undefined)}
        aria-describedby={describedBy ?? undefined}
        onChange={handleChange}
      />
      <span className="poodle-radio__indicator" aria-hidden="true">
        <span className="poodle-radio__dot" />
      </span>
      {label ? <span className="poodle-radio__label">{label}</span> : null}
    </label>
  );
}
