<script lang="ts">
  import { createEventDispatcher, onDestroy } from "svelte";
  import type { HTMLInputAttributes } from "svelte/elements";

  import Icon from "./Icon.svelte";
  import Spinner from "./Spinner.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    ControlDensity,
    ControlSize,
    InputValidationStatus,
    InputValidator,
    SemanticControlSizeRole,
    ValidationResult,
    ValidationState,
  } from "./types";

  export let id = "";
  export let value: string | null = null;
  export let defaultValue = "";
  export let placeholder: string | null = null;
  export let name: string | undefined = undefined;
  export let autocomplete: HTMLInputAttributes["autocomplete"] = undefined;
  export let disabled = false;
  export let readOnly = false;
  export let required = false;
  export let pattern: string | undefined = undefined;
  export let spellcheck: HTMLInputAttributes["spellcheck"] = undefined;
  export let autocapitalize: HTMLInputAttributes["autocapitalize"] = undefined;
  export let enterKeyHint:
    | "enter"
    | "done"
    | "go"
    | "next"
    | "previous"
    | "search"
    | "send"
    | null = null;
  export let debounce: number | null = null;
  export let validate: InputValidator | undefined = undefined;
  export let validationContext: unknown = undefined;
  export let validationDebounce = 300;
  export let validateOnBlur = true;
  export let showValidationStatus = true;
  export let validationState: ValidationState = "none";
  export let ariaLabel: string | null = null;
  export let describedBy: string | null = null;
  export let list: string | null = null;
  export let inputMode:
    | "none"
    | "search"
    | "text"
    | "tel"
    | "url"
    | "email"
    | "numeric"
    | "decimal"
    | null = null;
  export let type: HTMLInputElement["type"] | "multiline" = "text";
  /** Number of visible text rows. When > 1 and type is not explicitly set, auto-switches to multiline. */
  export let rows: number | null = null;
  /** Resize behaviour for multiline mode. */
  export let resize: "vertical" | "horizontal" | "both" | "none" = "vertical";
  export let prefix: string | null = null;
  export let suffix: string | null = null;
  export let maxLength: number | null = null;
  export let showCharCount = false;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
    validationChange: { status: InputValidationStatus; valid: boolean; message: string };
    submit: { value: string };
    cancel: void;
    keydown: KeyboardEvent;
    focus: FocusEvent;
    blur: FocusEvent;
  }>();

  const uiPresentation = getUiPresentation();
  let uncontrolledValue = defaultValue;
  let liveValue = value ?? defaultValue;
  let debounceTimer: ReturnType<typeof setTimeout> | null = null;
  let validationTimer: ReturnType<typeof setTimeout> | null = null;
  let activeValidationKey: string | null = null;
  let internalValidationStatus: InputValidationStatus = "idle";
  let internalValidationMessage = "";
  let lastValidatedValue = "";
  let previousContextKey = serializeValidationContext(validationContext);
  let previousValidationSnapshot = "";
  let previousControlledValue = value;

  $: isControlled = value !== null;
  $: if (isControlled) {
    if (value !== previousControlledValue) {
      previousControlledValue = value;
      liveValue = value ?? "";
    }
  } else {
    previousControlledValue = value;
    liveValue = uncontrolledValue;
  }
  $: currentValue = liveValue;
  $: effectiveValidationState = validate
    ? internalValidationStatus === "validating"
      ? "pending"
      : internalValidationStatus === "valid"
        ? "valid"
        : internalValidationStatus === "invalid"
          ? "invalid"
          : validationState
    : validationState;
  $: ariaInvalid = effectiveValidationState === "invalid" ? true : undefined;
  $: ariaBusy = effectiveValidationState === "pending" ? true : undefined;
  $: charCount = currentValue.length;
  $: charCountText = maxLength ? `${charCount}/${maxLength}` : `${charCount}`;
  $: isOverLimit = maxLength !== null && charCount > maxLength;
  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  // Auto-detect multiline: explicit type="multiline", or rows > 1 with default type
  $: isMultiline = type === "multiline" || (type === "text" && rows !== null && rows > 1);
  $: showValidationIndicator = showValidationStatus && effectiveValidationState !== "none";
  $: validationIcon =
    effectiveValidationState === "valid"
      ? "check"
      : effectiveValidationState === "invalid"
        ? "x"
        : null;
  $: contextKey = serializeValidationContext(validationContext);

  $: if (validate && liveValue !== lastValidatedValue) {
    triggerValidation(liveValue, false);
  }

  $: if (validate && contextKey !== previousContextKey) {
    previousContextKey = contextKey;
    if (liveValue) {
      triggerValidation(liveValue, false);
    }
  }

  $: validationSnapshot = validate
    ? `${internalValidationStatus}::${internalValidationMessage}`
    : "";

  $: if (validate && validationSnapshot !== previousValidationSnapshot) {
    previousValidationSnapshot = validationSnapshot;
    dispatch("validationChange", {
      status: internalValidationStatus,
      valid: internalValidationStatus === "valid" || internalValidationStatus === "idle",
      message: internalValidationMessage,
    });
  }

  onDestroy(() => {
    if (debounceTimer) {
      clearTimeout(debounceTimer);
    }
    if (validationTimer) {
      clearTimeout(validationTimer);
    }
  });

  function serializeValidationContext(context: unknown): string {
    try {
      return JSON.stringify(context ?? null);
    } catch {
      return "[unserializable-context]";
    }
  }

  function handleInput(event: Event): void {
    const nextValue = (event.currentTarget as HTMLInputElement).value;
    liveValue = nextValue;

    if (!isControlled) {
      uncontrolledValue = nextValue;
    }

    if (debounce && debounce > 0) {
      if (debounceTimer) clearTimeout(debounceTimer);
      debounceTimer = setTimeout(() => {
        dispatch("valueChange", { value: nextValue });
      }, debounce);
    } else {
      dispatch("valueChange", { value: nextValue });
    }
  }

  function handleKeydown(event: KeyboardEvent): void {
    dispatch("keydown", event);

    if (event.key === "Enter") {
      dispatch("submit", { value: liveValue });
    }

    if (event.key === "Escape") {
      dispatch("cancel");
    }
  }

  function buildValidationKey(inputValue: string, context: unknown): string {
    return JSON.stringify({ value: inputValue, context: serializeValidationContext(context) });
  }

  function clearValidationTimers(): void {
    if (validationTimer) {
      clearTimeout(validationTimer);
      validationTimer = null;
    }
  }

  $: if (!validate) {
    clearValidationTimers();
    activeValidationKey = null;
    internalValidationStatus = "idle";
    internalValidationMessage = "";
    lastValidatedValue = "";
  }

  function triggerValidation(inputValue: string, immediate: boolean): void {
    if (!validate) return;

    clearValidationTimers();

    if (!inputValue.trim()) {
      activeValidationKey = null;
      internalValidationStatus = "idle";
      internalValidationMessage = "";
      lastValidatedValue = "";
      return;
    }

    const validationKey = buildValidationKey(inputValue, validationContext);
    activeValidationKey = validationKey;
    internalValidationStatus = "validating";
    internalValidationMessage = "";

    const runValidation = async (): Promise<void> => {
      try {
        const result = await validate?.(inputValue, validationContext);
        if (activeValidationKey !== validationKey || inputValue !== liveValue) return;
        internalValidationStatus = result?.valid ? "valid" : "invalid";
        internalValidationMessage = result?.message ?? "";
        lastValidatedValue = inputValue;
        activeValidationKey = null;
      } catch {
        if (activeValidationKey !== validationKey || inputValue !== liveValue) return;
        internalValidationStatus = "invalid";
        internalValidationMessage = "Could not validate";
        lastValidatedValue = inputValue;
        activeValidationKey = null;
      }
    };

    if (immediate) {
      void runValidation();
      return;
    }

    if (validationDebounce <= 0) {
      void runValidation();
      return;
    }

    validationTimer = setTimeout(() => {
      validationTimer = null;
      void runValidation();
    }, validationDebounce);
  }
