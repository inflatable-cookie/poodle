<script lang="ts">
  import { onDestroy, type Snippet } from "svelte";
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
    required?: boolean;
    pattern?: string | undefined;
    spellcheck?: HTMLInputAttributes["spellcheck"];
    autocapitalize?: HTMLInputAttributes["autocapitalize"];
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

  let {
    id = "",
    value = $bindable<string | null | undefined>(undefined),
    defaultValue = "",
    placeholder = null,
    name = undefined,
    autocomplete = undefined,
    disabled = false,
    readOnly = false,
    required = false,
    pattern = undefined,
    spellcheck = undefined,
    autocapitalize = undefined,
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

  const uiPresentation = getUiPresentation();
  const RESERVED_SLUGS = [
    "new",
    "edit",
    "delete",
    "create",
    "update",
    "list",
    "admin",
    "api",
    "auth",
    "login",
    "logout",
    "register",
    "settings",
    "profile",
    "dashboard",
    "search",
  ] as const;

  let liveValue = $state("");
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

  const isSearch = $derived(type === "search");
  const isSlug = $derived(type === "slug");
  const isMultiline = $derived(type === "multiline" || (type === "text" && rows !== null && rows > 1));
  const nativeInputType = $derived(isSlug ? "text" : type);
  const hasLeadingAffordance = $derived(Boolean(leadingSnippet) || isSearch);
  const hasTrailingAffordance = $derived(Boolean(trailingSnippet));
  const currentValue = $derived(liveValue);
  const canClear = $derived(isSearch && showClearButton && !disabled && !readOnly && currentValue.length > 0);
  const effectiveValidationState = $derived(
    validate
      ? internalValidationStatus === "validating"
        ? "pending"
        : internalValidationStatus === "valid"
          ? "valid"
          : internalValidationStatus === "invalid"
            ? "invalid"
            : validationState
      : validationState
  );
  const ariaInvalid = $derived(effectiveValidationState === "invalid" ? true : undefined);
  const ariaBusy = $derived(effectiveValidationState === "pending" ? true : undefined);
  const charCount = $derived(currentValue.length);
  const charCountText = $derived(maxLength ? `${charCount}/${maxLength}` : `${charCount}`);
  const isOverLimit = $derived(maxLength !== null && charCount > maxLength);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
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
  const multilineBottomPadding = $derived(
    showCharCount
      ? "calc(var(--poodle-text-input-padding-block) + 1.5rem)"
      : "var(--poodle-text-input-padding-block)"
  );

  $effect(() => {
    const resolvedValue = (value ?? defaultValue) || "";
    if (resolvedValue !== liveValue) {
      liveValue = resolvedValue;
    }
  });

  $effect(() => {
    if (isSlug && source !== null) {
      if (!userEditedSlug || liveValue === previousGeneratedSlug || liveValue === "") {
        if (previousGeneratedSlug !== generatedSlug) {
          previousGeneratedSlug = generatedSlug;
        }
        if (liveValue !== generatedSlug) {
          commitValue(generatedSlug, { markSlugEdited: false, immediate: true });
        }
      }
    }
  });

  $effect(() => {
    if (validate && liveValue !== lastValidatedValue) {
      triggerValidation(liveValue, false);
    }
  });

  $effect(() => {
    if (validate && contextKey !== previousContextKey) {
      previousContextKey = contextKey;
      if (liveValue) {
        triggerValidation(liveValue, false);
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

  function slugify(input: string): string {
    return input
      .normalize("NFD")
      .replace(/[\u0300-\u036f]/g, "")
      .toLowerCase()
      .trim()
      .replace(/[^a-z0-9\s-]/g, "")
      .replace(/[\s_]+/g, "-")
      .replace(/-+/g, "-")
      .replace(/^-|-$/g, "");
  }

  function isValidSlugFormat(slug: string, limit: number = 100): boolean {
    return /^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(slug) && slug.length >= 2 && slug.length <= limit;
  }

  function isReservedSlug(slug: string): boolean {
    return (RESERVED_SLUGS as readonly string[]).includes(slug);
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
    liveValue = nextValue;
    if (value !== undefined) {
      value = nextValue;
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
    onValueChange?.(liveValue);
  }

  function handleInput(event: Event): void {
    const nextValue = normalizeInputValue((event.currentTarget as HTMLInputElement).value);
    commitValue(nextValue, { markSlugEdited: isSlug });
  }

  function handleSubmit(): void {
    onSubmit?.(liveValue);
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
      triggerValidation(liveValue, true);
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

    const runValidation = async (): Promise<void> => {
      try {
        const result = isSlug
          ? await validateSlugValue(inputValue)
          : await validate?.(inputValue, effectiveValidationContext);
        if (activeValidationKey !== nextValidationKey || inputValue !== liveValue) return;
        internalValidationStatus = result?.valid ? "valid" : "invalid";
        internalValidationMessage = result?.message ?? "";
        lastValidatedValue = inputValue;
        activeValidationKey = null;
      } catch {
        if (activeValidationKey !== nextValidationKey || inputValue !== liveValue) return;
        internalValidationStatus = "invalid";
        internalValidationMessage = "Could not validate";
        lastValidatedValue = inputValue;
        activeValidationKey = null;
      }
    };

    if (immediate || validationDebounce <= 0) {
      void runValidation();
      return;
    }

    validationTimer = setTimeout(() => {
      validationTimer = null;
      void runValidation();
    }, validationDebounce);
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

    if (isReservedSlug(candidate)) {
      return { valid: false, message: "This slug is reserved." };
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
  style={`--poodle-text-input-control-padding-start: ${controlPaddingStart}; --poodle-text-input-control-padding-end: ${controlPaddingEnd}; --poodle-text-input-multiline-padding-end: ${multilineBottomPadding};`}
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
        rows={rows ?? 4}
        style={resize !== "vertical" ? `resize: ${resize};` : undefined}
        maxlength={maxLength ?? undefined}
        disabled={disabled}
        readonly={readOnly}
        aria-label={ariaLabel ?? undefined}
        aria-describedby={describedBy ?? undefined}
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
      ></textarea>
    {:else}
      <input
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
        enterkeyhint={enterKeyHint ?? undefined}
        maxlength={maxLength ?? undefined}
        disabled={disabled}
        readonly={readOnly}
        aria-label={ariaLabel ?? undefined}
        aria-describedby={describedBy ?? undefined}
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

<style>
  .poodle-text-input {
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
    --poodle-text-input-padding-inline: var(--poodle-space-control-x);
    --poodle-text-input-padding-block: var(--poodle-space-control-y);
    --poodle-text-input-adornment-gap: var(--poodle-space-inline-sm);
    --poodle-text-input-density-inline-adjust: 0rem;
    --poodle-text-input-density-block-adjust: 0rem;
    --poodle-text-input-font-size: var(--poodle-typography-body-size);
    --poodle-text-input-line-height: var(--poodle-typography-body-lineHeight);
    display: flex;
    align-items: center;
    min-height: var(--poodle-size-control-height);
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

  .poodle-text-input:focus-within {
    border-color: var(--poodle-text-input-border-focus);
    background: var(--poodle-text-input-fill-focus);
    box-shadow: var(--poodle-text-input-shadow-focus);
  }

  .poodle-text-input[data-validation-state="invalid"] {
    border-color: var(--poodle-color-status-danger);
  }

  .poodle-text-input[data-validation-state="valid"] {
    border-color: var(--poodle-color-status-success);
  }

  .poodle-text-input[data-validation-state="pending"] {
    border-color: var(--poodle-color-accent-base);
  }

  .poodle-text-input:has(.poodle-text-input__control:disabled) {
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-text-input[data-size="xs"] {
    min-height: 1.5rem;
    --poodle-text-input-padding-inline: calc(var(--poodle-space-control-x) - 0.25rem);
    --poodle-text-input-padding-block: calc(var(--poodle-space-control-y) - 0.125rem);
    --poodle-text-input-font-size: 0.75rem;
  }

  .poodle-text-input[data-size="sm"] {
    min-height: 1.75rem;
    --poodle-text-input-padding-inline: calc(var(--poodle-space-control-x) - 0.125rem);
    --poodle-text-input-padding-block: calc(var(--poodle-space-control-y) - 0.0625rem);
    --poodle-text-input-font-size: 0.8125rem;
  }

  .poodle-text-input[data-size="md"] {
    min-height: var(--poodle-size-control-height-md, var(--poodle-size-control-height));
  }

  .poodle-text-input[data-size="lg"] {
    min-height: 2.75rem;
    --poodle-text-input-padding-inline: calc(var(--poodle-space-control-x) + 0.125rem);
    --poodle-text-input-padding-block: calc(var(--poodle-space-control-y) + 0.0625rem);
    --poodle-text-input-font-size: 0.9375rem;
  }

  .poodle-text-input[data-size="xl"] {
    min-height: 3.25rem;
    --poodle-text-input-padding-inline: calc(var(--poodle-space-control-x) + 0.1875rem);
    --poodle-text-input-padding-block: calc(var(--poodle-space-control-y) + 0.125rem);
    --poodle-text-input-font-size: 1rem;
  }

  .poodle-text-input[data-density="compact"] {
    --poodle-text-input-density-inline-adjust: -0.125rem;
    --poodle-text-input-density-block-adjust: -0.0625rem;
  }

  .poodle-text-input[data-density="comfortable"] {
    --poodle-text-input-density-inline-adjust: 0.125rem;
    --poodle-text-input-density-block-adjust: 0.0625rem;
  }

  .poodle-text-input__affix {
    display: inline-flex;
    align-items: center;
    align-self: stretch;
    padding-inline: 0.625rem;
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-text-input-font-size);
    line-height: var(--poodle-text-input-line-height);
    font-weight: var(--poodle-typography-body-weight);
    color: var(--poodle-color-text-secondary);
    opacity: var(--poodle-state-opacity-muted);
    user-select: none;
    white-space: nowrap;
  }

  .poodle-text-input__affix--prefix {
    border-right: 0.0625rem solid var(--poodle-color-border-default);
  }

  .poodle-text-input__affix--suffix {
    border-left: 0.0625rem solid var(--poodle-color-border-default);
  }

  .poodle-text-input__field {
    position: relative;
    flex: 1;
    min-width: 0;
    display: flex;
    align-items: stretch;
  }

  .poodle-text-input__control {
    width: 100%;
    min-width: 0;
    border: none;
    background: transparent;
    color: inherit;
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-text-input-font-size);
    line-height: var(--poodle-text-input-line-height);
    font-weight: var(--poodle-typography-body-weight);
    padding-block: calc(var(--poodle-text-input-padding-block) + var(--poodle-text-input-density-block-adjust));
    padding-inline-start: var(--poodle-text-input-control-padding-start);
    padding-inline-end: var(--poodle-text-input-control-padding-end);
    outline: none;
  }

  .poodle-text-input__control::placeholder {
    color: var(--poodle-color-text-muted);
  }

  .poodle-text-input__control:disabled {
    cursor: not-allowed;
  }

  .poodle-text-input__control--multiline {
    min-height: calc(1lh * 4);
    padding-block-end: var(--poodle-text-input-multiline-padding-end);
    resize: vertical;
  }

  .poodle-text-input--multiline {
    align-items: stretch;
    min-height: auto;
  }

  .poodle-text-input__affordance,
  .poodle-text-input__validation-indicator,
  .poodle-text-input__clear {
    position: absolute;
    inset-block: 0;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: calc(var(--poodle-icon-size-default) + var(--poodle-text-input-adornment-gap));
    color: var(--poodle-color-text-muted);
  }

  .poodle-text-input__affordance--leading {
    inset-inline-start: 0.5rem;
  }

  .poodle-text-input__affordance--trailing {
    inset-inline-end: calc(var(--poodle-text-input-adornment-gap) * 2 + var(--poodle-icon-size-default));
  }

  .poodle-text-input__clear {
    inset-inline-end: calc(var(--poodle-icon-size-default) + var(--poodle-text-input-adornment-gap));
    border: none;
    background: transparent;
    cursor: pointer;
    padding: 0;
  }

  .poodle-text-input__validation-indicator {
    inset-inline-end: 0.5rem;
  }

  .poodle-text-input__validation-indicator--pending {
    color: var(--poodle-color-accent-base);
  }

  .poodle-text-input__validation-indicator--valid {
    color: var(--poodle-color-status-success);
  }

  .poodle-text-input__validation-indicator--invalid {
    color: var(--poodle-color-status-danger);
  }

  .poodle-text-input__char-count {
    position: absolute;
    inset-inline-end: 0.5rem;
    inset-block-end: 0.375rem;
    font: var(--poodle-typography-code-xs);
    color: var(--poodle-color-text-muted);
    pointer-events: none;
  }

  .poodle-text-input__char-count--over {
    color: var(--poodle-color-status-danger);
  }

  .poodle-text-input[data-type="slug"] .poodle-text-input__control,
  .poodle-text-input[data-type="slug"] .poodle-text-input__affix--prefix,
  .poodle-text-input[data-type="slug"] .poodle-text-input__affix--suffix {
    font-family: var(--poodle-typography-code-family);
    font-size: calc(1em * var(--poodle-typography-code-adjustmentRatio));
  }
</style>
