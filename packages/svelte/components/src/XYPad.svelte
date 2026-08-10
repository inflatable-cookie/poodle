<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/xy-pad.css";
  import {
    createXYPadContext, formatAudioValue, linearValueLaw, xyPadPointToNorm,
    xyPadTransition, xyPadVisualState, type AudioAutomationState,
    type AudioValueFormat, type AudioValueLaw, type XYPadContext, type XYPadEffect,
  } from "@inflatable-cookie/poodle-core";
  import XYPadVisual from "./audio/XYPadVisual.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    size?: ControlSize | null; sizeRole?: SemanticControlSizeRole; density?: ControlDensity | null;
    x?: number; y?: number; minX?: number; maxX?: number; minY?: number; maxY?: number;
    lawX?: AudioValueLaw; lawY?: AudioValueLaw; defaultX?: number; defaultY?: number;
    keyboardStepX?: number; keyboardStepY?: number; formatX?: AudioValueFormat; formatY?: AudioValueFormat;
    automation?: AudioAutomationState; disabled?: boolean; ariaLabel?: string | null;
    onValueChange?: (x: number, y: number) => void; onValueCommit?: (x: number, y: number) => void;
    onGestureBegin?: () => void; onGestureEnd?: () => void;
  }

  let {
    size = null, sizeRole = "control", density = null,
    x = $bindable(0), y = $bindable(0), minX = 0, maxX = 1, minY = 0, maxY = 1,
    lawX = linearValueLaw, lawY = linearValueLaw, defaultX = 0, defaultY = 0,
    keyboardStepX = 0.01, keyboardStepY = 0.01,
    formatX = { type: "number", decimals: 2 }, formatY = { type: "number", decimals: 2 },
    automation = "none", disabled = false, ariaLabel = null,
    onValueChange, onValueCommit, onGestureBegin, onGestureEnd,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  let root: HTMLDivElement;
  let machine = $state(createXYPadContext());
  let activePointer: number | null = null;
  const context = $derived<XYPadContext>({
    ...machine, x, y, minX, maxX, minY, maxY, lawX, lawY, defaultX, defaultY,
    keyboardStepX, keyboardStepY, automation, disabled,
  });
  const visualState = $derived(xyPadVisualState(context));
  const xText = $derived(formatAudioValue(x, formatX));
  const yText = $derived(formatAudioValue(y, formatY));

  function runEffects(effects: XYPadEffect[]): void {
    for (const effect of effects) {
      if (effect.type === "emitValueChange") { x = effect.x; y = effect.y; onValueChange?.(effect.x, effect.y); }
      else if (effect.type === "emitValueCommit") { x = effect.x; y = effect.y; onValueCommit?.(effect.x, effect.y); }
      else if (effect.type === "beginGesture") onGestureBegin?.();
      else if (effect.type === "endGesture") onGestureEnd?.();
    }
  }

  function send(event: Parameters<typeof xyPadTransition>[1]): void {
    const result = xyPadTransition(context, event); machine = result.context; runEffects(result.effects);
  }

  function pointerNorm(event: PointerEvent): { xNorm: number; yNorm: number } {
    return xyPadPointToNorm({ x: event.clientX, y: event.clientY }, root.getBoundingClientRect());
  }

  function pointerDown(event: PointerEvent): void {
    if (event.button !== 0 || disabled) return;
    event.preventDefault(); activePointer = event.pointerId; root.setPointerCapture(event.pointerId);
    send({ type: "DRAG_BEGIN", ...pointerNorm(event), fine: event.shiftKey });
  }

  function pointerMove(event: PointerEvent): void {
    if (activePointer !== event.pointerId) return;
    send({ type: "DRAG_MOVE", ...pointerNorm(event), fine: event.shiftKey });
  }

  function pointerEnd(event: PointerEvent): void {
    if (activePointer !== event.pointerId) return;
    activePointer = null; send({ type: "DRAG_END" });
  }

  function axisKeydown(event: KeyboardEvent, axis: "x" | "y"): void {
    const negative = axis === "x" ? ["ArrowLeft", "ArrowDown", "PageDown"] : ["ArrowDown", "ArrowLeft", "PageDown"];
    const positive = axis === "x" ? ["ArrowRight", "ArrowUp", "PageUp"] : ["ArrowUp", "ArrowRight", "PageUp"];
    if (negative.includes(event.key) || positive.includes(event.key)) {
      event.preventDefault(); send({ type: "NUDGE", axis, direction: positive.includes(event.key) ? 1 : -1, multiplier: event.key.startsWith("Page") ? 10 : 1, fine: event.shiftKey });
    } else if (event.key === "Home" || event.key === "End") {
      event.preventDefault(); send({ type: "BOUND", axis, bound: event.key === "Home" ? "min" : "max" });
    }
  }
</script>

<div
  bind:this={root}
  class="poodle-xy-pad"
  role="group"
  aria-label={ariaLabel ?? undefined}
  aria-disabled={disabled}
  data-scope="xy-pad"
  data-part="root"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  onpointerdown={pointerDown}
  onpointermove={pointerMove}
  onpointerup={pointerEnd}
  onpointercancel={pointerEnd}
  onmouseenter={() => send({ type: "HOVER", value: true })}
  onmouseleave={() => send({ type: "HOVER", value: false })}
  ondblclick={() => send({ type: "RESET" })}
  onfocusin={() => send({ type: "FOCUS", value: true })}
  onfocusout={(event) => { if (!root.contains(event.relatedTarget as Node | null)) send({ type: "FOCUS", value: false }); }}
>
  <XYPadVisual {visualState} />
  <div
    class="poodle-xy-pad__axis"
    role="slider" tabindex={disabled ? undefined : 0}
    aria-label={`${ariaLabel ?? "XY pad"} X`}
    aria-valuemin={minX} aria-valuemax={maxX} aria-valuenow={x} aria-valuetext={xText}
    aria-disabled={disabled}
    onkeydown={(event) => axisKeydown(event, "x")}
  ></div>
  <div
    class="poodle-xy-pad__axis"
    role="slider" tabindex={disabled ? undefined : 0}
    aria-label={`${ariaLabel ?? "XY pad"} Y`}
    aria-valuemin={minY} aria-valuemax={maxY} aria-valuenow={y} aria-valuetext={yText}
    aria-disabled={disabled}
    onkeydown={(event) => axisKeydown(event, "y")}
  ></div>
</div>
