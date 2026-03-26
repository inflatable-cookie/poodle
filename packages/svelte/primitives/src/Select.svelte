<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Icon from "./Icon.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlSize, SemanticControlSizeRole, SelectOption, SelectOptionGroup, SelectItems } from "./types";

  export let id: string | undefined = undefined;
  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let placeholder: string | null = null;
  export let options: SelectItems = [];
  export let disabled = false;
  export let ariaLabel: string | null = null;
  export let describedBy: string | null = null;
  export let name: string | undefined = undefined;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
  }>();

  const uiPresentation = getUiPresentation();
  let uncontrolledValue = defaultValue;

  $: resolvedSize = size ?? resolveSemanticControlSize(uiPresentation?.sizeScale ?? "md", sizeRole);
  $: isControlled = value !== null;
  $: currentValue = (isControlled ? value : uncontrolledValue) ?? "";
  $: hasSelection = currentValue !== "";
  $: isGrouped = options.length > 0 && "options" in options[0];

  function handleChange(event: Event): void {
    const nextValue = (event.currentTarget as HTMLSelectElement).value;

    if (!isControlled) {
      uncontrolledValue = nextValue;
    }

    dispatch("valueChange", { value: nextValue });
  }
</script>

<div class="select" data-placeholder={!hasSelection} data-size={resolvedSize}>
  <select
    {id}
    {name}
    class="select__control"
    value={currentValue}
    disabled={disabled}
    aria-label={ariaLabel ?? undefined}
    aria-describedby={describedBy ?? undefined}
    on:change={handleChange}
  >
    {#if placeholder}
      <option value="" disabled>{placeholder}</option>
    {/if}

    {#if isGrouped}
      {#each options as group}
        <optgroup label={(group as SelectOptionGroup).label}>
          {#each (group as SelectOptionGroup).options as option (option.value)}
            <option value={option.value} disabled={option.isDisabled === true}>
              {option.label}
            </option>
          {/each}
        </optgroup>
      {/each}
    {:else}
      {#each options as option}
        <option value={(option as SelectOption).value} disabled={(option as SelectOption).isDisabled === true}>
          {(option as SelectOption).label}
        </option>
      {/each}
    {/if}
  </select>

  <span class="select__indicator" aria-hidden="true"><Icon name="chevron-down" /></span>
</div>

<style>
  .select {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    border: 0.0625rem solid var(
      --poodle-treatment-interactive-subtle-border,
      var(--poodle-color-border-default)
    );
    border-radius: var(--poodle-treatment-interactive-subtle-radius, var(--poodle-radius-control));
    background: var(--poodle-treatment-interactive-subtle-fill, var(--poodle-color-background-surface));
    box-shadow: var(--poodle-treatment-interactive-subtle-shadow, none);
    transition:
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .select:focus-within {
    border-color: var(--poodle-color-accent-focusRing);
    background: var(
      --poodle-treatment-interactive-subtle-fill-focus,
      var(--poodle-color-background-surface)
    );
    box-shadow: var(
      --poodle-treatment-interactive-subtle-shadow-focus,
      0 0 0 var(--poodle-border-width-focus)
        color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent)
    );
  }

  .select:has(.select__control:disabled) {
    opacity: var(--poodle-state-opacity-disabled);
  }

  .select__control {
    min-width: 0;
    width: 100%;
    height: calc(var(--poodle-size-control-height) - (var(--poodle-border-width-default) * 2));
    padding: 0;
    border: 0;
    background: transparent;
    color: var(--poodle-color-text-primary);
    font-family: var(--poodle-typography-body-family);
    font-size: var(--poodle-typography-body-size);
    line-height: var(--poodle-typography-body-lineHeight);
    outline: 0;
    appearance: none;
  }

  .select[data-placeholder="true"] .select__control {
    color: var(--poodle-color-text-secondary);
  }

  .select__indicator {
    color: var(--poodle-color-icon-muted);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.75rem;
    line-height: 1;
    pointer-events: none;
  }

  .select__control optgroup {
    font-weight: 600;
    color: var(--poodle-color-text-secondary);
  }

  .select__control option {
    font-weight: normal;
    color: var(--poodle-color-text-primary);
  }

  /* Size variants */
  .select[data-size="xs"] {
    min-height: calc(var(--poodle-size-control-height) - 0.5rem);
    padding: 0 calc(var(--poodle-space-control-x) - 0.125rem);
  }

  .select[data-size="xs"] .select__control {
    height: calc(var(--poodle-size-control-height) - 0.5rem - (var(--poodle-border-width-default) * 2));
    font-size: 0.75rem;
  }

  .select[data-size="sm"] {
    min-height: calc(var(--poodle-size-control-height) - 0.375rem);
    padding: 0 calc(var(--poodle-space-control-x) - 0.0625rem);
  }

  .select[data-size="sm"] .select__control {
    height: calc(var(--poodle-size-control-height) - 0.375rem - (var(--poodle-border-width-default) * 2));
  }

  .select[data-size="lg"] {
    min-height: calc(var(--poodle-size-control-height) + 0.375rem);
    padding: 0 calc(var(--poodle-space-control-x) + 0.125rem);
  }

  .select[data-size="lg"] .select__control {
    height: calc(var(--poodle-size-control-height) + 0.375rem - (var(--poodle-border-width-default) * 2));
    font-size: 0.9375rem;
  }

  .select[data-size="xl"] {
    min-height: calc(var(--poodle-size-control-height) + 0.5rem);
    padding: 0 calc(var(--poodle-space-control-x) + 0.1875rem);
  }

  .select[data-size="xl"] .select__control {
    height: calc(var(--poodle-size-control-height) + 0.5rem - (var(--poodle-border-width-default) * 2));
    font-size: 1rem;
  }
</style>
