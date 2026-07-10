<script lang="ts">
  import {
    normalizeRangeValue,
    rangeSliderTransition,
    safeSliderMax,
    type RangeSliderContext,
  } from "@poodle/headless";

  import { joinStyles } from "./internal";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";

  import type { ControlDensity, ControlSize, Orientation, SemanticControlSizeRole } from "./types";

  interface Props {
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    value?: [number, number];
    min?: number;
    max?: number;
    step?: number;
    orientation?: Orientation;
    disabled?: boolean;
    ariaLabel?: string | null;
    lowerValueText?: string | null;
    upperValueText?: string | null;
    onValueChange?: ((value: [number, number]) => void) | undefined;
    onValueCommit?: ((value: [number, number]) => void) | undefined;
  }

  let {
    size = null,
    sizeRole = "control",
    density = null,
    value = $bindable<[number, number]>([0, 100]),
    min = 0,
    max = 100,
    step = 1,
    orientation = "horizontal",
    disabled = false,
    ariaLabel = null,
    lowerValueText = null,
    upperValueText = null,
    onValueChange = undefined,
    onValueCommit = undefined,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const machineContext = $derived<RangeSliderContext>({ value, min, max, step, disabled });
  const safeMax = $derived(safeSliderMax(min, max));
  const displayRange = $derived(normalizeRangeValue(machineContext));
  const displayLower = $derived(displayRange[0]);
  const displayUpper = $derived(displayRange[1]);
  const lowerPercent = $derived(((displayLower - min) / (safeMax - min)) * 100);
  const upperPercent = $derived(((displayUpper - min) / (safeMax - min)) * 100);
  const rangeStyle = $derived(joinStyles([
    `--poodle-range-start: ${lowerPercent}%`,
    `--poodle-range-end: ${upperPercent}%`,
  ]));

  function send(type: "INPUT" | "COMMIT", thumb: "lower" | "upper", event: Event): void {
    const raw = Number((event.currentTarget as HTMLInputElement).value);
    const result = rangeSliderTransition(machineContext, { type, thumb, raw });

    for (const effect of result.effects) {
      value = effect.value;

      if (effect.type === "emitValueChange") {
        onValueChange?.(effect.value);
      } else if (effect.type === "emitValueCommit") {
        onValueCommit?.(effect.value);
      }
    }
  }
</script>

<div class="poodle-range-slider" data-orientation={orientation} data-disabled={disabled} style={rangeStyle} data-size={resolvedSize} data-density={resolvedDensity}>
  <span class="poodle-range-slider__track" aria-hidden="true">
    <span class="poodle-range-slider__fill"></span>
  </span>

  <input
    class="poodle-range-slider__control poodle-range-slider__control--lower"
    type="range"
    min={min}
    max={safeMax}
    {step}
    value={displayLower}
    disabled={disabled}
    aria-label={ariaLabel ? `${ariaLabel} minimum` : "Minimum value"}
    aria-valuetext={lowerValueText ?? undefined}
    oninput={(event) => send("INPUT", "lower", event)}
    onchange={(event) => send("COMMIT", "lower", event)}
  />

  <input
    class="poodle-range-slider__control poodle-range-slider__control--upper"
    type="range"
    min={min}
    max={safeMax}
    {step}
    value={displayUpper}
    disabled={disabled}
    aria-label={ariaLabel ? `${ariaLabel} maximum` : "Maximum value"}
    aria-valuetext={upperValueText ?? undefined}
    oninput={(event) => send("INPUT", "upper", event)}
    onchange={(event) => send("COMMIT", "upper", event)}
  />
</div>

<style>
  .poodle-range-slider {
    position: relative;
    display: inline-flex;
    align-items: center;
    width: 100%;
    min-height: 1.5rem;
  }

  .poodle-range-slider[data-orientation="vertical"] {
    width: 1.5rem;
    min-width: 1.5rem;
    min-height: 10rem;
    height: 100%;
    justify-content: center;
  }

  .poodle-range-slider[data-disabled="true"] {
    opacity: var(--poodle-state-opacity-disabled);
  }

  .poodle-range-slider__track {
    position: absolute;
    inset: 50% 0 0;
    height: 0.375rem;
    transform: translateY(-50%);
    border-radius: 999px;
    background: color-mix(in srgb, var(--poodle-color-background-surface) 88%, transparent);
  }

  .poodle-range-slider[data-orientation="vertical"] .poodle-range-slider__track {
    inset: 0 auto 0 50%;
    width: 0.375rem;
    height: 100%;
    transform: translateX(-50%);
  }

  .poodle-range-slider__fill {
    position: absolute;
    left: var(--poodle-range-start);
    width: calc(var(--poodle-range-end) - var(--poodle-range-start));
    height: 100%;
    border-radius: inherit;
    background: var(--poodle-color-accent-base);
  }

  .poodle-range-slider[data-orientation="vertical"] .poodle-range-slider__fill {
    left: 0;
    bottom: var(--poodle-range-start);
    width: 100%;
    height: calc(var(--poodle-range-end) - var(--poodle-range-start));
  }

  .poodle-range-slider__control {
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

  .poodle-range-slider[data-orientation="vertical"] .poodle-range-slider__control {
    width: 10rem;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%) rotate(-90deg);
  }

  .poodle-range-slider__control::-webkit-slider-runnable-track {
    height: 0.375rem;
    background: transparent;
  }

  .poodle-range-slider__control::-moz-range-track {
    height: 0.375rem;
    background: transparent;
  }

  .poodle-range-slider__control::-webkit-slider-thumb {
    width: 1rem;
    height: 1rem;
    margin-top: -0.3125rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: 999px;
    background: var(--poodle-color-background-elevated);
    box-shadow: 0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent);
    appearance: none;
    cursor: pointer;
    pointer-events: auto;
  }

  .poodle-range-slider__control::-moz-range-thumb {
    width: 1rem;
    height: 1rem;
    border: 0.0625rem solid var(--poodle-color-border-default);
    border-radius: 999px;
    background: var(--poodle-color-background-elevated);
    box-shadow: 0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent);
    cursor: pointer;
    pointer-events: auto;
  }

  .poodle-range-slider__control:focus-visible {
    outline: none;
  }

  .poodle-range-slider__control:focus-visible::-webkit-slider-thumb {
    box-shadow:
      0 0 0 0.1875rem color-mix(in srgb, var(--poodle-color-accent-focusRing) 32%, transparent),
      0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent);
  }

  .poodle-range-slider__control:focus-visible::-moz-range-thumb {
    box-shadow:
      0 0 0 0.1875rem color-mix(in srgb, var(--poodle-color-accent-focusRing) 32%, transparent),
      0 0.125rem 0.5rem color-mix(in srgb, black 18%, transparent);
  }

  /* Size variants */
  .poodle-range-slider[data-size="xs"] { min-height: 1.25rem; }
  .poodle-range-slider[data-size="xs"] .poodle-range-slider__control::-webkit-slider-thumb { width: 0.75rem; height: 0.75rem; margin-top: -0.1875rem; }
  .poodle-range-slider[data-size="xs"] .poodle-range-slider__control::-moz-range-thumb { width: 0.75rem; height: 0.75rem; }
  .poodle-range-slider[data-size="sm"] { min-height: 1.375rem; }
  .poodle-range-slider[data-size="sm"] .poodle-range-slider__control::-webkit-slider-thumb { width: 0.875rem; height: 0.875rem; margin-top: -0.25rem; }
  .poodle-range-slider[data-size="sm"] .poodle-range-slider__control::-moz-range-thumb { width: 0.875rem; height: 0.875rem; }
  .poodle-range-slider[data-size="lg"] .poodle-range-slider__control::-webkit-slider-thumb { width: 1.125rem; height: 1.125rem; margin-top: -0.375rem; }
  .poodle-range-slider[data-size="lg"] .poodle-range-slider__control::-moz-range-thumb { width: 1.125rem; height: 1.125rem; }
  .poodle-range-slider[data-size="xl"] .poodle-range-slider__control::-webkit-slider-thumb { width: 1.25rem; height: 1.25rem; margin-top: -0.4375rem; }
  .poodle-range-slider[data-size="xl"] .poodle-range-slider__control::-moz-range-thumb { width: 1.25rem; height: 1.25rem; }

  /* Density variants */
  .poodle-range-slider[data-density="compact"] { padding: 0.25rem 0; }
  .poodle-range-slider[data-density="comfortable"] { padding: 0.75rem 0; }
</style>
