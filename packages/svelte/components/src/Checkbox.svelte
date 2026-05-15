<script lang="ts">
  import Icon from "./Icon.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    id?: string | undefined;
    checked?: boolean | undefined;
    defaultChecked?: boolean;
    mixed?: boolean;
    disabled?: boolean;
    readOnly?: boolean;
    label?: string | null;
    ariaLabel?: string | null;
    describedBy?: string | null;
    selectedColor?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onCheckedChange?: ((checked: boolean) => void) | undefined;
  }

  let input: HTMLInputElement | null = null;
  const uiPresentation = getUiPresentation();

  let {
    id = undefined,
    checked = $bindable<boolean | undefined>(undefined),
    defaultChecked = false,
    mixed = false,
    disabled = false,
    readOnly = false,
    label = null,
    ariaLabel = null,
    describedBy = null,
    selectedColor = null,
    size = null,
    sizeRole = "control",
    density = null,
    onCheckedChange = undefined,
  }: Props = $props();

  let seededDefaultChecked = $state(false);
  let uncontrolledChecked = $state(false);

  const currentChecked = $derived(checked === undefined ? uncontrolledChecked : checked);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const checkboxStyles = $derived(selectedColor ? `--poodle-checkbox-selected-color: ${selectedColor}` : undefined);

  $effect(() => {
    if (!seededDefaultChecked && checked === undefined) {
      uncontrolledChecked = defaultChecked;
      seededDefaultChecked = true;
    }
  });

  $effect(() => {
    if (input) {
      input.indeterminate = mixed;
    }
  });

  function handleChange(event: Event): void {
    const nextChecked = (event.currentTarget as HTMLInputElement).checked;

    if (readOnly) {
      (event.currentTarget as HTMLInputElement).checked = currentChecked;
      return;
    }

    if (checked === undefined) {
      uncontrolledChecked = nextChecked;
    }

    checked = nextChecked;
    onCheckedChange?.(nextChecked);
  }
</script>

<label class="poodle-checkbox" data-disabled={disabled} data-size={resolvedSize} data-density={resolvedDensity} style={checkboxStyles}>
  <input
    bind:this={input}
    {id}
    class="poodle-checkbox__control"
    type="checkbox"
    checked={currentChecked}
    disabled={disabled}
    aria-label={label ? undefined : ariaLabel ?? undefined}
    aria-describedby={describedBy ?? undefined}
    aria-readonly={readOnly ? "true" : undefined}
    onchange={handleChange}
  />
  <span class="poodle-checkbox__indicator" aria-hidden="true">
    {#if mixed}
      <span class="poodle-checkbox__mark"><Icon name="minus" /></span>
    {:else if currentChecked}
      <span class="poodle-checkbox__mark"><Icon name="check" /></span>
    {/if}
  </span>
  {#if label}
    <span class="poodle-checkbox__label">{label}</span>
  {/if}
</label>

<style>
  .poodle-checkbox {
    --poodle-checkbox-selected-color: var(--poodle-color-accent-base);
    position: relative;
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    color: var(--poodle-color-text-primary);
    cursor: pointer;
  }

  .poodle-checkbox[data-disabled="true"] {
    opacity: var(--poodle-state-opacity-disabled);
    cursor: not-allowed;
  }

  .poodle-checkbox__control {
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

  .poodle-checkbox__indicator {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--poodle-size-icon-md);
    height: var(--poodle-size-icon-md);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: 0.3125rem;
    background: var(--poodle-color-background-surface);
    color: var(--poodle-color-text-inverse);
  }

  .poodle-checkbox__control:checked + .poodle-checkbox__indicator,
  .poodle-checkbox__control:indeterminate + .poodle-checkbox__indicator {
    border-color: var(--poodle-checkbox-selected-color);
    background: var(--poodle-checkbox-selected-color);
  }

  .poodle-checkbox__control:focus-visible + .poodle-checkbox__indicator {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-checkbox__mark {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: calc(var(--poodle-size-icon-md) - 0.25rem);
    height: calc(var(--poodle-size-icon-md) - 0.25rem);
    line-height: 1;
  }

  .poodle-checkbox__label {
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    font-weight: var(--poodle-typography-label-weight);
    line-height: var(--poodle-typography-label-lineHeight);
  }

  /* Density variants */
  .poodle-checkbox[data-density="compact"] {
    gap: 0.25rem;
  }

  .poodle-checkbox[data-density="comfortable"] {
    gap: var(--poodle-space-inline-md);
  }

  /* Size variants — label */
  .poodle-checkbox[data-size="xs"] .poodle-checkbox__label { font-size: 0.6875rem; }
  .poodle-checkbox[data-size="sm"] .poodle-checkbox__label { font-size: 0.75rem; }
  .poodle-checkbox[data-size="lg"] .poodle-checkbox__label { font-size: 0.875rem; }
  .poodle-checkbox[data-size="xl"] .poodle-checkbox__label { font-size: 0.9375rem; }

  /* Size variants — indicator */
  .poodle-checkbox[data-size="xs"] .poodle-checkbox__indicator {
    width: var(--poodle-size-icon-xs);
    height: var(--poodle-size-icon-xs);
    border-radius: 0.1875rem;
  }

  .poodle-checkbox[data-size="xs"] .poodle-checkbox__mark {
    width: calc(var(--poodle-size-icon-xs) - 0.25rem);
    height: calc(var(--poodle-size-icon-xs) - 0.25rem);
  }

  .poodle-checkbox[data-size="sm"] .poodle-checkbox__indicator {
    width: var(--poodle-size-icon-sm);
    height: var(--poodle-size-icon-sm);
    border-radius: 0.25rem;
  }

  .poodle-checkbox[data-size="sm"] .poodle-checkbox__mark {
    width: calc(var(--poodle-size-icon-sm) - 0.25rem);
    height: calc(var(--poodle-size-icon-sm) - 0.25rem);
  }

  .poodle-checkbox[data-size="lg"] .poodle-checkbox__indicator {
    width: var(--poodle-size-icon-lg);
    height: var(--poodle-size-icon-lg);
    border-radius: 0.375rem;
  }

  .poodle-checkbox[data-size="lg"] .poodle-checkbox__mark {
    width: calc(var(--poodle-size-icon-lg) - 0.375rem);
    height: calc(var(--poodle-size-icon-lg) - 0.375rem);
  }

  .poodle-checkbox[data-size="xl"] .poodle-checkbox__indicator {
    width: var(--poodle-size-icon-xl);
    height: var(--poodle-size-icon-xl);
    border-radius: 0.4375rem;
  }

  .poodle-checkbox[data-size="xl"] .poodle-checkbox__mark {
    width: calc(var(--poodle-size-icon-xl) - 0.375rem);
    height: calc(var(--poodle-size-icon-xl) - 0.375rem);
  }
</style>
