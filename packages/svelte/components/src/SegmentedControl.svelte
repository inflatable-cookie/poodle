<script module lang="ts">
  let nextSegmentedControlId = 0;
</script>

<script lang="ts">
  import { singleSelectTransition } from "@poodle/headless";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type {
    ControlDensity,
    ControlSize,
    SegmentedControlOption,
    SemanticControlSizeRole,
  } from "./types";

  interface Props {
    value?: string | null | undefined;
    defaultValue?: string | null;
    options?: SegmentedControlOption[];
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    disabled?: boolean;
    ariaLabel?: string | null;
    name?: string | undefined;
    equalWidth?: boolean;
    onValueChange?: ((value: string) => void) | undefined;
  }

  let {
    value = $bindable<string | null | undefined>(undefined),
    defaultValue = null,
    options = [],
    size = null,
    sizeRole = "control",
    density = null,
    disabled = false,
    ariaLabel = null,
    name = undefined,
    equalWidth = true,
    onValueChange = undefined,
  }: Props = $props();

  const generatedName = `poodle-segmented-control-${++nextSegmentedControlId}`;
  let uncontrolledValue = $state<string | null>(null);
  let seededDefaultValue = $state(false);
  const uiPresentation = getUiPresentation();

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
  class="poodle-segmented-control"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-equal-width={equalWidth}
  role="radiogroup"
  aria-label={ariaLabel ?? undefined}
>
  {#each options as option (option.value)}
    <label class="poodle-segmented-control__segment" data-selected={currentValue === option.value} title={option.title ?? undefined}>
      <input
        class="poodle-segmented-control__control"
        type="radio"
        name={name ?? generatedName}
        value={option.value}
        checked={currentValue === option.value}
        disabled={disabled || option.disabled === true}
        aria-label={option.ariaLabel ?? undefined}
        onchange={() => handleChange(option.value)}
      />
      <span class="poodle-segmented-control__label">{option.label}</span>
    </label>
  {/each}
</div>

<style>
  .poodle-segmented-control {
    --poodle-segmented-control-height: var(--poodle-size-control-height);
    --poodle-segmented-control-x: var(--poodle-space-control-x);
    display: grid;
    grid-auto-flow: column;
    grid-auto-columns: minmax(0, 1fr);
    gap: 0.125rem;
    padding: 0.125rem;
    border: 0.0625rem solid var(
      --poodle-treatment-interactive-border,
      color-mix(in srgb, var(--poodle-color-border-subtle) 84%, transparent)
    );
    border-radius: var(--poodle-treatment-interactive-radius, var(--poodle-radius-control));
    background: var(
      --poodle-treatment-interactive-fill,
      color-mix(in srgb, var(--poodle-surface) 93%, var(--poodle-color-text-primary))
    );
    box-shadow: var(--poodle-treatment-interactive-shadow, none);
  }

  .poodle-segmented-control[data-equal-width="false"] {
    width: fit-content;
    grid-auto-columns: max-content;
    justify-content: start;
  }

  .poodle-segmented-control[data-size="xs"] {
    --poodle-segmented-control-height: 1.5rem;
  }

  .poodle-segmented-control[data-size="sm"] {
    --poodle-segmented-control-height: 1.75rem;
  }

  .poodle-segmented-control[data-size="md"] {
    --poodle-segmented-control-height: 2.25rem;
  }

  .poodle-segmented-control[data-size="lg"] {
    --poodle-segmented-control-height: 2.75rem;
  }

  .poodle-segmented-control[data-size="xl"] {
    --poodle-segmented-control-height: 3.25rem;
  }

  .poodle-segmented-control[data-density="compact"] {
    --poodle-segmented-control-x: 0.5rem;
  }

  .poodle-segmented-control[data-density="default"] {
    --poodle-segmented-control-x: 0.75rem;
  }

  .poodle-segmented-control[data-density="comfortable"] {
    --poodle-segmented-control-x: 1rem;
  }

  .poodle-segmented-control__segment {
    position: relative;
    display: grid;
    min-width: 0;
    cursor: pointer;
  }

  .poodle-segmented-control__control {
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

  .poodle-segmented-control__label {
    display: block;
    min-width: 0;
    min-height: calc(var(--poodle-segmented-control-height) - 0.25rem);
    padding: 0 var(--poodle-segmented-control-x);
    border-radius: calc(var(--poodle-treatment-interactive-radius, var(--poodle-radius-control)) - 0.125rem);
    color: var(--poodle-color-text-secondary);
    font-family: var(--poodle-typography-label-family);
    font-size: 0.75rem;
    font-weight: 600;
    line-height: calc(var(--poodle-segmented-control-height) - 0.25rem);
    text-align: center;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    transition:
      background var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      color var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard),
      box-shadow var(--poodle-motion-duration-interaction) var(--poodle-motion-easing-standard);
  }

  .poodle-segmented-control__segment[data-selected="true"] .poodle-segmented-control__label {
    background: var(--poodle-color-accent-base);
    color: var(--poodle-color-text-inverse);
    box-shadow: inset 0 0.0625rem 0 color-mix(in srgb, white 12%, transparent);
  }

  .poodle-segmented-control__control:focus-visible + .poodle-segmented-control__label {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }

  .poodle-segmented-control__control:disabled + .poodle-segmented-control__label {
    opacity: var(--poodle-state-opacity-disabled);
    cursor: not-allowed;
  }
</style>