</script>

<div class="text-input" class:text-input--multiline={isMultiline} data-validation-state={effectiveValidationState} data-size={resolvedSize} data-density={resolvedDensity}>
  {#if prefix}
    <span class="text-input__affix text-input__affix--prefix">{prefix}</span>
  {/if}

  {#if $$slots.leading}
    <span class="text-input__affordance text-input__affordance--leading">
      <slot name="leading" />
    </span>
  {/if}

  {#if isMultiline}
    <textarea
      id={id || undefined}
      {name}
      class="text-input__control text-input__control--multiline"
      value={currentValue}
      {placeholder}
      {autocomplete}
      {spellcheck}
      autocapitalize={autocapitalize ?? undefined}
      rows={rows ?? 4}
      style={resize !== "vertical" ? `resize: ${resize}` : undefined}
      maxlength={maxLength ?? undefined}
      disabled={disabled}
      readonly={readOnly}
      aria-label={ariaLabel ?? undefined}
      aria-describedby={describedBy ?? undefined}
      aria-invalid={ariaInvalid}
      aria-busy={ariaBusy}
      on:input={handleInput}
      on:keydown={(event) => {
        if ((event.metaKey || event.ctrlKey) && event.key === "Enter") {
          dispatch("submit", { value: currentValue });
        }
        if (event.key === "Escape") {
          dispatch("cancel");
        }
        dispatch("keydown", event);
      }}
      on:focus={(event) => dispatch("focus", event)}
      on:blur={(event) => {
        if (debounceTimer) {
          clearTimeout(debounceTimer);
          debounceTimer = null;
          dispatch("valueChange", { value: liveValue });
        }
        if (validate && validateOnBlur) {
          triggerValidation(liveValue, true);
        }
        dispatch("blur", event);
      }}
    ></textarea>
  {:else}
    <input
      id={id || undefined}
      {name}
      list={list ?? undefined}
      {type}
      inputmode={inputMode ?? undefined}
      class="text-input__control"
      value={currentValue}
      {placeholder}
      {autocomplete}
      {required}
      {pattern}
      {spellcheck}
      autocapitalize={autocapitalize ?? undefined}
      enterkeyhint={enterKeyHint ?? undefined}
      maxlength={maxLength ?? undefined}
      disabled={disabled}
      readonly={readOnly}
      aria-label={ariaLabel ?? undefined}
      aria-describedby={describedBy ?? undefined}
      aria-invalid={ariaInvalid}
      aria-busy={ariaBusy}
      on:input={handleInput}
      on:keydown={handleKeydown}
      on:focus={(event) => dispatch("focus", event)}
      on:blur={(event) => {
        if (debounceTimer) {
          clearTimeout(debounceTimer);
          debounceTimer = null;
          dispatch("valueChange", { value: liveValue });
        }
        if (validate && validateOnBlur) {
          triggerValidation(liveValue, true);
        }
        dispatch("blur", event);
      }}
    />
  {/if}

  {#if $$slots.trailing}
    <span class="text-input__affordance text-input__affordance--trailing">
      <slot name="trailing" />
    </span>
  {/if}

  {#if showValidationIndicator}
    <span
      class="text-input__validation-indicator"
      class:text-input__validation-indicator--pending={effectiveValidationState === "pending"}
      class:text-input__validation-indicator--valid={effectiveValidationState === "valid"}
      class:text-input__validation-indicator--invalid={effectiveValidationState === "invalid"}
      aria-hidden="true"
    >
      {#if effectiveValidationState === "pending"}
        <Spinner variant="ring" sizeRole="chrome" tone="current" />
      {:else if validationIcon}
        <Icon icon={validationIcon} />
      {/if}
    </span>
  {/if}

  {#if suffix}
    <span class="text-input__affix text-input__affix--suffix">{suffix}</span>
  {/if}

  {#if showCharCount}
    <span class="text-input__char-count" class:text-input__char-count--over={isOverLimit} aria-live="polite">
      {charCountText}
    </span>
  {/if}
</div>

<style>
  .text-input {
    --poodle-text-input-radius: var(--poodle-treatment-interactive-subtle-radius, var(--poodle-radius-control));
    --poodle-text-input-fill: var(
      --poodle-treatment-interactive-subtle-fill,
      var(--poodle-color-background-surface)
    );
    --poodle-text-input-fill-focus: var(
      --poodle-treatment-interactive-subtle-fill-focus,
      var(--poodle-text-input-fill)
    );
    --poodle-text-input-border: var(
      --poodle-treatment-interactive-subtle-border,
      var(--poodle-color-border-default)
    );
    --poodle-text-input-border-focus: var(
      --poodle-treatment-interactive-subtle-border-focus,
      var(--poodle-color-accent-focusRing)
    );
    --poodle-text-input-shadow: var(--poodle-treatment-interactive-subtle-shadow, none);
    --poodle-text-input-shadow-focus: var(
      --poodle-treatment-interactive-subtle-shadow-focus,
      0 0 0 var(--poodle-border-width-focus)
        color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent)
    );
    display: flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    border: 0.0625rem solid var(--poodle-text-input-border);
    border-radius: var(--poodle-text-input-radius);
    background: var(--poodle-text-input-fill);
    color: var(--poodle-color-text-primary);
    box-shadow: var(--poodle-text-input-shadow);
    transition:
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .text-input:focus-within {
    border-color: var(--poodle-text-input-border-focus);
    background: var(--poodle-text-input-fill-focus);
    box-shadow: var(--poodle-text-input-shadow-focus);
  }

  .text-input[data-validation-state="invalid"] {
    border-color: var(--poodle-color-status-danger);
  }

  .text-input[data-validation-state="valid"] {
    border-color: var(--poodle-color-status-success);
  }

  .text-input[data-validation-state="pending"] {
    border-color: var(--poodle-color-accent-base);
  }

  .text-input:has(.text-input__control:disabled) {
    opacity: var(--poodle-state-opacity-disabled);
  }

  .text-input__control {
    flex: 1;
    min-width: 0;
    width: 100%;
    height: calc(var(--poodle-size-control-height) - (var(--poodle-border-width-default) * 2));
    padding: 0;
    border: 0;
    background: transparent;
    color: inherit;
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
    outline: 0;
  }

  .text-input__control::placeholder {
    color: var(--poodle-color-text-muted, color-mix(in srgb, var(--poodle-color-text-secondary) 60%, transparent));
  }

  .text-input__affordance {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: var(--poodle-color-icon-muted);
    font-family: var(--poodle-typography-code-family);
    font-size: var(--poodle-icon-size-default);
  }

  .text-input__validation-indicator {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: var(--poodle-color-icon-muted);
  }

  .text-input__validation-indicator--pending {
    color: var(--poodle-color-accent-base);
  }

  .text-input__validation-indicator--valid {
    color: var(--poodle-color-status-success);
  }

  .text-input__validation-indicator--invalid {
    color: var(--poodle-color-status-danger);
  }

  .text-input__affix {
    display: inline-flex;
    align-items: center;
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    white-space: nowrap;
    user-select: none;
  }

  .text-input__affix--prefix {
    padding-right: var(--poodle-space-inline-sm);
    border-right: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 52%, transparent);
    margin-right: var(--poodle-space-inline-sm);
  }

  .text-input__affix--suffix {
    padding-left: var(--poodle-space-inline-sm);
    border-left: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 52%, transparent);
    margin-left: var(--poodle-space-inline-sm);
  }

  .text-input__char-count {
    display: inline-flex;
    align-items: center;
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.6875rem;
    white-space: nowrap;
  }

  .text-input__char-count--over {
    color: var(--poodle-color-status-danger);
  }

  /* Density variants */
  .text-input[data-density="compact"] {
    padding: 0 calc(var(--poodle-space-control-x) - 0.125rem);
  }

  .text-input[data-density="comfortable"] {
    padding: 0 calc(var(--poodle-space-control-x) + 0.125rem);
  }

  /* Multiline (textarea) mode */
  .text-input--multiline {
    min-height: auto;
  }

  .text-input__control--multiline {
    min-height: calc(1lh * 4);
    padding: var(--poodle-space-control-y) var(--poodle-space-control-x);
    resize: vertical;
    line-height: var(--poodle-typography-body-lineHeight);
  }

  .text-input--multiline[data-density="compact"] .text-input__control--multiline {
    padding: calc(var(--poodle-space-control-y, 0.375rem) - 0.125rem) calc(var(--poodle-space-control-x) - 0.125rem);
  }

  .text-input--multiline[data-density="comfortable"] .text-input__control--multiline {
    padding: calc(var(--poodle-space-control-y, 0.375rem) + 0.125rem) calc(var(--poodle-space-control-x) + 0.125rem);
  }

  /* Size variants */
  .text-input[data-size="xs"] {
    min-height: calc(var(--poodle-size-control-height) - 0.5rem);
    padding: 0 calc(var(--poodle-space-control-x) - 0.125rem);
  }

  .text-input[data-size="xs"] .text-input__control {
    height: calc(var(--poodle-size-control-height) - 0.5rem - (var(--poodle-border-width-default) * 2));
    font-size: 0.75rem;
  }

  .text-input[data-size="sm"] {
    min-height: calc(var(--poodle-size-control-height) - 0.375rem);
    padding: 0 calc(var(--poodle-space-control-x) - 0.0625rem);
  }

  .text-input[data-size="sm"] .text-input__control {
    height: calc(var(--poodle-size-control-height) - 0.375rem - (var(--poodle-border-width-default) * 2));
  }

  .text-input[data-size="lg"] {
    min-height: calc(var(--poodle-size-control-height) + 0.375rem);
    padding: 0 calc(var(--poodle-space-control-x) + 0.125rem);
  }

  .text-input[data-size="lg"] .text-input__control {
    height: calc(var(--poodle-size-control-height) + 0.375rem - (var(--poodle-border-width-default) * 2));
    font-size: 0.9375rem;
  }

  .text-input[data-size="xl"] {
    min-height: calc(var(--poodle-size-control-height) + 0.5rem);
    padding: 0 calc(var(--poodle-space-control-x) + 0.1875rem);
  }

  .text-input[data-size="xl"] .text-input__control {
    height: calc(var(--poodle-size-control-height) + 0.5rem - (var(--poodle-border-width-default) * 2));
    font-size: 1rem;
  }
</style>
