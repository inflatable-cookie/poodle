<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/range-slider.css";
  import {
    createRangeSliderControlContext,
    layoutRangeSliderBlock,
    measureInlineAdvance,
    normalizeRangeValue,
    physicalToValueNorm,
    rangeSliderControlTransition,
    rangeSliderVisualState,
    rangeSliderTransition,
    resolveRangeVisibleValue,
    safeSliderMax,
    type AudioValueLaw, type RangeSliderContext, type RangeSliderControlContext,
    type SliderDirection, type SliderPolarity, type SliderVariant,
  } from "@inflatable-cookie/poodle-core";
  import { onDestroy } from "svelte";

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
    direction?: SliderDirection;
    polarity?: SliderPolarity;
    centerValue?: number | null;
    law?: AudioValueLaw;
    orientation?: Orientation;
    disabled?: boolean;
    ariaLabel?: string | null;
    lowerValueText?: string | null;
    upperValueText?: string | null;
    visibleLabel?: string | null;
    formatVisibleValue?: ((value: number, thumb: "lower" | "upper") => string) | undefined;
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
    variant = "block",
    direction = "ltr",
    polarity = "unipolar",
    centerValue = null,
    law = { type: "linear" },
    orientation = "horizontal",
    disabled = false,
    ariaLabel = null,
    lowerValueText = null,
    upperValueText = null,
    visibleLabel = null,
    formatVisibleValue = undefined,
    onValueChange = undefined,
    onValueCommit = undefined,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const machineContext = $derived<RangeSliderContext>({ value, min, max, step, disabled });
  let controlMachine = $state(createRangeSliderControlContext());
  let root: HTMLDivElement;
  let capsule = $state<HTMLSpanElement | undefined>(undefined);
  let capsuleSpan = $state(0);
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
    `--poodle-range-negative-start: ${visualState.negativeFillStartNorm * 100}%`,
    `--poodle-range-negative-span: ${visualState.negativeFillSpanNorm * 100}%`,
    `--poodle-range-positive-start: ${visualState.positiveFillStartNorm * 100}%`,
    `--poodle-range-positive-span: ${visualState.positiveFillSpanNorm * 100}%`,
  ]));
  const block = $derived(variant === "block");
  const visibleLabelText = $derived(visibleLabel && visibleLabel !== "" ? visibleLabel : null);
  const lowerVisible = $derived(resolveRangeVisibleValue(displayLower, "lower", formatVisibleValue));
  const upperVisible = $derived(resolveRangeVisibleValue(displayUpper, "upper", formatVisibleValue));
  const blockLayout = $derived.by(() => {
    if (!block) return { labelInline: false, lowerInline: false, upperInline: false };
    const font = capsule ? getComputedStyle(capsule).font : "14px sans-serif";
    return layoutRangeSliderBlock({
      capsuleSpan,
      label: visibleLabelText,
      lowerText: lowerVisible,
      upperText: upperVisible,
      measure: (text) => measureInlineAdvance(text, font),
    });
  });

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
    const physical = orientation === "horizontal"
      ? (event.clientX - rect.left) / Math.max(rect.width, 1)
      : 1 - (event.clientY - rect.top) / Math.max(rect.height, 1);
    return physicalToValueNorm(physical, orientation === "horizontal" ? direction : "ltr");
  }
  function pointerDown(event: PointerEvent): void {
    if (event.button !== 0 || disabled) return;
    const target = block ? (event.currentTarget as HTMLElement) : root;
    if (!target) return;
    event.preventDefault();
    event.stopPropagation();
    activePointer = event.pointerId;
    target.setPointerCapture(event.pointerId);
    runControl({ type: "POINTER_BEGIN", valueNorm: pointNorm(event) });
  }
  function pointerMove(event: PointerEvent): void {
    if (activePointer === event.pointerId) {
      event.stopPropagation();
      runControl({ type: "POINTER_MOVE", valueNorm: pointNorm(event) });
    }
  }
  function terminate(pointerId: number | null = null): void {
    if (activePointer === null || (pointerId !== null && activePointer !== pointerId)) return;
    activePointer = null;
    runControl({ type: "POINTER_END" });
  }
  function pointerEnd(event: PointerEvent): void {
    event.stopPropagation();
    terminate(event.pointerId);
  }
  function controlKey(event: KeyboardEvent, thumb: "lower" | "upper"): void {
    const keyDirection = ({ ArrowLeft: -1, ArrowDown: -1, ArrowRight: 1, ArrowUp: 1 } as Record<string, -1 | 1>)[event.key];
    const current = thumb === "lower" ? displayLower : displayUpper;
    const raw = event.key === "Home" ? min : event.key === "End" ? safeMax : keyDirection ? current + keyDirection * step : null;
    if (raw == null) return;
    event.preventDefault();
    const changed = rangeSliderTransition(machineContext, { type: "INPUT", thumb, raw });
    const committed = rangeSliderTransition(changed.context, { type: "COMMIT", thumb, raw: thumb === "lower" ? changed.context.value[0] : changed.context.value[1] });
    for (const effect of [...changed.effects, ...committed.effects]) {
      value = effect.value;
      if (effect.type === "emitValueChange") onValueChange?.(effect.value); else onValueCommit?.(effect.value);
    }
  }

  $effect(() => {
    if (!block || !capsule) return;
    const axis = orientation;
    const observer = new ResizeObserver(() => {
      const rect = capsule?.getBoundingClientRect();
      capsuleSpan = !rect ? 0 : axis === "vertical" ? rect.height : rect.width;
    });
    observer.observe(capsule);
    const rect = capsule.getBoundingClientRect();
    capsuleSpan = axis === "vertical" ? rect.height : rect.width;
    return () => observer.disconnect();
  });
  $effect(() => { if (disabled) terminate(); });
  onDestroy(() => terminate());
