<script lang="ts">
  import "@inflatable-cookie/poodle-styles/range-slider.css";
  import {
    normalizeRangeValue,
    rangeSliderTransition,
    safeSliderMax,
    type RangeSliderContext,
  } from "@inflatable-cookie/poodle-headless";

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

