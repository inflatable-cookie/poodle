<script module lang="ts">
  let nextRadioGroupId = 0;
</script>

<script lang="ts">
  import { singleSelectTransition } from "@poodle/headless";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, Orientation, RadioGroupOption, SemanticControlSizeRole } from "./types";

  interface Props {
    value?: string | null | undefined;
    defaultValue?: string | null;
    options?: RadioGroupOption[];
    orientation?: Orientation;
    disabled?: boolean;
    ariaLabel?: string | null;
    describedBy?: string | null;
    name?: string | undefined;
    selectedColor?: string | null;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    onValueChange?: ((value: string) => void) | undefined;
  }

  let {
    value = $bindable<string | null | undefined>(undefined),
    defaultValue = null,
    options = [],
    orientation = "vertical",
    disabled = false,
    ariaLabel = null,
    describedBy = null,
    name = undefined,
    selectedColor = null,
    size = null,
    sizeRole = "control",
    density = null,
    onValueChange = undefined,
  }: Props = $props();

  const generatedName = `poodle-radio-group-${++nextRadioGroupId}`;
  const uiPresentation = getUiPresentation();
  let uncontrolledValue = $state<string | null>(null);
  let seededDefaultValue = $state(false);

  $effect.pre(() => {
    if (!seededDefaultValue) {
      uncontrolledValue = defaultValue;
      seededDefaultValue = true;
    }
  });

  const isControlled = $derived(value !== undefined);
  const currentValue = $derived(isControlled ? value : uncontrolledValue);
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const radioGroupStyles = $derived(selectedColor ? `--poodle-radio-selected-color: ${selectedColor}` : undefined);

  function handleChange(nextValue: string): void {
    const result = singleSelectTransition(
      {
        value: currentValue ?? null,
        options: options.map((option) => ({ value: option.value, disabled: disabled || option.disabled === true })),
        disabled,
      },
      { type: "SELECT", value: nextValue },
    );

    for (const effect of result.effects) {
      if (effect.type === "emitValueChange") {
        if (!isControlled) {
          uncontrolledValue = effect.value;
        } else {
          value = effect.value;
        }

        onValueChange?.(effect.value);
      }
    }
  }
</script>

<div
  class="poodle-radio-group"
  data-orientation={orientation}
  data-disabled={disabled}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  role="radiogroup"
  aria-label={ariaLabel ?? undefined}
  aria-describedby={describedBy ?? undefined}
  style={radioGroupStyles}
>
  {#each options as option (option.value)}
    <label class="poodle-radio-group__option" data-disabled={disabled || option.disabled === true}>
      <input
        class="poodle-radio-group__control"
        type="radio"
        name={name ?? generatedName}
        value={option.value}
        checked={currentValue === option.value}
        disabled={disabled || option.disabled === true}
        onchange={() => handleChange(option.value)}
      />
      <span class="poodle-radio-group__indicator" aria-hidden="true">
        <span class="poodle-radio-group__dot"></span>
      </span>
      <span class="poodle-radio-group__label">{option.label}</span>
    </label>
  {/each}
</div>

<style>
  .poodle-radio-group {
    --poodle-radio-selected-color: var(--poodle-color-accent-base);
    display: grid;
    gap: var(--poodle-space-stack-sm);
  }

  .poodle-radio-group[data-orientation="horizontal"] {
    grid-auto-flow: column;
    grid-auto-columns: minmax(0, max-content);
    gap: var(--poodle-space-inline-md);
    align-items: center;
  }

  .poodle-radio-group__option {
    position: relative;
    display: inline-grid;
    grid-template-columns: auto minmax(0, 1fr);
    align-items: center;
    gap: var(--poodle-space-inline-sm);
    min-width: 0;
    color: var(--poodle-recipe-radio-group-option-text, var(--poodle-color-text-primary));
    cursor: pointer;
  }

  .poodle-radio-group__option[data-disabled="true"] {
    cursor: not-allowed;
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-radio-group__control {
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

  .poodle-radio-group__indicator {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: calc(var(--poodle-size-icon-md) + 0.125rem);
    height: calc(var(--poodle-size-icon-md) + 0.125rem);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: 999px;
    background: var(--poodle-recipe-radio-group-indicator-fill, var(--poodle-color-background-surface));
    transition:
      border-color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-radio-group__dot {
    width: calc(var(--poodle-size-icon-md) * 0.5);
    height: calc(var(--poodle-size-icon-md) * 0.5);
    border-radius: 999px;
    background: var(--poodle-recipe-radio-group-dot-fill, transparent);
    transition: background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-radio-group__control:checked + .poodle-radio-group__indicator {
    border-color: var(--poodle-recipe-radio-group-indicator-border, var(--poodle-radio-selected-color));
  }

  .poodle-radio-group__control:checked + .poodle-radio-group__indicator .poodle-radio-group__dot {
    background: var(--poodle-recipe-radio-group-checked-dot-fill, var(--poodle-radio-selected-color));
  }

  .poodle-radio-group__control:focus-visible + .poodle-radio-group__indicator {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-radio-group__label {
    min-width: 0;
    font-family: var(--poodle-typography-label-family);
    font-size: var(--poodle-typography-label-size);
    font-weight: var(--poodle-typography-label-weight);
    line-height: var(--poodle-typography-label-lineHeight);
  }

  /* Density variants */
  .poodle-radio-group[data-density="compact"] {
    gap: var(--poodle-space-stack-sm);
  }

  .poodle-radio-group[data-density="comfortable"] {
    gap: var(--poodle-space-stack-lg);
  }

  /* Size variants */
  .poodle-radio-group[data-size="xs"] .poodle-radio-group__indicator {
    width: calc(var(--poodle-size-icon-xs) + 0.25rem);
    height: calc(var(--poodle-size-icon-xs) + 0.25rem);
  }

  .poodle-radio-group[data-size="xs"] .poodle-radio-group__dot {
    width: 0.4rem;
    height: 0.4rem;
  }

  .poodle-radio-group[data-size="sm"] .poodle-radio-group__indicator {
    width: calc(var(--poodle-size-icon-sm) + 0.25rem);
    height: calc(var(--poodle-size-icon-sm) + 0.25rem);
  }

  .poodle-radio-group[data-size="sm"] .poodle-radio-group__dot {
    width: 0.45rem;
    height: 0.45rem;
  }

  .poodle-radio-group[data-size="lg"] .poodle-radio-group__indicator {
    width: calc(var(--poodle-size-icon-lg) + 0.125rem);
    height: calc(var(--poodle-size-icon-lg) + 0.125rem);
  }

  .poodle-radio-group[data-size="lg"] .poodle-radio-group__dot {
    width: 0.55rem;
    height: 0.55rem;
  }

  .poodle-radio-group[data-size="xl"] .poodle-radio-group__indicator {
    width: calc(var(--poodle-size-icon-xl) + 0.125rem);
    height: calc(var(--poodle-size-icon-xl) + 0.125rem);
  }

  .poodle-radio-group[data-size="xl"] .poodle-radio-group__dot {
    width: 0.6rem;
    height: 0.6rem;
  }
</style>
