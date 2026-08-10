<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/drag-number-field.css";
  import {
    audioValueText, createDragNumberContext, dragNumberTransition,
    dragNumberVisualState, type AudioValueEffect, type AudioValueFormat,
    type AudioValueLaw, type DragNumberContext,
  } from "@inflatable-cookie/poodle-core";
  import { tick } from "svelte";
  import ValueVisual from "./audio/ValueVisual.svelte";

  interface Props {
    value?: number; min?: number; max?: number; step?: number;
    dragSensitivity?: number; format?: AudioValueFormat; disabled?: boolean;
    ariaLabel?: string | null; onValueChange?: (value: number) => void;
    onValueCommit?: (value: number) => void; onGestureBegin?: () => void;
    onGestureEnd?: () => void;
  }

  let {
    value = $bindable(0), min = Number.MIN_SAFE_INTEGER, max = Number.MAX_SAFE_INTEGER,
    step = 1, dragSensitivity = 0.1, format = { type: "number", decimals: 2 },
    disabled = false, ariaLabel = null, onValueChange, onValueCommit,
    onGestureBegin, onGestureEnd,
  }: Props = $props();

  const law = $derived<AudioValueLaw>({ type: "stepped", step });
  let machine = $state(createDragNumberContext());
  let root: HTMLDivElement;
  let entry = $state<HTMLInputElement>();
  let entryDraft = $state("");
  let activePointer: number | null = null;
  let pointerStart = 0;
  let dragging = false;
  const context = $derived<DragNumberContext>({ ...machine, value, min, max, law, defaultValue: value, keyboardStep: step, format, dragSensitivity, disabled });
  const visualState = $derived(dragNumberVisualState(context));
  const valueText = $derived(audioValueText(context));

  function runEffects(effects: AudioValueEffect[]): void {
    for (const effect of effects) {
      if (effect.type === "emitValueChange") { value = effect.value; onValueChange?.(effect.value); }
      else if (effect.type === "emitValueCommit") { value = effect.value; onValueCommit?.(effect.value); }
      else if (effect.type === "beginGesture") onGestureBegin?.();
      else if (effect.type === "endGesture") onGestureEnd?.();
      else if (effect.type === "requestEntryFocus") { entryDraft = valueText; void tick().then(() => { entry?.focus(); entry?.select(); }); }
    }
  }
  function send(event: Parameters<typeof dragNumberTransition>[1]): void {
    const result = dragNumberTransition(context, event); machine = result.context; runEffects(result.effects);
  }
  function pointerDown(event: PointerEvent): void {
    if (event.button !== 0 || disabled) return;
    event.preventDefault(); activePointer = event.pointerId; pointerStart = event.clientX; dragging = false;
    root.setPointerCapture(event.pointerId);
  }
  function pointerMove(event: PointerEvent): void {
    if (activePointer !== event.pointerId) return;
    if (!dragging) {
      if (Math.abs(event.clientX - pointerStart) < 2) return;
      dragging = true;
      send({ type: "DRAG_BEGIN", position: pointerStart, fine: event.shiftKey });
    }
    send({ type: "DRAG_MOVE", position: event.clientX, fine: event.shiftKey });
  }
  function pointerEnd(event: PointerEvent, openEntry = true): void {
    if (activePointer !== event.pointerId) return;
    activePointer = null;
    if (dragging) send({ type: "DRAG_END" });
    else if (openEntry) send({ type: "ENTRY_OPEN" });
    dragging = false;
  }
  function keydown(event: KeyboardEvent): void {
    if (event.key === "Enter") { event.preventDefault(); send({ type: "ENTRY_OPEN" }); }
    else if (["ArrowDown", "ArrowLeft", "ArrowUp", "ArrowRight"].includes(event.key)) { event.preventDefault(); send({ type: "KEY_NUDGE", direction: event.key === "ArrowDown" || event.key === "ArrowLeft" ? -1 : 1, fine: event.shiftKey }); }
    else if (event.key === "Home" || event.key === "End") { event.preventDefault(); send({ type: "KEY_BOUND", bound: event.key === "Home" ? "min" : "max" }); }
  }
</script>

<div
  bind:this={root}
  class="poodle-drag-number-field"
  role="spinbutton"
  tabindex={disabled ? undefined : 0}
  aria-label={ariaLabel ?? undefined}
  aria-valuemin={Number.isSafeInteger(min) ? min : undefined}
  aria-valuemax={Number.isSafeInteger(max) ? max : undefined}
  aria-valuenow={value}
  aria-valuetext={valueText}
  aria-disabled={disabled}
  data-scope="drag-number-field"
  data-part="root"
  data-state={visualState.drag === "none" ? "idle" : visualState.drag}
  onpointerdown={pointerDown} onpointermove={pointerMove} onpointerup={pointerEnd} onpointercancel={(event) => pointerEnd(event, false)}
  onmouseenter={() => send({ type: "HOVER", value: true })} onmouseleave={() => send({ type: "HOVER", value: false })}
  onfocus={() => send({ type: "FOCUS", value: true })} onblur={() => send({ type: "FOCUS", value: false })}
  onkeydown={keydown}
>
  <ValueVisual {visualState} text={valueText} kind="drag-number" />
  {#if context.entryOpen}
    <input bind:this={entry} class="poodle-drag-number-field__entry" aria-label={`${ariaLabel ?? "Number"} value`} value={entryDraft}
      oninput={(event) => entryDraft = event.currentTarget.value}
      onkeydown={(event) => { if (event.key === "Enter") { event.preventDefault(); send({ type: "ENTRY_COMMIT", text: entryDraft }); root.focus(); } else if (event.key === "Escape") { event.preventDefault(); send({ type: "ENTRY_CANCEL" }); root.focus(); } }}
      onblur={() => send({ type: "ENTRY_COMMIT", text: entryDraft })} />
  {/if}
</div>
