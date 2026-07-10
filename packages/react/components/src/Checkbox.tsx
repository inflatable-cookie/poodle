import { useState } from "react";
import { checkboxParts, checkboxTransition, type CheckboxContext } from "@poodle/headless";

import "./checkbox.css";

export interface CheckboxProps {
  checked?: boolean;
  defaultChecked?: boolean;
  mixed?: boolean;
  disabled?: boolean;
  readOnly?: boolean;
  label?: string | null;
  ariaLabel?: string | null;
  onCheckedChange?: (checked: boolean) => void;
}

/**
 * Machine-backed shell: same `checkboxTransition`/`checkboxParts` the Svelte
 * layer uses; React adapter is useState + effect execution.
 */
export function Checkbox({
  checked,
  defaultChecked = false,
  mixed = false,
  disabled = false,
  readOnly = false,
  label = null,
  ariaLabel = null,
  onCheckedChange,
}: CheckboxProps) {
  const [uncontrolledChecked, setUncontrolledChecked] = useState(defaultChecked);
  const isControlled = checked !== undefined;
  const currentChecked = isControlled ? checked : uncontrolledChecked;

  const context: CheckboxContext = { checked: currentChecked, mixed, disabled, readOnly };
  const parts = checkboxParts(context, { ariaLabel, describedBy: null, hasVisibleLabel: label !== null });

  function handleChange(event: React.ChangeEvent<HTMLInputElement>) {
    const control = event.currentTarget;
    const result = checkboxTransition(context, { type: "TOGGLE", nextChecked: control.checked });

    for (const effect of result.effects) {
      if (effect.type === "revertNativeChecked") {
        control.checked = currentChecked;
      } else if (effect.type === "emitCheckedChange") {
        if (!isControlled) {
          setUncontrolledChecked(effect.checked);
        }

        onCheckedChange?.(effect.checked);
      }
    }
  }

  const { ["data-scope"]: scope, ["data-part"]: rootPart, ["data-state"]: state, ["data-disabled"]: dataDisabled } =
    parts.root as Record<string, string | boolean | undefined>;

  return (
    <label
      className="poodle-checkbox"
      data-scope={scope as string}
      data-part={rootPart as string}
      data-state={state as string}
      data-disabled={String(dataDisabled)}
      data-size="md"
      data-density="default"
    >
      <input
        className="poodle-checkbox__control"
        type="checkbox"
        checked={currentChecked}
        disabled={disabled}
        aria-label={label ? undefined : ariaLabel ?? undefined}
        aria-readonly={readOnly ? "true" : undefined}
        ref={(node) => {
          if (node) node.indeterminate = mixed;
        }}
        onChange={handleChange}
      />
      <span className="poodle-checkbox__indicator" aria-hidden="true" data-part="indicator">
        {(mixed || currentChecked) && <span className="poodle-checkbox__mark">{mixed ? "−" : "✓"}</span>}
      </span>
      {label && (
        <span className="poodle-checkbox__label" data-part="label">
          {label}
        </span>
      )}
    </label>
  );
}
