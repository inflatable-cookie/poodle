<script lang="ts">
  import "@inflatable-cookie/poodle-core/styles/envelope-editor.css";
  import {
    createEnvelopeContext,
    envelopeHitTest,
    envelopePointToNorm,
    envelopeTransition,
    envelopeVisualState,
    type EnvelopeContext,
    type EnvelopeEffect,
    type EnvelopePoint,
  } from "@inflatable-cookie/poodle-core";
  import EnvelopeVisual from "./audio/EnvelopeVisual.svelte";
  import { getUiPresentation, resolveSemanticControlSize } from "./presentation";
  import type { ControlDensity, ControlSize, SemanticControlSizeRole } from "./types";

  interface Props {
    size?: ControlSize | null;
    sizeRole?: SemanticControlSizeRole;
    density?: ControlDensity | null;
    points?: EnvelopePoint[];
    step?: number;
    disabled?: boolean;
    ariaLabel?: string | null;
    snapPoint?: (point: Pick<EnvelopePoint, "x" | "y">, points: EnvelopePoint[]) => Pick<EnvelopePoint, "x" | "y">;
    onPointsChange?: (points: EnvelopePoint[]) => void;
    onPointsCommit?: (points: EnvelopePoint[]) => void;
    onGestureBegin?: () => void;
    onGestureEnd?: () => void;
  }

  let {
    size = null, sizeRole = "control", density = null,
    points = $bindable([]), step = 0.01, disabled = false, ariaLabel = null,
    snapPoint = (point) => point, onPointsChange, onPointsCommit,
    onGestureBegin, onGestureEnd,
  }: Props = $props();

  const uiPresentation = getUiPresentation();
  const resolvedSize = $derived(size ?? resolveSemanticControlSize($uiPresentation.sizeScale, sizeRole));
  const resolvedDensity = $derived(density ?? $uiPresentation.density);

  let root: HTMLDivElement;
  let machine = $state(createEnvelopeContext());
  let activePointer: number | null = null;
  let pointSequence = 0;
  const context = $derived<EnvelopeContext>({ ...machine, points, step, disabled });
  const visualState = $derived(envelopeVisualState(context));

  function runEffects(effects: EnvelopeEffect[]): void {
    for (const effect of effects) {
      if (effect.type === "emitPointsChange") { points = effect.points; onPointsChange?.(effect.points); }
      else if (effect.type === "emitPointsCommit") { points = effect.points; onPointsCommit?.(effect.points); }
      else if (effect.type === "beginGesture") onGestureBegin?.();
      else if (effect.type === "endGesture") onGestureEnd?.();
    }
  }

  function send(event: Parameters<typeof envelopeTransition>[1]): void {
    const result = envelopeTransition(context, event);
    machine = result.context;
    runEffects(result.effects);
  }

  function pointerPoint(event: PointerEvent | MouseEvent): Pick<EnvelopePoint, "x" | "y"> {
    const rect = root.getBoundingClientRect();
    const proposed = envelopePointToNorm({ x: event.clientX, y: event.clientY }, rect);
    return snapPoint(proposed, context.points);
  }

  function pointerDown(event: PointerEvent): void {
    if (event.button !== 0 || disabled) return;
    const rect = root.getBoundingClientRect();
    const id = envelopeHitTest(context.points, { x: event.clientX, y: event.clientY }, rect);
    if (id === null) return;
    event.preventDefault();
    activePointer = event.pointerId;
    root.setPointerCapture(event.pointerId);
    send({ type: "DRAG_BEGIN", id });
  }

  function pointerMove(event: PointerEvent): void {
    if (activePointer !== event.pointerId) return;
    send({ type: "DRAG_MOVE", point: pointerPoint(event) });
  }

  function pointerEnd(event: PointerEvent): void {
    if (activePointer !== event.pointerId) return;
    activePointer = null;
    send({ type: "DRAG_END" });
  }

  function pointerHover(event: PointerEvent): void {
    if (activePointer !== null) return;
    const rect = root.getBoundingClientRect();
    send({ type: "HOVER_POINT", id: envelopeHitTest(context.points, { x: event.clientX, y: event.clientY }, rect) });
  }

  function addPoint(event: MouseEvent): void {
    if (disabled) return;
    const rect = root.getBoundingClientRect();
    if (envelopeHitTest(context.points, { x: event.clientX, y: event.clientY }, rect) !== null) return;
    event.preventDefault();
    const point = pointerPoint(event);
    let id: string;
    do id = `envelope-point-${++pointSequence}`;
    while (context.points.some((candidate) => candidate.id === id));
    send({ type: "ADD_POINT", point: { id, ...point, curve: 0 } });
  }

  function pointKeydown(event: KeyboardEvent, id: string): void {
    send({ type: "SELECT_POINT", id });
    const multiplier = event.key === "PageUp" || event.key === "PageDown" ? 10 : 1;
    if (["ArrowLeft", "ArrowRight", "ArrowUp", "ArrowDown", "PageUp", "PageDown"].includes(event.key)) {
      event.preventDefault();
      const vertical = ["ArrowUp", "ArrowDown", "PageUp", "PageDown"].includes(event.key);
      const positive = ["ArrowRight", "ArrowUp", "PageUp"].includes(event.key);
      send({ type: "NUDGE_SELECTED", axis: vertical ? "y" : "x", direction: positive ? 1 : -1, multiplier, fine: event.shiftKey });
    } else if (event.key === "Delete" || event.key === "Backspace") {
      event.preventDefault(); send({ type: "REMOVE_SELECTED" });
    } else if (event.key === "[" || event.key === "]") {
      event.preventDefault(); send({ type: "NUDGE_CURVE", direction: event.key === "]" ? 1 : -1, fine: event.shiftKey });
    }
  }
</script>

<div
  bind:this={root}
  class="poodle-envelope-editor"
  role="group"
  aria-label={ariaLabel ?? undefined}
  aria-disabled={disabled}
  data-scope="envelope-editor"
  data-part="root"
  data-size={resolvedSize}
  data-density={resolvedDensity}
  onpointerdown={pointerDown}
  onpointermove={(event) => { pointerMove(event); pointerHover(event); }}
  onpointerup={pointerEnd}
  onpointercancel={pointerEnd}
  onpointerleave={() => send({ type: "HOVER_POINT", id: null })}
  ondblclick={addPoint}
  onfocusin={() => send({ type: "FOCUS", value: true })}
  onfocusout={(event) => { if (!root.contains(event.relatedTarget as Node | null)) send({ type: "FOCUS", value: false }); }}
>
  <EnvelopeVisual {visualState} />
  {#each visualState.points as point, index (point.id)}
    <button
      type="button"
      class="poodle-envelope-editor__point-control"
      style={`left: ${point.xNorm * 100}%; top: ${(1 - point.yNorm) * 100}%`}
      aria-label={`Point ${index + 1}, X ${Math.round(point.xNorm * 100)} percent, Y ${Math.round(point.yNorm * 100)} percent, curve ${point.curve.toFixed(2)}`}
      aria-pressed={point.selected}
      disabled={disabled}
      onfocus={() => send({ type: "SELECT_POINT", id: point.id })}
      onkeydown={(event) => pointKeydown(event, point.id)}
    ></button>
  {/each}
</div>
