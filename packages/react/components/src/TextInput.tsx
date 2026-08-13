import {
  forwardRef,
  useEffect,
  useImperativeHandle,
  useRef,
  useState,
  type ChangeEvent,
  type CSSProperties,
  type FocusEvent,
  type KeyboardEvent,
  type ReactNode,
} from "react";
import { isValidSlugFormat, slugify, validationStatusToState } from "@inflatable-cookie/poodle-core";

import "@inflatable-cookie/poodle-core/styles/text-input.css";

import { Icon } from "./Icon";
import { resolveSemanticControlSize, useUiPresentation } from "./presentation";
import { Spinner } from "./Spinner";
import type {
  ControlDensity,
  ControlSize,
  InputValidationStatus,
  InputValidator,
  SemanticControlSizeRole,
  TextInputValidationChange,
  ValidationResult,
  ValidationState,
} from "./types";

export interface TextInputProps {
  id?: string;
  value?: string | null;
  defaultValue?: string;
  placeholder?: string | null;
  name?: string;
  autoComplete?: string;
  disabled?: boolean;
  readOnly?: boolean;
  autofocus?: boolean;
  required?: boolean;
  pattern?: string;
  spellCheck?: boolean;
  autoCapitalize?: string;
  enterKeyHint?: "enter" | "done" | "go" | "next" | "previous" | "search" | "send" | null;
  debounce?: number | null;
  validate?: InputValidator;
  validationContext?: unknown;
  validationKey?: unknown;
  validationDebounce?: number;
  validateOnBlur?: boolean;
  showValidationStatus?: boolean;
  validationState?: ValidationState;
  ariaLabel?: string | null;
  describedBy?: string | null;
  list?: string | null;
  inputMode?: "none" | "search" | "text" | "tel" | "url" | "email" | "numeric" | "decimal" | null;
  type?: string | "multiline" | "slug";
  rows?: number | null;
  resize?: "vertical" | "horizontal" | "both" | "none";
  source?: string | null;
  prefix?: string | null;
  suffix?: string | null;
  maxLength?: number | null;
  showCharCount?: boolean;
  size?: ControlSize | null;
  sizeRole?: SemanticControlSizeRole;
  density?: ControlDensity | null;
  showClearButton?: boolean;
  onValueChange?: (value: string) => void;
  onValidationChange?: (detail: TextInputValidationChange) => void;
  onSubmit?: (value: string) => void;
  onCancel?: () => void;
  onClear?: () => void;
  onKeyDown?: (event: KeyboardEvent) => void;
  onFocus?: (event: FocusEvent) => void;
  onBlur?: (event: FocusEvent) => void;
  leading?: ReactNode;
  trailing?: ReactNode;
}

export interface TextInputHandle {
  focus: () => void;
}

function serializeValidationContext(context: unknown): string {
  try {
    return JSON.stringify(context ?? null);
  } catch {
    return "[unserializable-context]";
  }
}

function mergeValidationContext(context: unknown, key: unknown): unknown {
  if (key === undefined || key === null || key === "") return context;
  if (context === undefined || context === null) return { validationKey: key };
  if (typeof context === "object") return { ...(context as Record<string, unknown>), validationKey: key };
  return { value: context, validationKey: key };
}

