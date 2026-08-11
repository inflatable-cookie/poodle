<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/slider.css";
  import {
    createSliderControlContext, normalizeSliderValue, safeSliderMax,
    sliderControlTransition, sliderTransition, sliderVisualState,
    type AudioValueLaw, type SliderContext, type SliderControlContext,
    type SliderPolarity, type SliderVariant,
  } from "@inflatable-cookie/poodle-core";

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
    variant?: SliderVariant;
    polarity?: SliderPolarity;
    centerValue?: number | null;
    law?: AudioValueLaw;
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
    variant = "standard",
    polarity = "unipolar",
    centerValue = null,
    law = { type: "linear" },
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
  let controlMachine = $state(createSliderControlContext());
  let root: HTMLDivElement;
  let activePointer: number | null = null;
  const controlContext = $derived<SliderControlContext>({ ...controlMachine, value, min, max, step, disabled, law, polarity, centerValue });
  const visualState = $derived(sliderVisualState(controlContext));
  const safeMax = $derived(safeSliderMax(min, max));
  const displayValue = $derived(normalizeSliderValue(machineContext, value));
  const sliderStyle = $derived(joinStyles([
    `--poodle-slider-percent: ${visualState.valueNorm * 100}%`,
    `--poodle-slider-fill-start: ${variant === "standard" ? 0 : visualState.fillStartNorm * 100}%`,
    `--poodle-slider-fill-span: ${(variant === "standard" ? visualState.valueNorm : visualState.fillSpanNorm) * 100}%`,
    `--poodle-slider-center: ${visualState.centerNorm * 100}%`,
  ]));

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

  function runControl(event: Parameters<typeof sliderControlTransition>[1]): void {
    const result = sliderControlTransition(controlContext, event);
    controlMachine = result.context;
    for (const effect of result.effects) {
      value = effect.value;
      if (effect.type === "emitValueChange") onValueChange?.(effect.value);
      else onValueCommit?.(effect.value);
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
    event.preventDefault();
    activePointer = event.pointerId;
    root.setPointerCapture(event.pointerId);
    runControl({ type: "POINTER_BEGIN", valueNorm: pointNorm(event) });
  }
  function pointerMove(event: PointerEvent): void {
    if (activePointer === event.pointerId) runControl({ type: "POINTER_MOVE", valueNorm: pointNorm(event) });
  }
  function pointerEnd(event: PointerEvent): void {
    if (activePointer === event.pointerId) { activePointer = null; runControl({ type: "POINTER_END" }); }
  }
  function embeddedKey(event: KeyboardEvent): void {
    const direction = ({ ArrowLeft: -1, ArrowDown: -1, ArrowRight: 1, ArrowUp: 1 } as Record<string, -1 | 1>)[event.key];
    const raw = event.key === "Home" ? min : event.key === "End" ? safeMax : direction ? value + direction * step : null;
    if (raw == null) return;
    event.preventDefault();
    const changed = sliderTransition(machineContext, { type: "INPUT", raw });
    const committed = sliderTransition(changed.context, { type: "COMMIT", raw: changed.context.value });
    for (const effect of [...changed.effects, ...committed.effects]) {
      value = effect.value;
      if (effect.type === "emitValueChange") onValueChange?.(effect.value); else onValueCommit?.(effect.value);
    }
  }
</script>

<!-- The embedded branch supplies slider semantics; the standard branch delegates them to the native input. -->
<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<div bind:this={root} class="poodle-slider" data-orientation={orientation} data-disabled={disabled} data-variant={variant} data-polarity={polarity} data-state={visualState.pointerActive ? "active" : "idle"} style={sliderStyle} data-size={resolvedSize} data-density={resolvedDensity}
  role={variant === "embedded" ? "slider" : undefined} tabindex={variant === "embedded" && !disabled ? 0 : undefined}
  aria-label={variant === "embedded" ? ariaLabel ?? undefined : undefined} aria-valuemin={variant === "embedded" ? min : undefined} aria-valuemax={variant === "embedded" ? safeMax : undefined} aria-valuenow={variant === "embedded" ? visualState.value : undefined} aria-valuetext={variant === "embedded" ? valueText ?? undefined : undefined} aria-orientation={variant === "embedded" ? orientation : undefined} aria-disabled={variant === "embedded" ? disabled : undefined}
  onpointerdown={pointerDown} onpointermove={pointerMove} onpointerup={pointerEnd} onpointercancel={pointerEnd} onkeydown={variant === "embedded" ? embeddedKey : undefined}>
  <span class="poodle-slider__track" aria-hidden="true">
    <span class="poodle-slider__fill"></span>
    <span class="poodle-slider__center"></span>
  </span>
  {#if variant === "standard"}<input
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
  />{/if}
</div>
