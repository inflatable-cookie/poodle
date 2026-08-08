<script lang="ts">
  import "@inflatable-cookie/poodle-styles/number-input.css";
  import {
    clampNullable,
    parseNumberish,
    parseStep,
    validationStatusToState,
  } from "@inflatable-cookie/poodle-headless";

  import { default as Icon } from "./Icon.svelte";
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
  const effectiveValidationState = $derived.by(() =>
    validate ? validationStatusToState(internalValidationStatus, validationState) : validationState
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
    currentValue: number | string | null | undefined,
    initialValue: number | string | null,
  ): "number" | "string" {
    if (typeof currentValue === "string" || typeof initialValue === "string") return "string";
    return "number";
  }

  function clampIfNeeded(nextValue: number): number {
    return clampNullable(nextValue, parsedMin, parsedMax);
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

