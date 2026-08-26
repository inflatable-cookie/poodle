import { useRef, useState, type KeyboardEvent as ReactKeyboardEvent } from "react";
import {
  toggleGroupArrowTarget,
  toggleGroupIsSelected,
  toggleGroupTabStopValue,
  toggleGroupTransition,
  type ToggleGroupContext,
} from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/toggle-group.css";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole, ToggleGroupOption } from "./types";

export interface ToggleGroupProps {
  value?: string | string[] | null;
  defaultValue?: string | string[] | null;
  options?: ToggleGroupOption[];
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  selectionMode?: "single" | "multiple";
  allowDeactivation?: boolean;
  disabled?: boolean;
  ariaLabel?: string | null;
  onValueChange?: (value: string | string[] | null) => void;
}

export function ToggleGroup({
  value,
  defaultValue = null,
  options = [],
  size = null,
  sizeRole = "control",
  density = null,
  selectionMode = "single",
  allowDeactivation = false,
  disabled = false,
  ariaLabel = null,
  onValueChange,
}: ToggleGroupProps) {
  const uiPresentation = useUiPresentation();
  const rootRef = useRef<HTMLDivElement>(null);
  const [uncontrolledValue, setUncontrolledValue] = useState<string | string[] | null>(
    () => defaultValue ?? (selectionMode === "multiple" ? [] : null),
  );

  const controlled = value !== undefined;
  const currentValue = controlled ? value : uncontrolledValue;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;

  const machineContext: ToggleGroupContext = {
    value: currentValue ?? null,
    options: options.map((option) => ({ value: option.value, disabled: disabled || option.disabled === true })),
    selectionMode,
    allowDeactivation,
    disabled,
  };

  const tabStop = toggleGroupTabStopValue(machineContext);
  const isSelected = (optionValue: string) => toggleGroupIsSelected(machineContext, optionValue);

  function itemTabIndex(optionValue: string, itemDisabled: boolean): number | undefined {
    if (selectionMode !== "single") {
      return undefined;
    }

    if (itemDisabled) {
      return -1;
    }

    return tabStop === optionValue ? 0 : -1;
  }

  function itemElement(optionValue: string): HTMLButtonElement | undefined {
    const root = rootRef.current;

    if (!root) {
      return undefined;
    }

    return Array.from(root.querySelectorAll<HTMLButtonElement>("[data-toggle-value]")).find(
      (element) => element.dataset.toggleValue === optionValue,
    );
  }

  function toggle(optionValue: string): void {
    const result = toggleGroupTransition(machineContext, { type: "TOGGLE", value: optionValue });
    for (const effect of result.effects) {
      if (effect.type === "emitValueChange") {
        if (!controlled) setUncontrolledValue(effect.value);
        onValueChange?.(effect.value);
      }
    }
  }

  function handleKeyDown(event: ReactKeyboardEvent<HTMLButtonElement>, optionValue: string): void {
    if (selectionMode !== "single") {
      return;
    }

    const direction = event.key === "ArrowRight" ? 1 : event.key === "ArrowLeft" ? -1 : null;

    if (direction === null) {
      return;
    }

    event.preventDefault();
    const target = toggleGroupArrowTarget(machineContext, optionValue, direction);

    if (!target) {
      return;
    }

    toggle(target);
    itemElement(target)?.focus();
  }

  return (
    <div
      ref={rootRef}
      className="poodle-toggle-group"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      role={selectionMode === "multiple" ? "group" : "radiogroup"}
      aria-label={ariaLabel ?? undefined}
    >
      {options.map((option) => {
        const selected = isSelected(option.value);
        const itemDisabled = disabled || option.disabled === true;
        return (
          <button
            key={option.value}
            type="button"
            className={`poodle-toggle-group__item${selected ? " poodle-selected" : ""}`}
            data-selected={selected ? "true" : "false"}
            data-toggle-value={option.value}
            tabIndex={itemTabIndex(option.value, itemDisabled)}
            disabled={itemDisabled}
            role={selectionMode === "multiple" ? "button" : "radio"}
            aria-label={option.ariaLabel ?? undefined}
            aria-pressed={selectionMode === "multiple" ? selected : undefined}
            aria-checked={selectionMode === "single" ? selected : undefined}
            onClick={() => toggle(option.value)}
            onKeyDown={(event) => handleKeyDown(event, option.value)}
          >
            {option.label}
          </button>
        );
      })}
    </div>
  );
}
