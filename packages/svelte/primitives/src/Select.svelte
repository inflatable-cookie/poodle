<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Icon from "./Icon.svelte";
  import type { SelectOption, SelectOptionGroup, SelectItems } from "./types";

  export let id: string | undefined = undefined;
  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let placeholder: string | null = null;
  export let options: SelectItems = [];
  export let isDisabled = false;
  export let ariaLabel: string | null = null;
  export let describedBy: string | null = null;
  export let name: string | undefined = undefined;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
  }>();

  let uncontrolledValue = defaultValue;

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

<div class="select" data-placeholder={!hasSelection}>
  <select
    {id}
    {name}
    class="select__control"
    value={currentValue}
    disabled={isDisabled}
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

  <span class="select__indicator" aria-hidden="true"><Icon name="chevron-down" size="sm" /></span>
</div>

<style>
  .select {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    gap: var(--flint-space-inline-sm);
    min-height: var(--flint-size-control-height);
    padding: 0 var(--flint-space-control-x);
    border: 0.0625rem solid var(
      --flint-treatment-interactive-subtle-border,
      var(--flint-color-border-default)
    );
    border-radius: var(--flint-treatment-interactive-subtle-radius, var(--flint-radius-control));
    background: var(--flint-treatment-interactive-subtle-fill, var(--flint-color-background-surface));
    box-shadow: var(--flint-treatment-interactive-subtle-shadow, none);
    transition:
      border-color var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard),
      box-shadow var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard),
      background var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard);
  }

  .select:focus-within {
    border-color: var(--flint-color-accent-focusRing);
    background: var(
      --flint-treatment-interactive-subtle-fill-focus,
      var(--flint-color-background-surface)
    );
    box-shadow: var(
      --flint-treatment-interactive-subtle-shadow-focus,
      0 0 0 var(--flint-border-width-focus)
        color-mix(in srgb, var(--flint-color-accent-focusRing) 28%, transparent)
    );
  }

  .select:has(.select__control:disabled) {
    opacity: var(--flint-state-opacity-disabled);
  }

  .select__control {
    min-width: 0;
    width: 100%;
    height: calc(var(--flint-size-control-height) - (var(--flint-border-width-default) * 2));
    padding: 0;
    border: 0;
    background: transparent;
    color: var(--flint-color-text-primary);
    font-family: var(--flint-typography-body-family);
    font-size: var(--flint-typography-body-size);
    line-height: var(--flint-typography-body-lineHeight);
    outline: 0;
    appearance: none;
  }

  .select[data-placeholder="true"] .select__control {
    color: var(--flint-color-text-secondary);
  }

  .select__indicator {
    color: var(--flint-color-icon-muted);
    font-family: var(--flint-typography-code-family);
    font-size: 0.75rem;
    line-height: 1;
    pointer-events: none;
  }

  .select__control optgroup {
    font-weight: 600;
    color: var(--flint-color-text-secondary);
  }

  .select__control option {
    font-weight: normal;
    color: var(--flint-color-text-primary);
  }
</style>
