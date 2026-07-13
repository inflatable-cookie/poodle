import { useState, type ChangeEvent, type CSSProperties } from "react";
import { switchTransition } from "@poodle/headless";

import "@poodle/styles/switch.css";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export type SwitchTone = "default" | "primary" | "success" | "warning" | "danger";

export interface SwitchProps {
  id?: string;
  checked?: boolean;
  defaultChecked?: boolean;
  disabled?: boolean;
  readOnly?: boolean;
  label?: string | null;
  leftLabel?: string | null;
  rightLabel?: string | null;
  ariaLabel?: string | null;
  describedBy?: string | null;
  name?: string;
  offColor?: string | null;
  onColor?: string | null;
  leftTone?: SwitchTone;
  rightTone?: SwitchTone;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onCheckedChange?: (checked: boolean) => void;
}

function toneToColor(tone: SwitchTone): string | null {
  switch (tone) {
    case "primary":
      // Stylesheet default for the on side is already the accent token via
      // the recipe chain; null keeps the inline prop channel quiet so
      // app-scope --poodle-recipe-switch-* overrides can reach the
      // component. Explicit onColor/offColor props still win.
      return null;
    case "success":
      return "var(--poodle-color-status-success)";
    case "warning":
      return "var(--poodle-color-status-warning)";
    case "danger":
      return "var(--poodle-color-status-danger)";
    default:
      return null;
  }
}

export function Switch({
  id,
  checked,
  defaultChecked = false,
  disabled = false,
  readOnly = false,
  label = null,
  leftLabel = null,
  rightLabel = null,
  ariaLabel = null,
  describedBy = null,
  name,
  offColor = null,
  onColor = null,
  leftTone = "default",
  rightTone = "primary",
  size = null,
  sizeRole = "control",
  density = null,
  onCheckedChange,
}: SwitchProps) {
  const uiPresentation = useUiPresentation();
  const [uncontrolledChecked, setUncontrolledChecked] = useState(defaultChecked);

  const isControlled = checked !== undefined;
  const currentChecked = isControlled ? checked === true : uncontrolledChecked;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const resolvedOffColor = offColor ?? toneToColor(leftTone);
  const resolvedOnColor = onColor ?? toneToColor(rightTone);
  const fallbackAriaLabel = [leftLabel, rightLabel].filter((v): v is string => Boolean(v && v.trim())).join(" / ");
  const computedAriaLabel = label ?? ariaLabel ?? (fallbackAriaLabel || null);

  const switchStyles: (CSSProperties & Record<string, string>) | undefined =
    resolvedOffColor || resolvedOnColor
      ? {
          ...(resolvedOffColor ? { "--poodle-switch-off-color": resolvedOffColor } : null),
          ...(resolvedOnColor ? { "--poodle-switch-on-color": resolvedOnColor } : null),
        }
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
      className="poodle-switch"
      data-disabled={disabled}
      data-read-only={readOnly}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      data-dual-label={leftLabel || rightLabel ? "true" : undefined}
      style={switchStyles}
    >
      <input
        id={id}
        name={name}
        className="poodle-switch__control"
        type="checkbox"
        role="switch"
        checked={currentChecked}
        disabled={disabled}
        aria-label={computedAriaLabel ?? undefined}
        aria-describedby={describedBy ?? undefined}
        aria-readonly={readOnly ? "true" : undefined}
        onChange={handleChange}
      />
      {leftLabel ? <span className="poodle-switch__label poodle-switch__label--left">{leftLabel}</span> : null}
      <span className="poodle-switch__track" aria-hidden="true">
        <span className="poodle-switch__thumb" />
      </span>
      {rightLabel ? (
        <span className="poodle-switch__label poodle-switch__label--right">{rightLabel}</span>
      ) : label ? (
        <span className="poodle-switch__label">{label}</span>
      ) : null}
    </label>
  );
}
