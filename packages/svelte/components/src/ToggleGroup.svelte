<script lang="ts">
  import { toggleGroupIsSelected, toggleGroupTransition, type ToggleGroupContext } from "@poodle/headless";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
    ToggleGroupOption,
  } from "./types";

  interface Props {
    value?: string | string[] | null | undefined;
    defaultValue?: string | string[] | null;
    options?: ToggleGroupOption[];
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    selectionMode?: "single" | "multiple";
    allowDeactivation?: boolean;
    disabled?: boolean;
    ariaLabel?: string | null;
    onValueChange?: ((value: string | string[] | null) => void) | undefined;
  }

  let {
    value = $bindable<string | string[] | null | undefined>(undefined),
    defaultValue = null,
    options = [],
    size = null,
    sizeRole = "control",
    density = null,
    selectionMode = "single",
    allowDeactivation = false,
    disabled = false,
    ariaLabel = null,
    onValueChange = undefined,
  }: Props = $props();

  let uncontrolledValue = $state<string | string[] | null>(null);
  let seededDefaultValue = $state(false);
  const uiPresentation = getUiPresentation();

  $effect.pre(() => {
    if (!seededDefaultValue) {
      uncontrolledValue = defaultValue ?? (selectionMode === "multiple" ? [] : null);
      seededDefaultValue = true;
    }
  });

  const controlled = $derived(value !== undefined);
  const currentValue = $derived(controlled ? value : uncontrolledValue);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  const machineContext = $derived<ToggleGroupContext>({
    value: currentValue ?? null,
    options: options.map((option) => ({ value: option.value, disabled: disabled || option.disabled === true })),
    selectionMode,
    allowDeactivation,
    disabled,
  });

  function isSelected(optionValue: string): boolean {
    return toggleGroupIsSelected(machineContext, optionValue);
  }

  function toggle(optionValue: string): void {
    const result = toggleGroupTransition(machineContext, { type: "TOGGLE", value: optionValue });

    for (const effect of result.effects) {
      if (effect.type === "emitValueChange") {
        if (!controlled) {
          uncontrolledValue = effect.value;
        } else {
          value = effect.value;
        }

        onValueChange?.(effect.value);
      }
    }
  }
</script>

<div
  class="poodle-toggle-group"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  role={selectionMode === "multiple" ? "group" : "radiogroup"}
  aria-label={ariaLabel ?? undefined}
>
  {#each options as option (option.value)}
    <button
      type="button"
      class="poodle-toggle-group__item"
      class:poodle-selected={isSelected(option.value)}
      data-selected={isSelected(option.value) ? "true" : "false"}
      disabled={disabled || option.disabled === true}
      role={selectionMode === "multiple" ? "button" : "radio"}
      aria-label={option.ariaLabel ?? undefined}
      aria-pressed={selectionMode === "multiple" ? (isSelected(option.value) ? "true" : "false") : undefined}
      aria-checked={selectionMode === "single" ? (isSelected(option.value) ? "true" : "false") : undefined}
      onclick={() => toggle(option.value)}
    >
      {option.label}
    </button>
  {/each}
</div>

<style>
  .poodle-toggle-group {
    --poodle-toggle-group-height: var(--poodle-size-control-height);
    --poodle-toggle-group-x: var(--poodle-space-control-x);
    --poodle-toggle-group-gap: 0.25rem;
    display: inline-flex;
    flex-wrap: wrap;
    gap: var(--poodle-toggle-group-gap);
  }

  .poodle-toggle-group[data-size="xs"] {
    --poodle-toggle-group-height: 1.5rem;
  }

  .poodle-toggle-group[data-size="sm"] {
    --poodle-toggle-group-height: 1.75rem;
  }

  .poodle-toggle-group[data-size="md"] {
    --poodle-toggle-group-height: 2.25rem;
  }

  .poodle-toggle-group[data-size="lg"] {
    --poodle-toggle-group-height: 2.75rem;
  }

  .poodle-toggle-group[data-size="xl"] {
    --poodle-toggle-group-height: 3.25rem;
  }

  .poodle-toggle-group[data-density="compact"] {
    --poodle-toggle-group-x: 0.5rem;
    --poodle-toggle-group-gap: 0.1875rem;
  }

  .poodle-toggle-group[data-density="default"] {
    --poodle-toggle-group-x: 0.75rem;
    --poodle-toggle-group-gap: 0.25rem;
  }

  .poodle-toggle-group[data-density="comfortable"] {
    --poodle-toggle-group-x: 1rem;
    --poodle-toggle-group-gap: 0.375rem;
  }

  .poodle-toggle-group__item {
    min-height: calc(var(--poodle-toggle-group-height) - 0.25rem);
    padding: 0 var(--poodle-toggle-group-x);
    border: 0.0625rem solid color-mix(in srgb, var(--poodle-color-border-subtle) 82%, transparent);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-recipe-toggle-group-item-fill, color-mix(in srgb, var(--poodle-surface) 93%, var(--poodle-color-text-primary)));
    box-shadow: var(--poodle-recipe-toggle-group-item-shadow, none);
    color: var(--poodle-recipe-toggle-group-item-text, var(--poodle-color-text-primary));
    cursor: pointer;
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    font-weight: 600;
    line-height: 1;
    transition:
      border-color 180ms ease,
      background 180ms ease,
      box-shadow 180ms ease,
      color 180ms ease;
  }

  .poodle-toggle-group__item.poodle-selected {
    background: var(--poodle-recipe-toggle-group-selected-item-fill, linear-gradient(
        color-mix(in srgb, var(--poodle-color-accent-base) 22%, transparent),
        color-mix(in srgb, var(--poodle-color-accent-base) 22%, transparent)
      ),
      color-mix(in srgb, var(--poodle-surface) 93%, var(--poodle-color-text-primary)));
    border-color: var(--poodle-recipe-toggle-group-item-border, color-mix(in srgb, var(--poodle-color-accent-base) 42%, var(--poodle-color-border-default)));
  }

  .poodle-toggle-group__item:focus-visible {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-toggle-group__item:disabled {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }
</style>