</script>

<div bind:this={root} class="poodle-range-slider" role="group" data-orientation={orientation} data-disabled={disabled} data-variant={variant} data-direction={direction === "rtl" ? direction : undefined} data-polarity={visualState.polarity} data-fill-split={visualState.fillSplitAtCenter} data-state={visualState.pointerActive ? "active" : "idle"} style={rangeStyle} data-size={resolvedSize} data-density={resolvedDensity} dir={direction === "rtl" ? direction : undefined}
  onpointerdown={pointerDown} onpointermove={pointerMove} onpointerup={pointerEnd} onpointercancel={pointerEnd} onlostpointercapture={pointerEnd}>
  {#if block}
    <span bind:this={capsule} class="poodle-range-slider__capsule" aria-hidden="true">
      <span class="poodle-range-slider__track">
        <span class="poodle-range-slider__fill poodle-range-slider__fill--negative"></span>
        <span class="poodle-range-slider__fill poodle-range-slider__fill--positive"></span>
        <span class="poodle-range-slider__center"></span>
      </span>
      {#if orientation === "vertical"}
        {#if blockLayout.lowerInline || blockLayout.upperInline || blockLayout.labelInline}
          <!-- One stable upright column painted through window/remainder clips; glyphs never move. -->
          <span class="poodle-range-slider__inline poodle-range-slider__inline--selected">
            <span class="poodle-range-slider__inline-row poodle-range-slider__inline-row--vertical">
              <span class="poodle-range-slider__inline-value poodle-range-slider__inline-value--upper">{blockLayout.upperInline ? upperVisible : ""}</span>
              <span class="poodle-range-slider__inline-label">{blockLayout.labelInline ? visibleLabelText : ""}</span>
              <span class="poodle-range-slider__inline-value poodle-range-slider__inline-value--lower">{blockLayout.lowerInline ? lowerVisible : ""}</span>
            </span>
          </span>
          <span class="poodle-range-slider__inline poodle-range-slider__inline--remainder-top">
            <span class="poodle-range-slider__inline-row poodle-range-slider__inline-row--vertical">
              <span class="poodle-range-slider__inline-value poodle-range-slider__inline-value--upper">{blockLayout.upperInline ? upperVisible : ""}</span>
              <span class="poodle-range-slider__inline-label">{blockLayout.labelInline ? visibleLabelText : ""}</span>
              <span class="poodle-range-slider__inline-value poodle-range-slider__inline-value--lower">{blockLayout.lowerInline ? lowerVisible : ""}</span>
            </span>
          </span>
          <span class="poodle-range-slider__inline poodle-range-slider__inline--remainder-bottom">
            <span class="poodle-range-slider__inline-row poodle-range-slider__inline-row--vertical">
              <span class="poodle-range-slider__inline-value poodle-range-slider__inline-value--upper">{blockLayout.upperInline ? upperVisible : ""}</span>
              <span class="poodle-range-slider__inline-label">{blockLayout.labelInline ? visibleLabelText : ""}</span>
              <span class="poodle-range-slider__inline-value poodle-range-slider__inline-value--lower">{blockLayout.lowerInline ? lowerVisible : ""}</span>
            </span>
          </span>
        {/if}
      {:else}
        {#if blockLayout.lowerInline || blockLayout.upperInline || blockLayout.labelInline}
          <span class="poodle-range-slider__inline poodle-range-slider__inline--selected">
            <span class="poodle-range-slider__inline-row">
              <span class="poodle-range-slider__inline-value poodle-range-slider__inline-value--lower">{blockLayout.lowerInline ? lowerVisible : ""}</span>
              <span class="poodle-range-slider__inline-label">{blockLayout.labelInline ? visibleLabelText : ""}</span>
              <span class="poodle-range-slider__inline-value poodle-range-slider__inline-value--upper">{blockLayout.upperInline ? upperVisible : ""}</span>
            </span>
          </span>
          <span class="poodle-range-slider__inline poodle-range-slider__inline--remainder-start">
            <span class="poodle-range-slider__inline-row">
              <span class="poodle-range-slider__inline-value poodle-range-slider__inline-value--lower">{blockLayout.lowerInline ? lowerVisible : ""}</span>
              <span class="poodle-range-slider__inline-label">{blockLayout.labelInline ? visibleLabelText : ""}</span>
              <span class="poodle-range-slider__inline-value poodle-range-slider__inline-value--upper">{blockLayout.upperInline ? upperVisible : ""}</span>
            </span>
          </span>
          <span class="poodle-range-slider__inline poodle-range-slider__inline--remainder-end">
            <span class="poodle-range-slider__inline-row">
              <span class="poodle-range-slider__inline-value poodle-range-slider__inline-value--lower">{blockLayout.lowerInline ? lowerVisible : ""}</span>
              <span class="poodle-range-slider__inline-label">{blockLayout.labelInline ? visibleLabelText : ""}</span>
              <span class="poodle-range-slider__inline-value poodle-range-slider__inline-value--upper">{blockLayout.upperInline ? upperVisible : ""}</span>
            </span>
          </span>
        {/if}
      {/if}
    </span>
    <div class="poodle-range-slider__hit poodle-range-slider__hit--lower" data-part="hit" data-thumb="lower" role="slider" tabindex={disabled ? undefined : 0} aria-label={ariaLabel ? `${ariaLabel} minimum` : "Minimum value"} aria-valuemin={min} aria-valuemax={displayUpper} aria-valuenow={displayLower} aria-valuetext={lowerValueText ?? undefined} aria-orientation={orientation} aria-disabled={disabled} onkeydown={(event) => controlKey(event, "lower")} onpointerdown={pointerDown} onpointermove={pointerMove} onpointerup={pointerEnd} onpointercancel={pointerEnd} onlostpointercapture={pointerEnd}><span class="poodle-range-slider__thumb"></span></div>
    <div class="poodle-range-slider__hit poodle-range-slider__hit--upper" data-part="hit" data-thumb="upper" role="slider" tabindex={disabled ? undefined : 0} aria-label={ariaLabel ? `${ariaLabel} maximum` : "Maximum value"} aria-valuemin={displayLower} aria-valuemax={safeMax} aria-valuenow={displayUpper} aria-valuetext={upperValueText ?? undefined} aria-orientation={orientation} aria-disabled={disabled} onkeydown={(event) => controlKey(event, "upper")} onpointerdown={pointerDown} onpointermove={pointerMove} onpointerup={pointerEnd} onpointercancel={pointerEnd} onlostpointercapture={pointerEnd}><span class="poodle-range-slider__thumb"></span></div>
  {:else}
    <span class="poodle-range-slider__track" aria-hidden="true">
      <span class="poodle-range-slider__fill poodle-range-slider__fill--negative"></span>
      <span class="poodle-range-slider__fill poodle-range-slider__fill--positive"></span>
      <span class="poodle-range-slider__center"></span>
    </span>

    <div class="poodle-range-slider__embedded-control poodle-range-slider__embedded-control--lower" role="slider" tabindex={disabled ? undefined : 0} aria-label={ariaLabel ? `${ariaLabel} minimum` : "Minimum value"} aria-valuemin={min} aria-valuemax={displayUpper} aria-valuenow={displayLower} aria-valuetext={lowerValueText ?? undefined} aria-orientation={orientation} aria-disabled={disabled} onkeydown={(event) => controlKey(event, "lower")}></div>
    <div class="poodle-range-slider__embedded-control poodle-range-slider__embedded-control--upper" role="slider" tabindex={disabled ? undefined : 0} aria-label={ariaLabel ? `${ariaLabel} maximum` : "Maximum value"} aria-valuemin={displayLower} aria-valuemax={safeMax} aria-valuenow={displayUpper} aria-valuetext={upperValueText ?? undefined} aria-orientation={orientation} aria-disabled={disabled} onkeydown={(event) => controlKey(event, "upper")}></div>
  {/if}
</div>
