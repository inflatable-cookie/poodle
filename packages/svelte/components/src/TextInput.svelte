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
  // Card 048 R2: the DOM reads the generated definition — the data-*
  // attribute names, the part class names, and the TXT-16 padding custom
  // properties come from text_input.rs via `text-input-ts`, never from
  // hand-written literals in this component. A rename in the definition
  // moves the DOM; `effigy ir:check` gates drift in the artifact.
  import { textInputDefinition } from "./generated/text-input";

  const parts: Record<string, string> = Object.fromEntries(
    textInputDefinition.parts.map((part) => [part.id, part.className]),
  );
  const attributes: Record<string, string> = Object.fromEntries(
    textInputDefinition.attributes.map((attribute) => [attribute.id, attribute.name]),
  );
  const styleProps: Record<string, string> = Object.fromEntries(
    textInputDefinition.styleProps.map((prop) => [prop.id, prop.name]),
  );

  function partClass(id: string): string {
    const className = parts[id];
    if (!className) throw new Error(`definition lacks part '${id}'`);
    return className;
  }

  function attributeName(id: string): string {
    const name = attributes[id];
    if (!name) throw new Error(`definition lacks attribute '${id}'`);
    return name;
  }

  function stylePropName(id: string): string {
    const name = styleProps[id];
    if (!name) throw new Error(`definition lacks style prop '${id}'`);
    return name;
  }

  // The anatomy classes (T §2) — the definition names them; the markup
  // renders them. The input-control and clear-button base classes are the
  // one Svelte literal: the focus-coverage gate (focus-ring-drift.ts)
  // resolves focusable-element classes by literal source scan, and
  // text-input.css draws a stacked :focus-within wrapper ring whose
  // focusables must show outline coverage (recorded in the R7 inventory).
  const rootClass = partClass("root");
  const prefixClass = partClass("prefix");
  const fieldClass = partClass("field");
  const leadingAffordanceClass = partClass("leading-affordance");
  const trailingAffordanceClass = partClass("trailing-affordance");
  const indicatorClass = partClass("validation-indicator");
  const suffixClass = partClass("suffix");
  const charCountClass = partClass("char-count");

  // The state-derived attribute names (TXT-18) — the definition names
  // them; the values stay runtime-derived (CROSS-13).
  const dataValidationState = attributeName("validation-state");
  const dataSize = attributeName("size");
  const dataDensity = attributeName("density");
  const dataType = attributeName("type");

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
  // Card 048: IME composition stays browser-native, but the value path is
  // gated — the browser fires `input` events with the partial buffer during
  // composition, and none of them may reach onValueChange. The buffer is
  // recorded while composing and committed exactly once on compositionend
  // (per the UI Events spec, the final committed input event fires before
  // compositionend, so the end handler is the single commit point). The
  // composition's text editing itself is never intercepted (TXT-24's web
  // half).
  let composing = $state(false);
  let compositionBuffer: string | null = $state(null);

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
  // The root attribute emission (TXT-18): the names come from the
  // definition, the values stay runtime-derived (CROSS-13).
  const rootAttributes = $derived({
    [dataValidationState]: effectiveValidationState,
    [dataSize]: resolvedSize,
    [dataDensity]: resolvedDensity,
    [dataType]: type,
  });
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

  function handleCompositionStart(): void {
    composing = true;
  }

  function handleCompositionEnd(): void {
    composing = false;
    if (compositionBuffer !== null) {
      commitValue(normalizeInputValue(compositionBuffer), { markSlugEdited: isSlug });
      compositionBuffer = null;
    }
  }

  function handleInput(event: Event): void {
    const value = (event.currentTarget as HTMLInputElement).value;
    if (composing) {
      // A composition is in progress: buffer the current text and do not
      // fire onValueChange — the commit lands once on compositionend.
      compositionBuffer = value;
      return;
    }
    commitValue(normalizeInputValue(value), { markSlugEdited: isSlug });
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
  {...rootAttributes}
  class={`${rootClass}${isMultiline ? ` ${rootClass}--multiline` : ""}`}
  style={`${stylePropName("control-padding-start")}: ${controlPaddingStart}; ${stylePropName("control-padding-end")}: ${controlPaddingEnd}; ${stylePropName("multiline-padding-end")}: ${multilineBottomPadding}; ${stylePropName("clear-inset-inline-end")}: ${clearInsetInlineEnd}; ${stylePropName("trailing-inset-inline-end")}: ${trailingInsetInlineEnd};`}
>
  {#if prefix}
    <span class={prefixClass}>{prefix}</span>
  {/if}

  <div class={fieldClass}>
    {#if leadingSnippet}
      <span class={leadingAffordanceClass}>
        {@render leadingSnippet()}
      </span>
    {:else if isSearch}
      <span class={leadingAffordanceClass} aria-hidden="true">
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
        oncompositionstart={handleCompositionStart}
        oncompositionend={handleCompositionEnd}
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
        oncompositionstart={handleCompositionStart}
        oncompositionend={handleCompositionEnd}
        onkeydown={handleKeydown}
        onfocus={onFocus}
        onblur={handleBlurEvent}
      />
    {/if}

    {#if trailingSnippet}
      <span class={trailingAffordanceClass}>
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
        class={`${indicatorClass}${effectiveValidationState === "pending" ? ` ${indicatorClass}--pending` : ""}${effectiveValidationState === "valid" ? ` ${indicatorClass}--valid` : ""}${effectiveValidationState === "invalid" ? ` ${indicatorClass}--invalid` : ""}`}
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
    <span class={suffixClass}>{suffix}</span>
  {/if}

  {#if showCharCount}
    <span class={`${charCountClass}${isOverLimit ? ` ${charCountClass}--over` : ""}`} aria-live="polite">
      {charCountText}
    </span>
  {/if}
</div>

{#if internalValidationMessage && effectiveValidationState === "invalid"}
  <p class="poodle-text-input__validation-message" id={validationMessageId ?? undefined} aria-live="polite">
    {internalValidationMessage}
  </p>
{/if}
