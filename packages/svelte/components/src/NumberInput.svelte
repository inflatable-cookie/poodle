<script lang="ts">
  import Icon from "./Icon.svelte";
  import { formatNumber, snapToStep } from "./internal";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type {
    ControlDensity,
    ControlSize,
    InputValidationStatus,
    InputValidator,
    SemanticControlSizeRole,
    ValidationState,
  } from "./types";

  interface NumberInputValidationChange {
    status: InputValidationStatus;
    valid: boolean;
    message: string;
  }

  interface Props {
    id?: string;
    value?: number | string | null | undefined;
    defaultValue?: number | string | null;
    placeholder?: string | null;
    name?: string | undefined;
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
    validate?: InputValidator | undefined;
    validationContext?: unknown;
    validationState?: ValidationState;
    showSteppers?: boolean;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onValueChange?: ((value: number | string | null) => void) | undefined;
    onValidationChange?: ((detail: NumberInputValidationChange) => void) | undefined;
    onSubmit?: ((value: number | string | null) => void) | undefined;
    onIncrement?: ((value: number | string | null) => void) | undefined;
    onDecrement?: ((value: number | string | null) => void) | undefined;
    onFocus?: ((event: FocusEvent) => void) | undefined;
    onBlur?: ((event: FocusEvent) => void) | undefined;
  }

  const uiPresentation = getUiPresentation();

  let {
    id = "",
    value = $bindable<number | string | null | undefined>(undefined),
    defaultValue = null,
    placeholder = null,
    name = undefined,
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
    validate = undefined,
    validationContext = undefined,
    validationState = "none",
    showSteppers = false,
    size = null,
    sizeRole = "control",
    density = null,
    onValueChange = undefined,
    onValidationChange = undefined,
    onSubmit = undefined,
    onIncrement = undefined,
    onDecrement = undefined,
    onFocus = undefined,
    onBlur = undefined,
  }: Props = $props();

  let internalValidationStatus = $state<InputValidationStatus>("idle");
  let validationMessage = $state("");
  let activeValidationKey = $state(0);
  let uncontrolledValue = $state<number | null>(null);
  let draftValue = $state("");
  let isEditing = $state(false);

  const valueMode = $derived(inferValueMode(value, defaultValue));
  const parsedValue = $derived(parseNumberish(value));
  const parsedDefaultValue = $derived(parseNumberish(defaultValue));
  const parsedMin = $derived(parseNumberish(min));
  const parsedMax = $derived(parseNumberish(max));
  const resolvedStep = $derived(parseStep(step));
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const isControlled = $derived(value !== undefined);
  const currentValue = $derived(isControlled ? parsedValue : uncontrolledValue);
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

  $effect(() => {
    if (!isControlled && uncontrolledValue === null && parsedDefaultValue !== null) {
      uncontrolledValue = parsedDefaultValue;
    }
  });

  $effect(() => {
    if (!isEditing) {
      draftValue = formatNumber(currentValue, precision);
    }
  });

  function inferValueMode(
    currentValue: number | string | null,
    initialValue: number | string | null,
  ): "number" | "string" {
    if (typeof currentValue === "string" || typeof initialValue === "string") return "string";
    return "number";
  }

  function parseNumberish(input: number | string | null | undefined): number | null {
    if (input === null || input === undefined || input === "") return null;
    const nextValue = Number(input);
    return Number.isFinite(nextValue) ? nextValue : null;
  }

  function parseStep(input: number | string | null): number {
    if (input === null || input === "") return 1;
    const nextValue = Number(input);
    return Number.isFinite(nextValue) && nextValue > 0 ? nextValue : 1;
  }

  function clampIfNeeded(nextValue: number): number {
    let clampedValue = nextValue;

    if (parsedMin !== null) {
      clampedValue = Math.max(clampedValue, parsedMin);
    }

    if (parsedMax !== null) {
      clampedValue = Math.min(clampedValue, parsedMax);
    }

    return clampedValue;
  }

  function emitValidationChange(): void {
    onValidationChange?.({
      status: internalValidationStatus,
      valid: internalValidationStatus === "valid" || internalValidationStatus === "idle",
      message: validationMessage,
    });
  }

  async function runValidation(nextValue: number | string | null): Promise<void> {
    const validationValue =
      nextValue === null || nextValue === undefined ? "" : typeof nextValue === "number" ? String(nextValue) : nextValue;

    if (!validate || validationValue.trim() === "") {
      internalValidationStatus = "idle";
      validationMessage = "";
      emitValidationChange();
      return;
    }

    const validationKey = ++activeValidationKey;
    internalValidationStatus = "validating";
    validationMessage = "";
    emitValidationChange();

    try {
      const result = await validate(validationValue, validationContext);
      if (validationKey !== activeValidationKey) return;
      internalValidationStatus = result.valid ? "valid" : "invalid";
      validationMessage = result.message ?? "";
      emitValidationChange();
    } catch {
      if (validationKey !== activeValidationKey) return;
      internalValidationStatus = "invalid";
      validationMessage = "Could not validate";
      emitValidationChange();
    }
  }

  function coerceOutgoingValue(nextValue: number | null): number | string | null {
    if (valueMode === "string") {
      return nextValue === null ? "" : String(nextValue);
    }

    return nextValue;
  }

  function commitValue(nextValue: number | null): void {
    if (!isControlled) {
      uncontrolledValue = nextValue;
    } else {
      value = coerceOutgoingValue(nextValue);
    }

    const outgoingValue = coerceOutgoingValue(nextValue);
    onValueChange?.(outgoingValue);
    void runValidation(outgoingValue);
  }

  function handleInput(event: Event): void {
    draftValue = (event.currentTarget as HTMLInputElement).value;

    if (draftValue.trim() === "") {
      commitValue(null);
      return;
    }

    const parsedNextValue = Number(draftValue);

    if (!Number.isNaN(parsedNextValue)) {
      commitValue(parsedNextValue);
    }
  }

  function handleBlur(event: FocusEvent): void {
    isEditing = false;

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
    draftValue = formatNumber(nextValue, precision);
    const outgoingValue = coerceOutgoingValue(nextValue);
    if (eventName === "increment") {
      onIncrement?.(outgoingValue);
      return;
    }

    onDecrement?.(outgoingValue);
  }
