import { useEffect, useRef, useState, type ChangeEvent, type CSSProperties } from "react";
import { checkboxParts, checkboxTransition, type CheckboxContext } from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/checkbox.css";

import { Icon } from "./Icon";
import { reactifyPart } from "./parts";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface CheckboxProps {
  id?: string;
  checked?: boolean;
  defaultChecked?: boolean;
  mixed?: boolean;
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

/**
 * Machine-backed shell: same `checkboxTransition`/`checkboxParts` the Svelte
 * layer uses; React adapter is useState + effect execution.
 */
export function Checkbox({
  id,
  checked,
  defaultChecked = false,
  mixed = false,
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
}: CheckboxProps) {
  const uiPresentation = useUiPresentation();
  const inputRef = useRef<HTMLInputElement | null>(null);
  const [uncontrolledChecked, setUncontrolledChecked] = useState(defaultChecked);
  const isControlled = checked !== undefined;
  const currentChecked = isControlled ? checked : uncontrolledChecked;

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const checkboxStyles = selectedColor
    ? ({ "--poodle-checkbox-selected-color": selectedColor } as CSSProperties)
    : undefined;

  const context: CheckboxContext = { checked: currentChecked, mixed, disabled, readOnly };
  const parts = checkboxParts(context, {
    id,
    ariaLabel,
    describedBy,
    hasVisibleLabel: label !== null && label !== "",
  });

  useEffect(() => {
    if (inputRef.current) inputRef.current.indeterminate = mixed;
  }, [mixed]);

  function handleChange(event: ChangeEvent<HTMLInputElement>) {
    const control = event.currentTarget;
    const result = checkboxTransition(context, { type: "TOGGLE", nextChecked: control.checked });

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
      {...reactifyPart(parts.root as Record<string, unknown>)}
      className="poodle-checkbox"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      style={checkboxStyles}
    >
      <input
        ref={inputRef}
        {...reactifyPart(parts.control as Record<string, unknown>)}
        className="poodle-checkbox__control"
        checked={currentChecked}
        onChange={handleChange}
      />
      <span {...reactifyPart(parts.indicator as Record<string, unknown>)} className="poodle-checkbox__indicator">
        {mixed ? (
          <span className="poodle-checkbox__mark">
            <Icon name="minus" />
          </span>
        ) : currentChecked ? (
          <span className="poodle-checkbox__mark">
            <Icon name="check" />
          </span>
        ) : null}
      </span>
      {label ? (
        <span {...reactifyPart(parts.label as Record<string, unknown>)} className="poodle-checkbox__label">
          {label}
        </span>
      ) : null}
    </label>
  );
}
