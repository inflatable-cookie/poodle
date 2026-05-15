<script lang="ts">
  import { clamp, joinStyles, snapToStep } from "./internal";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, Orientation, SemanticControlSizeRole } from "./types";

  interface Props {
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    value?: number;
    min?: number;
    max?: number;
    step?: number;
    orientation?: Orientation;
    disabled?: boolean;
    ariaLabel?: string | null;
    valueText?: string | null;
    onValueChange?: ((value: number) => void) | undefined;
    onValueCommit?: ((value: number) => void) | undefined;
  }

  let {
    size = null,
    sizeRole = "control",
    density = null,
    value = $bindable(0),
    min = 0,
    max = 100,
    step = 1,
    orientation = "horizontal",
    disabled = false,
    ariaLabel = null,
    valueText = null,
    onValueChange = undefined,
    onValueCommit = undefined,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const safeMax = $derived(max <= min ? min + 1 : max);
  const displayValue = $derived(clamp(snapToStep(value, min, step), min, safeMax));
  const percentage = $derived(((displayValue - min) / (safeMax - min)) * 100);
  const sliderStyle = $derived(joinStyles([`--poodle-slider-percent: ${percentage}%`]));

  function handleInput(event: Event): void {
    const next = clamp(snapToStep(Number((event.currentTarget as HTMLInputElement).value), min, step), min, safeMax);
    value = next;
    onValueChange?.(next);
  }

  function handleChange(event: Event): void {
    const next = clamp(snapToStep(Number((event.currentTarget as HTMLInputElement).value), min, step), min, safeMax);
    value = next;
    onValueCommit?.(next);
  }
</script>

<div class="poodle-slider" data-orientation={orientation} data-disabled={disabled} style={sliderStyle} data-size={resolvedSize} data-density={resolvedDensity}>
  <span class="poodle-slider__track" aria-hidden="true">
    <span class="poodle-slider__fill"></span>
  </span>
  <input
    class="poodle-slider__control"
    type="range"
    min={min}
    max={safeMax}
    {step}
    value={displayValue}
    disabled={disabled}
    aria-label={ariaLabel ?? undefined}
    aria-valuetext={valueText ?? undefined}
    oninput={handleInput}
    onchange={handleChange}
  />
</div>

<style>
  .poodle-slider {
    position: relative;
    display: inline-flex;
    align-items: center;
    width: 100%;
    min-height: 1.5rem;
  }

  .poodle-slider[data-orientation="vertical"] {
    width: 1.5rem;
    min-width: 1.5rem;
    min-height: 10rem;
    height: 100%;
    justify-content: center;
  }

  .poodle-slider[data-disabled="true"] {
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-slider__track {
    position: absolute;
    inset: 50% 0 0;
    height: 0.375rem;
    transform: translateY(-50%);
    border-radius: 999px;
    background: color-mix(in srgb, var(--poodle-color-background-surface) 88%, transparent);
  }

  .poodle-slider[data-orientation="vertical"] .poodle-slider__track {
    inset: 0 auto 0 50%;
    width: 0.375rem;
    height: 100%;
    transform: translateX(-50%);
  }

  .poodle-slider__fill {
    display: block;
    width: var(--poodle-slider-percent);
    height: 100%;
    border-radius: inherit;
    background: var(--poodle-color-accent-base);
  }

  .poodle-slider[data-orientation="vertical"] .poodle-slider__fill {
    position: absolute;
    bottom: 0;
    width: 100%;
    height: var(--poodle-slider-percent);
  }

  .poodle-slider__control {
    position: relative;
    z-index: 1;
    width: 100%;
    margin: 0;
    background: transparent;
    appearance: none;
  }

  .poodle-slider[data-orientation="vertical"] .poodle-slider__control {
    width: 10rem;
    transform: rotate(-90deg);
  }

  .poodle-slider__control::-webkit-slider-runnable-track {
    height: 0.375rem;
    background: transparent;
  }

  .poodle-slider__control::-moz-range-track {
    height: 0.375rem;
    background: transparent;
  }

  .poodle-slider__control::-webkit-slider-thumb {
    width: 1rem;
    height: 1rem;
    margin-top: -0.3125rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: 999px;
    background: var(--poodle-color-background-elevated);
    box-shadow: 0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent);
    appearance: none;
  }

  .poodle-slider__control::-moz-range-thumb {
    width: 1rem;
    height: 1rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: 999px;
    background: var(--poodle-color-background-elevated);
    box-shadow: 0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent);
  }

  .poodle-slider__control:focus-visible {
    outline: none;
  }

  .poodle-slider__control:focus-visible::-webkit-slider-thumb {
    box-shadow:
      0 0 0 0.1875rem color-mix(in srgb, var(--poodle-color-accent-focusRing) 32%, transparent),
      0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent);
  }

  .poodle-slider__control:focus-visible::-moz-range-thumb {
    box-shadow:
      0 0 0 0.1875rem color-mix(in srgb, var(--poodle-color-accent-focusRing) 32%, transparent),
      0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent);
  }

  /* Size variants */
  .poodle-slider[data-size="xs"] { min-height: 1.25rem; }
  .poodle-slider[data-size="xs"] .poodle-slider__control::-webkit-slider-thumb { width: 0.75rem; height: 0.75rem; margin-top: -0.1875rem; }
  .poodle-slider[data-size="xs"] .poodle-slider__control::-moz-range-thumb { width: 0.75rem; height: 0.75rem; }
  .poodle-slider[data-size="sm"] { min-height: 1.375rem; }
  .poodle-slider[data-size="sm"] .poodle-slider__control::-webkit-slider-thumb { width: 0.875rem; height: 0.875rem; margin-top: -0.25rem; }
  .poodle-slider[data-size="sm"] .poodle-slider__control::-moz-range-thumb { width: 0.875rem; height: 0.875rem; }
  .poodle-slider[data-size="lg"] .poodle-slider__control::-webkit-slider-thumb { width: 1.125rem; height: 1.125rem; margin-top: -0.375rem; }
  .poodle-slider[data-size="lg"] .poodle-slider__control::-moz-range-thumb { width: 1.125rem; height: 1.125rem; }
  .poodle-slider[data-size="xl"] .poodle-slider__control::-webkit-slider-thumb { width: 1.25rem; height: 1.25rem; margin-top: -0.4375rem; }
  .poodle-slider[data-size="xl"] .poodle-slider__control::-moz-range-thumb { width: 1.25rem; height: 1.25rem; }

  /* Density variants */
  .poodle-slider[data-density="compact"] { padding: 0.25rem 0; }
  .poodle-slider[data-density="comfortable"] { padding: 0.75rem 0; }
</style>
