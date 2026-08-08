<script module lang="ts">
  let nextRadioGroupId = 0;
</script>

<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/radio-group.css";
  import { singleSelectTransition } from "@inflatable-cookie/poodle-core";

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

