<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import NumberEntry from "./NumberEntry.svelte";

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

  let internalValidationStatus: InputValidationStatus = "idle";
  let validationMessage = "";
  let activeValidationKey = 0;

  $: valueMode = inferValueMode(value, defaultValue);
  $: parsedValue = parseNumberish(value);
  $: parsedDefaultValue = parseNumberish(defaultValue);
  $: parsedMin = parseNumberish(min);
  $: parsedMax = parseNumberish(max);
  $: resolvedStep = parseStep(step);
  $: effectiveValidationState = validate
    ? internalValidationStatus === "validating"
      ? "pending"
      : internalValidationStatus === "valid"
        ? "valid"
        : internalValidationStatus === "invalid"
          ? "invalid"
          : validationState
    : validationState;

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

  function handleValueChange(nextValue: number | null): void {
    value = coerceOutgoingValue(nextValue);
    dispatch("valueChange", { value });
    void runValidation(value);
  }
</script>

<div class:with-prefix={Boolean(prefix)} class="number-input">
  {#if prefix}
    <span class="number-input__prefix">{prefix}</span>
  {/if}

  <NumberEntry
    {id}
    value={parsedValue}
    defaultValue={parsedDefaultValue}
    {placeholder}
    {name}
    {disabled}
    {readOnly}
    {required}
    min={parsedMin}
    max={parsedMax}
    step={resolvedStep}
    {precision}
    {showSteppers}
    {size}
    {sizeRole}
    {density}
    validationState={effectiveValidationState}
    {ariaLabel}
    {describedBy}
    on:valueChange={(event) => handleValueChange(event.detail.value)}
    on:submit={(event) => dispatch("submit", { value: coerceOutgoingValue(event.detail.value) })}
    on:increment={(event) => dispatch("increment", { value: coerceOutgoingValue(event.detail.value) })}
    on:decrement={(event) => dispatch("decrement", { value: coerceOutgoingValue(event.detail.value) })}
    on:focus={(event) => dispatch("focus", event.detail)}
    on:blur={(event) => dispatch("blur", event.detail)}
  />
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

  .number-input :global(.number-entry) {
    flex: 1 1 auto;
  }
</style>
