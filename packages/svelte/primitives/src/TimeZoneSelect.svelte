<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { defaultTimeZoneOptions } from "./date";

  import type { TimeZoneOption } from "./types";

  export let id: string | undefined = undefined;
  export let value: string | null = null;
  export let defaultValue: string | null = null;
  export let placeholder: string | null = "Select time zone";
  export let options: TimeZoneOption[] = [];
  export let isDisabled = false;
  export let ariaLabel: string | null = null;
  export let describedBy: string | null = null;
  export let name: string | undefined = undefined;

  const dispatch = createEventDispatcher<{
    valueChange: { value: string };
  }>();

  let uncontrolledValue = defaultValue;

  $: availableOptions = options.length > 0 ? options : defaultTimeZoneOptions();
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

<div class="time-zone-select" data-placeholder={!hasSelection}>
  <select
    {id}
    {name}
    class="time-zone-select__control"
    value={currentValue}
    disabled={isDisabled}
    aria-label={ariaLabel ?? undefined}
    aria-describedby={describedBy ?? undefined}
    on:change={handleChange}
  >
    {#if placeholder}
      <option value="" disabled>{placeholder}</option>
    {/if}

    {#each availableOptions as option (option.value)}
      <option value={option.value} disabled={option.isDisabled === true}>
        {option.label}
      </option>
    {/each}
  </select>

  <span class="time-zone-select__indicator" aria-hidden="true">▾</span>
</div>

<style>
  .time-zone-select {
    display: grid;
    grid-template-columns: minmax(0, 1fr) auto;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    min-height: var(--poodle-size-control-height);
    padding: 0 var(--poodle-space-control-x);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: var(--poodle-radius-control);
    background: var(--poodle-color-background-surface);
    transition:
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .time-zone-select:focus-within {
    border-color: var(--poodle-color-accent-focusRing);
    box-shadow:
      0 0 0 var(--poodle-border-width-focus)
        color-mix(in srgb, var(--poodle-color-accent-focusRing) 28%, transparent);
  }

  .time-zone-select:has(.time-zone-select__control:disabled) {
    opacity: var(--poodle-state-opacity-disabled);
  }

  .time-zone-select__control {
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

  .time-zone-select[data-placeholder="true"] .time-zone-select__control {
    color: var(--poodle-color-text-secondary);
  }

  .time-zone-select__indicator {
    color: var(--poodle-color-icon-muted);
    font-family: var(--poodle-typography-code-family);
    font-size: 0.75rem;
    line-height: 1;
    pointer-events: none;
  }
</style>
