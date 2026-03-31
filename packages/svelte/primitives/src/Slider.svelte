<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { clamp, joinStyles, snapToStep } from "./internal";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, Orientation, SemanticControlSizeRole } from "./types";

  export let size: ControlSize | null = null;
  export let sizeRole: SemanticControlSizeRole = "control";
  export let density: ControlDensity | null = null;

  export let value = 0;
  export let min = 0;
  export let max = 100;
  export let step = 1;
  export let orientation: Orientation = "horizontal";
  export let disabled = false;
  export let ariaLabel: string | null = null;
  export let valueText: string | null = null;

  const uiPresentation = getUiPresentation();

  const dispatch = createEventDispatcher<{
    valueChange: { value: number };
    valueCommit: { value: number };
  }>();

  $: resolvedSize = size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole);
  $: resolvedDensity = density ?? $uiPresentation.density;
  $: safeMax = max <= min ? min + 1 : max;
  $: safeValue = clamp(snapToStep(value, min, step), min, safeMax);
  $: percentage = ((safeValue - min) / (safeMax - min)) * 100;
  $: sliderStyle = joinStyles([`--poodle-slider-percent: ${percentage}`]);

  function parseValue(nextRawValue: string): number {
    return clamp(snapToStep(Number(nextRawValue), min, step), min, safeMax);
  }
</script>

<div class="slider" data-orientation={orientation} data-disabled={disabled} style={sliderStyle} data-size={resolvedSize} data-density={resolvedDensity}>
  <span class="slider__track" aria-hidden="true">
    <span class="slider__fill"></span>
  </span>
  <input
    class="slider__control"
    type="range"
    min={min}
    max={safeMax}
    {step}
    value={safeValue}
    disabled={disabled}
    aria-label={ariaLabel ?? undefined}
    aria-valuetext={valueText ?? undefined}
    on:input={(event) =>
      dispatch("valueChange", { value: parseValue((event.currentTarget as HTMLInputElement).value) })}
    on:change={(event) =>
      dispatch("valueCommit", { value: parseValue((event.currentTarget as HTMLInputElement).value) })}
  />
</div>

<style>
  .slider {
    position: relative;
    display: inline-flex;
    align-items: center;
    width: 100%;
    min-height: 1.5rem;
  }

  .slider[data-orientation="vertical"] {
    width: 1.5rem;
    min-width: 1.5rem;
    min-height: 10rem;
    height: 100%;
    justify-content: center;
  }

  .slider[data-disabled="true"] {
    opacity: var(--poodle-state-opacity-disabled);
  }

  .slider__track {
    position: absolute;
    inset: 50% 0 0;
    height: 0.375rem;
    transform: translateY(-50%);
    border-radius: 999px;
    background: color-mix(in srgb, var(--poodle-color-background-surface) 88%, transparent);
  }

  .slider[data-orientation="vertical"] .slider__track {
    inset: 0 auto 0 50%;
    width: 0.375rem;
    height: 100%;
    transform: translateX(-50%);
  }

  .slider__fill {
    --_thumb: 1rem;
    --_half: calc(var(--_thumb) / 2);
    --_track: calc(100% - var(--_thumb));
    display: block;
    width: calc(var(--_half) + var(--_track) * var(--poodle-slider-percent) / 100);
    height: 100%;
    border-radius: inherit;
    background: var(--poodle-color-accent-base);
  }

  .slider[data-orientation="vertical"] .slider__fill {
    position: absolute;
    bottom: 0;
    width: 100%;
    height: calc(var(--_half) + var(--_track) * var(--poodle-slider-percent) / 100);
  }

  .slider__control {
    position: relative;
    z-index: 1;
    width: 100%;
    margin: 0;
    background: transparent;
    appearance: none;
  }

  .slider[data-orientation="vertical"] .slider__control {
    width: 10rem;
    transform: rotate(-90deg);
  }

  .slider__control::-webkit-slider-runnable-track {
    height: 0.375rem;
    background: transparent;
  }

  .slider__control::-moz-range-track {
    height: 0.375rem;
    background: transparent;
  }

  .slider__control::-webkit-slider-thumb {
    width: 1rem;
    height: 1rem;
    margin-top: -0.3125rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: 999px;
    background: var(--poodle-color-background-elevated);
    box-shadow: 0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent);
    appearance: none;
  }

  .slider__control::-moz-range-thumb {
    width: 1rem;
    height: 1rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: 999px;
    background: var(--poodle-color-background-elevated);
    box-shadow: 0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent);
  }

  .slider__control:focus-visible {
    outline: none;
  }

  .slider__control:focus-visible::-webkit-slider-thumb {
    box-shadow:
      0 0 0 0.1875rem color-mix(in srgb, var(--poodle-color-accent-focusRing) 32%, transparent),
      0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent);
  }

  .slider__control:focus-visible::-moz-range-thumb {
    box-shadow:
      0 0 0 0.1875rem color-mix(in srgb, var(--poodle-color-accent-focusRing) 32%, transparent),
      0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent);
  }

  /* Size variants */
  .slider[data-size="xs"] { min-height: 1.25rem; }
  .slider[data-size="xs"] .slider__control::-webkit-slider-thumb { width: 0.75rem; height: 0.75rem; margin-top: -0.1875rem; }
  .slider[data-size="xs"] .slider__control::-moz-range-thumb { width: 0.75rem; height: 0.75rem; }
  .slider[data-size="sm"] { min-height: 1.375rem; }
  .slider[data-size="sm"] .slider__control::-webkit-slider-thumb { width: 0.875rem; height: 0.875rem; margin-top: -0.25rem; }
  .slider[data-size="sm"] .slider__control::-moz-range-thumb { width: 0.875rem; height: 0.875rem; }
  .slider[data-size="lg"] .slider__control::-webkit-slider-thumb { width: 1.125rem; height: 1.125rem; margin-top: -0.375rem; }
  .slider[data-size="lg"] .slider__control::-moz-range-thumb { width: 1.125rem; height: 1.125rem; }
  .slider[data-size="xl"] .slider__control::-webkit-slider-thumb { width: 1.25rem; height: 1.25rem; margin-top: -0.4375rem; }
  .slider[data-size="xl"] .slider__control::-moz-range-thumb { width: 1.25rem; height: 1.25rem; }

  /* Density variants */
  .slider[data-density="compact"] { padding: 0.25rem 0; }
  .slider[data-density="comfortable"] { padding: 0.75rem 0; }
</style>