export const TextInput = forwardRef<TextInputHandle, TextInputProps>(function TextInput(
  {
    id = "",
    value,
    defaultValue = "",
    placeholder = null,
    name,
    autoComplete,
    disabled = false,
    readOnly = false,
    autofocus = false,
    required = false,
    pattern,
    spellCheck,
    autoCapitalize,
    enterKeyHint = null,
    debounce = null,
    validate,
    validationContext,
    validationKey,
    validationDebounce = 300,
    validateOnBlur = true,
    showValidationStatus = true,
    validationState = "none",
    ariaLabel = null,
    describedBy = null,
    list = null,
    inputMode = null,
    type = "text",
    rows = null,
    resize = "vertical",
    source = null,
    prefix = null,
    suffix = null,
    maxLength = null,
    showCharCount = false,
    size = null,
    sizeRole = "control",
    density = null,
    showClearButton = true,
    onValueChange,
    onValidationChange,
    onSubmit,
    onCancel,
    onClear,
    onKeyDown,
    onFocus,
    onBlur,
    leading: leadingSlot,
    trailing: trailingSlot,
  },
  ref,
) {
  const uiPresentation = useUiPresentation();
  const generatedInputId = useRef(`poodle-text-input-${Math.random().toString(36).slice(2, 10)}`).current;

  const [uncontrolledValue, setUncontrolledValue] = useState(defaultValue);
  const [internalValidationStatus, setInternalValidationStatus] = useState<InputValidationStatus>("idle");
  const [internalValidationMessage, setInternalValidationMessage] = useState("");
  const [userEditedSlug, setUserEditedSlug] = useState(false);

  const controlRef = useRef<HTMLInputElement | HTMLTextAreaElement | null>(null);
  const debounceTimer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const validationTimer = useRef<ReturnType<typeof setTimeout> | null>(null);
  const activeValidationKey = useRef<string | null>(null);
  // Card 048: IME composition stays browser-native, but the value path is
  // gated — the browser fires `input` events with the partial buffer during
  // composition, and none of them may reach onValueChange. The buffer is
  // recorded while composing and committed exactly once on compositionend
  // (per the UI Events spec, the final committed input event fires before
  // compositionend, so the end handler is the single commit point). React's
  // onChange is value-diff-based, so an event-only isComposing filter would
  // swallow the final committed event entirely (its value equals the last
  // buffer the tracker saw).
  const composing = useRef(false);
  const compositionBuffer = useRef<string | null>(null);
  const lastValidatedValue = useRef("");
  const previousContextKey = useRef("");
  const previousValidationSnapshot = useRef("");
  const previousGeneratedSlug = useRef("");
  const previousDefaultValue = useRef(defaultValue);

  const isSearch = type === "search";
  const isSlug = type === "slug";
  const isMultiline = type === "multiline" || (type === "text" && rows !== null && rows > 1);
  const nativeInputType = isSlug ? "text" : type;
  const hasLeadingAffordance = Boolean(leadingSlot) || isSearch;
  const hasTrailingAffordance = Boolean(trailingSlot);
  const isControlled = value !== undefined;
  const currentValue = isControlled ? (value ?? "") : uncontrolledValue;
  const canClear = isSearch && showClearButton && !disabled && !readOnly && currentValue.length > 0;
  const effectiveValidationState = validate
    ? validationStatusToState(internalValidationStatus, validationState)
    : validationState;
  const ariaInvalid = effectiveValidationState === "invalid" ? true : undefined;
  const ariaBusy = effectiveValidationState === "pending" ? true : undefined;
  const validationMessageId = internalValidationMessage
    ? `${id || name || generatedInputId}-validation-message`
    : null;
  const effectiveDescribedBy = [describedBy, validationMessageId].filter(Boolean).join(" ") || undefined;
  const charCount = currentValue.length;
  const charCountText = maxLength ? `${charCount}/${maxLength}` : `${charCount}`;
  const isOverLimit = maxLength !== null && charCount > maxLength;
  const resolvedSize = size ?? resolveSemanticControlSize(uiPresentation.sizeScale, sizeRole);
  const resolvedDensity = density ?? uiPresentation.density;
  const showValidationIndicator = showValidationStatus && effectiveValidationState !== "none";
  const validationIcon =
    effectiveValidationState === "valid" ? "check" : effectiveValidationState === "invalid" ? "x" : null;
  const effectiveValidationContext = mergeValidationContext(validationContext, validationKey);
  const contextKey = serializeValidationContext(effectiveValidationContext);
  const generatedSlug = isSlug ? slugify(source ?? "") : "";
  const fieldEndAdornmentCount =
    Number(hasTrailingAffordance) + Number(canClear) + Number(showValidationIndicator);
  const controlPaddingStart = hasLeadingAffordance
    ? "calc(var(--poodle-text-input-padding-inline) + var(--poodle-icon-size-default) + (var(--poodle-text-input-adornment-gap) * 1.5))"
    : "var(--poodle-text-input-padding-inline)";
  const controlPaddingEnd =
    fieldEndAdornmentCount > 0
      ? `calc(var(--poodle-text-input-padding-inline) + (${fieldEndAdornmentCount} * var(--poodle-icon-size-default)) + (${fieldEndAdornmentCount} * var(--poodle-text-input-adornment-gap)))`
      : "var(--poodle-text-input-padding-inline)";
  const multilineBottomPadding = showCharCount
    ? "calc(var(--poodle-text-input-padding-block) + 1.5rem)"
    : "var(--poodle-text-input-padding-block)";

  // re-seed uncontrolled value when defaultValue changes (Svelte parity)
  useEffect(() => {
    if (!isControlled && defaultValue !== previousDefaultValue.current) {
      previousDefaultValue.current = defaultValue;
      setUncontrolledValue(defaultValue);
    }
  }, [isControlled, defaultValue]);

  function normalizeInputValue(input: string): string {
    return isSlug ? slugify(input) : input;
  }

  function emitValueChange(nextValue: string, immediate: boolean): void {
    if (immediate || !debounce || debounce <= 0) {
      onValueChange?.(nextValue);
      return;
    }
    if (debounceTimer.current) clearTimeout(debounceTimer.current);
    debounceTimer.current = setTimeout(() => {
      debounceTimer.current = null;
      onValueChange?.(nextValue);
    }, debounce);
  }

  function commitValue(nextValue: string, options: { markSlugEdited?: boolean; immediate?: boolean } = {}): void {
    if (!isControlled) setUncontrolledValue(nextValue);
    if (isSlug && options.markSlugEdited !== false) setUserEditedSlug(true);
    emitValueChange(nextValue, options.immediate === true);
  }

  // slug auto-generation from source
  useEffect(() => {
    if (!isSlug || source === null) return;
    if (!userEditedSlug || currentValue === previousGeneratedSlug.current || currentValue === "") {
      if (previousGeneratedSlug.current !== generatedSlug) {
        previousGeneratedSlug.current = generatedSlug;
      }
      if (currentValue !== generatedSlug) {
        commitValue(generatedSlug, { markSlugEdited: false, immediate: true });
      }
    }
  }, [isSlug, source, generatedSlug, userEditedSlug, currentValue]);

  function buildValidationKey(inputValue: string, context: unknown): string {
    return JSON.stringify({ value: inputValue, context: serializeValidationContext(context) });
  }

  function clearValidationTimers(): void {
    if (validationTimer.current) {
      clearTimeout(validationTimer.current);
      validationTimer.current = null;
    }
  }

  async function validateSlugValue(inputValue: string): Promise<ValidationResult> {
    const candidate = `${prefix ?? ""}${inputValue}`.trim();
    const limit = maxLength ?? 100;

    if (!candidate) return { valid: !required, message: required ? "Required" : "" };
    if (!isValidSlugFormat(candidate, limit)) {
      return { valid: false, message: "Use lowercase letters, numbers, and hyphens only." };
    }
    if (!validate) return { valid: true, message: "" };
    return await validate(candidate, effectiveValidationContext);
  }

  const currentValueRef = useRef(currentValue);
  currentValueRef.current = currentValue;

  async function runValidation(inputValue: string, key: string): Promise<void> {
    try {
      const result = isSlug ? await validateSlugValue(inputValue) : await validate?.(inputValue, effectiveValidationContext);
      if (activeValidationKey.current !== key || inputValue !== currentValueRef.current) return;
      setInternalValidationStatus(result?.valid ? "valid" : "invalid");
      setInternalValidationMessage(result?.message ?? "");
      lastValidatedValue.current = inputValue;
      activeValidationKey.current = null;
    } catch {
      if (activeValidationKey.current !== key || inputValue !== currentValueRef.current) return;
      setInternalValidationStatus("invalid");
      setInternalValidationMessage("Could not validate");
      lastValidatedValue.current = inputValue;
      activeValidationKey.current = null;
    }
  }

  function triggerValidation(inputValue: string, immediate: boolean): void {
    if (!validate) return;
    clearValidationTimers();

    if (!inputValue.trim() && !isSlug) {
      activeValidationKey.current = null;
      setInternalValidationStatus("idle");
      setInternalValidationMessage("");
      lastValidatedValue.current = "";
      return;
    }

    const nextKey = buildValidationKey(inputValue, effectiveValidationContext);
    activeValidationKey.current = nextKey;
    setInternalValidationStatus("validating");
    setInternalValidationMessage("");

    if (immediate || validationDebounce <= 0) {
      void runValidation(inputValue, nextKey);
      return;
    }
    validationTimer.current = setTimeout(() => {
      validationTimer.current = null;
      void runValidation(inputValue, nextKey);
    }, validationDebounce);
  }

  // validate on value change
  useEffect(() => {
    if (
      validate &&
      currentValue !== lastValidatedValue.current &&
      activeValidationKey.current !== buildValidationKey(currentValue, effectiveValidationContext)
    ) {
      triggerValidation(currentValue, false);
    }
  }, [currentValue, validate]);

  // re-validate on context change
  useEffect(() => {
    if (validate && contextKey !== previousContextKey.current) {
      previousContextKey.current = contextKey;
      if (currentValue && activeValidationKey.current !== buildValidationKey(currentValue, effectiveValidationContext)) {
        triggerValidation(currentValue, false);
      }
    }
  }, [contextKey, validate]);

  // emit validation snapshot changes
  useEffect(() => {
    const snapshot = validate ? `${internalValidationStatus}::${internalValidationMessage}` : "";
    if (validate && snapshot !== previousValidationSnapshot.current) {
      previousValidationSnapshot.current = snapshot;
      onValidationChange?.({
        status: internalValidationStatus,
        valid: internalValidationStatus === "valid" || internalValidationStatus === "idle",
        message: internalValidationMessage,
      });
    }
  }, [validate, internalValidationStatus, internalValidationMessage]);

  // reset when validator removed
  useEffect(() => {
    if (!validate) {
      clearValidationTimers();
      activeValidationKey.current = null;
      setInternalValidationStatus("idle");
      setInternalValidationMessage("");
      lastValidatedValue.current = "";
      previousValidationSnapshot.current = "";
    }
  }, [validate]);

  useEffect(
    () => () => {
      if (debounceTimer.current) clearTimeout(debounceTimer.current);
      if (validationTimer.current) clearTimeout(validationTimer.current);
    },
    [],
  );

  function flushDebouncedValue(): void {
    if (!debounceTimer.current) return;
    clearTimeout(debounceTimer.current);
    debounceTimer.current = null;
    onValueChange?.(currentValueRef.current);
  }

  function handleCompositionStart(): void {
    composing.current = true;
  }

  function handleCompositionEnd(): void {
    composing.current = false;
    if (compositionBuffer.current !== null) {
      commitValue(normalizeInputValue(compositionBuffer.current), { markSlugEdited: isSlug });
      compositionBuffer.current = null;
    }
  }

  function handleInput(event: ChangeEvent<HTMLInputElement | HTMLTextAreaElement>): void {
    const value = event.currentTarget.value;
    if (composing.current) {
      // A composition is in progress: buffer the current text and do not
      // fire onValueChange — the commit lands once on compositionend.
      compositionBuffer.current = value;
      return;
    }
    commitValue(normalizeInputValue(value), { markSlugEdited: isSlug });
  }

  function handleClear(): void {
    commitValue("", { markSlugEdited: isSlug, immediate: true });
    onClear?.();
  }

  function handleBlurEvent(event: FocusEvent): void {
    flushDebouncedValue();
    if (validate && validateOnBlur) triggerValidation(currentValueRef.current, true);
    onBlur?.(event);
  }

  // Svelte exports `focus()`; MenuSurface.tsx is the precedent for exposing an
  // imperative handle through useImperativeHandle. Focuses the underlying
  // control, never the wrapper.
  useImperativeHandle(ref, () => ({
    focus: () => controlRef.current?.focus(),
  }));

  const rootStyle = {
    "--poodle-text-input-control-padding-start": controlPaddingStart,
    "--poodle-text-input-control-padding-end": controlPaddingEnd,
    "--poodle-text-input-multiline-padding-end": multilineBottomPadding,
  } as CSSProperties;

  return (
    <>
      <div
        className={`poodle-text-input${isMultiline ? " poodle-text-input--multiline" : ""}`}
        data-validation-state={effectiveValidationState}
        data-size={resolvedSize}
        data-density={resolvedDensity}
        data-type={type}
        style={rootStyle}
      >
        {prefix ? <span className="poodle-text-input__affix poodle-text-input__affix--prefix">{prefix}</span> : null}

        <div className="poodle-text-input__field">
          {leadingSlot ? (
            <span className="poodle-text-input__affordance poodle-text-input__affordance--leading">{leadingSlot}</span>
          ) : isSearch ? (
            <span className="poodle-text-input__affordance poodle-text-input__affordance--leading" aria-hidden="true">
              <Icon icon="search" />
            </span>
          ) : null}

          {isMultiline ? (
            <textarea
              ref={controlRef}
              id={id || undefined}
              name={name}
              className="poodle-text-input__control poodle-text-input__control--multiline"
              value={currentValue}
              placeholder={placeholder ?? undefined}
              autoComplete={autoComplete}
              spellCheck={spellCheck}
              autoCapitalize={autoCapitalize}
              rows={rows ?? 4}
              style={resize !== "vertical" ? { resize } : undefined}
              maxLength={maxLength ?? undefined}
              disabled={disabled}
              readOnly={readOnly}
              autoFocus={autofocus || undefined}
              aria-label={ariaLabel ?? undefined}
              aria-describedby={effectiveDescribedBy}
              aria-invalid={ariaInvalid}
              aria-busy={ariaBusy}
              onChange={handleInput}
              onCompositionStart={handleCompositionStart}
              onCompositionEnd={handleCompositionEnd}
              onKeyDown={(event) => {
                onKeyDown?.(event);
                if ((event.metaKey || event.ctrlKey) && event.key === "Enter") onSubmit?.(currentValueRef.current);
                if (event.key === "Escape") onCancel?.();
              }}
              onFocus={onFocus}
              onBlur={handleBlurEvent}
            />
          ) : (
            <input
              ref={controlRef}
              id={id || undefined}
              name={name}
              list={list ?? undefined}
              type={nativeInputType}
              inputMode={(isSlug ? "text" : inputMode) ?? undefined}
              className="poodle-text-input__control"
              value={currentValue}
              placeholder={placeholder ?? undefined}
              autoComplete={autoComplete}
              required={required}
              pattern={pattern}
              spellCheck={isSlug ? false : spellCheck}
              autoCapitalize={isSlug ? "off" : autoCapitalize}
              enterKeyHint={enterKeyHint ?? undefined}
              maxLength={maxLength ?? undefined}
              disabled={disabled}
              readOnly={readOnly}
              autoFocus={autofocus || undefined}
              aria-label={ariaLabel ?? undefined}
              aria-describedby={effectiveDescribedBy}
              aria-invalid={ariaInvalid}
              aria-busy={ariaBusy}
              onChange={handleInput}
              onCompositionStart={handleCompositionStart}
              onCompositionEnd={handleCompositionEnd}
              onKeyDown={(event) => {
                onKeyDown?.(event);
                if (event.key === "Enter") onSubmit?.(currentValueRef.current);
                if (event.key === "Escape") onCancel?.();
              }}
              onFocus={onFocus}
              onBlur={handleBlurEvent}
            />
          )}

          {trailingSlot ? (
            <span className="poodle-text-input__affordance poodle-text-input__affordance--trailing">{trailingSlot}</span>
          ) : null}

          {canClear ? (
            <button className="poodle-text-input__clear" type="button" aria-label="Clear search query" onClick={handleClear}>
              <Icon icon="x" />
            </button>
          ) : null}

          {showValidationIndicator ? (
            <span
              className={[
                "poodle-text-input__validation-indicator",
                effectiveValidationState === "pending" ? "poodle-text-input__validation-indicator--pending" : "",
                effectiveValidationState === "valid" ? "poodle-text-input__validation-indicator--valid" : "",
                effectiveValidationState === "invalid" ? "poodle-text-input__validation-indicator--invalid" : "",
              ]
                .filter(Boolean)
                .join(" ")}
              aria-hidden="true"
            >
              {effectiveValidationState === "pending" ? (
                <Spinner variant="ring" sizeRole="chrome" tone="current" />
              ) : validationIcon ? (
                <Icon icon={validationIcon} />
              ) : null}
            </span>
          ) : null}
        </div>

        {suffix ? <span className="poodle-text-input__affix poodle-text-input__affix--suffix">{suffix}</span> : null}

        {showCharCount ? (
          <span
            className={`poodle-text-input__char-count${isOverLimit ? " poodle-text-input__char-count--over" : ""}`}
            aria-live="polite"
          >
            {charCountText}
          </span>
        ) : null}
      </div>

      {internalValidationMessage && effectiveValidationState === "invalid" ? (
        <p className="poodle-text-input__validation-message" id={validationMessageId ?? undefined} aria-live="polite">
          {internalValidationMessage}
        </p>
      ) : null}
    </>
  );
});
