<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Icon from "./Icon.svelte";
  import { clamp, formatNumber, snapToStep } from "./internal";

  import type { ValidationState } from "./types";

  export let id: string;
  export let value: number | null = null;
  export let defaultValue: number | null = null;
  export let placeholder: string | null = null;
  export let min: number | null = null;
  export let max: number | null = null;
  export let step = 1;
  export let precision: number | null = null;
  export let name: string | undefined = undefined;
  export let isDisabled = false;
  export let isReadOnly = false;
  export let validationState: ValidationState = "none";
  export let showSteppers = false;
  export let ariaLabel: string | null = null;
  export let describedBy: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: number | null };
    submit: { value: number | null };
    increment: { value: number | null };
    decrement: { value: number | null };
    focus: FocusEvent;
    blur: FocusEvent;
  }>();

  let uncontrolledValue = defaultValue;
  let draftValue = formatNumber(defaultValue, precision);
  let isEditing = false;

  $: isControlled = value !== null;
  $: currentValue = isControlled ? value : uncontrolledValue;
  $: if (!isEditing) {
    draftValue = formatNumber(currentValue, precision);
  }
  $: ariaInvalid = validationState === "invalid" ? "true" : undefined;
  $: ariaBusy = validationState === "pending" ? "true" : undefined;

  function clampIfNeeded(nextValue: number): number {
    let result = nextValue;

    if (min !== null) {
      result = Math.max(result, min);
    }

    if (max !== null) {
      result = Math.min(result, max);
    }

    return result;
  }

  function commitValue(nextValue: number | null): void {
    if (!isControlled) {
      uncontrolledValue = nextValue;
    }

    draftValue = formatNumber(nextValue, precision);
    dispatch("valueChange", { value: nextValue });
  }

  function handleInput(event: Event): void {
    draftValue = (event.currentTarget as HTMLInputElement).value;

    if (draftValue.trim() === "") {
      commitValue(null);
      return;
    }

    const parsedValue = Number(draftValue);

    if (!Number.isNaN(parsedValue)) {
      commitValue(parsedValue);
    }
  }

  function handleBlur(event: FocusEvent): void {
    isEditing = false;

    if (draftValue.trim() !== "") {
      const parsedValue = Number(draftValue);

      if (!Number.isNaN(parsedValue)) {
        commitValue(clampIfNeeded(snapToStep(parsedValue, min ?? 0, step)));
      }
    }

    dispatch("blur", event);
  }

  function adjust(delta: number, eventName: "increment" | "decrement"): void {
    const baseline = currentValue ?? min ?? 0;
    const nextValue = clampIfNeeded(snapToStep(baseline + delta, min ?? 0, step));
    commitValue(nextValue);
    dispatch(eventName, { value: nextValue });
  }
</script>

<div class="number-entry" data-validation-state={validationState}>
  <input
    {id}
    {name}
    class="number-entry__control"
    type="text"
    inputmode="decimal"
    value={draftValue}
    {placeholder}
    disabled={isDisabled}
    readonly={isReadOnly}
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
        dispatch("submit", { value: currentValue });
      }

      if (event.key === "ArrowUp" && !isReadOnly) {
        event.preventDefault();
        adjust(step, "increment");
      }

      if (event.key === "ArrowDown" && !isReadOnly) {
        event.preventDefault();
        adjust(-step, "decrement");
      }
    }}
  />

  {#if showSteppers}
    <div class="number-entry__steppers">
      <button type="button" disabled={isDisabled || isReadOnly} on:click={() => adjust(step, "increment")}>
        <Icon name="plus" size="sm" />
      </button>
      <button type="button" disabled={isDisabled || isReadOnly} on:click={() => adjust(-step, "decrement")}>
        <Icon name="minus" size="sm" />
      </button>
    </div>
  {/if}
</div>

<style>
  .number-entry {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: stretch;
    height: var(--pug-size-control-height);
    border: 0.0625rem solid var(--pug-color-border-default);
    border-radius: var(--pug-radius-control);
    background: var(--pug-color-background-surface);
    overflow: hidden;
  }

  .number-entry[data-validation-state="invalid"] {
    border-color: var(--pug-color-status-danger);
  }

  .number-entry[data-validation-state="valid"] {
    border-color: var(--pug-color-status-success);
  }

  .number-entry[data-validation-state="pending"] {
    border-color: var(--pug-color-accent-base);
  }

  .number-entry:focus-within {
    box-shadow:
      0 0 0 var(--pug-border-width-focus)
      color-mix(in srgb, var(--pug-color-accent-focusRing) 28%, transparent);
  }

  .number-entry__control {
    min-width: 0;
    padding: 0 var(--pug-space-control-x);
    border: 0;
    background: transparent;
    color: var(--pug-color-text-primary);
    font-family: var(--pug-typography-body-family);
    font-size: var(--pug-typography-body-size);
    line-height: var(--pug-typography-body-lineHeight);
    outline: 0;
  }

  .number-entry__steppers {
    display: grid;
    grid-template-rows: 1fr 1fr;
    gap: 0;
    padding: 0.0625rem;
  }

  .number-entry__steppers button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.25rem;
    min-height: 0;
    border: 0;
    border-radius: calc(var(--pug-radius-control) - 0.125rem);
    background: color-mix(in srgb, var(--pug-color-background-elevated) 88%, transparent);
    color: var(--pug-color-text-primary);
    cursor: pointer;
    font-size: 0;
    padding: 0;
    overflow: hidden;
  }

  .number-entry__steppers button :global(.pug-icon) {
    transform: scale(0.625);
  }

  .number-entry__steppers button:disabled {
    cursor: not-allowed;
    opacity: var(--pug-state-opacity-disabled);
  }
</style>
