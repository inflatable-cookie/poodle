<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import type { TriStateValue } from "./types";

  export let value: TriStateValue = "default";
  export let options: Record<TriStateValue, string> = {
    excluded: "Exclude",
    default: "Default",
    included: "Include",
  };
  export let isDisabled = false;
  export let ariaLabel: string;

  const dispatch = createEventDispatcher<{
    valueChange: { value: TriStateValue };
  }>();

  const orderedValues: TriStateValue[] = ["excluded", "default", "included"];
</script>

<div class="tri-state-switch" role="radiogroup" aria-label={ariaLabel}>
  {#each orderedValues as optionValue}
    <button
      type="button"
      class="tri-state-switch__segment"
      data-state={optionValue}
      data-selected={value === optionValue}
      disabled={isDisabled}
      role="radio"
      aria-checked={value === optionValue ? "true" : "false"}
      aria-label={options[optionValue]}
      on:click={() => dispatch("valueChange", { value: optionValue })}
    >
      {options[optionValue]}
    </button>
  {/each}
</div>

<style>
  .tri-state-switch {
    display: inline-grid;
    grid-auto-flow: column;
    gap: 0.125rem;
    padding: 0.125rem;
    border: 0.0625rem solid color-mix(in srgb, var(--flint-color-border-subtle) 84%, transparent);
    border-radius: var(--flint-radius-control);
    background: color-mix(in srgb, var(--flint-color-background-surface) 82%, transparent);
  }

  .tri-state-switch__segment {
    min-height: calc(var(--flint-size-control-height) - 0.25rem);
    padding: 0 0.75rem;
    border: 0;
    border-radius: calc(var(--flint-radius-control) - 0.125rem);
    background: transparent;
    color: var(--flint-color-text-secondary);
    cursor: pointer;
    font-family: var(--flint-typography-label-family);
    font-size: 0.75rem;
    font-weight: 600;
    line-height: 1;
    transition:
      background var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard),
      color var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard),
      box-shadow var(--flint-motion-duration-interaction) var(--flint-motion-easing-standard);
  }

  .tri-state-switch__segment[data-selected="true"] {
    color: var(--flint-color-text-primary);
    box-shadow: inset 0 0.0625rem 0 color-mix(in srgb, white 12%, transparent);
  }

  .tri-state-switch__segment[data-state="excluded"][data-selected="true"] {
    background: color-mix(in srgb, var(--flint-color-status-danger) 18%, var(--flint-color-background-surface));
  }

  .tri-state-switch__segment[data-state="default"][data-selected="true"] {
    background: color-mix(in srgb, var(--flint-color-background-elevated) 88%, var(--flint-color-background-surface));
  }

  .tri-state-switch__segment[data-state="included"][data-selected="true"] {
    background: color-mix(in srgb, var(--flint-color-status-success) 18%, var(--flint-color-background-surface));
  }

  .tri-state-switch__segment:focus-visible {
    outline: var(--flint-border-width-focus) solid var(--flint-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .tri-state-switch__segment:disabled {
    cursor: not-allowed;
    opacity: var(--flint-state-opacity-disabled);
  }
</style>
