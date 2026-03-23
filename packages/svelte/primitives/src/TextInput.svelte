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
  export let inputMode:
    | "none"
    | "search"
    | "text"
    | "tel"
    | "url"
    | "email"
    | "numeric"
    | "decimal"
    | null = null;
  export let type: HTMLInputElement["type"] = "text";
  export let prefix: string | null = null;
  export let suffix: string | null = null;
  export let maxLength: number | null = null;
  export let showCharCount = false;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
    submit: { value: string };
    cancel: void;
    keydown: KeyboardEvent;
    focus: FocusEvent;
    blur: FocusEvent;
  }>();

  let uncontrolledValue = defaultValue;

  $: isControlled = value !== null;
  $: currentValue = isControlled ? value ?? "" : uncontrolledValue;
  $: ariaInvalid = validationState === "invalid" ? true : undefined;
  $: ariaBusy = validationState === "pending" ? true : undefined;
  $: charCount = currentValue.length;
  $: charCountText = maxLength ? `${charCount}/${maxLength}` : `${charCount}`;
  $: isOverLimit = maxLength !== null && charCount > maxLength;

  function handleInput(event: Event): void {
    const nextValue = (event.currentTarget as HTMLInputElement).value;

    if (!isControlled) {
      uncontrolledValue = nextValue;
    }

    dispatch("valueChange", { value: nextValue });
  }

  function handleKeydown(event: KeyboardEvent): void {
    dispatch("keydown", event);

    if (event.key === "Enter") {
      dispatch("submit", { value: currentValue });
    }

    if (event.key === "Escape") {
      dispatch("cancel");
    }
  }
</script>

<div class="text-input" data-validation-state={validationState}>
  {#if prefix}
    <span class="text-input__affix text-input__affix--prefix">{prefix}</span>
  {/if}

  {#if $$slots.leading}
    <span class="text-input__affordance text-input__affordance--leading">
      <slot name="leading" />
    </span>
  {/if}

  <input
    {id}
    {name}
    {type}
    inputmode={inputMode ?? undefined}
    class="text-input__control"
    value={currentValue}
    {placeholder}
    maxlength={maxLength ?? undefined}
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

  {#if suffix}
    <span class="text-input__affix text-input__affix--suffix">{suffix}</span>
  {/if}

  {#if showCharCount}
    <span class="text-input__char-count" class:text-input__char-count--over={isOverLimit} aria-live="polite">
      {charCountText}
    </span>
  {/if}
</div>

<style>
  .text-input {
    --poodle-text-input-radius: var(--poodle-treatment-interactive-subtle-radius, var(--poodle-radius-control));
    --poodle-text-input-fill: var(
      --poodle-treatment-interactive-subtle-fill,
      var(--poodle-color-background-surface)
    );
    --poodle-text-input-fill-focus: var(
      --poodle-treatment-interactive-subtle-fill-focus,
      var(--poodle-text-input-fill)
    );
    --poodle-text-input-border: var(
      --poodle-treatment-interactive-subtle-border,
      var(--poodle-color-border-default)
    );
    --poodle-text-input-border-focus: var(
      --poodle-treatment-interactive-subtle-border-focus,
      var(--poodle-color-accent-focusRing)
    );
    --poodle-text-input-shadow: var(--poodle-treatment-interactive-subtle-shadow, none);
    --poodle-text-input-shadow-focus: var(
      --poodle-treatment-interactive-subtle-shadow-focus,
      0 0 0 var(--poodle-border-width-focus)
        color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent)
    );
    display: flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    border: 0.0625rem solid var(--poodle-text-input-border);
    border-radius: var(--poodle-text-input-radius);
    background: var(--poodle-text-input-fill);
    color: var(--poodle-color-text-primary);
    box-shadow: var(--poodle-text-input-shadow);
    transition:
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .text-input:focus-within {
    border-color: var(--poodle-text-input-border-focus);
    background: var(--poodle-text-input-fill-focus);
    box-shadow: var(--poodle-text-input-shadow-focus);
  }

  .text-input[data-validation-state="invalid"] {
    border-color: var(--poodle-color-status-danger);
  }

  .text-input[data-validation-state="valid"] {
    border-color: var(--poodle-color-status-success);
  }

  .text-input[data-validation-state="pending"] {
    border-color: var(--poodle-color-accent-base);
  }

  .text-input:has(.text-input__control:disabled) {
    opacity: var(--poodle-state-opacity-disabled);
  }

  .text-input__control {
    flex: 1;
    min-width: 0;
    width: 100%;
    height: calc(var(--poodle-size-control-height) - (var(--poodle-border-width-default) * 2));
    padding: 0;
    border: 0;
    background: transparent;
    color: inherit;
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
    outline: 0;
  }

  .text-input__control::placeholder {
    color: var(--poodle-color-text-secondary);
  }

  .text-input__affordance {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    color: var(--poodle-color-icon-muted);
    font-family: var(--poodle-typography-code-family);
    font-size: var(--poodle-icon-size-default);
  }

  .text-input__affix {
    display: inline-flex;
    align-items: center;
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    white-space: nowrap;
    user-select: none;
  }

  .text-input__affix--prefix {
    padding-right: var(--poodle-space-inline-sm);
    border-right: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 52%, transparent);
    margin-right: var(--poodle-space-inline-sm);
  }

  .text-input__affix--suffix {
    padding-left: var(--poodle-space-inline-sm);
    border-left: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 52%, transparent);
    margin-left: var(--poodle-space-inline-sm);
  }

  .text-input__char-count {
    display: inline-flex;
    align-items: center;
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.6875rem;
    white-space: nowrap;
  }

  .text-input__char-count--over {
    color: var(--poodle-color-status-danger);
  }
</style>