</script>

<div class:poodle-with-prefix={Boolean(prefix)} class:poodle-with-suffix={Boolean(suffix)} class="poodle-number-input">
  {#if prefix}
    <span class="poodle-number-input__prefix">{prefix}</span>
  {/if}

  <div
    class="poodle-number-input__field"
    data-validation-state={effectiveValidationState}
    data-size={resolvedSize}
    data-density={resolvedDensity}
    data-disabled={disabled}
  >
    <input
      {id}
      {name}
      class="poodle-number-input__control"
      type="text"
      inputmode="decimal"
      value={draftValue}
      {placeholder}
      disabled={disabled}
      readonly={readOnly}
      required={required}
      aria-label={ariaLabel ?? undefined}
      aria-describedby={describedBy ?? undefined}
      aria-invalid={ariaInvalid}
      aria-busy={ariaBusy}
      oninput={handleInput}
      onfocus={(event) => {
        isEditing = true;
        onFocus?.(event);
      }}
      onblur={handleBlur}
      onkeydown={(event) => {
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

    {#if showSteppers}
      <div class="poodle-number-input__steppers">
        <button type="button" disabled={disabled || readOnly} onclick={() => adjust(resolvedStep, "increment")}>
          <Icon name="plus" />
        </button>
        <button type="button" disabled={disabled || readOnly} onclick={() => adjust(-resolvedStep, "decrement")}>
          <Icon name="minus" />
        </button>
      </div>
    {/if}
  </div>

  {#if suffix}
    <span class="poodle-number-input__suffix">{suffix}</span>
  {/if}
</div>

<style>
  .poodle-number-input {
    display: flex;
    align-items: stretch;
    gap: 0.5rem;
  }

  .poodle-number-input__prefix,
  .poodle-number-input__suffix {
    display: inline-flex;
    align-items: center;
    padding: 0 0.75rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    color: var(--poodle-color-text-muted);
    background: var(--poodle-color-background-surface);
    white-space: nowrap;
  }

  .poodle-number-input__field {
    flex: 1 1 auto;
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: stretch;
    height: var(--poodle-size-control-height);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    overflow: hidden;
  }

  .poodle-number-input__field[data-validation-state="invalid"] {
    border-color: var(--poodle-color-status-danger);
  }

  .poodle-number-input__field[data-validation-state="valid"] {
    border-color: var(--poodle-color-status-success);
  }

  .poodle-number-input__field[data-validation-state="pending"] {
    border-color: var(--poodle-color-accent-base);
  }

  .poodle-number-input__field:focus-within {
    box-shadow:
      0 0 0 var(--poodle-border-width-focus)
      color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent);
  }

  .poodle-number-input__control {
    min-width: 0;
    padding: 0 var(--poodle-space-control-x);
    border: 0;
    background: transparent;
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
    outline: 0;
  }

  .poodle-number-input__steppers {
    display: grid;
    grid-template-rows: 1fr 1fr;
    gap: 0;
    padding: 0.0625rem;
  }

  .poodle-number-input__steppers button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: calc(var(--poodle-size-icon-md) + 0.5rem);
    min-height: 0;
    border: 0;
    border-radius: calc(var(--poodle-radius-control) - 0.125rem);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 88%, transparent);
    color: var(--poodle-color-text-primary);
    cursor: pointer;
    font-size: 0;
    padding: 0;
    overflow: hidden;
  }

  .poodle-number-input__field[data-disabled="true"] {
    opacity: var(--poodle-state-opacity-disabled);
    cursor: not-allowed;
  }

  .poodle-number-input__field[data-disabled="true"] .poodle-number-input__control {
    cursor: not-allowed;
  }

  .poodle-number-input__steppers button:disabled {
    cursor: not-allowed;
  }

  .poodle-number-input__field[data-density="compact"] .poodle-number-input__control {
    padding: 0 calc(var(--poodle-space-control-x) - 0.125rem);
  }

  .poodle-number-input__field[data-density="comfortable"] .poodle-number-input__control {
    padding: 0 calc(var(--poodle-space-control-x) + 0.125rem);
  }

  .poodle-number-input__field[data-size="xs"] { height: 1.5rem; }
  .poodle-number-input__field[data-size="xs"] .poodle-number-input__control { font-size: 0.75rem; }

  .poodle-number-input__field[data-size="sm"] { height: 1.75rem; }
  .poodle-number-input__field[data-size="sm"] .poodle-number-input__control { font-size: 0.8125rem; }

  .poodle-number-input__field[data-size="lg"] { height: 2.75rem; }
  .poodle-number-input__field[data-size="lg"] .poodle-number-input__control { font-size: 0.9375rem; }

  .poodle-number-input__field[data-size="xl"] { height: 3.25rem; }
  .poodle-number-input__field[data-size="xl"] .poodle-number-input__control { font-size: 1rem; }
</style>
