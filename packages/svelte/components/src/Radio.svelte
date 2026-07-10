<script lang="ts">
  import { switchTransition } from "@poodle/headless";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
  } from "./types";

  interface Props {
    id?: string | undefined;
    name?: string | undefined;
    value?: string | undefined;
    checked?: boolean | undefined;
    defaultChecked?: boolean;
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

  const uiPresentation = getUiPresentation();

  let {
    id = undefined,
    name = undefined,
    value = undefined,
    checked = $bindable<boolean | undefined>(undefined),
    defaultChecked = false,
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

  const currentChecked = $derived(
    checked === undefined ? uncontrolledChecked : checked,
  );
  const resolvedSize = $derived(
    size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole),
  );
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const radioStyles = $derived(
    selectedColor ? `--poodle-radio-selected-color: ${selectedColor}` : undefined,
  );

  $effect(() => {
    if (!seededDefaultChecked && checked === undefined) {
      uncontrolledChecked = defaultChecked;
      seededDefaultChecked = true;
    }
  });

  function handleChange(event: Event): void {
    const control = event.currentTarget as HTMLInputElement;
    const result = switchTransition(
      { checked: currentChecked, disabled, readOnly },
      { type: "TOGGLE", nextChecked: control.checked },
    );

    for (const effect of result.effects) {
      if (effect.type === "revertNativeChecked") {
        control.checked = currentChecked;
      } else if (effect.type === "emitCheckedChange") {
        if (checked === undefined) {
          uncontrolledChecked = effect.checked;
        } else {
          checked = effect.checked;
        }

        onCheckedChange?.(effect.checked);
      }
    }
  }
</script>

<label
  class="poodle-radio"
  data-disabled={disabled}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  style={radioStyles}
>
  <input
    {id}
    {name}
    {value}
    class="poodle-radio__control"
    type="radio"
    checked={currentChecked}
    disabled={disabled}
    aria-label={label ? undefined : ariaLabel ?? undefined}
    aria-describedby={describedBy ?? undefined}
    onchange={handleChange}
  />
  <span class="poodle-radio__indicator" aria-hidden="true">
    <span class="poodle-radio__dot"></span>
  </span>
  {#if label}
    <span class="poodle-radio__label">{label}</span>
  {/if}
</label>

<style>
  .poodle-radio {
    --poodle-radio-selected-color: var(--poodle-color-accent-base);
    position: relative;
    display: inline-flex;
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    color: var(--poodle-color-text-primary);
    cursor: pointer;
  }

  .poodle-radio[data-disabled="true"] {
    opacity: var(--poodle-state-opacity-disabled);
    cursor: not-allowed;
  }

  .poodle-radio__control {
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

  .poodle-radio__indicator {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: calc(var(--poodle-size-icon-md) + 0.125rem);
    height: calc(var(--poodle-size-icon-md) + 0.125rem);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: 999px;
    background: var(--poodle-color-background-surface);
    transition:
      border-color var(--poodle-motion-duration-interaction)
        var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction)
        var(--poodle-motion-easing-standard);
  }

  .poodle-radio__dot {
    width: calc(var(--poodle-size-icon-md) * 0.5);
    height: calc(var(--poodle-size-icon-md) * 0.5);
    border-radius: 999px;
    background: transparent;
    transition: background var(--poodle-motion-duration-interaction)
      var(--poodle-motion-easing-standard);
  }

  .poodle-radio__control:checked + .poodle-radio__indicator {
    border-color: var(--poodle-radio-selected-color);
  }

  .poodle-radio__control:checked + .poodle-radio__indicator .poodle-radio__dot {
    background: var(--poodle-radio-selected-color);
  }

  .poodle-radio__control:focus-visible + .poodle-radio__indicator {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-radio__label {
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    font-weight: var(--poodle-typography-label-weight);
    line-height: var(--poodle-typography-label-lineHeight);
  }

  .poodle-radio[data-density="compact"] {
    gap: 0.375rem;
  }

  .poodle-radio[data-density="comfortable"] {
    gap: var(--poodle-space-inline-md);
  }

  .poodle-radio[data-size="xs"] .poodle-radio__indicator {
    width: calc(var(--poodle-size-icon-xs) + 0.25rem);
    height: calc(var(--poodle-size-icon-xs) + 0.25rem);
  }

  .poodle-radio[data-size="xs"] .poodle-radio__dot {
    width: 0.4rem;
    height: 0.4rem;
  }

  .poodle-radio[data-size="sm"] .poodle-radio__indicator {
    width: calc(var(--poodle-size-icon-sm) + 0.25rem);
    height: calc(var(--poodle-size-icon-sm) + 0.25rem);
  }

  .poodle-radio[data-size="sm"] .poodle-radio__dot {
    width: 0.45rem;
    height: 0.45rem;
  }

  .poodle-radio[data-size="lg"] .poodle-radio__indicator {
    width: calc(var(--poodle-size-icon-lg) + 0.125rem);
    height: calc(var(--poodle-size-icon-lg) + 0.125rem);
  }

  .poodle-radio[data-size="lg"] .poodle-radio__dot {
    width: 0.55rem;
    height: 0.55rem;
  }

  .poodle-radio[data-size="xl"] .poodle-radio__indicator {
    width: calc(var(--poodle-size-icon-xl) + 0.125rem);
    height: calc(var(--poodle-size-icon-xl) + 0.125rem);
  }

  .poodle-radio[data-size="xl"] .poodle-radio__dot {
    width: 0.6rem;
    height: 0.6rem;
  }
</style>
