<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;
  export let value: string | null = null;
  export let defaultValue = "";
  export let length = 6;
  export let disabled = false;
  export let ariaLabel: string | null = null;
  export let mask = false;

  const uiPresentation = getUiPresentation();

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
    complete: { value: string };
  }>();

  let uncontrolledValue = defaultValue;
  let inputs: Array<HTMLInputElement | null> = [];

  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);
  $: resolvedDensity = density ?? uiPresentation?.density ?? "default";
  $: controlled = value !== null;
  $: currentValue = controlled ? value ?? "" : uncontrolledValue;
  $: digits = Array.from({ length }, (_, index) => currentValue[index] ?? "");

  function updateValue(index: number, nextCharacter: string): void {
    const nextDigits = [...digits];
    nextDigits[index] = nextCharacter.slice(-1);
    const nextValue = nextDigits.join("").slice(0, length);

    if (!controlled) {
      uncontrolledValue = nextValue;
    }

    dispatch("valueChange", { value: nextValue });

    if (nextCharacter && index < length - 1) {
      inputs[index + 1]?.focus();
    }

    if (nextValue.length === length && !nextDigits.includes("")) {
      dispatch("complete", { value: nextValue });
    }
  }
</script>

<div class="pin-input" role="group" aria-label={ariaLabel ?? undefined} data-size={resolvedSize} data-density={resolvedDensity}>
  {#each digits as digit, index}
    <input
      bind:this={inputs[index]}
      class="pin-input__cell"
      type={mask ? "password" : "text"}
      inputmode="numeric"
      pattern="[0-9]*"
      maxlength="1"
      value={digit}
      disabled={disabled}
      aria-label={`Digit ${index + 1}`}
      on:input={(event) => updateValue(index, (event.currentTarget as HTMLInputElement).value)}
      on:keydown={(event) => {
        if (event.key === "Backspace" && !digit && index > 0) {
          inputs[index - 1]?.focus();
        }

        if (event.key === "ArrowLeft" && index > 0) {
          event.preventDefault();
          inputs[index - 1]?.focus();
        }

        if (event.key === "ArrowRight" && index < length - 1) {
          event.preventDefault();
          inputs[index + 1]?.focus();
        }
      }}
    />
  {/each}
</div>

<style>
  .pin-input {
    display: inline-flex;
    gap: var(--poodle-space-inline-sm);
  }

  .pin-input__cell {
    width: var(--poodle-size-control-height);
    height: calc(var(--poodle-size-control-height) + 0.25rem);
    padding: 0;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-code-family);
    font-size: 1rem;
    line-height: 1;
    text-align: center;
  }

  .pin-input__cell:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .pin-input__cell:disabled {
    opacity: var(--poodle-state-opacity-disabled);
  }

  /* Size variants */
  .pin-input[data-size="xs"] .pin-input__cell { width: calc(var(--poodle-size-control-height) - 0.5rem); height: calc(var(--poodle-size-control-height) - 0.25rem); font-size: 0.8125rem; }
  .pin-input[data-size="sm"] .pin-input__cell { width: calc(var(--poodle-size-control-height) - 0.25rem); height: var(--poodle-size-control-height); font-size: 0.875rem; }
  .pin-input[data-size="lg"] .pin-input__cell { width: calc(var(--poodle-size-control-height) + 0.25rem); height: calc(var(--poodle-size-control-height) + 0.5rem); font-size: 1.125rem; }
  .pin-input[data-size="xl"] .pin-input__cell { width: calc(var(--poodle-size-control-height) + 0.5rem); height: calc(var(--poodle-size-control-height) + 0.75rem); font-size: 1.25rem; }

  /* Density variants */
  .pin-input[data-density="compact"] { gap: var(--poodle-space-inline-xs); }
  .pin-input[data-density="comfortable"] { gap: var(--poodle-space-inline-md); }
</style>
