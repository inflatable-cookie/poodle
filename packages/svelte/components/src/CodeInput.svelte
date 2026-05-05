<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { HTMLInputAttributes } from "svelte/elements";

  import Field from "./Field.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole, ValidationState } from "./types";

  export let id: string | null = null;
  export let value: string | null = null;
  export let defaultValue = "";
  export let name = "code";
  export let label = "Authenticator code";
  export let hint: string | null = null;
  export let error: string | null = null;
  export let disabled = false;
  export let length = 6;
  export let mask = false;
  export let ariaLabel: string | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;
  export let autocomplete: HTMLInputAttributes["autocomplete"] = "one-time-code";
  export let validationState: ValidationState = "none";

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
    complete: { value: string };
  }>();

  const uiPresentation = getUiPresentation();

  let uncontrolledValue = defaultValue;
  let inputRef: HTMLInputElement | null = null;
  let caretIndex = 0;
  let hasFocus = false;

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: effectiveId = id ?? `code-${name}`;
  $: isControlled = value !== null;
  $: currentValue = sanitizeValue(isControlled ? value ?? "" : uncontrolledValue);
  $: digits = Array.from({ length }, (_, index) => currentValue[index] ?? "");
  $: effectiveValidationState = error ? "invalid" : validationState;
  $: activeCaretIndex = Math.min(caretIndex, Math.max(length - 1, 0));
  $: slotBorderColor =
    effectiveValidationState === "invalid"
      ? "var(--poodle-color-status-danger)"
      : "var(--poodle-color-border-default)";
  $: slotFocusColor =
    effectiveValidationState === "invalid"
      ? "var(--poodle-color-status-danger)"
      : "var(--poodle-color-accent-border)";
  $: slotFocusRing =
    effectiveValidationState === "invalid"
      ? "color-mix(in srgb, var(--poodle-color-status-danger) 24%, transparent)"
      : "color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent)";

  function sanitizeValue(input: string): string {
    return input.replace(/\D/g, "").slice(0, length);
  }

  function updateValue(nextRawValue: string): void {
    const nextValue = sanitizeValue(nextRawValue);

    if (!isControlled) {
      uncontrolledValue = nextValue;
    }

    dispatch("valueChange", { value: nextValue });

    if (nextValue.length === length) {
      dispatch("complete", { value: nextValue });
    }
  }

  function syncCaret(): void {
    if (!inputRef) return;
    const selectionStart = inputRef.selectionStart ?? currentValue.length;
    caretIndex = Math.min(selectionStart, Math.max(length - 1, 0));
  }

  function handleInput(event: Event): void {
    const input = event.currentTarget as HTMLInputElement;
    updateValue(input.value);
    syncCaret();
  }

  function handleKeydown(): void {
    requestAnimationFrame(syncCaret);
  }

  function handleFocus(): void {
    hasFocus = true;
    caretIndex = Math.min(currentValue.length, Math.max(length - 1, 0));
  }

  function handleBlur(): void {
    hasFocus = false;
  }

  function handleSlotClick(index: number): void {
    inputRef?.focus();

    if (!inputRef) return;

    const nextPosition = Math.min(index, currentValue.length);
    inputRef.setSelectionRange(nextPosition, nextPosition);
    caretIndex = Math.min(nextPosition, Math.max(length - 1, 0));
  }

  function displayDigit(digit: string): string {
    if (!digit) return "";
    return mask ? "\u2022" : digit;
  }
</script>

<Field
  id={effectiveId}
  {label}
  hint={hint}
  error={error}
  validationState={effectiveValidationState}
>
  <svelte:fragment let:describedBy>
    <input type="hidden" {name} value={currentValue} />

    <div
      class="poodle-code-input"
      class:poodle-code-input--disabled={disabled}
      class:poodle-code-input--focused={hasFocus}
      data-size={resolvedSize}
      data-density={resolvedDensity}
      style={`--code-slot-border:${slotBorderColor}; --code-slot-focus:${slotFocusColor}; --code-slot-focus-ring:${slotFocusRing};`}
      role="group"
      aria-label={ariaLabel ?? label}
    >
      <input
        bind:this={inputRef}
        id={effectiveId}
        class="poodle-code-input__control"
        type="text"
        inputmode="numeric"
        pattern="[0-9]*"
        maxlength={length}
        {disabled}
        value={currentValue}
        {autocomplete}
        aria-label={ariaLabel ?? label}
        aria-describedby={describedBy ?? undefined}
        aria-invalid={effectiveValidationState === "invalid" ? "true" : undefined}
        oninput={handleInput}
        onkeydown={handleKeydown}
        onkeyup={handleKeydown}
        onfocus={handleFocus}
        onblur={handleBlur}
      />

      {#each digits as digit, index}
        <button
          type="button"
          class="poodle-code-input__slot"
          class:poodle-code-input__slot--active={hasFocus && index === activeCaretIndex}
          class:poodle-code-input__slot--filled={digit.length > 0}
          class:poodle-code-input__slot--split-after={length === 6 && index === 2}
          tabindex={-1}
          onclick={() => handleSlotClick(index)}
          aria-hidden="true"
        >
          {displayDigit(digit)}
        </button>
      {/each}
    </div>
  </svelte:fragment>
</Field>

<style>
  .poodle-code-input {
    position: relative;
    display: inline-flex;
    gap: var(--poodle-space-inline-sm);
    width: max-content;
  }

  .poodle-code-input__control {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    opacity: 0;
    cursor: text;
    z-index: 1;
  }

  .poodle-code-input__slot {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2.25rem;
    height: 2.25rem;
    padding: 0;
    border: 0.0625rem solid var(--code-slot-border, var(--poodle-color-border-default));
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-code-family);
    font-size: 1rem;
    font-weight: 600;
    line-height: 1;
    text-align: center;
    cursor: text;
    transition: border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard), box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
    z-index: 0;
  }

  .poodle-code-input__slot--active {
    border-color: var(--code-slot-focus, var(--poodle-color-accent-border));
    box-shadow: 0 0 0 var(--poodle-border-width-focus) var(--code-slot-focus-ring, color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent));
  }

  .poodle-code-input__slot--split-after {
    margin-right: var(--poodle-space-inline-md);
  }

  .poodle-code-input--disabled .poodle-code-input__slot {
    opacity: var(--poodle-state-opacity-disabled);
    cursor: not-allowed;
  }

  .poodle-code-input[data-size="xs"] .poodle-code-input__slot {
    width: 1.5rem;
    height: 1.5rem;
    font-size: 0.8125rem;
  }

  .poodle-code-input[data-size="sm"] .poodle-code-input__slot {
    width: 1.75rem;
    height: 1.75rem;
    font-size: 0.875rem;
  }

  .poodle-code-input[data-size="lg"] .poodle-code-input__slot {
    width: 2.75rem;
    height: 2.75rem;
    font-size: 1.125rem;
  }

  .poodle-code-input[data-size="xl"] .poodle-code-input__slot {
    width: 3.25rem;
    height: 3.25rem;
    font-size: 1.25rem;
  }

  .poodle-code-input[data-density="compact"] {
    gap: 0.25rem;
  }

  .poodle-code-input[data-density="comfortable"] {
    gap: var(--poodle-space-inline-md);
  }

  @media (max-width: 30rem) {
    .poodle-code-input__slot {
      width: 2rem;
      height: 2rem;
      font-size: 0.9375rem;
    }

    .poodle-code-input__slot--split-after {
      margin-right: var(--poodle-space-inline-sm);
    }
  }
</style>
