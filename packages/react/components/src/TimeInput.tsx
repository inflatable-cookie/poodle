import { useEffect, useState, type ChangeEvent, type KeyboardEvent } from "react";

import {
  timeInputContext,
  timeInputInvalid,
  timeInputTransition,
  type TimeInputDraft,
} from "@inflatable-cookie/poodle-core";
import "@inflatable-cookie/poodle-core/styles/time-input.css";

import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

export interface TimeInputProps {
  id?: string | null;
  value?: string | null;
  defaultValue?: string | null;
  min?: string | null;
  max?: string | null;
  step?: number;
  disabled?: boolean;
  ariaLabel?: string | null;
  describedBy?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onValueChange?: (value: string | null) => void;
}

export function TimeInput({
  id = null,
  value,
  defaultValue = null,
  min = null,
  max = null,
  step = 60,
  disabled = false,
  ariaLabel = null,
  describedBy = null,
  size = null,
  sizeRole = "control",
  density = null,
  onValueChange,
}: TimeInputProps) {
  const uiPresentation = useUiPresentation();
  const [uncontrolledValue, setUncontrolledValue] = useState<string | null>(defaultValue);
  const [localDraft, setLocalDraft] = useState<TimeInputDraft | null>(null);
  const [nativeDraftText, setNativeDraftText] = useState<string | null>(null);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const isControlled = value !== undefined;
  const committed = (isControlled ? value : uncontrolledValue) ?? null;
  const machineContext = timeInputContext({
    committed,
    defaultValue,
    draft: localDraft,
    min,
    max,
    step,
    disabled,
  });
  const displayValue = nativeDraftText ?? committed ?? "";
  const invalid = timeInputInvalid(machineContext);

  useEffect(() => {
    setLocalDraft(null);
    setNativeDraftText(null);
  }, [value]);

  function commitEmitted(next: string | null): void {
    if (!isControlled) {
      setUncontrolledValue(next);
    }

    onValueChange?.(next);
  }

  function handleChange(event: ChangeEvent<HTMLInputElement>): void {
    const input = event.currentTarget;
    const text = input.value;

    // Native incomplete drafts report `value === ""` with `validity.badInput`.
    // A deliberate clear reports empty without badInput.
    if (text === "" && input.validity.badInput) {
      setLocalDraft(localDraft ?? { hour: "", minute: "", second: "" });
      setNativeDraftText("");
      return;
    }

    const result = timeInputTransition(machineContext, { type: "COMMIT_TEXT", text });
    setLocalDraft(result.context.draft);
    setNativeDraftText(result.context.draft === null ? null : text);

    for (const effect of result.effects) {
      commitEmitted(effect.value);
    }
  }

  function revertDraft(type: "BLUR" | "ESCAPE"): void {
    if (localDraft === null) {
      return;
    }

    const result = timeInputTransition(machineContext, { type });
    setLocalDraft(result.context.draft);
    setNativeDraftText(null);
  }

  function handleBlur(): void {
    revertDraft("BLUR");
  }

  function handleKeyDown(event: KeyboardEvent<HTMLInputElement>): void {
    if (event.key !== "Escape") {
      return;
    }

    event.preventDefault();
    revertDraft("ESCAPE");
  }

  return (
    <input
      id={id ?? undefined}
      className="poodle-time-input"
      data-size={resolvedSize}
      data-density={resolvedDensity}
      type="time"
      value={displayValue}
      min={min ?? undefined}
      max={max ?? undefined}
      step={step}
      disabled={disabled}
      aria-invalid={invalid ? "true" : undefined}
      aria-label={ariaLabel ?? undefined}
      aria-describedby={describedBy ?? undefined}
      onChange={handleChange}
      onBlur={handleBlur}
      onKeyDown={handleKeyDown}
    />
  );
}
