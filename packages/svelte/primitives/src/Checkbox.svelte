<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import Icon from "./Icon.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  export let id: string | undefined = undefined;
  export let checked = false;
  export let defaultChecked = false;
  export let mixed = false;
  export let disabled = false;
  export let readOnly = false;
  export let label: string | null = null;
  export let ariaLabel: string | null = null;
  export let describedBy: string | null = null;
  export let selectedColor: string | null = null;
  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  const dispatch = createEventDispatcher<{
    checkedChange: { checked: boolean };
  }>();

  let input: HTMLInputElement | null = null;
  let uncontrolledChecked = defaultChecked;
  const uiPresentation = getUiPresentation();

  $: currentChecked = checked ?? uncontrolledChecked;
  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: checkboxStyles = selectedColor ? `--poodle-checkbox-selected-color: ${selectedColor}` : undefined;
  $: if (input) {
    input.indeterminate = mixed;
  }

  function handleChange(event: Event): void {
    const nextChecked = (event.currentTarget as HTMLInputElement).checked;

    if (readOnly) {
      (event.currentTarget as HTMLInputElement).checked = currentChecked;
      return;
    }

    uncontrolledChecked = nextChecked;
    dispatch("checkedChange", { checked: nextChecked });
  }
</script>

<label class="checkbox" data-disabled={disabled} data-size={resolvedSize} data-density={resolvedDensity} style={checkboxStyles}>
  <input
    bind:this={input}
    {id}
    class="checkbox__control"
    type="checkbox"
    checked={currentChecked}
    disabled={disabled}
    aria-label={label ? undefined : ariaLabel ?? undefined}
    aria-describedby={describedBy ?? undefined}
    aria-readonly={readOnly ? "true" : undefined}
    on:change={handleChange}
  />
  <span class="checkbox__indicator" aria-hidden="true">
    {#if mixed}
      <span class="checkbox__mark"><Icon name="minus" /></span>
    {:else if currentChecked}
      <span class="checkbox__mark"><Icon name="check" /></span>
    {/if}
  </span>
  {#if label}
    <span class="checkbox__label">{label}</span>
  {/if}
</label>

<style>
  .checkbox {
    --poodle-checkbox-selected-color: var(--poodle-color-accent-base);
    position: relative;
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    color: var(--poodle-color-text-primary);
    cursor: pointer;
  }

  .checkbox[data-disabled="true"] {
    opacity: var(--poodle-state-opacity-disabled);
    cursor: not-allowed;
  }

  .checkbox__control {
    position: absolute;
    width: 1px;
    height: 1px;
    margin: -1px;
    padding: 0;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
  }

  .checkbox__indicator {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: calc(var(--poodle-size-icon-default) + 0.125rem);
    height: calc(var(--poodle-size-icon-default) + 0.125rem);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: 0.3125rem;
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-inverse);
  }

  .checkbox__control:checked + .checkbox__indicator,
  .checkbox__control:indeterminate + .checkbox__indicator {
    border-color: var(--poodle-checkbox-selected-color);
    background: var(--poodle-checkbox-selected-color);
  }

  .checkbox__control:focus-visible + .checkbox__indicator {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .checkbox__mark {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: calc(var(--poodle-size-icon-default) - 0.125rem);
    height: calc(var(--poodle-size-icon-default) - 0.125rem);
    line-height: 1;
  }

  .checkbox__label {
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    font-weight: var(--poodle-typography-label-weight);
    line-height: var(--poodle-typography-label-lineHeight);
  }

  /* Density variants */
  .checkbox[data-density="compact"] {
    gap: 0.25rem;
  }

  .checkbox[data-density="comfortable"] {
    gap: var(--poodle-space-inline-md);
  }

  /* Size variants — label */
  .checkbox[data-size="xs"] .checkbox__label { font-size: 0.6875rem; }
  .checkbox[data-size="sm"] .checkbox__label { font-size: 0.75rem; }
  .checkbox[data-size="lg"] .checkbox__label { font-size: 0.875rem; }
  .checkbox[data-size="xl"] .checkbox__label { font-size: 0.9375rem; }

  /* Size variants — indicator */
  .checkbox[data-size="xs"] .checkbox__indicator {
    width: calc(var(--poodle-size-icon-default) - 0.125rem);
    height: calc(var(--poodle-size-icon-default) - 0.125rem);
  }

  .checkbox[data-size="xs"] .checkbox__mark {
    width: calc(var(--poodle-size-icon-default) - 0.375rem);
    height: calc(var(--poodle-size-icon-default) - 0.375rem);
  }

  .checkbox[data-size="sm"] .checkbox__indicator {
    width: var(--poodle-size-icon-default);
    height: var(--poodle-size-icon-default);
  }

  .checkbox[data-size="sm"] .checkbox__mark {
    width: calc(var(--poodle-size-icon-default) - 0.25rem);
    height: calc(var(--poodle-size-icon-default) - 0.25rem);
  }

  .checkbox[data-size="lg"] .checkbox__indicator {
    width: calc(var(--poodle-size-icon-default) + 0.375rem);
    height: calc(var(--poodle-size-icon-default) + 0.375rem);
  }

  .checkbox[data-size="lg"] .checkbox__mark {
    width: var(--poodle-size-icon-default);
    height: var(--poodle-size-icon-default);
  }

  .checkbox[data-size="xl"] .checkbox__indicator {
    width: calc(var(--poodle-size-icon-default) + 0.625rem);
    height: calc(var(--poodle-size-icon-default) + 0.625rem);
  }

  .checkbox[data-size="xl"] .checkbox__mark {
    width: calc(var(--poodle-size-icon-default) + 0.125rem);
    height: calc(var(--poodle-size-icon-default) + 0.125rem);
  }
</style>
