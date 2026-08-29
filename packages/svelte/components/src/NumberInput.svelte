<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/number-input.css";
  import {
    formatNumberCommitted,
    numberDraftConstraintValid,
    numberInputContext,
    numberInputDisplayText,
    numberInputInvalid,
    numberInputTransition,
    parseNumberDecimal,
    numberDecimalToNumber,
    stepNumberValue,
    validationStatusToState,
    type NumberInputEffect,
  } from "@inflatable-cookie/poodle-core";

  import { default as Icon } from "./Icon.svelte";
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
    value?: number | null | undefined;
    defaultValue?: number | null;
    draftValue?: string | null | undefined;
    placeholder?: string | null;
    name?: string | undefined;
    disabled?: boolean;
    readOnly?: boolean;
    required?: boolean;
    min?: number | null;
    max?: number | null;
    step?: number | null;
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
    onValueChange?: ((value: number | null) => void) | undefined;
    onDraftValueChange?: ((draft: string | null) => void) | undefined;
    onValidationChange?: ((detail: NumberInputValidationChange) => void) | undefined;
    onCommit?: ((value: number | null) => void) | undefined;
    onFocus?: ((event: FocusEvent) => void) | undefined;
    onBlur?: ((event: FocusEvent) => void) | undefined;
  }

  const uiPresentation = getUiPresentation();

  let {
    id = "",
    value = $bindable<number | null | undefined>(undefined),
    defaultValue = null,
    draftValue = $bindable<string | null | undefined>(undefined),
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
    onDraftValueChange = undefined,
    onValidationChange = undefined,
    onCommit = undefined,
    onFocus = undefined,
    onBlur = undefined,
  }: Props = $props();

  let internalValidationStatus = $state<InputValidationStatus>("idle");
  let validationMessage = $state("");
  let activeValidationKey = $state(0);
  let uncontrolledValue = $state<number | null>(null);
  let seededDefaultValue = $state(false);
  let localDraft = $state<string | null>(null);
  let lastControlledValue = $state<number | null | undefined>(undefined);
  let lastEmittedValue = $state<number | null | undefined>(undefined);

  $effect.pre(() => {
    if (!seededDefaultValue) {
      uncontrolledValue = defaultValue;
      seededDefaultValue = true;
    }
  });

  $effect(() => {
    if (value === undefined || value === lastControlledValue) {
      return;
    }

    const previous = lastControlledValue;
    lastControlledValue = value;

    if (previous !== undefined && value === lastEmittedValue) {
      return;
    }

    if (draftValue === undefined) {
      localDraft = null;
    }
  });

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const isControlled = $derived(value !== undefined);
  const draftControlled = $derived(draftValue !== undefined);
  const committed = $derived((isControlled ? value : uncontrolledValue) ?? null);
  const activeDraft = $derived(draftControlled ? draftValue : localDraft);
  const machineContext = $derived(
    numberInputContext({
      committed,
      defaultValue,
      draft: activeDraft ?? null,
      min,
      max,
      step,
      precision,
      disabled,
      readOnly,
    }),
  );
  const displayValue = $derived(numberInputDisplayText(machineContext));
  const draftInvalid = $derived(numberInputInvalid(machineContext));
  const effectiveValidationState = $derived.by(() =>
    validate ? validationStatusToState(internalValidationStatus, validationState) : validationState,
  );
  const ariaInvalid = $derived(
    draftInvalid || effectiveValidationState === "invalid" ? true : undefined,
  );
  const ariaBusy = $derived(effectiveValidationState === "pending" ? true : undefined);
  const ariaValueNow = $derived.by(() => {
    if (activeDraft !== null && activeDraft !== undefined) {
      if (
        !numberDraftConstraintValid(activeDraft, min, max, step, precision)
      ) {
        return undefined;
      }

      const decimal = parseNumberDecimal(activeDraft);
      return decimal === null ? undefined : numberDecimalToNumber(decimal);
    }

    return committed === null ? undefined : committed;
  });
  const stepFrom = $derived.by(() => {
    if (activeDraft !== null && activeDraft !== undefined) {
      if (activeDraft === "") {
        return null;
      }

      if (!numberDraftConstraintValid(activeDraft, min, max, step, precision)) {
        return undefined;
      }

      const decimal = parseNumberDecimal(activeDraft);
      return decimal === null ? undefined : numberDecimalToNumber(decimal);
    }

    return committed;
  });
  const canIncrement = $derived(
    stepFrom === undefined
      ? false
      : stepNumberValue(stepFrom, 1, min, max, step, precision) !== null,
  );
  const canDecrement = $derived(
    stepFrom === undefined
      ? false
      : stepNumberValue(stepFrom, -1, min, max, step, precision) !== null,
  );

  function emitValidationChange(): void {
    onValidationChange?.({
      status: internalValidationStatus,
      valid: internalValidationStatus === "valid" || internalValidationStatus === "idle",
      message: validationMessage,
    });
  }

  async function runValidation(nextValue: number | null): Promise<void> {
    // Invalidate any in-flight validation before the idle early-return so a
    // clear/replacement cannot be overwritten by a stale resolve.
    const validationKey = ++activeValidationKey;

    if (!validate || nextValue === null) {
      internalValidationStatus = "idle";
      validationMessage = "";
      emitValidationChange();
      return;
    }

    const validationValue = formatNumberCommitted(nextValue, precision);
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

  function applyDraft(next: string | null): void {
    if (draftControlled) {
      draftValue = next;
    } else {
      localDraft = next;
    }

    onDraftValueChange?.(next);
  }

  function applyValue(next: number | null): void {
    lastEmittedValue = next;

    if (!isControlled) {
      uncontrolledValue = next;
    } else {
      value = next;
      lastControlledValue = next;
    }

    onValueChange?.(next);
    void runValidation(next);
  }

  function applyEffects(effects: NumberInputEffect[]): void {
    for (const effect of effects) {
      switch (effect.type) {
        case "emitDraftValueChange":
          applyDraft(effect.draft);
          break;
        case "emitValueChange":
          applyValue(effect.value);
          break;
        case "emitCommit":
          onCommit?.(effect.value);
          break;
      }
    }
  }

  function dispatch(
    event:
      | { type: "RAW_EDIT"; text: string }
      | { type: "CLEAR" }
      | { type: "ENTER" }
      | { type: "BLUR" }
      | { type: "ESCAPE" }
      | { type: "STEP"; direction: 1 | -1 }
      | { type: "HOME" }
      | { type: "END" },
  ): void {
    const result = numberInputTransition(machineContext, event);

    if (!draftControlled) {
      localDraft = result.context.draft;
    }

    applyEffects(result.effects);
  }

  function handleInput(event: Event): void {
    dispatch({ type: "RAW_EDIT", text: (event.currentTarget as HTMLInputElement).value });
  }

  function handleBlur(event: FocusEvent): void {
    dispatch({ type: "BLUR" });
    onBlur?.(event);
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
      role="spinbutton"
      value={displayValue}
      {placeholder}
      disabled={disabled}
      readonly={readOnly}
      required={required}
      aria-label={ariaLabel ?? undefined}
      aria-describedby={describedBy ?? undefined}
      aria-invalid={ariaInvalid}
      aria-busy={ariaBusy}
      aria-valuenow={ariaValueNow}
      aria-valuemin={min ?? undefined}
      aria-valuemax={max ?? undefined}
      oninput={handleInput}
      onfocus={(event) => onFocus?.(event)}
      onblur={handleBlur}
      onkeydown={(event) => {
        if (event.key === "Enter") {
          event.preventDefault();
          dispatch({ type: "ENTER" });
          return;
        }

        if (event.key === "Escape") {
          event.preventDefault();
          dispatch({ type: "ESCAPE" });
          return;
        }

        if (event.key === "ArrowUp") {
          event.preventDefault();
          dispatch({ type: "STEP", direction: 1 });
          return;
        }

        if (event.key === "ArrowDown") {
          event.preventDefault();
          dispatch({ type: "STEP", direction: -1 });
          return;
        }

        if (event.key === "Home") {
          event.preventDefault();
          dispatch({ type: "HOME" });
          return;
        }

        if (event.key === "End") {
          event.preventDefault();
          dispatch({ type: "END" });
        }
      }}
    />

    {#if showSteppers}
      <div class="poodle-number-input__steppers">
        <button
          type="button"
          aria-label="Increment"
          disabled={disabled || readOnly || !canIncrement}
          onclick={() => dispatch({ type: "STEP", direction: 1 })}
        >
          <Icon name="plus" />
        </button>
        <button
          type="button"
          aria-label="Decrement"
          disabled={disabled || readOnly || !canDecrement}
          onclick={() => dispatch({ type: "STEP", direction: -1 })}
        >
          <Icon name="minus" />
        </button>
      </div>
    {/if}
  </div>

  {#if suffix}
    <span class="poodle-number-input__suffix">{suffix}</span>
  {/if}
</div>
