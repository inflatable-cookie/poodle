import { useEffect, useRef, useState, type ChangeEvent, type FocusEvent } from "react";
import { clampNullable, parseNumberish, parseStep, validationStatusToState } from "@inflatable-cookie/poodle-headless";

import "@inflatable-cookie/poodle-styles/number-input.css";

import { Icon } from "./Icon";
import { formatNumber, snapToStep } from "./internal";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type {
  ControlDensity,
  ControlSize,
  InputValidationStatus,
  InputValidator,
  SemanticControlSizeRole,
  ValidationState,
} from "./types";

export interface NumberInputValidationChange {
  status: InputValidationStatus;
  valid: boolean;
  message: string;
}

export interface NumberInputProps {
  id?: string;
  value?: number | string | null;
  defaultValue?: number | string | null;
  placeholder?: string | null;
  name?: string;
  disabled?: boolean;
  readOnly?: boolean;
  required?: boolean;
  min?: number | string | null;
  max?: number | string | null;
  step?: number | string | null;
  precision?: number | null;
  ariaLabel?: string | null;
  describedBy?: string | null;
  prefix?: string | null;
  suffix?: string | null;
  validate?: InputValidator;
  validationContext?: unknown;
  validationState?: ValidationState;
  showSteppers?: boolean;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  onValueChange?: (value: number | string | null) => void;
  onValidationChange?: (detail: NumberInputValidationChange) => void;
  onSubmit?: (value: number | string | null) => void;
  onIncrement?: (value: number | string | null) => void;
  onDecrement?: (value: number | string | null) => void;
  onFocus?: (event: FocusEvent<HTMLInputElement>) => void;
  onBlur?: (event: FocusEvent<HTMLInputElement>) => void;
}

