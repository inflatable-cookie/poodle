<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/slider.css";
  import {
    assertHorizontalBlockAppearance,
    createSliderControlContext, layoutSliderBlock, measureInlineAdvance,
    normalizeSliderValue, physicalToValueNorm, resolveSliderVisibleValue, safeSliderMax,
    sliderControlTransition, sliderTransition, sliderVisualState,
    type AudioValueLaw, type SliderAppearance, type SliderContext, type SliderControlContext,
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
    value?: number;
    min?: number;
    max?: number;
    step?: number;
    variant?: SliderVariant;
    appearance?: SliderAppearance;
    direction?: SliderDirection;
    polarity?: SliderPolarity;
    centerValue?: number | null;
    law?: AudioValueLaw;
    orientation?: Orientation;
    disabled?: boolean;
    ariaLabel?: string | null;
    valueText?: string | null;
    visibleLabel?: string | null;
    formatVisibleValue?: ((value: number) => string) | undefined;
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
    appearance = "track",
    direction = "ltr",
    polarity = "unipolar",
    centerValue = null,
    law = { type: "linear" },
    orientation = "horizontal",
    disabled = false,
    ariaLabel = null,
    valueText = null,
    visibleLabel = null,
    formatVisibleValue = undefined,
    onValueChange = undefined,
    onValueCommit = undefined,
  }: Props = $props();

  const uiPresentation = getUiPresentation();

  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);
  const machineContext = $derived<SliderContext>({ value, min, max, step, disabled });
  let controlMachine = $state(createSliderControlContext());
  let root: HTMLDivElement;
  let capsule = $state<HTMLSpanElement | undefined>(undefined);
  let capsuleSpan = $state(0);
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
  const block = $derived.by(() => {
    assertHorizontalBlockAppearance(appearance, orientation);
    return appearance === "block";
  });
  const usesControlPointer = $derived(block || variant === "embedded");
  const visibleValueText = $derived(resolveSliderVisibleValue(displayValue, formatVisibleValue));
  const visibleLabelText = $derived(visibleLabel && visibleLabel !== "" ? visibleLabel : null);
  const blockLayout = $derived.by(() => {
    if (!block) return { inline: false, fallback: null };
    const font = capsule ? getComputedStyle(capsule).font : "14px sans-serif";
    return layoutSliderBlock({
      capsuleSpan,
      selectedNorm: visualState.valueNorm,
      label: visibleLabelText,
      valueText: visibleValueText,
      measure: (text) => measureInlineAdvance(text, font),
    });
  });

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
    const physical = orientation === "horizontal"
      ? (event.clientX - rect.left) / Math.max(rect.width, 1)
      : 1 - (event.clientY - rect.top) / Math.max(rect.height, 1);
    return physicalToValueNorm(physical, orientation === "horizontal" ? direction : "ltr");
  }

  function pointerDown(event: PointerEvent): void {
    if (!usesControlPointer || event.button !== 0 || disabled) return;
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
  function embeddedKey(event: KeyboardEvent): void {
    if (disabled) return;
    const keyDirection = ({ ArrowLeft: -1, ArrowDown: -1, ArrowRight: 1, ArrowUp: 1 } as Record<string, -1 | 1>)[event.key];
    const raw = event.key === "Home" ? min : event.key === "End" ? safeMax : keyDirection ? value + keyDirection * step : null;
    if (raw == null) return;
    event.preventDefault();
    const changed = sliderTransition(machineContext, { type: "INPUT", raw });
    const committed = sliderTransition(changed.context, { type: "COMMIT", raw: changed.context.value });
    for (const effect of [...changed.effects, ...committed.effects]) {
      value = effect.value;
      if (effect.type === "emitValueChange") onValueChange?.(effect.value); else onValueCommit?.(effect.value);
    }
  }

  $effect(() => {
    if (!block || !capsule) return;
    const observer = new ResizeObserver(() => {
      capsuleSpan = capsule?.getBoundingClientRect().width ?? 0;
    });
    observer.observe(capsule);
    capsuleSpan = capsule.getBoundingClientRect().width;
    return () => observer.disconnect();
  });

  $effect(() => {
    if (disabled) terminate();
  });

  onDestroy(() => terminate());
</script>

<!-- The embedded branch supplies slider semantics; the standard branch delegates them to the native input. -->
<!-- svelte-ignore a11y_no_noninteractive_tabindex -->
<div bind:this={root} class="poodle-slider" data-orientation={orientation} data-disabled={disabled} data-variant={variant} data-appearance={block ? "block" : undefined} data-direction={block || direction === "rtl" ? direction : undefined} data-polarity={visualState.polarity} data-fill-tone={visualState.fillTone} data-state={visualState.pointerActive ? "active" : "idle"} style={sliderStyle} data-size={resolvedSize} data-density={resolvedDensity} dir={block || direction === "rtl" ? direction : undefined}
  role={usesControlPointer ? "slider" : undefined} tabindex={usesControlPointer && !disabled ? 0 : undefined}
  aria-label={usesControlPointer ? ariaLabel ?? undefined : undefined} aria-valuemin={usesControlPointer ? min : undefined} aria-valuemax={usesControlPointer ? safeMax : undefined} aria-valuenow={usesControlPointer ? visualState.value : undefined} aria-valuetext={usesControlPointer ? valueText ?? undefined : undefined} aria-orientation={usesControlPointer ? orientation : undefined} aria-disabled={usesControlPointer ? disabled : undefined}
  onpointerdown={usesControlPointer ? pointerDown : undefined} onpointermove={usesControlPointer ? pointerMove : undefined} onpointerup={usesControlPointer ? pointerEnd : undefined} onpointercancel={usesControlPointer ? pointerEnd : undefined} onlostpointercapture={usesControlPointer ? pointerEnd : undefined} onkeydown={usesControlPointer ? embeddedKey : undefined}>
  {#if block}
    <span bind:this={capsule} class="poodle-slider__capsule" aria-hidden="true">
      <span class="poodle-slider__track">
        <span class="poodle-slider__fill">{#if blockLayout.inline && visibleLabelText}{visibleLabelText}{/if}</span>
        <span class="poodle-slider__remainder">{#if blockLayout.inline && visibleValueText}{visibleValueText}{/if}</span>
        <span class="poodle-slider__center"></span>
      </span>
      <!-- svelte-ignore a11y_no_static_element_interactions -->
      <span class="poodle-slider__hit" data-part="hit" onpointerdown={pointerDown} onpointermove={pointerMove} onpointerup={pointerEnd} onpointercancel={pointerEnd} onlostpointercapture={pointerEnd}><span class="poodle-slider__thumb"></span></span>
    </span>
    {#if blockLayout.fallback}<span class="poodle-slider__fallback" aria-hidden="true">{blockLayout.fallback}</span>{/if}
  {:else}
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
  {/if}
</div>
