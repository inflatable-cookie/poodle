<script module lang="ts">
  let triStateSwitchInstanceCount = 0;
</script>

<script lang="ts">
  import { singleSelectTransition } from "@poodle/headless";

  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type {
    ControlDensity,
    ControlSize,
    SemanticControlSizeRole,
    TriStateValue,
  } from "./types";

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

<style>
  .poodle-tri-state-switch {
    --poodle-tri-state-excluded-color: var(--poodle-color-status-danger);
    --poodle-tri-state-default-color: var(--poodle-color-text-primary);
    --poodle-tri-state-included-color: var(--poodle-color-status-success);
    --poodle-tri-state-excluded-track: color-mix(in srgb, var(--poodle-tri-state-excluded-color) 14%, color-mix(in srgb, var(--poodle-color-background-canvas) 70%, black));
    --poodle-tri-state-default-track: color-mix(in srgb, var(--poodle-tri-state-default-color) 8%, color-mix(in srgb, var(--poodle-color-background-canvas) 70%, black));
    --poodle-tri-state-included-track: color-mix(in srgb, var(--poodle-tri-state-included-color) 14%, color-mix(in srgb, var(--poodle-color-background-canvas) 70%, black));
    --poodle-tri-state-height: var(--poodle-size-control-height);
    --poodle-tri-state-x: var(--poodle-space-control-x);
    --poodle-tri-state-track-inset: 0.125rem;
    --poodle-tri-state-min-content-width: 3rem;
    position: relative;
    display: inline-grid;
    width: max-content;
    grid-template-columns: repeat(3, minmax(0, 1fr));
    align-items: stretch;
    padding: var(--poodle-tri-state-track-inset);
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: 999px;
    background: color-mix(in srgb, var(--poodle-color-background-canvas) 75%, black);
    color: var(--poodle-color-text-primary);
    isolation: isolate;
  }

  .poodle-tri-state-switch[data-size="xs"] {
    --poodle-tri-state-height: 1.5rem;
    --poodle-tri-state-min-content-width: 2.5rem;
  }

  .poodle-tri-state-switch[data-size="sm"] {
    --poodle-tri-state-height: 1.75rem;
    --poodle-tri-state-min-content-width: 2.625rem;
  }

  .poodle-tri-state-switch[data-size="md"] {
    --poodle-tri-state-height: 2.25rem;
    --poodle-tri-state-min-content-width: 3rem;
  }

  .poodle-tri-state-switch[data-size="lg"] {
    --poodle-tri-state-height: 2.75rem;
    --poodle-tri-state-min-content-width: 3.375rem;
  }

  .poodle-tri-state-switch[data-size="xl"] {
    --poodle-tri-state-height: 3.25rem;
    --poodle-tri-state-min-content-width: 3.75rem;
  }

  .poodle-tri-state-switch[data-density="compact"] {
    --poodle-tri-state-x: 0.5rem;
    --poodle-tri-state-track-inset: 0.0625rem;
  }

  .poodle-tri-state-switch[data-density="default"] {
    --poodle-tri-state-x: 0.75rem;
    --poodle-tri-state-track-inset: 0.125rem;
  }

  .poodle-tri-state-switch[data-density="comfortable"] {
    --poodle-tri-state-x: 1rem;
    --poodle-tri-state-track-inset: 0.1875rem;
  }

  .poodle-tri-state-switch[data-disabled="true"] {
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-tri-state-switch__selection {
    position: absolute;
    top: var(--poodle-tri-state-track-inset);
    bottom: var(--poodle-tri-state-track-inset);
    left: var(--poodle-tri-state-track-inset);
    width: calc((100% - (var(--poodle-tri-state-track-inset) * 2)) / 3);
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

  .poodle-tri-state-switch[data-state="excluded"] .poodle-tri-state-switch__selection {
    background: var(--poodle-tri-state-excluded-track);
    border-color: color-mix(in srgb, var(--poodle-tri-state-excluded-color) 58%, var(--poodle-color-border-default));
  }

  .poodle-tri-state-switch[data-state="default"] .poodle-tri-state-switch__selection {
    background: var(--poodle-tri-state-default-track);
    border-color: var(--poodle-color-border-default);
  }

  .poodle-tri-state-switch[data-state="included"] .poodle-tri-state-switch__selection {
    background: var(--poodle-tri-state-included-track);
    border-color: color-mix(in srgb, var(--poodle-tri-state-included-color) 58%, var(--poodle-color-border-default));
  }

  .poodle-tri-state-switch__option {
    position: relative;
    z-index: 1;
    cursor: pointer;
  }

  .poodle-tri-state-switch[data-disabled="true"] .poodle-tri-state-switch__option {
    cursor: not-allowed;
  }

  .poodle-tri-state-switch__control {
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

  .poodle-tri-state-switch__segment {
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: calc(var(--poodle-tri-state-height) - (var(--poodle-tri-state-track-inset) * 2));
    min-width: calc(var(--poodle-tri-state-min-content-width) + (var(--poodle-tri-state-x) * 2));
    padding: 0 var(--poodle-tri-state-x);
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

  .poodle-tri-state-switch__option[data-selected="true"] .poodle-tri-state-switch__segment {
    color: var(--poodle-color-text-primary);
  }

  .poodle-tri-state-switch__option[data-state="excluded"][data-selected="true"] .poodle-tri-state-switch__segment {
    color: var(--poodle-tri-state-excluded-color);
  }

  .poodle-tri-state-switch__option[data-state="default"][data-selected="true"] .poodle-tri-state-switch__segment {
    color: var(--poodle-color-text-primary);
  }

  .poodle-tri-state-switch__option[data-state="included"][data-selected="true"] .poodle-tri-state-switch__segment {
    color: var(--poodle-tri-state-included-color);
  }

  .poodle-tri-state-switch__control:focus-visible + .poodle-tri-state-switch__segment {
    outline: var(--poodle-border-width-focus) solid var(--poodle-color-accent-focusRing);
    outline-offset: 0.125rem;
  }
</style>
