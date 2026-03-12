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
  $: ariaInvalid = validationState === "invalid" ? "true" : undefined;
  $: ariaBusy = validationState === "pending" ? "true" : undefined;

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
    --pug-text-area-radius: var(--pug-treatment-interactive-subtle-radius, var(--pug-radius-control));
    --pug-text-area-fill: var(
      --pug-treatment-interactive-subtle-fill,
      var(--pug-color-background-surface)
    );
    --pug-text-area-fill-focus: var(
      --pug-treatment-interactive-subtle-fill-focus,
      var(--pug-text-area-fill)
    );
    --pug-text-area-border: var(
      --pug-treatment-interactive-subtle-border,
      var(--pug-color-border-default)
    );
    --pug-text-area-border-focus: var(
      --pug-treatment-interactive-subtle-border-focus,
      var(--pug-color-accent-focusRing)
    );
    --pug-text-area-shadow: var(--pug-treatment-interactive-subtle-shadow, none);
    --pug-text-area-shadow-focus: var(
      --pug-treatment-interactive-subtle-shadow-focus,
      0 0 0 var(--pug-border-width-focus)
        color-mix(in srgb, var(--pug-color-accent-focusRing) 28%, transparent)
    );
    display: grid;
    min-height: 0;
    padding: var(--pug-space-control-y) var(--pug-space-control-x);
    border: 0.0625rem solid var(--pug-text-area-border);
    border-radius: var(--pug-text-area-radius);
    background: var(--pug-text-area-fill);
    color: var(--pug-color-text-primary);
    box-shadow: var(--pug-text-area-shadow);
    transition:
      border-color var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      box-shadow var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      background var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .text-area:focus-within {
    border-color: var(--pug-text-area-border-focus);
    background: var(--pug-text-area-fill-focus);
    box-shadow: var(--pug-text-area-shadow-focus);
  }

  .text-area[data-validation-state="invalid"] {
    border-color: var(--pug-color-status-danger);
  }

  .text-area[data-validation-state="valid"] {
    border-color: var(--pug-color-status-success);
  }

  .text-area[data-validation-state="pending"] {
    border-color: var(--pug-color-accent-base);
  }

  .text-area:has(.text-area__control:disabled) {
    opacity: var(--pug-state-opacity-disabled);
  }

  .text-area__control {
    min-width: 0;
    width: 100%;
    min-height: calc(1lh * 4);
    padding: 0;
    border: 0;
    resize: vertical;
    background: transparent;
    color: inherit;
    font-family: var(--pug-typography-body-family);
    font-size: var(--pug-typography-body-size);
    line-height: var(--pug-typography-body-lineHeight);
    outline: 0;
  }

  .text-area__control::placeholder {
    color: var(--pug-color-text-secondary);
  }
</style>
