<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/text-input.css";
  import { isValidSlugFormat, slugify, validationStatusToState } from "@inflatable-cookie/poodle-core";
  import { onDestroy, type Snippet } from "svelte";
  import type { HTMLInputAttributes } from "svelte/elements";

  import { default as Icon } from "./Icon.svelte";
  import { default as Spinner } from "./Spinner.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
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

  interface Props {
    id?: string;
    value?: string | null;
    defaultValue?: string;
    placeholder?: string | null;
    name?: string | undefined;
    autocomplete?: HTMLInputAttributes["autocomplete"];
    disabled?: boolean;
    readOnly?: boolean;
    autofocus?: boolean;
    required?: boolean;
    pattern?: string | undefined;
    spellcheck?: HTMLInputAttributes["spellcheck"];
    autocapitalize?: HTMLInputAttributes["autocapitalize"];
    autocorrect?: "on" | "off" | undefined;
    enterKeyHint?:
      | "enter"
      | "done"
      | "go"
      | "next"
      | "previous"
      | "search"
      | "send"
      | null;
    debounce?: number | null;
    validate?: InputValidator | undefined;
    validationContext?: unknown;
    validationKey?: unknown;
    validationDebounce?: number;
    validateOnBlur?: boolean;
    showValidationStatus?: boolean;
    validationState?: ValidationState;
    ariaLabel?: string | null;
    describedBy?: string | null;
    list?: string | null;
    inputMode?:
      | "none"
      | "search"
      | "text"
      | "tel"
      | "url"
      | "email"
      | "numeric"
      | "decimal"
      | null;
    type?: HTMLInputElement["type"] | "multiline" | "slug";
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
    onValueChange?: ((value: string) => void) | undefined;
    onValidationChange?: ((detail: TextInputValidationChange) => void) | undefined;
    onSubmit?: ((value: string) => void) | undefined;
    onCancel?: (() => void) | undefined;
    onClear?: (() => void) | undefined;
    onKeyDown?: ((event: KeyboardEvent) => void) | undefined;
    onFocus?: ((event: FocusEvent) => void) | undefined;
    onBlur?: ((event: FocusEvent) => void) | undefined;
    leading?: Snippet<[]>;
    trailing?: Snippet<[]>;
  }

  let control = $state<HTMLInputElement | HTMLTextAreaElement | null>(null);

  let {
    id = "",
    value = $bindable<string | null | undefined>(undefined),
    defaultValue = "",
    placeholder = null,
    name = undefined,
    autocomplete = undefined,
    disabled = false,
    readOnly = false,
    autofocus = false,
    required = false,
    pattern = undefined,
    spellcheck = undefined,
    autocapitalize = undefined,
    autocorrect = undefined,
    enterKeyHint = null,
    debounce = null,
    validate = undefined,
    validationContext = undefined,
    validationKey = undefined,
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
    onValueChange = undefined,
    onValidationChange = undefined,
    onSubmit = undefined,
    onCancel = undefined,
    onClear = undefined,
    onKeyDown = undefined,
    onFocus = undefined,
    onBlur = undefined,
    leading: leadingSnippet = undefined,
    trailing: trailingSnippet = undefined,
  }: Props = $props();

  export { focus };

  const uiPresentation = getUiPresentation();
  const generatedInputId = `poodle-text-input-${crypto.randomUUID()}`;

  let uncontrolledValue = $state("");
  let debounceTimer: ReturnType<typeof setTimeout> | null = $state(null);
  let validationTimer: ReturnType<typeof setTimeout> | null = $state(null);
  let activeValidationKey: string | null = $state(null);
  let internalValidationStatus: InputValidationStatus = $state("idle");
  let internalValidationMessage = $state("");
  let lastValidatedValue = $state("");
  let previousContextKey = $state("");
  let previousValidationSnapshot = $state("");
  let userEditedSlug = $state(false);
  let previousGeneratedSlug = $state("");
  let previousDefaultValue = $state("");

  const isSearch = $derived(type === "search");
  const isSlug = $derived(type === "slug");
  const isMultiline = $derived(type === "multiline" || (type === "text" && rows !== null && rows > 1));
  const nativeInputType = $derived(isSlug ? "text" : type);
  const hasLeadingAffordance = $derived(Boolean(leadingSnippet) || isSearch);
  const hasTrailingAffordance = $derived(Boolean(trailingSnippet));
  const isControlled = $derived(value !== undefined);
  const currentValue = $derived(isControlled ? (value ?? "") : uncontrolledValue);
  const canClear = $derived(isSearch && showClearButton && !disabled && !readOnly && currentValue.length > 0);
  const effectiveValidationState = $derived.by(() =>
    validate ? validationStatusToState(internalValidationStatus, validationState) : validationState
  );
  const ariaInvalid = $derived(effectiveValidationState === "invalid" ? true : undefined);
  const ariaBusy = $derived(effectiveValidationState === "pending" ? true : undefined);
  const validationMessageId = $derived(
    internalValidationMessage ? `${id || name || generatedInputId}-validation-message` : null,
  );
  const effectiveDescribedBy = $derived(
    [describedBy, validationMessageId].filter(Boolean).join(" ") || undefined,
  );
  const charCount = $derived(currentValue.length);
  const charCountText = $derived(maxLength ? `${charCount}/${maxLength}` : `${charCount}`);

  const isOverLimit = $derived(maxLength !== null && charCount > maxLength);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const autocorrectAttributes = $derived(autocorrect ? { autocorrect } : {});
  const showValidationIndicator = $derived(showValidationStatus && effectiveValidationState !== "none");
  const validationIcon = $derived(
    effectiveValidationState === "valid"
      ? "check"
      : effectiveValidationState === "invalid"
        ? "x"
        : null
  );
  const effectiveValidationContext = $derived(mergeValidationContext(validationContext, validationKey));
  const contextKey = $derived(serializeValidationContext(effectiveValidationContext));
  const generatedSlug = $derived(isSlug ? slugify(source ?? "") : "");
  const fieldEndAdornmentCount = $derived(
    Number(hasTrailingAffordance) + Number(canClear) + Number(showValidationIndicator)
  );
  const controlPaddingStart = $derived(
    hasLeadingAffordance
      ? "calc(var(--poodle-text-input-padding-inline) + var(--poodle-icon-size-default) + (var(--poodle-text-input-adornment-gap) * 1.5))"
      : "var(--poodle-text-input-padding-inline)"
  );
  const controlPaddingEnd = $derived(
    fieldEndAdornmentCount > 0
      ? `calc(var(--poodle-text-input-padding-inline) + (${fieldEndAdornmentCount} * var(--poodle-icon-size-default)) + (${fieldEndAdornmentCount} * var(--poodle-text-input-adornment-gap)))`
      : "var(--poodle-text-input-padding-inline)"
  );
  const clearInsetInlineEnd = $derived(
    showValidationIndicator
      ? "calc(0.5rem + var(--poodle-icon-size-default) + var(--poodle-text-input-adornment-gap))"
      : "0.5rem"
  );
  const trailingFollowingAdornmentCount = $derived(Number(canClear) + Number(showValidationIndicator));
  const trailingInsetInlineEnd = $derived(
    trailingFollowingAdornmentCount > 0
      ? `calc(0.5rem + (${trailingFollowingAdornmentCount} * var(--poodle-icon-size-default)) + (${trailingFollowingAdornmentCount} * var(--poodle-text-input-adornment-gap)))`
      : "0.5rem"
  );
  const multilineBottomPadding = $derived(
    showCharCount
      ? "calc(var(--poodle-text-input-padding-block) + 1.5rem)"
      : "var(--poodle-text-input-padding-block)"
  );

  $effect(() => {
    if (!isControlled && defaultValue !== previousDefaultValue) {
      previousDefaultValue = defaultValue;
      uncontrolledValue = defaultValue;
    }
  });

  $effect(() => {
    if (isSlug && source !== null) {
      if (!userEditedSlug || currentValue === previousGeneratedSlug || currentValue === "") {
        if (previousGeneratedSlug !== generatedSlug) {
          previousGeneratedSlug = generatedSlug;
        }
        if (currentValue !== generatedSlug) {
          commitValue(generatedSlug, { markSlugEdited: false, immediate: true });
        }
      }
    }
  });

  $effect(() => {
    if (
      validate &&
      currentValue !== lastValidatedValue &&
      activeValidationKey !== buildValidationKey(currentValue, effectiveValidationContext)
    ) {
      triggerValidation(currentValue, false);
    }
  });

  $effect(() => {
    if (validate && contextKey !== previousContextKey) {
      previousContextKey = contextKey;
      if (
        currentValue &&
        activeValidationKey !== buildValidationKey(currentValue, effectiveValidationContext)
      ) {
        triggerValidation(currentValue, false);
      }
    }
  });

  $effect(() => {
    const snapshot = validate ? `${internalValidationStatus}::${internalValidationMessage}` : "";
    if (validate && snapshot !== previousValidationSnapshot) {
      previousValidationSnapshot = snapshot;
      onValidationChange?.({
        status: internalValidationStatus,
        valid: internalValidationStatus === "valid" || internalValidationStatus === "idle",
        message: internalValidationMessage,
      });
    }
  });

  $effect(() => {
    if (!validate) {
      clearValidationTimers();
      activeValidationKey = null;
      internalValidationStatus = "idle";
      internalValidationMessage = "";
      lastValidatedValue = "";
      previousValidationSnapshot = "";
    }
  });

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

  function mergeValidationContext(context: unknown, key: unknown): unknown {
    if (key === undefined || key === null || key === "") {
      return context;
    }

    if (context === undefined || context === null) {
      return { validationKey: key };
    }

    if (typeof context === "object") {
      return { ...(context as Record<string, unknown>), validationKey: key };
    }

    return { value: context, validationKey: key };
  }

  function normalizeInputValue(input: string): string {
    return isSlug ? slugify(input) : input;
  }

  function emitValueChange(nextValue: string, immediate: boolean): void {
    if (immediate || !debounce || debounce <= 0) {
      onValueChange?.(nextValue);
      return;
    }

    if (debounceTimer) clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => {
      debounceTimer = null;
      onValueChange?.(nextValue);
    }, debounce);
  }

  function commitValue(
    nextValue: string,
    options: { markSlugEdited?: boolean; immediate?: boolean } = {},
  ): void {
    if (isControlled) {
      value = nextValue;
    } else {
      uncontrolledValue = nextValue;
    }

    if (isSlug && options.markSlugEdited !== false) {
      userEditedSlug = true;
    }

    emitValueChange(nextValue, options.immediate === true);
  }

  function flushDebouncedValue(): void {
    if (!debounceTimer) return;
    clearTimeout(debounceTimer);
    debounceTimer = null;
    onValueChange?.(currentValue);
  }

  function focus(): void {
    control?.focus();
  }

  function handleInput(event: Event): void {
    const nextValue = normalizeInputValue((event.currentTarget as HTMLInputElement).value);
    commitValue(nextValue, { markSlugEdited: isSlug });
  }

  function handleSubmit(): void {
    onSubmit?.(currentValue);
  }

  function handleCancel(): void {
    onCancel?.();
  }

  function handleKeydown(event: KeyboardEvent): void {
    onKeyDown?.(event);

    if (event.key === "Enter") {
      handleSubmit();
    }

    if (event.key === "Escape") {
      handleCancel();
    }
  }

  function handleClear(): void {
    commitValue("", { markSlugEdited: isSlug, immediate: true });
    onClear?.();
  }

  function handleBlurEvent(event: FocusEvent): void {
    flushDebouncedValue();
    if (validate && validateOnBlur) {
      triggerValidation(currentValue, true);
    }
    onBlur?.(event);
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

  function triggerValidation(inputValue: string, immediate: boolean): void {
    if (!validate) return;

    clearValidationTimers();

    if (!inputValue.trim() && !isSlug) {
      activeValidationKey = null;
      internalValidationStatus = "idle";
      internalValidationMessage = "";
      lastValidatedValue = "";
      return;
    }

    const nextValidationKey = buildValidationKey(inputValue, effectiveValidationContext);
    activeValidationKey = nextValidationKey;
    internalValidationStatus = "validating";
    internalValidationMessage = "";

    if (immediate || validationDebounce <= 0) {
      void runValidation(inputValue, nextValidationKey);
      return;
    }

    validationTimer = setTimeout(() => {
      validationTimer = null;
      void runValidation(inputValue, nextValidationKey);
    }, validationDebounce);
  }

  async function runValidation(inputValue: string, validationKey: string): Promise<void> {
    try {
      const result = isSlug
        ? await validateSlugValue(inputValue)
        : await validate?.(inputValue, effectiveValidationContext);
      if (activeValidationKey !== validationKey || inputValue !== currentValue) return;
      internalValidationStatus = result?.valid ? "valid" : "invalid";
      internalValidationMessage = result?.message ?? "";
      lastValidatedValue = inputValue;
      activeValidationKey = null;
    } catch {
      if (activeValidationKey !== validationKey || inputValue !== currentValue) return;
      internalValidationStatus = "invalid";
      internalValidationMessage = "Could not validate";
      lastValidatedValue = inputValue;
      activeValidationKey = null;
    }
  }

  async function validateSlugValue(inputValue: string): Promise<ValidationResult> {
    const candidate = `${prefix ?? ""}${inputValue}`.trim();
    const limit = maxLength ?? 100;

    if (!candidate) {
      return { valid: !required, message: required ? "Required" : "" };
    }

    if (!isValidSlugFormat(candidate, limit)) {
      return { valid: false, message: "Use lowercase letters, numbers, and hyphens only." };
    }

    if (!validate) {
      return { valid: true, message: "" };
    }

    return await validate(candidate, effectiveValidationContext);
  }
</script>

<div
  class="poodle-text-input"
  class:poodle-text-input--multiline={isMultiline}
  data-validation-state={effectiveValidationState}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-type={type}
  style={`--poodle-text-input-control-padding-start: ${controlPaddingStart}; --poodle-text-input-control-padding-end: ${controlPaddingEnd}; --poodle-text-input-multiline-padding-end: ${multilineBottomPadding}; --poodle-text-input-clear-inset-inline-end: ${clearInsetInlineEnd}; --poodle-text-input-trailing-inset-inline-end: ${trailingInsetInlineEnd};`}
>
  {#if prefix}
    <span class="poodle-text-input__affix poodle-text-input__affix--prefix">{prefix}</span>
  {/if}

  <div class="poodle-text-input__field">
    {#if leadingSnippet}
      <span class="poodle-text-input__affordance poodle-text-input__affordance--leading">
        {@render leadingSnippet()}
      </span>
    {:else if isSearch}
      <span class="poodle-text-input__affordance poodle-text-input__affordance--leading" aria-hidden="true">
        <Icon icon="search" />
      </span>
    {/if}

    {#if isMultiline}
      <textarea
        id={id || undefined}
        {name}
        class="poodle-text-input__control poodle-text-input__control--multiline"
        value={currentValue}
        {placeholder}
        {autocomplete}
        {spellcheck}
        autocapitalize={autocapitalize ?? undefined}
        {...autocorrectAttributes}
        rows={rows ?? 4}
        style={resize !== "vertical" ? `resize: ${resize};` : undefined}
        maxlength={maxLength ?? undefined}
        disabled={disabled}
        readonly={readOnly}
        autofocus={autofocus || undefined}
        aria-label={ariaLabel ?? undefined}
        aria-describedby={effectiveDescribedBy}
        aria-invalid={ariaInvalid}
        aria-busy={ariaBusy}
        oninput={handleInput}
        onkeydown={(event) => {
          onKeyDown?.(event);
          if ((event.metaKey || event.ctrlKey) && event.key === "Enter") {
            handleSubmit();
          }
          if (event.key === "Escape") {
            handleCancel();
          }
        }}
        onfocus={onFocus}
        onblur={handleBlurEvent}
        bind:this={control}
      ></textarea>
    {:else}
      <input
        bind:this={control}
        id={id || undefined}
        {name}
        list={list ?? undefined}
        type={nativeInputType}
        inputmode={(isSlug ? "text" : inputMode) ?? undefined}
        class="poodle-text-input__control"
        value={currentValue}
        {placeholder}
        {autocomplete}
        {required}
        {pattern}
        spellcheck={isSlug ? false : spellcheck}
        autocapitalize={isSlug ? "off" : autocapitalize ?? undefined}
        {...(isSlug ? { autocorrect: "off" } : autocorrectAttributes)}
        enterkeyhint={enterKeyHint ?? undefined}
        maxlength={maxLength ?? undefined}
        disabled={disabled}
        readonly={readOnly}
        autofocus={autofocus || undefined}
        aria-label={ariaLabel ?? undefined}
        aria-describedby={effectiveDescribedBy}
        aria-invalid={ariaInvalid}
        aria-busy={ariaBusy}
        oninput={handleInput}
        onkeydown={handleKeydown}
        onfocus={onFocus}
        onblur={handleBlurEvent}
      />
    {/if}

    {#if trailingSnippet}
      <span class="poodle-text-input__affordance poodle-text-input__affordance--trailing">
        {@render trailingSnippet()}
      </span>
    {/if}

    {#if canClear}
      <button
        class="poodle-text-input__clear"
        type="button"
        aria-label="Clear search query"
        onclick={handleClear}
      >
        <Icon icon="x" />
      </button>
    {/if}

    {#if showValidationIndicator}
      <span
        class="poodle-text-input__validation-indicator"
        class:poodle-text-input__validation-indicator--pending={effectiveValidationState === "pending"}
        class:poodle-text-input__validation-indicator--valid={effectiveValidationState === "valid"}
        class:poodle-text-input__validation-indicator--invalid={effectiveValidationState === "invalid"}
        aria-hidden="true"
      >
        {#if effectiveValidationState === "pending"}
          <Spinner variant="ring" sizeRole="chrome" tone="current" />
        {:else if validationIcon}
          <Icon icon={validationIcon} />
        {/if}
      </span>
    {/if}
  </div>

  {#if suffix}
    <span class="poodle-text-input__affix poodle-text-input__affix--suffix">{suffix}</span>
  {/if}

  {#if showCharCount}
    <span class="poodle-text-input__char-count" class:poodle-text-input__char-count--over={isOverLimit} aria-live="polite">
      {charCountText}
    </span>
  {/if}
</div>

{#if internalValidationMessage && effectiveValidationState === "invalid"}
  <p class="poodle-text-input__validation-message" id={validationMessageId ?? undefined} aria-live="polite">
    {internalValidationMessage}
  </p>
{/if}
