<script module lang="ts">
  import { rangeSliderDefinition } from "./generated/range-slider";

  // The definition owns the rendered vocabulary (card 045 R2): the
  // anatomy's DOM classes, the eight data-* attribute names, and the seven
  // fill-geometry custom properties. A rename in
  // packages/codegen/src/models/range_slider.rs moves the DOM here with no
  // hand edit; `effigy ir:check` gates drift in the artifact.
  const parts = new Map<string, string>(rangeSliderDefinition.parts.map((part) => [part.id, part.className]));
  const attributes = new Map<string, string>(rangeSliderDefinition.attributes.map((attribute) => [attribute.id, attribute.name]));
  const styleProps = new Map<string, string>(rangeSliderDefinition.styleProps.map((prop) => [prop.id, prop.name]));

  function partClass(id: string): string {
    const className = parts.get(id);
    if (!className) throw new Error(`RangeSlider definition has no part '${id}'`);
    return className;
  }

  function attributeName(id: string): string {
    const name = attributes.get(id);
    if (!name) throw new Error(`RangeSlider definition has no attribute '${id}'`);
    return name;
  }

  function stylePropName(id: string): string {
    const name = styleProps.get(id);
    if (!name) throw new Error(`RangeSlider definition has no style prop '${id}'`);
    return name;
  }

  const rootClass = partClass("root");
  const trackClass = partClass("track");
  const fillNegativeClass = partClass("fill-negative");
  const fillPositiveClass = partClass("fill-positive");
  const centerClass = partClass("center");
  const controlLowerClass = partClass("control-lower");
  const controlUpperClass = partClass("control-upper");
  const embeddedLowerClass = partClass("embedded-lower");
  const embeddedUpperClass = partClass("embedded-upper");

  const dataOrientation = attributeName("orientation");
  const dataDisabled = attributeName("disabled");
  const dataVariant = attributeName("variant");
  const dataPolarity = attributeName("polarity");
  const dataFillSplit = attributeName("fill-split");
  const dataState = attributeName("state");
  const dataSize = attributeName("size");
  const dataDensity = attributeName("density");

  const styleRangeStart = stylePropName("range-start");
  const styleRangeEnd = stylePropName("range-end");
  const styleRangeCenter = stylePropName("range-center");
  const styleNegativeStart = stylePropName("range-negative-start");
  const styleNegativeSpan = stylePropName("range-negative-span");
  const stylePositiveStart = stylePropName("range-positive-start");
  const stylePositiveSpan = stylePropName("range-positive-span");
</script>

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
  // The eight data-* attributes: names come from the definition's
  // attributes, values are the runtime's projection (CROSS-13; the
  // emission-policy logic stays in the runtime — a g13.008 question).
  const dataAttributes = $derived({
    [dataOrientation]: orientation,
    [dataDisabled]: disabled,
    [dataVariant]: variant,
    [dataPolarity]: visualState.polarity,
    [dataFillSplit]: visualState.fillSplitAtCenter,
    [dataState]: visualState.pointerActive ? "active" : "idle",
    [dataSize]: resolvedSize,
    [dataDensity]: resolvedDensity,
  });
  // The fill geometry (RNG-17): the property names come from the
  // definition's styleProps; the values are the machine's visual-state
  // numbers projected to percentages (CROSS-14, IR-06 — drawing consumes
  // serializable state).
  const rangeStyle = $derived(joinStyles([
    `${styleRangeStart}: ${lowerPercent}%`,
    `${styleRangeEnd}: ${upperPercent}%`,
    `${styleRangeCenter}: ${visualState.centerNorm * 100}%`,
    `${styleNegativeStart}: ${visualState.negativeFillStartNorm * 100}%`,
    `${styleNegativeSpan}: ${visualState.negativeFillSpanNorm * 100}%`,
    `${stylePositiveStart}: ${visualState.positiveFillStartNorm * 100}%`,
    `${stylePositiveSpan}: ${visualState.positiveFillSpanNorm * 100}%`,
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

<div bind:this={root} class={rootClass} role="group" {...dataAttributes} style={rangeStyle}
  onpointerdown={pointerDown} onpointermove={pointerMove} onpointerup={pointerEnd} onpointercancel={pointerEnd}>
  <span class={trackClass} aria-hidden="true">
    <span class={fillNegativeClass}></span>
    <span class={fillPositiveClass}></span>
    <span class={centerClass}></span>
  </span>

  {#if variant === "standard"}<input
    class={controlLowerClass}
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
    class={controlUpperClass}
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
    <div class={embeddedLowerClass} role="slider" tabindex={disabled ? undefined : 0} aria-label={ariaLabel ? `${ariaLabel} minimum` : "Minimum value"} aria-valuemin={min} aria-valuemax={displayUpper} aria-valuenow={displayLower} aria-valuetext={lowerValueText ?? undefined} aria-orientation={orientation} aria-disabled={disabled} onkeydown={(event) => embeddedKey(event, "lower")}></div>
    <div class={embeddedUpperClass} role="slider" tabindex={disabled ? undefined : 0} aria-label={ariaLabel ? `${ariaLabel} maximum` : "Maximum value"} aria-valuemin={displayLower} aria-valuemax={safeMax} aria-valuenow={displayUpper} aria-valuetext={upperValueText ?? undefined} aria-orientation={orientation} aria-disabled={disabled} onkeydown={(event) => embeddedKey(event, "upper")}></div>
  {/if}
</div>
