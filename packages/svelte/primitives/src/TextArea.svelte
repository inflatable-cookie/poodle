<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import type { ValidationState } from "./types";

  export let id: string;
  export let value: string | null = null;
  export let defaultValue = "";
  export let placeholder: string | null = null;
  export let rows = 4;
  export let name: string | undefined = undefined;
  export let isDisabled = false;
  export let isReadOnly = false;
  export let validationState: ValidationState = "none";
  export let ariaLabel: string | null = null;
  export let describedBy: string | null = null;

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
  $: ariaInvalid = validationState === "invalid" ? true : undefined;
  $: ariaBusy = validationState === "pending" ? true : undefined;

  function handleInput(event: Event): void {
    const nextValue = (event.currentTarget as HTMLTextAreaElement).value;

    if (!isControlled) {
      uncontrolledValue = nextValue;
    }

    dispatch("valueChange", { value: nextValue });
  }

  function handleKeydown(event: KeyboardEvent): void {
    if ((event.metaKey || event.ctrlKey) && event.key === "Enter") {
      dispatch("submit", { value: currentValue });
    }

    if (event.key === "Escape") {
      dispatch("cancel");
    }
  }
</script>

<div class="text-area" data-validation-state={validationState}>
  <textarea
    {id}
    {name}
    class="text-area__control"
    value={currentValue}
    {placeholder}
    {rows}
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
  ></textarea>
</div>

<style>
  .text-area {
    --flint-text-area-radius: var(--flint-treatment-interactive-subtle-radius, var(--flint-radius-control));
    --flint-text-area-fill: var(
      --flint-treatment-interactive-subtle-fill,
      var(--flint-color-background-surface)
    );
    --flint-text-area-fill-focus: var(
      --flint-treatment-interactive-subtle-fill-focus,
      var(--flint-text-area-fill)
    );
    --flint-text-area-border: var(
      --flint-treatment-interactive-subtle-border,
      var(--flint-color-border-default)
    );
    --flint-text-area-border-focus: var(
      --flint-treatment-interactive-subtle-border-focus,
      var(--flint-color-accent-focusRing)
    );
    --flint-text-area-shadow: var(--flint-treatment-interactive-subtle-shadow, none);
    --flint-text-area-shadow-focus: var(
      --flint-treatment-interactive-subtle-shadow-focus,
      0 0 0 var(--flint-border-width-focus)
        color-mix(in srgb, var(--flint-color-accent-focusRing) 28%, transparent)
    );
    display: grid;
    min-height: 0;
    border: 0.0625rem solid var(--flint-text-area-border);
    border-radius: var(--flint-text-area-radius);
    background: var(--flint-text-area-fill);
    color: var(--flint-color-text-primary);
    box-shadow: var(--flint-text-area-shadow);
    transition:
      border-color var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard),
      box-shadow var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard),
      background var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard);
  }

  .text-area:focus-within {
    border-color: var(--flint-text-area-border-focus);
    background: var(--flint-text-area-fill-focus);
    box-shadow: var(--flint-text-area-shadow-focus);
  }

  .text-area[data-validation-state="invalid"] {
    border-color: var(--flint-color-status-danger);
  }

  .text-area[data-validation-state="valid"] {
    border-color: var(--flint-color-status-success);
  }

  .text-area[data-validation-state="pending"] {
    border-color: var(--flint-color-accent-base);
  }

  .text-area:has(.text-area__control:disabled) {
    opacity: var(--flint-state-opacity-disabled);
  }

  .text-area__control {
    min-width: 0;
    width: 100%;
    min-height: calc(1lh * 4);
    padding: var(--flint-space-control-y) var(--flint-space-control-x);
    border: 0;
    resize: vertical;
    background: transparent;
    color: inherit;
    font-family: var(--flint-typography-body-family);
    font-size: var(--flint-typography-body-size);
    line-height: var(--flint-typography-body-lineHeight);
    outline: 0;
  }

  .text-area__control::placeholder {
    color: var(--flint-color-text-secondary);
  }
</style>
