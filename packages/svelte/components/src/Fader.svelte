<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/fader.css";
  import {
    audioValueText, createFaderContext, faderPointToNorm, faderTransition,
    faderVisualState, normalizeAudioValue, type AudioAutomationState,
    type AudioValueEffect, type AudioValueFormat, type AudioValueLaw,
    type FaderContext,
  } from "@inflatable-cookie/poodle-core";
  import { onDestroy, tick } from "svelte";
  import FaderVisual from "./audio/FaderVisual.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    size?: ControlSize | null; sizeRole?: SemanticControlSizeRole; density?: ControlDensity | null;
    value?: number; min?: number; max?: number; law?: AudioValueLaw;
    orientation?: "horizontal" | "vertical"; detents?: number[]; detentSnap?: number;
    defaultValue?: number; keyboardStep?: number; format?: AudioValueFormat;
    automation?: AudioAutomationState; disabled?: boolean; ariaLabel?: string | null;
    onValueChange?: (value: number) => void; onValueCommit?: (value: number) => void;
    onGestureBegin?: () => void; onGestureEnd?: () => void;
  }

  let {
    size = null, sizeRole = "control", density = null,
    value = $bindable(0), min = 0, max = 1, law = { type: "linear" },
    orientation = "vertical", detents = [], detentSnap = 0.015,
    defaultValue = 0, keyboardStep = 0.01, format = { type: "number", decimals: 2 },
    automation = "none", disabled = false, ariaLabel = null,
    onValueChange, onValueCommit, onGestureBegin, onGestureEnd,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  let machine = $state(createFaderContext());
  let root: HTMLDivElement;
  let entry = $state<HTMLInputElement>();
  let entryDraft = $state("");
  let activePointer: number | null = null;
  let skipEntryBlur = false;
  const context = $derived<FaderContext>({ ...machine, value, min, max, law, orientation, detents, detentSnap, defaultValue, keyboardStep, format, automation, disabled });
  const visualState = $derived(faderVisualState(context));
  const valueText = $derived(audioValueText(context));
  const detentNorms = $derived(detents.map((detent) => normalizeAudioValue(detent, min, max, law)));

  function runEffects(effects: AudioValueEffect[]): void {
    for (const effect of effects) {
      if (effect.type === "emitValueChange") { value = effect.value; onValueChange?.(effect.value); }
      else if (effect.type === "emitValueCommit") { value = effect.value; onValueCommit?.(effect.value); }
      else if (effect.type === "beginGesture") onGestureBegin?.();
      else if (effect.type === "endGesture") onGestureEnd?.();
      else if (effect.type === "requestEntryFocus") { entryDraft = valueText; skipEntryBlur = false; void tick().then(() => { entry?.focus(); entry?.select(); }); }
    }
  }

  function send(event: Parameters<typeof faderTransition>[1]): void {
    const result = faderTransition(context, event); machine = result.context; runEffects(result.effects);
  }

  function pointNorm(event: PointerEvent): number {
    return faderPointToNorm({ x: event.clientX, y: event.clientY }, root.getBoundingClientRect(), orientation);
  }

  function pointerDown(event: PointerEvent): void {
    // One primary pointer owns the gesture. A second pointer-down cannot
    // replace the active pointer or open a second gesture.
    if (event.button !== 0 || disabled || activePointer !== null) return;
    event.preventDefault(); activePointer = event.pointerId; root.setPointerCapture(event.pointerId);
    const valueNorm = pointNorm(event);
    send({ type: "DRAG_BEGIN", position: valueNorm, fine: event.shiftKey });
    send({ type: "DRAG_SET_NORM", valueNorm, fine: event.shiftKey });
  }
  function pointerMove(event: PointerEvent): void { if (activePointer === event.pointerId) send({ type: "DRAG_SET_NORM", valueNorm: pointNorm(event), fine: event.shiftKey }); }
  function pointerUp(event: PointerEvent): void { if (activePointer === event.pointerId) { activePointer = null; send({ type: "DRAG_END" }); } }

  /**
   * Pointer cancel, lost capture, and teardown all close the gesture the same
   * way, so a captured gesture can never outlive its pointer or its component.
   * A stale pointer id is ignored, and the machine makes a repeat inert.
   */
  function cancelGesture(pointerId?: number): void {
    if (activePointer === null || (pointerId !== undefined && activePointer !== pointerId)) return;
    activePointer = null;
    send({ type: "DRAG_CANCEL" });
  }

  onDestroy(() => cancelGesture());

  function keydown(event: KeyboardEvent): void {
    if (event.key === "Enter") { event.preventDefault(); send({ type: "ENTRY_OPEN" }); return; }
    const directions: Record<string, -1 | 1> = { ArrowLeft: -1, ArrowDown: -1, ArrowRight: 1, ArrowUp: 1, PageDown: -1, PageUp: 1 };
    const direction = directions[event.key];
    if (direction) { event.preventDefault(); send({ type: "KEY_NUDGE", direction, multiplier: event.key.startsWith("Page") ? 10 : 1, fine: event.shiftKey }); }
    else if (event.key === "Home" || event.key === "End") { event.preventDefault(); send({ type: "KEY_BOUND", bound: event.key === "Home" ? "min" : "max" }); }
  }

  function entryKeydown(event: KeyboardEvent): void {
    if (event.key === "Enter") { event.preventDefault(); skipEntryBlur = true; send({ type: "ENTRY_COMMIT", text: entryDraft }); root.focus(); }
    else if (event.key === "Escape") { event.preventDefault(); skipEntryBlur = true; send({ type: "ENTRY_CANCEL" }); root.focus(); }
  }

  /**
   * Enter and Escape already resolved the entry and moved focus back to the
   * root. The blur they cause must not commit a second time or reverse an
   * Escape; only an unresolved blur commits the draft.
   */
  function entryBlur(): void {
    if (skipEntryBlur) { skipEntryBlur = false; return; }
    send({ type: "ENTRY_COMMIT", text: entryDraft });
  }
</script>

<div
  bind:this={root}
  class="poodle-fader"
  role="slider"
  tabindex={disabled ? undefined : 0}
  aria-label={ariaLabel ?? undefined}
  aria-orientation={orientation}
  aria-valuemin={min}
  aria-valuemax={max}
  aria-valuenow={value}
  aria-valuetext={valueText}
  aria-disabled={disabled}
  data-scope="fader"
  data-part="root"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-orientation={orientation}
  data-state={visualState.drag === "none" ? "idle" : visualState.drag}
  onpointerdown={pointerDown} onpointermove={pointerMove} onpointerup={pointerUp} onpointercancel={(event) => cancelGesture(event.pointerId)}
  onlostpointercapture={(event) => cancelGesture(event.pointerId)}
  onmouseenter={() => send({ type: "HOVER", value: true })} onmouseleave={() => send({ type: "HOVER", value: false })}
  onfocus={() => send({ type: "FOCUS", value: true })} onblur={() => send({ type: "FOCUS", value: false })}
  onwheel={(event) => { event.preventDefault(); send({ type: "WHEEL", direction: event.deltaY < 0 ? 1 : -1, fine: event.shiftKey }); }}
  ondblclick={(event) => { event.preventDefault(); send({ type: "RESET" }); }}
  onkeydown={keydown}
>
  <FaderVisual {visualState} {orientation} detents={detentNorms} />
  {#if context.entryOpen}
    <input bind:this={entry} class="poodle-fader__entry" aria-label={`${ariaLabel ?? "Fader"} value`} value={entryDraft}
      oninput={(event) => entryDraft = event.currentTarget.value}
      onkeydown={entryKeydown}
      onblur={entryBlur} />
  {/if}
</div>
