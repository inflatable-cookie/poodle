<script module lang="ts">
  let triStateSwitchInstanceCount = 0;
</script>

<script lang="ts">
  import "@poodle/styles/tri-state-switch.css";
  import { singleSelectTransition } from "@poodle/headless";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation.ts";

  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
    TriStateValue,
  } from "./types.ts";

  interface Props {
    value?: TriStateValue;
    options?: Record<TriStateValue, string>;
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    disabled?: boolean;
    ariaLabel: string;
    excludedColor?: string | null;
    defaultColor?: string | null;
    includedColor?: string | null;
    onValueChange?: ((value: TriStateValue) => void) | undefined;
  }

  let {
    value = $bindable<TriStateValue>("default"),
    options = {
      excluded: "Exclude",
      default: "Default",
      included: "Include",
    },
    size = null,
    sizeRole = "control",
    density = null,
    disabled = false,
    ariaLabel,
    excludedColor = null,
    defaultColor = null,
    includedColor = null,
    onValueChange = undefined,
  }: Props = $props();

  const orderedValues: TriStateValue[] = ["excluded", "default", "included"];
  const instanceId = ++triStateSwitchInstanceCount;
  const groupName = `poodle-tri-state-switch-${instanceId}`;
  const uiPresentation = getUiPresentation();

  const selectedIndex = $derived(Math.max(0, orderedValues.indexOf(value)));
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const triStateStyles = $derived([
    excludedColor ? `--poodle-tri-state-excluded-color: ${excludedColor}` : "",
    defaultColor ? `--poodle-tri-state-default-color: ${defaultColor}` : "",
    includedColor ? `--poodle-tri-state-included-color: ${includedColor}` : "",
    `--poodle-tri-state-active-index: ${selectedIndex}`,
  ].filter(Boolean).join("; ") || undefined);

  function handleSelect(nextValue: TriStateValue): void {
    const result = singleSelectTransition(
      { value, options: orderedValues.map((candidate) => ({ value: candidate })), disabled },
      { type: "SELECT", value: nextValue },
    );

    for (const effect of result.effects) {
      if (effect.type === "emitValueChange") {
        value = effect.value as TriStateValue;
        onValueChange?.(effect.value as TriStateValue);
      }
    }
  }
</script>

<div
  class="poodle-tri-state-switch"
  role="radiogroup"
  aria-label={ariaLabel}
  aria-disabled={disabled ? "true" : undefined}
  data-state={value}
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-disabled={disabled}
  style={triStateStyles}
>
  <span class="poodle-tri-state-switch__selection" aria-hidden="true"></span>

  {#each orderedValues as optionValue}
    <label
      class="poodle-tri-state-switch__option"
      data-state={optionValue}
      data-selected={value === optionValue}
    >
      <input
        class="poodle-tri-state-switch__control"
        type="radio"
        name={groupName}
        checked={value === optionValue}
        disabled={disabled}
        aria-label={options[optionValue]}
        onchange={() => handleSelect(optionValue)}
      />
      <span class="poodle-tri-state-switch__segment">{options[optionValue]}</span>
    </label>
  {/each}
</div>

