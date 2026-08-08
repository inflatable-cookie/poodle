<script module lang="ts">
  let nextSegmentedControlId = 0;
</script>

<script lang="ts">
  import "@poodle/styles/segmented-control.css";
  import { singleSelectTransition } from "@poodle/headless";

  import Icon from "./Icon.svelte";
  import {
    getUiPresentation,
    resolveSemanticControlSize,
    resolveSupportingVisualSize,
  } from "./presentation";

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
  const resolvedIconSize = $derived(resolveSupportingVisualSize(resolvedSize));
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
    <label
      class="poodle-segmented-control__segment"
      data-selected={currentValue === option.value}
      data-icon-only={option.iconOnly === true && option.icon ? "true" : undefined}
      title={option.title ?? (option.iconOnly && option.icon ? option.label : undefined)}
    >
      <input
        class="poodle-segmented-control__control"
        type="radio"
        name={name ?? generatedName}
        value={option.value}
        checked={currentValue === option.value}
        disabled={disabled || option.disabled === true}
        aria-label={option.ariaLabel ?? (option.iconOnly && option.icon ? option.label : undefined)}
        onchange={() => handleChange(option.value)}
      />
      <span class="poodle-segmented-control__label">
        {#if option.icon}
          <Icon icon={option.icon} size={resolvedIconSize} />
        {/if}
        {#if !option.iconOnly || !option.icon}
          <span class="poodle-segmented-control__label-text">{option.label}</span>
        {/if}
      </span>
    </label>
  {/each}
</div>
