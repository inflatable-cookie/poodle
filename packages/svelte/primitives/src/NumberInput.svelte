<script lang="ts">
  import { createEventDispatcher } from "svelte";

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

  export let id = "";
  export let value: number | string | null = null;
  export let defaultValue: number | string | null = null;
  export let placeholder: string | null = null;
  export let name: string | undefined = undefined;
  export let disabled = false;
  export let readOnly = false;
  export let required = false;
  export let min: number | string | null = null;
  export let max: number | string | null = null;
  export let step: number | string | null = null;
  export let precision: number | null = null;
  export let ariaLabel: string | null = null;
  export let describedBy: string | null = null;
  export let prefix: string | null = null;
  export let validate: InputValidator | undefined = undefined;
  export let validationContext: unknown = undefined;
  export let validationState: ValidationState = "none";
  export let showSteppers = false;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: number | string | null };
    validationChange: { status: InputValidationStatus; valid: boolean; message: string };
    submit: { value: number | string | null };
    increment: { value: number | string | null };
    decrement: { value: number | string | null };
    focus: FocusEvent;
    blur: FocusEvent;
  }>();

  const uiPresentation = getUiPresentation();

  let internalValidationStatus: InputValidationStatus = "idle";
  let validationMessage = "";
  let activeValidationKey = 0;
  let uncontrolledValue: number | null = null;
  let draftValue = "";
  let isEditing = false;

  $: valueMode = inferValueMode(value, defaultValue);
  $: parsedValue = parseNumberish(value);
  $: parsedDefaultValue = parseNumberish(defaultValue);
  $: parsedMin = parseNumberish(min);
  $: parsedMax = parseNumberish(max);
  $: resolvedStep = parseStep(step);
  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: isControlled = value !== null;
  $: currentValue = isControlled ? parsedValue : uncontrolledValue;
  $: if (!isControlled && uncontrolledValue === null && parsedDefaultValue !== null) {
    uncontrolledValue = parsedDefaultValue;
  }
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
  $: if (!isEditing) {
    draftValue = formatNumber(currentValue, precision);
  }

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
    dispatch("validationChange", {
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
    }

    value = coerceOutgoingValue(nextValue);
    dispatch("valueChange", { value });
    void runValidation(value);
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

    dispatch("blur", event);
  }

  function adjust(delta: number, eventName: "increment" | "decrement"): void {
    const baseline = currentValue ?? parsedMin ?? 0;
    const nextValue = clampIfNeeded(snapToStep(baseline + delta, parsedMin ?? 0, resolvedStep));
    commitValue(nextValue);
    dispatch(eventName, { value: coerceOutgoingValue(nextValue) });
  }
</script>

<div class:with-prefix={Boolean(prefix)} class="number-input">
  {#if prefix}
    <span class="number-input__prefix">{prefix}</span>
  {/if}

  <div
    class="number-input__field"
    data-validation-state={effectiveValidationState}
    data-size={resolvedSize}
    data-density={resolvedDensity}
    data-disabled={disabled}
  >
    <input
      {id}
      {name}
      class="number-input__control"
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
      on:input={handleInput}
      on:focus={(event) => {
        isEditing = true;
        dispatch("focus", event);
      }}
      on:blur={handleBlur}
      on:keydown={(event) => {
        if (event.key === "Enter") {
          dispatch("submit", { value: coerceOutgoingValue(currentValue) });
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
      <div class="number-input__steppers">
        <button type="button" disabled={disabled || readOnly} on:click={() => adjust(resolvedStep, "increment")}>
          <Icon name="plus" />
        </button>
        <button type="button" disabled={disabled || readOnly} on:click={() => adjust(-resolvedStep, "decrement")}>
          <Icon name="minus" />
        </button>
      </div>
    {/if}
  </div>
</div>

<style>
  .number-input {
    display: flex;
    align-items: stretch;
    gap: 0.5rem;
  }

  .number-input__prefix {
    display: inline-flex;
    align-items: center;
    padding: 0 0.75rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    color: var(--poodle-color-text-muted);
    background: var(--poodle-color-background-surface);
    white-space: nowrap;
  }

  .number-input__field {
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

  .number-input__field[data-validation-state="invalid"] {
    border-color: var(--poodle-color-status-danger);
  }

  .number-input__field[data-validation-state="valid"] {
    border-color: var(--poodle-color-status-success);
  }

  .number-input__field[data-validation-state="pending"] {
    border-color: var(--poodle-color-accent-base);
  }

  .number-input__field:focus-within {
    box-shadow:
      0 0 0 var(--poodle-border-width-focus)
      color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent);
  }

  .number-input__control {
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

  .number-input__steppers {
    display: grid;
    grid-template-rows: 1fr 1fr;
    gap: 0;
    padding: 0.0625rem;
  }

  .number-input__steppers button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: calc(var(--poodle-size-icon-default) + 0.5rem);
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

  .number-input__field[data-disabled="true"] {
    opacity: var(--poodle-state-opacity-disabled);
    cursor: not-allowed;
  }

  .number-input__field[data-disabled="true"] .number-input__control {
    cursor: not-allowed;
  }

  .number-input__steppers button:disabled {
    cursor: not-allowed;
  }

  .number-input__field[data-density="compact"] .number-input__control {
    padding: 0 calc(var(--poodle-space-control-x) - 0.125rem);
  }

  .number-input__field[data-density="comfortable"] .number-input__control {
    padding: 0 calc(var(--poodle-space-control-x) + 0.125rem);
  }

  .number-input__field[data-size="xs"] {
    height: calc(var(--poodle-size-control-height) - 0.5rem);
  }

  .number-input__field[data-size="xs"] .number-input__control {
    font-size: 0.75rem;
  }

  .number-input__field[data-size="sm"] {
    height: calc(var(--poodle-size-control-height) - 0.375rem);
  }

  .number-input__field[data-size="lg"] {
    height: calc(var(--poodle-size-control-height) + 0.375rem);
  }

  .number-input__field[data-size="lg"] .number-input__control {
    font-size: 0.9375rem;
  }

  .number-input__field[data-size="xl"] {
    height: calc(var(--poodle-size-control-height) + 0.5rem);
  }

  .number-input__field[data-size="xl"] .number-input__control {
    font-size: 1rem;
  }
</style>
