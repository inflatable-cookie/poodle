import { useEffect, useRef, useState, type FocusEvent, type KeyboardEvent } from "react";
import {
  formatNumberCommitted,
  numberDecimalToNumber,
  numberDraftConstraintValid,
  numberInputContext,
  numberInputDisplayText,
  numberInputInvalid,
  numberInputTransition,
  parseNumberDecimal,
  stepNumberValue,
  validationStatusToState,
  type NumberInputEffect,
} from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/number-input.css";

import { Icon } from "./Icon";
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
  value?: number | null;
  defaultValue?: number | null;
  draftValue?: string | null;
  placeholder?: string | null;
  name?: string;
  disabled?: boolean;
  readOnly?: boolean;
  required?: boolean;
  min?: number | null;
  max?: number | null;
  step?: number | null;
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
  onValueChange?: (value: number | null) => void;
  onDraftValueChange?: (draft: string | null) => void;
  onValidationChange?: (detail: NumberInputValidationChange) => void;
  onCommit?: (value: number | null) => void;
  onFocus?: (event: FocusEvent<HTMLInputElement>) => void;
  onBlur?: (event: FocusEvent<HTMLInputElement>) => void;
}

function draftNumeric(
  draft: string | null | undefined,
  min: number | null,
  max: number | null,
  step: number | null,
  precision: number | null,
): number | null | undefined {
  // undefined = no draft channel → fall back to committed for a11y.
  // null from this helper is unused; empty string means omit aria-valuenow.
  if (draft === undefined) {
    return null;
  }

  if (draft === null || draft === "") {
    return undefined;
  }

  if (!numberDraftConstraintValid(draft, min, max, step, precision)) {
    return undefined;
  }

  const decimal = parseNumberDecimal(draft);
  return decimal === null ? undefined : numberDecimalToNumber(decimal);
}