export function NumberInput({
  id = "",
  value,
  defaultValue = null,
  placeholder = null,
  name,
  disabled = false,
  readOnly = false,
  required = false,
  min = null,
  max = null,
  step = null,
  precision = null,
  ariaLabel = null,
  describedBy = null,
  prefix = null,
  suffix = null,
  validate,
  validationContext,
  validationState = "none",
  showSteppers = false,
  size = null,
  sizeRole = "control",
  density = null,
  onValueChange,
  onValidationChange,
  onSubmit,
  onIncrement,
  onDecrement,
  onFocus,
  onBlur,
}: NumberInputProps) {
  const uiPresentation = useUiPresentation();

  const [internalValidationStatus, setInternalValidationStatus] = useState<InputValidationStatus>("idle");
  const [uncontrolledValue, setUncontrolledValue] = useState<number | null>(() => parseNumberish(defaultValue));
  const [draftValue, setDraftValue] = useState("");
  const [isEditing, setIsEditing] = useState(false);
  const activeValidationKey = useRef(0);
  const validationMessageRef = useRef("");

  const valueMode: "number" | "string" =
    typeof value === "string" || typeof defaultValue === "string" ? "string" : "number";
  const parsedValue = parseNumberish(value);
  const parsedMin = parseNumberish(min);
  const parsedMax = parseNumberish(max);
  const resolvedStep = parseStep(step);
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const isControlled = value !== undefined;
  const currentValue = isControlled ? parsedValue : uncontrolledValue;
  const effectiveValidationState = validate
    ? validationStatusToState(internalValidationStatus, validationState)
    : validationState;
  const ariaInvalid = effectiveValidationState === "invalid" ? true : undefined;
  const ariaBusy = effectiveValidationState === "pending" ? true : undefined;

  useEffect(() => {
    if (!isEditing) {
      setDraftValue(formatNumber(currentValue, precision));
    }
  }, [isEditing, currentValue, precision]);

  function emitValidationChange(status: InputValidationStatus, message: string): void {
    onValidationChange?.({ status, valid: status === "valid" || status === "idle", message });
  }

  async function runValidation(nextValue: number | string | null): Promise<void> {
    const validationValue =
      nextValue === null || nextValue === undefined ? "" : typeof nextValue === "number" ? String(nextValue) : nextValue;

    if (!validate || validationValue.trim() === "") {
      setInternalValidationStatus("idle");
      validationMessageRef.current = "";
      emitValidationChange("idle", "");
      return;
    }

    const validationKey = ++activeValidationKey.current;
    setInternalValidationStatus("validating");
    validationMessageRef.current = "";
    emitValidationChange("validating", "");

    try {
      const result = await validate(validationValue, validationContext);
      if (validationKey !== activeValidationKey.current) return;
      const status: InputValidationStatus = result.valid ? "valid" : "invalid";
      setInternalValidationStatus(status);
      validationMessageRef.current = result.message ?? "";
      emitValidationChange(status, result.message ?? "");
    } catch {
      if (validationKey !== activeValidationKey.current) return;
      setInternalValidationStatus("invalid");
      validationMessageRef.current = "Could not validate";
      emitValidationChange("invalid", "Could not validate");
    }
  }

  function clampIfNeeded(nextValue: number): number {
    return clampNullable(nextValue, parsedMin, parsedMax);
  }

  function coerceOutgoingValue(nextValue: number | null): number | string | null {
    if (valueMode === "string") {
      return nextValue === null ? "" : String(nextValue);
    }
    return nextValue;
  }

  function commitValue(nextValue: number | null): void {
    if (!isControlled) setUncontrolledValue(nextValue);
    const outgoingValue = coerceOutgoingValue(nextValue);
    onValueChange?.(outgoingValue);
    void runValidation(outgoingValue);
  }

  function handleInput(event: ChangeEvent<HTMLInputElement>): void {
    const next = event.currentTarget.value;
    setDraftValue(next);

    if (next.trim() === "") {
      commitValue(null);
      return;
    }

    const parsedNextValue = Number(next);
    if (!Number.isNaN(parsedNextValue)) {
      commitValue(parsedNextValue);
    }
  }

  function handleBlur(event: FocusEvent<HTMLInputElement>): void {
    setIsEditing(false);

    if (draftValue.trim() !== "") {
      const parsedNextValue = Number(draftValue);
      if (!Number.isNaN(parsedNextValue)) {
        commitValue(clampIfNeeded(snapToStep(parsedNextValue, parsedMin ?? 0, resolvedStep)));
      }
    }

    onBlur?.(event);
  }

  function adjust(delta: number, eventName: "increment" | "decrement"): void {
    const baseline = currentValue ?? parsedMin ?? 0;
    const nextValue = clampIfNeeded(snapToStep(baseline + delta, parsedMin ?? 0, resolvedStep));
    commitValue(nextValue);
    setDraftValue(formatNumber(nextValue, precision));
    const outgoingValue = coerceOutgoingValue(nextValue);
    if (eventName === "increment") {
      onIncrement?.(outgoingValue);
      return;
    }
    onDecrement?.(outgoingValue);
  }

  return (
    <div
      className={`poodle-number-input${prefix ? " poodle-with-prefix" : ""}${suffix ? " poodle-with-suffix" : ""}`}
    >
      {prefix ? <span className="poodle-number-input__prefix">{prefix}</span> : null}

      <div
        className="poodle-number-input__field"
        data-validation-state={effectiveValidationState}
        data-size={resolvedSize}
        data-density={resolvedDensity}
        data-disabled={disabled}
      >
        <input
          id={id}
          name={name}
          className="poodle-number-input__control"
          type="text"
          inputMode="decimal"
          value={draftValue}
          placeholder={placeholder ?? undefined}
          disabled={disabled}
          readOnly={readOnly}
          required={required}
          aria-label={ariaLabel ?? undefined}
          aria-describedby={describedBy ?? undefined}
          aria-invalid={ariaInvalid}
          aria-busy={ariaBusy}
          onChange={handleInput}
          onFocus={(event) => {
            setIsEditing(true);
            onFocus?.(event);
          }}
          onBlur={handleBlur}
          onKeyDown={(event) => {
            if (event.key === "Enter") {
              onSubmit?.(coerceOutgoingValue(currentValue));
            }
            if (event.key === "ArrowUp" && !readOnly) {
              event.preventDefault();
              adjust(resolvedStep, "increment");
            }
            if (event.key === "ArrowDown" && !readOnly) {
              event.preventDefault();
              adjust(-resolvedStep, "decrement");
            }
          }}
        />

        {showSteppers ? (
          <div className="poodle-number-input__steppers">
            <button type="button" disabled={disabled || readOnly} onClick={() => adjust(resolvedStep, "increment")}>
              <Icon name="plus" />
            </button>
            <button type="button" disabled={disabled || readOnly} onClick={() => adjust(-resolvedStep, "decrement")}>
              <Icon name="minus" />
            </button>
          </div>
        ) : null}
      </div>

      {suffix ? <span className="poodle-number-input__suffix">{suffix}</span> : null}
    </div>
  );
}
