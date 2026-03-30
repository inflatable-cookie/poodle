<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole, ValidationState } from "./types";

  export let id: string;
  export let value: string | null = null;
  export let defaultValue = "";
  export let placeholder: string | null = null;
  export let rows = 4;
  export let name: string | undefined = undefined;
  export let disabled = false;
  export let readOnly = false;
  export let validationState: ValidationState = "none";
  export let ariaLabel: string | null = null;
  export let describedBy: string | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
    submit: { value: string };
    cancel: void;
    focus: FocusEvent;
    blur: FocusEvent;
  }>();

  const uiPresentation = getUiPresentation();
  let uncontrolledValue = defaultValue;

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
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

<div class="text-area" data-validation-state={validationState} data-size={resolvedSize} data-density={resolvedDensity}>
  <textarea
    {id}
    {name}
    class="text-area__control"
    value={currentValue}
    {placeholder}
    {rows}
    disabled={disabled}
    readonly={readOnly}
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
    --poodle-text-area-radius: var(--poodle-treatment-interactive-subtle-radius, var(--poodle-radius-control));
    --poodle-text-area-fill: var(
      --poodle-treatment-interactive-subtle-fill,
      var(--poodle-color-background-surface)
    );
    --poodle-text-area-fill-focus: var(
      --poodle-treatment-interactive-subtle-fill-focus,
      var(--poodle-text-area-fill)
    );
    --poodle-text-area-border: var(
      --poodle-treatment-interactive-subtle-border,
      var(--poodle-color-border-default)
    );
    --poodle-text-area-border-focus: var(
      --poodle-treatment-interactive-subtle-border-focus,
      var(--poodle-color-accent-focusRing)
    );
    --poodle-text-area-shadow: var(--poodle-treatment-interactive-subtle-shadow, none);
    --poodle-text-area-shadow-focus: var(
      --poodle-treatment-interactive-subtle-shadow-focus,
      0 0 0 var(--poodle-border-width-focus)
        color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent)
    );
    display: grid;
    min-height: 0;
    border: 0.0625rem solid var(--poodle-text-area-border);
    border-radius: var(--poodle-text-area-radius);
    background: var(--poodle-text-area-fill);
    color: var(--poodle-color-text-primary);
    box-shadow: var(--poodle-text-area-shadow);
    transition:
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .text-area:focus-within {
    border-color: var(--poodle-text-area-border-focus);
    background: var(--poodle-text-area-fill-focus);
    box-shadow: var(--poodle-text-area-shadow-focus);
  }

  .text-area[data-validation-state="invalid"] {
    border-color: var(--poodle-color-status-danger);
  }

  .text-area[data-validation-state="valid"] {
    border-color: var(--poodle-color-status-success);
  }

  .text-area[data-validation-state="pending"] {
    border-color: var(--poodle-color-accent-base);
  }

  .text-area:has(.text-area__control:disabled) {
    opacity: var(--poodle-state-opacity-disabled);
  }

  .text-area__control {
    min-width: 0;
    width: 100%;
    min-height: calc(1lh * 4);
    padding: var(--poodle-space-control-y) var(--poodle-space-control-x);
    border: 0;
    resize: vertical;
    background: transparent;
    color: inherit;
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
    outline: 0;
  }

  .text-area__control::placeholder {
    color: var(--poodle-color-text-secondary);
  }

  /* Density variants */
  .text-area[data-density="compact"] .text-area__control {
    padding: calc(var(--poodle-space-control-y, 0.375rem) - 0.125rem) calc(var(--poodle-space-control-x) - 0.125rem);
  }

  .text-area[data-density="comfortable"] .text-area__control {
    padding: calc(var(--poodle-space-control-y, 0.375rem) + 0.125rem) calc(var(--poodle-space-control-x) + 0.125rem);
  }

  /* Size variants */
  .text-area[data-size="xs"] .text-area__control {
    padding: calc(var(--poodle-space-control-y) - 0.125rem) calc(var(--poodle-space-control-x) - 0.125rem);
    font-size: 0.75rem;
  }

  .text-area[data-size="sm"] .text-area__control {
    padding: calc(var(--poodle-space-control-y) - 0.0625rem) calc(var(--poodle-space-control-x) - 0.0625rem);
  }

  .text-area[data-size="lg"] .text-area__control {
    padding: calc(var(--poodle-space-control-y) + 0.0625rem) calc(var(--poodle-space-control-x) + 0.125rem);
    font-size: 0.9375rem;
  }

  .text-area[data-size="xl"] .text-area__control {
    padding: calc(var(--poodle-space-control-y) + 0.125rem) calc(var(--poodle-space-control-x) + 0.1875rem);
    font-size: 1rem;
  }
</style>
