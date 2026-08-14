import {
  Fragment,
  useRef,
  useState,
  type ChangeEvent,
  type CSSProperties,
  type FormEvent,
  type KeyboardEvent,
} from "react";
import {
  clampCodePosition,
  codeGroupEndIndices,
  codeInsertReplacement,
  codeSelectionRange,
  codeSlotSelection,
  sanitizeCodeValue,
} from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/code-input.css";

import { Field } from "./Field";
import { Icon } from "./Icon";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import type {
  ControlDensity,
  ControlSize,
  InputValidator,
  SemanticControlSizeRole,
  ValidationState,
} from "./types";

export interface CodeInputProps {
  id?: string | null;
  value?: string | null;
  defaultValue?: string;
  name?: string;
  label?: string;
  hint?: string | null;
  error?: string | null;
  disabled?: boolean;
  length?: number;
  groups?: readonly number[] | null;
  separator?: string | null;
  mask?: boolean;
  numbersOnly?: boolean;
  ariaLabel?: string | null;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  autoComplete?: string;
  validationState?: ValidationState;
  validate?: InputValidator;
  onValueChange?: (value: string) => void;
  onComplete?: (value: string) => void;
}

export function CodeInput({
  id = null,
  value,
  defaultValue = "",
  name = "code",
  label = "Authenticator code",
  hint = null,
  error = null,
  disabled = false,
  length = 6,
  groups = null,
  separator = null,
  mask = false,
  numbersOnly = true,
  ariaLabel = null,
  size = null,
  sizeRole = "control",
  density = null,
  autoComplete = "one-time-code",
  validationState = "none",
  validate,
  onValueChange,
  onComplete,
}: CodeInputProps) {
  const uiPresentation = useUiPresentation();

  const sanitize = (input: string) => sanitizeCodeValue(input, length, numbersOnly);

  const [uncontrolledValue, setUncontrolledValue] = useState(() => sanitize(defaultValue));
  const [caretIndex, setCaretIndex] = useState(0);
  const [hasFocus, setHasFocus] = useState(false);
  const [validatedCompletion, setValidatedCompletion] = useState<{
    value: string;
    valid: boolean;
  } | null>(null);
  const inputRef = useRef<HTMLInputElement | null>(null);
  const pendingSelection = useRef<{ start: number; end: number } | null>(null);
  const validationGeneration = useRef(0);

  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const effectiveId = id ?? `code-${name}`;
  const isControlled = value !== undefined;
  const currentValue = sanitize(isControlled ? (value ?? "") : uncontrolledValue);
  const currentValueRef = useRef(currentValue);
  currentValueRef.current = currentValue;
  const digits = Array.from({ length }, (_, index) => currentValue[index] ?? "");
  const groupEndIndices = codeGroupEndIndices(length, groups);
  const completionValidationState =
    !error && validatedCompletion?.value === currentValue
      ? validatedCompletion.valid
        ? "valid"
        : "invalid"
      : "none";
  const effectiveValidationState = error
    ? "invalid"
    : completionValidationState !== "none"
      ? completionValidationState
      : validationState;
  const activeCaretIndex = Math.min(caretIndex, Math.max(length - 1, 0));

  const slotBorderColor =
    effectiveValidationState === "invalid" ? "var(--poodle-color-status-danger)" : "var(--poodle-color-border-default)";
  const slotFocusColor =
    effectiveValidationState === "invalid" ? "var(--poodle-color-status-danger)" : "var(--poodle-color-accent-border)";
  const slotFocusRing =
    effectiveValidationState === "invalid"
      ? "color-mix(in srgb, var(--poodle-color-status-danger) 24%, transparent)"
      : "color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent)";

  async function validateCompletion(nextValue: string): Promise<void> {
    if (!validate) return;
    const generation = ++validationGeneration.current;
    setValidatedCompletion(null);

    try {
      const result = await validate(nextValue);
      if (generation !== validationGeneration.current || currentValueRef.current !== nextValue) return;
      setValidatedCompletion({ value: nextValue, valid: result.valid });
    } catch {
      if (generation !== validationGeneration.current || currentValueRef.current !== nextValue) return;
      setValidatedCompletion({ value: nextValue, valid: false });
    }
  }

  function updateValue(nextRawValue: string): void {
    const nextValue = sanitize(nextRawValue);
    validationGeneration.current += 1;
    setValidatedCompletion(null);
    if (!isControlled) setUncontrolledValue(nextValue);
    currentValueRef.current = nextValue;
    onValueChange?.(nextValue);
    if (nextValue.length === length) {
      onComplete?.(nextValue);
      void validateCompletion(nextValue);
    }
  }

  function syncCaret(): void {
    if (!inputRef.current) return;
    const selectionStart = inputRef.current.selectionStart ?? currentValueRef.current.length;
    setCaretIndex(Math.min(selectionStart, Math.max(length - 1, 0)));
  }

  function applyPendingSelection(): void {
    if (!inputRef.current || !pendingSelection.current) return;
    inputRef.current.setSelectionRange(pendingSelection.current.start, pendingSelection.current.end);
    setCaretIndex(Math.min(pendingSelection.current.start, Math.max(length - 1, 0)));
    pendingSelection.current = null;
  }

  function setActivePosition(index: number, selectFilled: boolean): void {
    if (!inputRef.current) return;
    const nextPosition = clampCodePosition(index, currentValueRef.current.length, length);
    const range = codeSelectionRange(nextPosition, currentValueRef.current.length, selectFilled);
    inputRef.current.setSelectionRange(range.start, range.end);
    setCaretIndex(nextPosition);
  }

  function handleInput(event: ChangeEvent<HTMLInputElement>): void {
    updateValue(event.currentTarget.value);
    requestAnimationFrame(() => {
      applyPendingSelection();
      syncCaret();
    });
  }

  function handleKeydown(event: KeyboardEvent<HTMLInputElement>): void {
    if (event.key === "ArrowLeft" || event.key === "ArrowRight") {
      event.preventDefault();
      if (!inputRef.current) return;
      const nextPosition = event.key === "ArrowLeft" ? caretIndex - 1 : caretIndex + 1;
      setActivePosition(nextPosition, true);
      return;
    }
    requestAnimationFrame(syncCaret);
  }

  function handleFocus(): void {
    setHasFocus(true);
    requestAnimationFrame(() => {
      const hadPendingSelection = pendingSelection.current !== null;
      applyPendingSelection();
      if (hadPendingSelection) return;
      setActivePosition(Math.min(currentValueRef.current.length, Math.max(length - 1, 0)), false);
    });
  }

  function handleSlotClick(index: number): void {
    inputRef.current?.focus();
    if (!inputRef.current) return;
    pendingSelection.current = codeSlotSelection(index, currentValueRef.current.length);
    requestAnimationFrame(() => {
      applyPendingSelection();
    });
  }

  function handleBeforeInput(event: FormEvent<HTMLInputElement>): void {
    const native = event.nativeEvent as InputEvent;
    if (disabled) return;

    if (!inputRef.current || !native.inputType?.startsWith("insert")) {
      if (
        numbersOnly &&
        native.data &&
        !/^\d+$/.test(native.data) &&
        native.inputType !== "deleteContentBackward" &&
        native.inputType !== "deleteContentForward"
      ) {
        event.preventDefault();
      }
      return;
    }

    const selectionStart = inputRef.current.selectionStart ?? currentValueRef.current.length;
    const selectionEnd = inputRef.current.selectionEnd ?? selectionStart;
    const replacement = codeInsertReplacement(
      currentValueRef.current,
      native.data ?? "",
      selectionStart,
      selectionEnd,
      length,
      numbersOnly,
    );

    event.preventDefault();
    if (!replacement) return;

    updateValue(replacement.value);
    requestAnimationFrame(() => {
      setActivePosition(replacement.caret, true);
    });
  }

  function displayDigit(digit: string): string {
    if (!digit) return "";
    return mask ? "•" : digit;
  }

  return (
    <Field
      id={effectiveId}
      label={label}
      hint={hint}
      error={error}
      validationState={effectiveValidationState}
      control={({ describedBy }) => (
        <>
          <input type="hidden" name={name} value={currentValue} />

          <div
            className={[
              "poodle-code-input",
              disabled ? "poodle-code-input--disabled" : "",
              hasFocus ? "poodle-code-input--focused" : "",
            ]
              .filter(Boolean)
              .join(" ")}
            data-size={resolvedSize}
            data-density={resolvedDensity}
            data-has-separator={separator ? true : undefined}
            style={
              {
                "--code-slot-border": slotBorderColor,
                "--code-slot-focus": slotFocusColor,
                "--code-slot-focus-ring": slotFocusRing,
              } as CSSProperties
            }
            role="group"
            aria-label={ariaLabel ?? label}
          >
            <input
              ref={inputRef}
              id={effectiveId}
              className="poodle-code-input__control"
              type="text"
              inputMode={numbersOnly ? "numeric" : "text"}
              pattern={numbersOnly ? "[0-9]*" : undefined}
              maxLength={length}
              disabled={disabled}
              value={currentValue}
              autoComplete={autoComplete}
              aria-label={ariaLabel ?? label}
              aria-describedby={describedBy ?? undefined}
              aria-invalid={effectiveValidationState === "invalid" ? "true" : undefined}
              onBeforeInput={handleBeforeInput}
              onChange={handleInput}
              onKeyDown={handleKeydown}
              onFocus={handleFocus}
              onBlur={() => setHasFocus(false)}
            />

            {digits.map((digit, index) => (
              <Fragment key={index}>
                <button
                  type="button"
                  className={[
                    "poodle-code-input__slot",
                    hasFocus && index === activeCaretIndex ? "poodle-code-input__slot--active" : "",
                    digit.length > 0 ? "poodle-code-input__slot--filled" : "",
                    groupEndIndices.includes(index) ? "poodle-code-input__slot--group-end" : "",
                  ]
                    .filter(Boolean)
                    .join(" ")}
                  tabIndex={-1}
                  onMouseDown={(event) => event.preventDefault()}
                  onClick={() => handleSlotClick(index)}
                  aria-hidden="true"
                >
                  {displayDigit(digit)}
                </button>
                {separator && groupEndIndices.includes(index) ? (
                  <span className="poodle-code-input__separator" aria-hidden="true">
                    {separator}
                  </span>
                ) : null}
              </Fragment>
            ))}

            {completionValidationState !== "none" ? (
              <span
                className={`poodle-code-input__validation-indicator poodle-code-input__validation-indicator--${completionValidationState}`}
                role="status"
                aria-label={completionValidationState === "valid" ? "Code check passed" : "Code check failed"}
              >
                <Icon icon={completionValidationState === "valid" ? "check" : "x"} />
              </span>
            ) : null}
          </div>
        </>
      )}
    />
  );
}
