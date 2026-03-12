<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import type { SelectOption } from "./types";

  export let id: string | undefined = undefined;
  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let placeholder: string | null = null;
  export let options: SelectOption[] = [];
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

    {#each options as option (option.value)}
      <option value={option.value} disabled={option.isDisabled === true}>
        {option.label}
      </option>
    {/each}
  </select>

  <span class="select__indicator" aria-hidden="true">▾</span>
</div>

<style>
  .select {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    gap: var(--pug-space-inline-sm);
    min-height: var(--pug-size-control-height);
    padding: 0 var(--pug-space-control-x);
    border: 0.0625rem solid var(--pug-color-border-default);
    border-radius: var(--pug-treatment-interactive-subtle-radius, var(--pug-radius-control));
    background: var(--pug-treatment-interactive-subtle-fill, var(--pug-color-background-surface));
    box-shadow: var(--pug-treatment-interactive-subtle-shadow, none);
    transition:
      border-color var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      box-shadow var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard),
      background var(--pug-motion-duration-interaction) var(--pug-motion-easing-standard);
  }

  .select:focus-within {
    border-color: var(--pug-color-accent-focusRing);
    background: var(
      --pug-treatment-interactive-subtle-fill-focus,
      var(--pug-color-background-surface)
    );
    box-shadow: var(
      --pug-treatment-interactive-subtle-shadow-focus,
      0 0 0 var(--pug-border-width-focus)
        color-mix(in srgb, var(--pug-color-accent-focusRing) 28%, transparent)
    );
  }

  .select:has(.select__control:disabled) {
    opacity: var(--pug-state-opacity-disabled);
  }

  .select__control {
    min-width: 0;
    width: 100%;
    height: calc(var(--pug-size-control-height) - (var(--pug-border-width-default) * 2));
    padding: 0;
    border: 0;
    background: transparent;
    color: var(--pug-color-text-primary);
    font-family: var(--pug-typography-body-family);
    font-size: var(--pug-typography-body-size);
    line-height: var(--pug-typography-body-lineHeight);
    outline: 0;
    appearance: none;
  }

  .select[data-placeholder="true"] .select__control {
    color: var(--pug-color-text-secondary);
  }

  .select__indicator {
    color: var(--pug-color-icon-muted);
    font-family: var(--pug-typography-code-family);
    font-size: 0.75rem;
    line-height: 1;
    pointer-events: none;
  }
</style>
