<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import type { ValidationState } from "./types";

  export let id: string;
  export let value: string | null = null;
  export let defaultValue = "";
  export let placeholder: string | null = null;
  export let name: string | undefined = undefined;
  export let isDisabled = false;
  export let isReadOnly = false;
  export let validationState: ValidationState = "none";
  export let ariaLabel: string | null = null;
  export let describedBy: string | null = null;
  export let inputMode: HTMLInputElement["inputMode"] | undefined = undefined;
  export let type: HTMLInputElement["type"] = "text";

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
    submit: { value: string };
    cancel: void;
    focus: FocusEvent;
    blur: FocusEvent;
  }>();

  let uncontrolledValue = defaultValue;

  $: isControlled = value !== null;
  $: currentValue = isControlled ? value ?? "" : uncontrolledValue;
  $: ariaInvalid = validationState === "invalid" ? "true" : undefined;
  $: ariaBusy = validationState === "pending" ? "true" : undefined;

  function handleInput(event: Event): void {
    const nextValue = (event.currentTarget as HTMLInputElement).value;

    if (!isControlled) {
      uncontrolledValue = nextValue;
    }

    dispatch("valueChange", { value: nextValue });
  }

  function handleKeydown(event: KeyboardEvent): void {
    if (event.key === "Enter") {
      dispatch("submit", { value: currentValue });
    }

    if (event.key === "Escape") {
      dispatch("cancel");
    }
  }
</script>

<div class="text-input" data-validation-state={validationState}>
  {#if $$slots.leading}
    <span class="text-input__affordance text-input__affordance--leading">
      <slot name="leading" />
    </span>
  {/if}

  <input
    {id}
    {name}
    {type}
    {inputMode}
    class="text-input__control"
    value={currentValue}
    {placeholder}
    disabled={isDisabled}
    readonly={isReadOnly}
    aria-label={ariaLabel ?? undefined}
    aria-describedby={describedBy ?? undefined}
    aria-invalid={ariaInvalid}
    aria-busy={ariaBusy}
    on:input={handleInput}
    on:keydown={handleKeydown}
    on:focus={(event) => dispatch("focus", event)}
    on:blur={(event) => dispatch("blur", event)}
  />

  {#if $$slots.trailing}
    <span class="text-input__affordance text-input__affordance--trailing">
      <slot name="trailing" />
    </span>
  {/if}
</div>

<style>
  .text-input {
    display: grid;
    grid-template-columns: auto minmax(0, 1fr) auto;
    align-items: center;
    gap: var(--pug-space-inline-sm);
    min-height: var(--pug-size-control-height);
    padding: 0 var(--pug-space-control-x);
    border: 1px solid var(--pug-color-border-default);
    border-radius: var(--pug-radius-control);
    background: var(--pug-color-background-surface);
    color: var(--pug-color-text-primary);
    transition:
      border-color var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      box-shadow var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      background var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .text-input:focus-within {
    border-color: var(--pug-color-accent-focusRing);
    box-shadow: 0 0 0 var(--pug-border-width-focus) color-mix(in srgb, var(--pug-color-accent-focusRing) 28%, transparent);
  }

  .text-input[data-validation-state="invalid"] {
    border-color: var(--pug-color-status-danger);
  }

  .text-input[data-validation-state="valid"] {
    border-color: var(--pug-color-status-success);
  }

  .text-input[data-validation-state="pending"] {
    border-color: var(--pug-color-accent-base);
  }

  .text-input:has(.text-input__control:disabled) {
    opacity: var(--pug-state-opacity-disabled);
  }

  .text-input__control {
    min-width: 0;
    width: 100%;
    height: calc(var(--pug-size-control-height) - (var(--pug-border-width-default) * 2));
    padding: 0;
    border: 0;
    background: transparent;
    color: inherit;
    font-family: var(--pug-typography-body-family);
    font-size: var(--pug-typography-body-size);
    line-height: var(--pug-typography-body-lineHeight);
    outline: 0;
  }

  .text-input__control::placeholder {
    color: var(--pug-color-text-secondary);
  }

  .text-input__affordance {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: var(--pug-color-icon-muted);
    font-family: var(--pug-typography-code-family);
    font-size: var(--pug-icon-size-default);
  }
</style>
