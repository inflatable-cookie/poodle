<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/range-slider.css";
  import {
    createRangeSliderControlContext,
    normalizeRangeValue,
    rangeSliderControlTransition,
    rangeSliderVisualState,
    rangeSliderTransition,
    safeSliderMax,
    type AudioValueLaw, type RangeSliderContext, type RangeSliderControlContext,
    type SliderPolarity, type SliderVariant,
  } from "@inflatable-cookie/poodle-core";

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
    variant?: SliderVariant;
    polarity?: SliderPolarity;
    centerValue?: number | null;
    law?: AudioValueLaw;
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
    variant = "standard",
    polarity = "unipolar",
    centerValue = null,
    law = { type: "linear" },
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
  let controlMachine = $state(createRangeSliderControlContext());
  let root: HTMLDivElement;
  let activePointer: number | null = null;
  const controlContext = $derived<RangeSliderControlContext>({ ...controlMachine, value, min, max, step, disabled, law, polarity, centerValue });
  const visualState = $derived(rangeSliderVisualState(controlContext));
  const safeMax = $derived(safeSliderMax(min, max));
  const displayRange = $derived(normalizeRangeValue(machineContext));
  const displayLower = $derived(displayRange[0]);
  const displayUpper = $derived(displayRange[1]);
  const lowerPercent = $derived(visualState.lowerNorm * 100);
  const upperPercent = $derived(visualState.upperNorm * 100);
  const rangeStyle = $derived(joinStyles([
    `--poodle-range-start: ${lowerPercent}%`,
    `--poodle-range-end: ${upperPercent}%`,
    `--poodle-range-center: ${visualState.centerNorm * 100}%`,
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

  function runControl(event: Parameters<typeof rangeSliderControlTransition>[1]): void {
    const result = rangeSliderControlTransition(controlContext, event);
    controlMachine = result.context;
    for (const effect of result.effects) {
      value = effect.value;
      if (effect.type === "emitValueChange") onValueChange?.(effect.value); else onValueCommit?.(effect.value);
    }
  }
  function pointNorm(event: PointerEvent): number {
    const rect = root.getBoundingClientRect();
    return orientation === "horizontal"
      ? Math.min(Math.max((event.clientX - rect.left) / Math.max(rect.width, 1), 0), 1)
      : 1 - Math.min(Math.max((event.clientY - rect.top) / Math.max(rect.height, 1), 0), 1);
  }
  function pointerDown(event: PointerEvent): void {
    if (variant !== "embedded" || event.button !== 0 || disabled) return;
    event.preventDefault(); activePointer = event.pointerId; root.setPointerCapture(event.pointerId);
    runControl({ type: "POINTER_BEGIN", valueNorm: pointNorm(event) });
  }
  function pointerMove(event: PointerEvent): void { if (activePointer === event.pointerId) runControl({ type: "POINTER_MOVE", valueNorm: pointNorm(event) }); }
  function pointerEnd(event: PointerEvent): void { if (activePointer === event.pointerId) { activePointer = null; runControl({ type: "POINTER_END" }); } }
  function embeddedKey(event: KeyboardEvent, thumb: "lower" | "upper"): void {
    const direction = ({ ArrowLeft: -1, ArrowDown: -1, ArrowRight: 1, ArrowUp: 1 } as Record<string, -1 | 1>)[event.key];
    const current = thumb === "lower" ? displayLower : displayUpper;
    const raw = event.key === "Home" ? min : event.key === "End" ? safeMax : direction ? current + direction * step : null;
    if (raw == null) return;
    event.preventDefault();
    const changed = rangeSliderTransition(machineContext, { type: "INPUT", thumb, raw });
    const committed = rangeSliderTransition(changed.context, { type: "COMMIT", thumb, raw: thumb === "lower" ? changed.context.value[0] : changed.context.value[1] });
    for (const effect of [...changed.effects, ...committed.effects]) {
      value = effect.value;
      if (effect.type === "emitValueChange") onValueChange?.(effect.value); else onValueCommit?.(effect.value);
    }
  }
</script>

<div bind:this={root} class="poodle-range-slider" role="group" data-orientation={orientation} data-disabled={disabled} data-variant={variant} data-polarity={polarity} data-state={visualState.pointerActive ? "active" : "idle"} style={rangeStyle} data-size={resolvedSize} data-density={resolvedDensity}
  onpointerdown={pointerDown} onpointermove={pointerMove} onpointerup={pointerEnd} onpointercancel={pointerEnd}>
  <span class="poodle-range-slider__track" aria-hidden="true">
    <span class="poodle-range-slider__fill"></span>
    <span class="poodle-range-slider__center"></span>
  </span>

  {#if variant === "standard"}<input
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
  />{:else}
    <div class="poodle-range-slider__embedded-control poodle-range-slider__embedded-control--lower" role="slider" tabindex={disabled ? undefined : 0} aria-label={ariaLabel ? `${ariaLabel} minimum` : "Minimum value"} aria-valuemin={min} aria-valuemax={displayUpper} aria-valuenow={displayLower} aria-valuetext={lowerValueText ?? undefined} aria-orientation={orientation} aria-disabled={disabled} onkeydown={(event) => embeddedKey(event, "lower")}></div>
    <div class="poodle-range-slider__embedded-control poodle-range-slider__embedded-control--upper" role="slider" tabindex={disabled ? undefined : 0} aria-label={ariaLabel ? `${ariaLabel} maximum` : "Maximum value"} aria-valuemin={displayLower} aria-valuemax={safeMax} aria-valuenow={displayUpper} aria-valuetext={upperValueText ?? undefined} aria-orientation={orientation} aria-disabled={disabled} onkeydown={(event) => embeddedKey(event, "upper")}></div>
  {/if}
</div>
