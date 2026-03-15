<script lang="ts">
  import { createEventDispatcher } from "svelte";

  import { clamp, joinStyles, snapToStep } from "./internal";

  import type { Orientation } from "./types";

  export let value: [number, number] = [0, 100];
  export let min = 0;
  export let max = 100;
  export let step = 1;
  export let orientation: Orientation = "horizontal";
  export let isDisabled = false;
  export let ariaLabel: string | null = null;
  export let lowerValueText: string | null = null;
  export let upperValueText: string | null = null;

  const dispatch = createEventDispatcher<{
    valueChange: { value: [number, number] };
    valueCommit: { value: [number, number] };
  }>();

  $: safeMax = max <= min ? min + 1 : max;
  $: lowerValue = clamp(Math.min(value[0], value[1]), min, safeMax);
  $: upperValue = clamp(Math.max(value[0], value[1]), min, safeMax);
  $: lowerPercent = ((lowerValue - min) / (safeMax - min)) * 100;
  $: upperPercent = ((upperValue - min) / (safeMax - min)) * 100;
  $: rangeStyle = joinStyles([
    `--pug-range-start: ${lowerPercent}%`,
    `--pug-range-end: ${upperPercent}%`,
  ]);

  function normalizeLower(nextValue: number): [number, number] {
    const snapped = clamp(snapToStep(nextValue, min, step), min, upperValue);
    return [snapped, upperValue];
  }

  function normalizeUpper(nextValue: number): [number, number] {
    const snapped = clamp(snapToStep(nextValue, min, step), lowerValue, safeMax);
    return [lowerValue, snapped];
  }
</script>

<div class="range-slider" data-orientation={orientation} data-disabled={isDisabled} style={rangeStyle}>
  <span class="range-slider__track" aria-hidden="true">
    <span class="range-slider__fill"></span>
  </span>

  <input
    class="range-slider__control range-slider__control--lower"
    type="range"
    min={min}
    max={safeMax}
    {step}
    value={lowerValue}
    disabled={isDisabled}
    aria-label={ariaLabel ? `${ariaLabel} minimum` : "Minimum value"}
    aria-valuetext={lowerValueText ?? undefined}
    on:input={(event) =>
      dispatch("valueChange", {
        value: normalizeLower(Number((event.currentTarget as HTMLInputElement).value)),
      })}
    on:change={(event) =>
      dispatch("valueCommit", {
        value: normalizeLower(Number((event.currentTarget as HTMLInputElement).value)),
      })}
  />

  <input
    class="range-slider__control range-slider__control--upper"
    type="range"
    min={min}
    max={safeMax}
    {step}
    value={upperValue}
    disabled={isDisabled}
    aria-label={ariaLabel ? `${ariaLabel} maximum` : "Maximum value"}
    aria-valuetext={upperValueText ?? undefined}
    on:input={(event) =>
      dispatch("valueChange", {
        value: normalizeUpper(Number((event.currentTarget as HTMLInputElement).value)),
      })}
    on:change={(event) =>
      dispatch("valueCommit", {
        value: normalizeUpper(Number((event.currentTarget as HTMLInputElement).value)),
      })}
  />
</div>

<style>
  .range-slider {
    position: relative;
    display: inline-flex;
    align-items: center;
    width: 100%;
    min-height: 1.5rem;
  }

  .range-slider[data-orientation="vertical"] {
    width: 1.5rem;
    min-width: 1.5rem;
    min-height: 10rem;
    height: 100%;
    justify-content: center;
  }

  .range-slider[data-disabled="true"] {
    opacity: var(--pug-state-opacity-disabled);
  }

  .range-slider__track {
    position: absolute;
    inset: 50% 0 0;
    height: 0.375rem;
    transform: translateY(-50%);
    border-radius: 999px;
    background: color-mix(in srgb, var(--pug-color-background-surface) 88%, transparent);
  }

  .range-slider[data-orientation="vertical"] .range-slider__track {
    inset: 0 auto 0 50%;
    width: 0.375rem;
    height: 100%;
    transform: translateX(-50%);
  }

  .range-slider__fill {
    position: absolute;
    left: var(--pug-range-start);
    width: calc(var(--pug-range-end) - var(--pug-range-start));
    height: 100%;
    border-radius: inherit;
    background: var(--pug-color-accent-base);
  }

  .range-slider[data-orientation="vertical"] .range-slider__fill {
    left: 0;
    bottom: var(--pug-range-start);
    width: 100%;
    height: calc(var(--pug-range-end) - var(--pug-range-start));
  }

  .range-slider__control {
    position: absolute;
    top: 50%;
    left: 0;
    right: 0;
    transform: translateY(-50%);
    width: 100%;
    margin: 0;
    background: transparent;
    appearance: none;
    pointer-events: none;
  }

  .range-slider[data-orientation="vertical"] .range-slider__control {
    width: 10rem;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%) rotate(-90deg);
  }

  .range-slider__control::-webkit-slider-runnable-track {
    height: 0.375rem;
    background: transparent;
  }

  .range-slider__control::-moz-range-track {
    height: 0.375rem;
    background: transparent;
  }

  .range-slider__control::-webkit-slider-thumb {
    width: 1rem;
    height: 1rem;
    margin-top: -0.3125rem;
    border: 0.0625rem solid var(--pug-color-border-default);
    border-radius: 999px;
    background: var(--pug-color-background-elevated);
    box-shadow: 0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent);
    appearance: none;
    pointer-events: auto;
  }

  .range-slider__control::-moz-range-thumb {
    width: 1rem;
    height: 1rem;
    border: 0.0625rem solid var(--pug-color-border-default);
    border-radius: 999px;
    background: var(--pug-color-background-elevated);
    box-shadow: 0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent);
    pointer-events: auto;
  }

  .range-slider__control:focus-visible {
    outline: none;
  }

  .range-slider__control:focus-visible::-webkit-slider-thumb {
    box-shadow:
      0 0 0 0.1875rem color-mix(in srgb, var(--pug-color-accent-focusRing) 32%, transparent),
      0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent);
  }

  .range-slider__control:focus-visible::-moz-range-thumb {
    box-shadow:
      0 0 0 0.1875rem color-mix(in srgb, var(--pug-color-accent-focusRing) 32%, transparent),
      0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent);
  }
</style>
