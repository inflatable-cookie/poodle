<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/code-input.css";
  import {
    clampCodePosition,
    codeGroupEndIndices,
    codeInsertReplacement,
    codeSelectionRange,
    codeSlotSelection,
    sanitizeCodeValue,
  } from "@inflatable-cookie/poodle-core";
  import type { HTMLInputAttributes } from "svelte/elements";

  import { default as Field } from "./Field.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole, ValidationState } from "./types";

  interface Props {
    id?: string | null;
    value?: string | null | undefined;
    defaultValue?: string;
    name?: string;
    label?: string;
    hint?: string | null;
    error?: string | null;
    disabled?: boolean;
    length?: number;
    groups?: readonly number[] | null;
    mask?: boolean;
    numbersOnly?: boolean;
    ariaLabel?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    autocomplete?: HTMLInputAttributes["autocomplete"];
    validationState?: ValidationState;
    onValueChange?: ((value: string) => void) | undefined;
    onComplete?: ((value: string) => void) | undefined;
  }

  const uiPresentation = getUiPresentation();

  let {
    id = null,
    value = $bindable<string | null | undefined>(undefined),
    defaultValue = "",
    name = "code",
    label = "Authenticator code",
    hint = null,
    error = null,
    disabled = false,
    length = 6,
    groups = null,
    mask = false,
    numbersOnly = true,
    ariaLabel = null,
    size = null,
    sizeRole = "control",
    density = null,
    autocomplete = "one-time-code",
    validationState = "none",
    onValueChange = undefined,
    onComplete = undefined,
  }: Props = $props();

  let seededDefaultValue = $state(false);
  let uncontrolledValue = $state("");
  let inputRef: HTMLInputElement | null = null;
  let caretIndex = $state(0);
  let hasFocus = $state(false);
  let pendingSelection: { start: number; end: number } | null = $state(null);

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const effectiveId = $derived(id ?? `code-${name}`);
  const isControlled = $derived(value !== undefined);
  const currentValue = $derived(sanitizeValue(isControlled ? value ?? "" : uncontrolledValue));
  const digits = $derived(Array.from({ length }, (_, index) => currentValue[index] ?? ""));
  const groupEndIndices = $derived(codeGroupEndIndices(length, groups));
  const effectiveValidationState = $derived(error ? "invalid" : validationState);
  const activeCaretIndex = $derived(Math.min(caretIndex, Math.max(length - 1, 0)));
  const slotBorderColor = $derived(
    effectiveValidationState === "invalid"
      ? "var(--poodle-color-status-danger)"
      : "var(--poodle-color-border-default)"
  );
  const slotFocusColor = $derived(
    effectiveValidationState === "invalid"
      ? "var(--poodle-color-status-danger)"
      : "var(--poodle-color-accent-border)"
  );
  const slotFocusRing = $derived(
    effectiveValidationState === "invalid"
      ? "color-mix(in srgb, var(--poodle-color-status-danger) 24%, transparent)"
      : "color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent)"
  );

  $effect(() => {
    if (!seededDefaultValue && value === undefined) {
      uncontrolledValue = sanitizeValue(defaultValue);
      seededDefaultValue = true;
    }
  });

  function sanitizeValue(input: string): string {
    return sanitizeCodeValue(input, length, numbersOnly);
  }

  function updateValue(nextRawValue: string): void {
    const nextValue = sanitizeValue(nextRawValue);

    if (!isControlled) {
      uncontrolledValue = nextValue;
    } else {
      value = nextValue;
    }

    onValueChange?.(nextValue);

    if (nextValue.length === length) {
      onComplete?.(nextValue);
    }
  }

  function syncCaret(): void {
    if (!inputRef) return;
    const selectionStart = inputRef.selectionStart ?? currentValue.length;
    caretIndex = Math.min(selectionStart, Math.max(length - 1, 0));
  }

  function applyPendingSelection(): void {
    if (!inputRef || !pendingSelection) return;
    inputRef.setSelectionRange(pendingSelection.start, pendingSelection.end);
    caretIndex = Math.min(pendingSelection.start, Math.max(length - 1, 0));
    pendingSelection = null;
  }

  function setActivePosition(index: number, selectFilled: boolean): void {
    if (!inputRef) return;

    const nextPosition = clampCodePosition(index, currentValue.length, length);
    const range = codeSelectionRange(nextPosition, currentValue.length, selectFilled);

    inputRef.setSelectionRange(range.start, range.end);
    caretIndex = nextPosition;
  }

  function handleInput(event: Event): void {
    const input = event.currentTarget as HTMLInputElement;
    updateValue(input.value);
    requestAnimationFrame(() => {
      applyPendingSelection();
      syncCaret();
    });
  }

  function handleKeydown(event: KeyboardEvent): void {
    if (event.key === "ArrowLeft" || event.key === "ArrowRight") {
      event.preventDefault();
      if (!inputRef) return;
      const nextPosition = event.key === "ArrowLeft" ? caretIndex - 1 : caretIndex + 1;
      setActivePosition(nextPosition, true);
      return;
    }

    requestAnimationFrame(syncCaret);
  }

  function handleFocus(): void {
    hasFocus = true;
    requestAnimationFrame(() => {
      const hadPendingSelection = pendingSelection !== null;
      applyPendingSelection();
      if (hadPendingSelection) return;
      setActivePosition(Math.min(currentValue.length, Math.max(length - 1, 0)), false);
    });
  }

  function handleBlur(): void {
    hasFocus = false;
  }

  function handleSlotClick(index: number): void {
    inputRef?.focus();

    if (!inputRef) return;

    pendingSelection = codeSlotSelection(index, currentValue.length);
    requestAnimationFrame(() => {
      applyPendingSelection();
    });
  }

  function handleBeforeInput(event: InputEvent): void {
    if (disabled) {
      return;
    }

    if (!inputRef || !event.inputType.startsWith("insert")) {
      if (
        numbersOnly &&
        event.data &&
        !/^\d+$/.test(event.data) &&
        event.inputType !== "deleteContentBackward" &&
        event.inputType !== "deleteContentForward"
      ) {
        event.preventDefault();
      }
      return;
    }

    const selectionStart = inputRef.selectionStart ?? currentValue.length;
    const selectionEnd = inputRef.selectionEnd ?? selectionStart;
    const replacement = codeInsertReplacement(
      currentValue,
      event.data ?? "",
      selectionStart,
      selectionEnd,
      length,
      numbersOnly,
    );

    event.preventDefault();

    if (!replacement) {
      return;
    }

    updateValue(replacement.value);
    requestAnimationFrame(() => {
      setActivePosition(replacement.caret, true);
    });
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
  {#snippet control({ describedBy })}
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
        inputmode={numbersOnly ? "numeric" : "text"}
        pattern={numbersOnly ? "[0-9]*" : undefined}
        maxlength={length}
        {disabled}
        value={currentValue}
        {autocomplete}
        aria-label={ariaLabel ?? label}
        aria-describedby={describedBy ?? undefined}
        aria-invalid={effectiveValidationState === "invalid" ? "true" : undefined}
        onbeforeinput={handleBeforeInput}
        oninput={handleInput}
        onkeydown={handleKeydown}
        onfocus={handleFocus}
        onblur={handleBlur}
      />

      {#each digits as digit, index}
        <button
          type="button"
          class="poodle-code-input__slot"
          class:poodle-code-input__slot--active={hasFocus && index === activeCaretIndex}
          class:poodle-code-input__slot--filled={digit.length > 0}
          class:poodle-code-input__slot--group-end={groupEndIndices.includes(index)}
          tabindex={-1}
          onmousedown={(event) => event.preventDefault()}
          onclick={() => handleSlotClick(index)}
          aria-hidden="true"
        >
          {displayDigit(digit)}
        </button>
      {/each}
    </div>
  {/snippet}
</Field>
