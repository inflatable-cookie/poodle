<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import type { ToggleGroupOption } from "./types";

  export let value: string | string[] | null = null;
  export let defaultValue: string | string[] | null = null;
  export let options: ToggleGroupOption[] = [];
  export let selectionMode: "single" | "multiple" = "single";
  export let isDisabled = false;
  export let ariaLabel: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string | string[] };
  }>();

  let uncontrolledValue = defaultValue ?? (selectionMode === "multiple" ? [] : null);

  $: controlled = value !== null;
  $: currentValue = controlled ? value : uncontrolledValue;

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
  class="toggle-group"
  role={selectionMode === "multiple" ? "group" : "radiogroup"}
  aria-label={ariaLabel ?? undefined}
>
  {#each options as option (option.value)}
    <button
      type="button"
      class="toggle-group__item"
      class:selected={isSelected(option.value)}
      data-selected={isSelected(option.value) ? "true" : "false"}
      disabled={isDisabled || option.isDisabled === true}
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
  .toggle-group {
    display: inline-flex;
    flex-wrap: wrap;
    gap: 0.25rem;
  }

  .toggle-group__item {
    min-height: calc(var(--pug-size-control-height) - 0.25rem);
    padding: 0 0.75rem;
    border: 0.0625rem solid var(
      --pug-treatment-interactive-border,
      color-mix(in srgb, var(--pug-color-border-subtle) 82%, transparent)
    );
    border-radius: var(--pug-treatment-interactive-radius, var(--pug-radius-control));
    background: var(
      --pug-treatment-interactive-fill,
      color-mix(in srgb, var(--pug-surface) 93%, var(--pug-color-text-primary))
    );
    box-shadow: var(--pug-treatment-interactive-shadow, none);
    color: var(--pug-color-text-primary);
    cursor: pointer;
    font-family: var(--pug-typography-label-family);
    font-size: 0.75rem;
    font-weight: 600;
    line-height: 1;
    transition:
      border-color 180ms ease,
      background 180ms ease,
      box-shadow 180ms ease,
      color 180ms ease;
  }

  .toggle-group__item.selected {
    background:
      linear-gradient(
        color-mix(in srgb, var(--pug-color-accent-base) 22%, transparent),
        color-mix(in srgb, var(--pug-color-accent-base) 22%, transparent)
      ),
      var(
        --pug-treatment-interactive-fill,
        color-mix(in srgb, var(--pug-surface) 93%, var(--pug-color-text-primary))
      );
    border-color: var(
      --pug-treatment-interactive-border-active,
      color-mix(in srgb, var(--pug-color-accent-base) 42%, var(--pug-color-border-default))
    );
  }

  .toggle-group__item:focus-visible {
    outline: var(--pug-border-width-focus) solid var(--pug-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .toggle-group__item:disabled {
    cursor: not-allowed;
    opacity: var(--pug-state-opacity-disabled);
  }
</style>
