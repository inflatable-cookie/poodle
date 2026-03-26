<script context="module" lang="ts">
  let triStateSwitchInstanceCount = 0;
</script>

<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import type { TriStateValue } from "./types";

  export let value: TriStateValue = "default";
  export let options: Record<TriStateValue, string> = {
    excluded: "Exclude",
    default: "Default",
    included: "Include",
  };
  export let disabled = false;
  export let ariaLabel: string;
  export let excludedColor: string | null = null;
  export let defaultColor: string | null = null;
  export let includedColor: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: TriStateValue };
  }>();

  const orderedValues: TriStateValue[] = ["excluded", "default", "included"];
  const instanceId = ++triStateSwitchInstanceCount;
  const groupName = `poodle-tri-state-switch-${instanceId}`;

  $: selectedIndex = Math.max(0, orderedValues.indexOf(value));
  $: triStateStyles = [
    excludedColor ? `--poodle-tri-state-excluded-color: ${excludedColor}` : "",
    defaultColor ? `--poodle-tri-state-default-color: ${defaultColor}` : "",
    includedColor ? `--poodle-tri-state-included-color: ${includedColor}` : "",
    `--poodle-tri-state-active-index: ${selectedIndex}`,
  ].filter(Boolean).join("; ") || undefined;

  function handleSelect(nextValue: TriStateValue): void {
    if (disabled || value === nextValue) {
      return;
    }

    value = nextValue;
    dispatch("valueChange", { value: nextValue });
  }
</script>

<div
  class="tri-state-switch"
  role="radiogroup"
  aria-label={ariaLabel}
  aria-disabled={disabled ? "true" : undefined}
  data-state={value}
  data-disabled={disabled}
  style={triStateStyles}
>
  <span class="tri-state-switch__selection" aria-hidden="true"></span>

  {#each orderedValues as optionValue}
    <label
      class="tri-state-switch__option"
      data-state={optionValue}
      data-selected={value === optionValue}
    >
      <input
        class="tri-state-switch__control"
        type="radio"
        name={groupName}
        checked={value === optionValue}
        disabled={disabled}
        aria-label={options[optionValue]}
        on:change={() => handleSelect(optionValue)}
      />
      <span class="tri-state-switch__segment">{options[optionValue]}</span>
    </label>
  {/each}
</div>

<style>
  .tri-state-switch {
    --poodle-tri-state-excluded-color: var(--poodle-color-status-danger);
    --poodle-tri-state-default-color: var(--poodle-color-text-primary);
    --poodle-tri-state-included-color: var(--poodle-color-status-success);
    --poodle-tri-state-excluded-track: color-mix(in srgb, var(--poodle-tri-state-excluded-color) 18%, var(--poodle-color-background-surface));
    --poodle-tri-state-default-track: color-mix(in srgb, var(--poodle-tri-state-default-color) 10%, var(--poodle-color-background-surface));
    --poodle-tri-state-included-track: color-mix(in srgb, var(--poodle-tri-state-included-color) 18%, var(--poodle-color-background-surface));
    position: relative;
    display: inline-grid;
    width: max-content;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    align-items: stretch;
    padding: 0.125rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: 999px;
    background: color-mix(in srgb, var(--poodle-color-text-primary) 18%, var(--poodle-color-background-surface));
    color: var(--poodle-color-text-primary);
    isolation: isolate;
  }

  .tri-state-switch[data-disabled="true"] {
    opacity: var(--poodle-state-opacity-disabled);
  }

  .tri-state-switch__selection {
    position: absolute;
    top: 0.125rem;
    bottom: 0.125rem;
    left: 0.125rem;
    width: calc((100% - 0.25rem) / 3);
    border: 0.0625rem solid transparent;
    border-radius: 999px;
    box-shadow:
      inset 0 0.0625rem 0 color-mix(in srgb, white 8%, transparent),
      0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent);
    transform: translateX(calc(var(--poodle-tri-state-active-index, 1) * 100%));
    transition:
      transform var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
    z-index: 0;
  }

  .tri-state-switch[data-state="excluded"] .tri-state-switch__selection {
    background: var(--poodle-tri-state-excluded-track);
    border-color: color-mix(in srgb, var(--poodle-tri-state-excluded-color) 58%, var(--poodle-color-border-default));
  }

  .tri-state-switch[data-state="default"] .tri-state-switch__selection {
    background: var(--poodle-tri-state-default-track);
    border-color: var(--poodle-color-border-default);
  }

  .tri-state-switch[data-state="included"] .tri-state-switch__selection {
    background: var(--poodle-tri-state-included-track);
    border-color: color-mix(in srgb, var(--poodle-tri-state-included-color) 58%, var(--poodle-color-border-default));
  }

  .tri-state-switch__option {
    position: relative;
    z-index: 1;
    cursor: pointer;
  }

  .tri-state-switch[data-disabled="true"] .tri-state-switch__option {
    cursor: not-allowed;
  }

  .tri-state-switch__control {
    position: absolute;
    opacity: 0;
    pointer-events: none;
  }

  .tri-state-switch__segment {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: calc(var(--poodle-size-control-height) - 0.25rem);
    min-width: 4.5rem;
    padding: 0 0.875rem;
    border-radius: 999px;
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    font-weight: var(--poodle-typography-label-weight);
    line-height: var(--poodle-typography-label-lineHeight);
    transition:
      color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      opacity var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .tri-state-switch__option[data-selected="true"] .tri-state-switch__segment {
    color: var(--poodle-color-text-primary);
  }

  .tri-state-switch__option[data-state="excluded"][data-selected="true"] .tri-state-switch__segment {
    color: var(--poodle-tri-state-excluded-color);
  }

  .tri-state-switch__option[data-state="default"][data-selected="true"] .tri-state-switch__segment {
    color: var(--poodle-color-text-primary);
  }

  .tri-state-switch__option[data-state="included"][data-selected="true"] .tri-state-switch__segment {
    color: var(--poodle-tri-state-included-color);
  }

  .tri-state-switch__control:focus-visible + .tri-state-switch__segment {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }
</style>
