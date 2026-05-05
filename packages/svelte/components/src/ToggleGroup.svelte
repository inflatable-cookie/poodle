<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
    ToggleGroupOption,
  } from "./types";

  export let value: string | string[] | null = null;
  export let defaultValue: string | string[] | null = null;
  export let options: ToggleGroupOption[] = [];
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;
  export let selectionMode: "single" | "multiple" = "single";
  export let disabled = false;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string | string[] };
  }>();

  let uncontrolledValue = defaultValue ?? (selectionMode === "multiple" ? [] : null);
  const uiPresentation = getUiPresentation();

  $: controlled = value !== null;
  $: currentValue = controlled ? value : uncontrolledValue;
  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;

  function isSelected(optionValue: string): boolean {
    if (selectionMode === "multiple") {
      return Array.isArray(currentValue) && currentValue.includes(optionValue);
    }

    return currentValue === optionValue;
  }

  function toggle(optionValue: string): void {
    let nextValue: string | string[];

    if (selectionMode === "multiple") {
      const current = Array.isArray(currentValue) ? currentValue : [];
      nextValue = current.includes(optionValue)
        ? current.filter((item) => item !== optionValue)
        : [...current, optionValue];
    } else {
      nextValue = optionValue;
    }

    if (!controlled) {
      uncontrolledValue = nextValue;
    }

    dispatch("valueChange", { value: nextValue });
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
      on:click={() => toggle(option.value)}
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
    border: 0.0625rem solid var(
      --poodle-treatment-interactive-border,
      color-mix(in srgb, var(--poodle-color-border-subtle) 82%, transparent)
    );
    border-radius: var(--poodle-treatment-interactive-radius, var(--poodle-radius-control));
    background: var(
      --poodle-treatment-interactive-fill,
      color-mix(in srgb, var(--poodle-surface) 93%, var(--poodle-color-text-primary))
    );
    box-shadow: var(--poodle-treatment-interactive-shadow, none);
    color: var(--poodle-color-text-primary);
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
    background:
      linear-gradient(
        color-mix(in srgb, var(--poodle-color-accent-base) 22%, transparent),
        color-mix(in srgb, var(--poodle-color-accent-base) 22%, transparent)
      ),
      var(
        --poodle-treatment-interactive-fill,
        color-mix(in srgb, var(--poodle-surface) 93%, var(--poodle-color-text-primary))
      );
    border-color: var(
      --poodle-treatment-interactive-border-active,
      color-mix(in srgb, var(--poodle-color-accent-base) 42%, var(--poodle-color-border-default))
    );
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