export function NumberInput({
  id = "",
  value,
  defaultValue = null,
  draftValue,
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
  onDraftValueChange,
  onValidationChange,
  onCommit,
  onFocus,
  onBlur,
}: NumberInputProps) {
  const uiPresentation = useUiPresentation();

  const [internalValidationStatus, setInternalValidationStatus] = useState<InputValidationStatus>("idle");
  const [uncontrolledValue, setUncontrolledValue] = useState<number | null>(defaultValue);
  const [localDraft, setLocalDraft] = useState<string | null>(null);
  const lastControlledValue = useRef<number | null | undefined>(undefined);
  const lastEmittedValue = useRef<number | null | undefined>(undefined);
  const activeValidationKey = useRef(0);
  const validationMessageRef = useRef("");

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const isControlled = value !== undefined;
  const draftControlled = draftValue !== undefined;
  const committed = (isControlled ? value : uncontrolledValue) ?? null;
  const activeDraft = draftControlled ? draftValue : localDraft;

  useEffect(() => {
    if (value === undefined || value === lastControlledValue.current) {
      return;
    }

    const previous = lastControlledValue.current;
    lastControlledValue.current = value;

    if (previous !== undefined && value === lastEmittedValue.current) {
      return;
    }

    if (!draftControlled) {
      setLocalDraft(null);
    }
  }, [value, draftControlled]);

  const machineContext = numberInputContext({
    committed,
    defaultValue,
    draft: activeDraft ?? null,
    min,
    max,
    step,
    precision,
    disabled,
    readOnly,
  });
  const displayValue = numberInputDisplayText(machineContext);
  const draftInvalid = numberInputInvalid(machineContext);
  const effectiveValidationState = validate
    ? validationStatusToState(internalValidationStatus, validationState)
    : validationState;
  const ariaInvalid = draftInvalid || effectiveValidationState === "invalid" ? true : undefined;
  const ariaBusy = effectiveValidationState === "pending" ? true : undefined;
  const draftNow = draftNumeric(activeDraft, min, max, step, precision);
  // No draft → committed; empty/invalid draft → omit; valid draft → that value.
  const ariaValueNow =
    activeDraft === undefined || activeDraft === null
      ? committed === null
        ? undefined
        : committed
      : draftNow === undefined || draftNow === null
        ? undefined
        : draftNow;
  const stepFrom =
    activeDraft === null || activeDraft === undefined
      ? committed
      : activeDraft === ""
        ? null
        : draftNow;
  const canIncrement =
    stepFrom === undefined ? false : stepNumberValue(stepFrom, 1, min, max, step, precision) !== null;
  const canDecrement =
    stepFrom === undefined ? false : stepNumberValue(stepFrom, -1, min, max, step, precision) !== null;

  function emitValidationChange(status: InputValidationStatus, message: string): void {
    onValidationChange?.({ status, valid: status === "valid" || status === "idle", message });
  }

  async function runValidation(nextValue: number | null): Promise<void> {
    // Invalidate any in-flight validation before the idle early-return so a
    // clear/replacement cannot be overwritten by a stale resolve.
    const validationKey = ++activeValidationKey.current;

    if (!validate || nextValue === null) {
      setInternalValidationStatus("idle");
      validationMessageRef.current = "";
      emitValidationChange("idle", "");
      return;
    }

    const validationValue = formatNumberCommitted(nextValue, precision);
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

  function applyDraft(next: string | null): void {
    if (!draftControlled) {
      setLocalDraft(next);
    }

    onDraftValueChange?.(next);
  }

  function applyValue(next: number | null): void {
    lastEmittedValue.current = next;

    if (!isControlled) {
      setUncontrolledValue(next);
    } else {
      lastControlledValue.current = next;
    }

    onValueChange?.(next);
    void runValidation(next);
  }

  function applyEffects(effects: NumberInputEffect[]): void {
    for (const effect of effects) {
      switch (effect.type) {
        case "emitDraftValueChange":
          applyDraft(effect.draft);
          break;
        case "emitValueChange":
          applyValue(effect.value);
          break;
        case "emitCommit":
          onCommit?.(effect.value);
          break;
      }
    }
  }

  function dispatch(
    event:
      | { type: "RAW_EDIT"; text: string }
      | { type: "CLEAR" }
      | { type: "ENTER" }
      | { type: "BLUR" }
      | { type: "ESCAPE" }
      | { type: "STEP"; direction: 1 | -1 }
      | { type: "HOME" }
      | { type: "END" },
  ): void {
    const result = numberInputTransition(machineContext, event);

    if (!draftControlled) {
      setLocalDraft(result.context.draft);
    }

    applyEffects(result.effects);
  }

  function handleKeyDown(event: KeyboardEvent<HTMLInputElement>): void {
    if (event.key === "Enter") {
      event.preventDefault();
      dispatch({ type: "ENTER" });
      return;
    }

    if (event.key === "Escape") {
      event.preventDefault();
      dispatch({ type: "ESCAPE" });
      return;
    }

    if (event.key === "ArrowUp") {
      event.preventDefault();
      dispatch({ type: "STEP", direction: 1 });
      return;
    }

    if (event.key === "ArrowDown") {
      event.preventDefault();
      dispatch({ type: "STEP", direction: -1 });
      return;
    }

    if (event.key === "Home") {
      event.preventDefault();
      dispatch({ type: "HOME" });
      return;
    }

    if (event.key === "End") {
      event.preventDefault();
      dispatch({ type: "END" });
    }
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
          role="spinbutton"
          value={displayValue}
          placeholder={placeholder ?? undefined}
          disabled={disabled}
          readOnly={readOnly}
          required={required}
          aria-label={ariaLabel ?? undefined}
          aria-describedby={describedBy ?? undefined}
          aria-invalid={ariaInvalid}
          aria-busy={ariaBusy}
          aria-valuenow={ariaValueNow}
          aria-valuemin={min ?? undefined}
          aria-valuemax={max ?? undefined}
          onChange={(event) => dispatch({ type: "RAW_EDIT", text: event.currentTarget.value })}
          onFocus={onFocus}
          onBlur={(event) => {
            dispatch({ type: "BLUR" });
            onBlur?.(event);
          }}
          onKeyDown={handleKeyDown}
        />

        {showSteppers ? (
          <div className="poodle-number-input__steppers">
            <button
              type="button"
              aria-label="Increment"
              disabled={disabled || readOnly || !canIncrement}
              onClick={() => dispatch({ type: "STEP", direction: 1 })}
            >
              <Icon name="plus" />
            </button>
            <button
              type="button"
              aria-label="Decrement"
              disabled={disabled || readOnly || !canDecrement}
              onClick={() => dispatch({ type: "STEP", direction: -1 })}
            >
              <Icon name="minus" />
            </button>
          </div>
        ) : null}
      </div>

      {suffix ? <span className="poodle-number-input__suffix">{suffix}</span> : null}
    </div>
  );
}
