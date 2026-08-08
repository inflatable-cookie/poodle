<script lang="ts">
  import "@poodle/styles/slider.css";
  import { normalizeSliderValue, safeSliderMax, sliderTransition, type SliderContext } from "@poodle/headless";

  import { joinStyles } from "./internal";
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
  const machineContext = $derived<SliderContext>({ value, min, max, step, disabled });
  const safeMax = $derived(safeSliderMax(min, max));
  const displayValue = $derived(normalizeSliderValue(machineContext, value));
  const percentage = $derived(((displayValue - min) / (safeMax - min)) * 100);
  const sliderStyle = $derived(joinStyles([`--poodle-slider-percent: ${percentage}%`]));

  function send(type: "INPUT" | "COMMIT", event: Event): void {
    const raw = Number((event.currentTarget as HTMLInputElement).value);
    const result = sliderTransition(machineContext, { type, raw });

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
    oninput={(event) => send("INPUT", event)}
    onchange={(event) => send("COMMIT", event)}
  />
</div>

