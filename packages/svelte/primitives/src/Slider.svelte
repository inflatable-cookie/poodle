<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { clamp, joinStyles, snapToStep } from "./internal";

  import type { Orientation } from "./types";

  export let value = 0;
  export let min = 0;
  export let max = 100;
  export let step = 1;
  export let orientation: Orientation = "horizontal";
  export let isDisabled = false;
  export let ariaLabel: string | null = null;
  export let valueText: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: number };
    valueCommit: { value: number };
  }>();

  $: safeMax = max <= min ? min + 1 : max;
  $: safeValue = clamp(snapToStep(value, min, step), min, safeMax);
  $: percentage = ((safeValue - min) / (safeMax - min)) * 100;
  $: sliderStyle = joinStyles([`--flint-slider-percent: ${percentage}%`]);

  function parseValue(nextRawValue: string): number {
    return clamp(snapToStep(Number(nextRawValue), min, step), min, safeMax);
  }
</script>

<div class="slider" data-orientation={orientation} data-disabled={isDisabled} style={sliderStyle}>
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
    disabled={isDisabled}
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
    opacity: var(--flint-state-opacity-disabled);
  }

  .slider__track {
    position: absolute;
    inset: 50% 0 0;
    height: 0.375rem;
    transform: translateY(-50%);
    border-radius: 999px;
    background: color-mix(in srgb, var(--flint-color-background-surface) 88%, transparent);
  }

  .slider[data-orientation="vertical"] .slider__track {
    inset: 0 auto 0 50%;
    width: 0.375rem;
    height: 100%;
    transform: translateX(-50%);
  }

  .slider__fill {
    display: block;
    width: var(--flint-slider-percent);
    height: 100%;
    border-radius: inherit;
    background: var(--flint-color-accent-base);
  }

  .slider[data-orientation="vertical"] .slider__fill {
    position: absolute;
    bottom: 0;
    width: 100%;
    height: var(--flint-slider-percent);
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
    border: 0.0625rem solid var(--flint-color-border-default);
    border-radius: 999px;
    background: var(--flint-color-background-elevated);
    box-shadow: 0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent);
    appearance: none;
  }

  .slider__control::-moz-range-thumb {
    width: 1rem;
    height: 1rem;
    border: 0.0625rem solid var(--flint-color-border-default);
    border-radius: 999px;
    background: var(--flint-color-background-elevated);
    box-shadow: 0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent);
  }

  .slider__control:focus-visible {
    outline: none;
  }

  .slider__control:focus-visible::-webkit-slider-thumb {
    box-shadow:
      0 0 0 0.1875rem color-mix(in srgb, var(--flint-color-accent-focusRing) 32%, transparent),
      0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent);
  }

  .slider__control:focus-visible::-moz-range-thumb {
    box-shadow:
      0 0 0 0.1875rem color-mix(in srgb, var(--flint-color-accent-focusRing) 32%, transparent),
      0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent);
  }
</style>
