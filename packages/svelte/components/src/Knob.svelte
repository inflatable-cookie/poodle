<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/knob.css";
  import {
    audioValueText, createKnobContext, hitTestCircle, knobPointToNorm,
    knobTransition, knobVisualState, type AudioAutomationState,
    type AudioValueEffect, type AudioValueFormat, type AudioValueLaw,
    type KnobContext,
  } from "@inflatable-cookie/poodle-core";
  import { onDestroy, tick } from "svelte";
  import KnobVisual from "./audio/KnobVisual.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    size?: ControlSize | null; sizeRole?: SemanticControlSizeRole; density?: ControlDensity | null;
    value?: number; min?: number; max?: number; law?: AudioValueLaw;
    defaultValue?: number; dragMode?: "vertical" | "circular";
    dragSensitivity?: number; keyboardStep?: number; format?: AudioValueFormat;
    automation?: AudioAutomationState; disabled?: boolean; ariaLabel?: string | null;
    onValueChange?: (value: number) => void; onValueCommit?: (value: number) => void;
    onGestureBegin?: () => void; onGestureEnd?: () => void;
  }

  let {
    size = null, sizeRole = "control", density = null,
    value = $bindable(0), min = 0, max = 1, law = { type: "linear" },
    defaultValue = 0, dragMode = "vertical", dragSensitivity = 160,
    keyboardStep = 0.01, format = { type: "number", decimals: 2 },
    automation = "none", disabled = false, ariaLabel = null,
    onValueChange, onValueCommit, onGestureBegin, onGestureEnd,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  let machine = $state(createKnobContext());
  let root: HTMLDivElement;
  let entry = $state<HTMLInputElement>();
  let entryDraft = $state("");
  let activePointer: number | null = null;
  /**
   * The machine state this adapter last produced, written before any host
   * callback runs. Terminal cleanup reads it rather than the reactive
   * `context`, because a host that removes the control from inside
   * `onGestureBegin` or `onValueChange` tears it down before that state is
   * observable through the component's own reactive graph.
   */
  let live: KnobContext = createKnobContext();
  let skipEntryBlur = false;
  const context = $derived<KnobContext>({ ...machine, value, min, max, law, defaultValue, dragMode, dragSensitivity, keyboardStep, format, automation, disabled });
  const visualState = $derived(knobVisualState(context));
  const valueText = $derived(audioValueText(context));

  function dispatch(effect: AudioValueEffect): void {
    if (effect.type === "emitValueChange") { value = effect.value; onValueChange?.(effect.value); }
    else if (effect.type === "emitValueCommit") { value = effect.value; onValueCommit?.(effect.value); }
    else if (effect.type === "beginGesture") onGestureBegin?.();
    else if (effect.type === "endGesture") onGestureEnd?.();
    else if (effect.type === "requestEntryFocus") {
      entryDraft = valueText;
      skipEntryBlur = false;
      void tick().then(() => { entry?.focus(); entry?.select(); });
    }
  }

  let effectBatches: AudioValueEffect[][] = [];
  let drainingEffects = false;

  /**
   * Effect batches drain in order even when a host tears the control down from
   * inside one of them. A teardown that lands mid-batch queues its terminal
   * instead of interleaving, so no callback from the accepted transition can
   * run after the terminal that teardown triggered.
   */
  function runEffects(effects: AudioValueEffect[]): void {
    effectBatches.push(effects);
    if (drainingEffects) return;
    drainingEffects = true;
    try {
      for (let batch = effectBatches.shift(); batch; batch = effectBatches.shift()) {
        for (const effect of batch) dispatch(effect);
      }
    } finally {
      drainingEffects = false;
    }
  }

  function commit(result: ReturnType<typeof knobTransition>): void {
    live = result.context;
    machine = result.context;
    runEffects(result.effects);
  }

  function send(event: Parameters<typeof knobTransition>[1]): void {
    commit(knobTransition(context, event));
  }

  /** Terminals resolve from the live snapshot, never from the reactive context. */
  function terminate(type: "DRAG_END" | "DRAG_CANCEL"): void {
    commit(knobTransition(live, { type }));
  }

  function pointerDown(event: PointerEvent): void {
    // One primary pointer owns the gesture. A second pointer-down cannot
    // replace the active pointer or open a second gesture.
    if (event.button !== 0 || disabled || activePointer !== null) return;
    const rect = root.getBoundingClientRect();
    if (!hitTestCircle({ x: event.clientX, y: event.clientY }, rect)) return;
    event.preventDefault();
    activePointer = event.pointerId;
    root.setPointerCapture(event.pointerId);
    const circularNorm = knobPointToNorm({ x: event.clientX, y: event.clientY }, rect);
    // Begin and circular press position resolve in one commit, so a host that
    // tears the control down from `onGestureBegin` never sees a second
    // transition run against a destroyed component.
    const begun = knobTransition(context, { type: "DRAG_BEGIN", position: dragMode === "circular" ? circularNorm : event.clientY, fine: event.shiftKey });
    if (dragMode === "circular") {
      const moved = knobTransition(begun.context, { type: "DRAG_SET_NORM", valueNorm: circularNorm, fine: event.shiftKey });
      commit({ context: moved.context, effects: [...begun.effects, ...moved.effects] });
    } else commit(begun);
  }

  function pointerMove(event: PointerEvent): void {
    if (activePointer !== event.pointerId) return;
    if (dragMode === "circular") {
      const rect = root.getBoundingClientRect();
      send({ type: "DRAG_SET_NORM", valueNorm: knobPointToNorm({ x: event.clientX, y: event.clientY }, rect), fine: event.shiftKey });
    } else send({ type: "DRAG_MOVE", position: event.clientY, fine: event.shiftKey });
  }

  function pointerUp(event: PointerEvent): void {
    if (activePointer !== event.pointerId) return;
    activePointer = null;
    terminate("DRAG_END");
  }

  /**
   * Pointer cancel, lost capture, and teardown all close the gesture the same
   * way, so a captured gesture can never outlive its pointer or its component.
   * A stale pointer id is ignored, and the machine makes a repeat inert.
   */
  function cancelGesture(pointerId: number | null = null): void {
    if (activePointer === null || (pointerId !== null && activePointer !== pointerId)) return;
    activePointer = null;
    terminate("DRAG_CANCEL");
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
  class="poodle-knob"
  role="slider"
  tabindex={disabled ? undefined : 0}
  aria-label={ariaLabel ?? undefined}
  aria-valuemin={min}
  aria-valuemax={max}
  aria-valuenow={value}
  aria-valuetext={valueText}
  aria-disabled={disabled}
  data-scope="knob"
  data-part="root"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  data-state={visualState.drag === "none" ? "idle" : visualState.drag}
  onpointerdown={pointerDown}
  onpointermove={pointerMove}
  onpointerup={pointerUp}
  onpointercancel={(event) => cancelGesture(event.pointerId)}
  onlostpointercapture={(event) => cancelGesture(event.pointerId)}
  onmouseenter={() => send({ type: "HOVER", value: true })}
  onmouseleave={() => send({ type: "HOVER", value: false })}
  onfocus={() => send({ type: "FOCUS", value: true })}
  onblur={() => send({ type: "FOCUS", value: false })}
  onwheel={(event) => { event.preventDefault(); send({ type: "WHEEL", direction: event.deltaY < 0 ? 1 : -1, fine: event.shiftKey }); }}
  ondblclick={(event) => { event.preventDefault(); send({ type: "RESET" }); }}
  onkeydown={keydown}
>
  <KnobVisual {visualState} />
  {#if context.entryOpen}
    <input
      bind:this={entry}
      class="poodle-knob__entry"
      aria-label={`${ariaLabel ?? "Knob"} value`}
      value={entryDraft}
      oninput={(event) => entryDraft = event.currentTarget.value}
      onkeydown={entryKeydown}
      onblur={entryBlur}
    />
  {/if}
</div>
