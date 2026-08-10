import { constrainAudioValue, denormalizeAudioValue, linearValueLaw, normalizeAudioValue, type AudioValueLaw } from "./laws";
import type { AudioAutomationState, AudioDragState, AudioPoint, AudioRect, XYPadVisualState } from "./types";

export interface XYPadContext {
  x: number;
  y: number;
  minX: number;
  maxX: number;
  minY: number;
  maxY: number;
  lawX: AudioValueLaw;
  lawY: AudioValueLaw;
  defaultX: number;
  defaultY: number;
  keyboardStepX: number;
  keyboardStepY: number;
  hover: boolean;
  focus: boolean;
  drag: AudioDragState;
  automation: AudioAutomationState;
  dragStartX: number;
  dragStartY: number;
  dragStartNormX: number;
  dragStartNormY: number;
  disabled: boolean;
}

export type XYPadEffect =
  | { type: "emitValueChange"; x: number; y: number }
  | { type: "emitValueCommit"; x: number; y: number }
  | { type: "beginGesture" }
  | { type: "endGesture" };

export type XYPadEvent =
  | { type: "SET_VALUES"; x: number; y: number }
  | { type: "HOVER"; value: boolean }
  | { type: "FOCUS"; value: boolean }
  | { type: "SET_AUTOMATION"; value: AudioAutomationState }
  | { type: "DRAG_BEGIN"; xNorm: number; yNorm: number; fine: boolean }
  | { type: "DRAG_MOVE"; xNorm: number; yNorm: number; fine: boolean }
  | { type: "DRAG_END" }
  | { type: "RESET" }
  | { type: "NUDGE"; axis: "x" | "y"; direction: -1 | 1; multiplier?: number; fine?: boolean }
  | { type: "BOUND"; axis: "x" | "y"; bound: "min" | "max" };

export interface XYPadResult {
  context: XYPadContext;
  effects: XYPadEffect[];
}

export function createXYPadContext(input: Partial<XYPadContext> = {}): XYPadContext {
  return {
    x: 0, y: 0, minX: 0, maxX: 1, minY: 0, maxY: 1,
    lawX: linearValueLaw, lawY: linearValueLaw,
    defaultX: 0, defaultY: 0, keyboardStepX: 0.01, keyboardStepY: 0.01,
    hover: false, focus: false, drag: "none", automation: "none",
    dragStartX: 0, dragStartY: 0, dragStartNormX: 0, dragStartNormY: 0,
    disabled: false,
    ...input,
  };
}

function constrained(context: XYPadContext, x: number, y: number): Pick<XYPadContext, "x" | "y"> {
  return {
    x: constrainAudioValue(x, context.minX, context.maxX, context.lawX),
    y: constrainAudioValue(y, context.minY, context.maxY, context.lawY),
  };
}

function atomic(context: XYPadContext, x: number, y: number): XYPadResult {
  const values = constrained(context, x, y);
  return { context: { ...context, ...values }, effects: [
    { type: "emitValueChange", ...values }, { type: "emitValueCommit", ...values },
  ] };
}

export function xyPadTransition(context: XYPadContext, event: XYPadEvent): XYPadResult {
  switch (event.type) {
    case "SET_VALUES": return { context: { ...context, ...constrained(context, event.x, event.y) }, effects: [] };
    case "HOVER": return { context: { ...context, hover: event.value }, effects: [] };
    case "FOCUS": return { context: { ...context, focus: event.value }, effects: [] };
    case "SET_AUTOMATION": return { context: { ...context, automation: event.value }, effects: [] };
    case "RESET": return context.disabled ? { context, effects: [] } : atomic(context, context.defaultX, context.defaultY);
    case "DRAG_BEGIN": {
      if (context.disabled) return { context, effects: [] };
      const drag: AudioDragState = event.fine ? "fine" : "coarse";
      const values = event.fine ? { x: context.x, y: context.y } : constrained(
        context,
        denormalizeAudioValue(event.xNorm, context.minX, context.maxX, context.lawX),
        denormalizeAudioValue(event.yNorm, context.minY, context.maxY, context.lawY),
      );
      return { context: {
        ...context, ...values, drag,
        dragStartX: values.x, dragStartY: values.y,
        dragStartNormX: event.xNorm, dragStartNormY: event.yNorm,
      }, effects: [
        { type: "beginGesture" },
        ...(event.fine ? [] : [{ type: "emitValueChange", ...values } as const]),
      ] };
    }
    case "DRAG_MOVE": {
      if (context.disabled || context.drag === "none") return { context, effects: [] };
      const nextDrag: AudioDragState = event.fine ? "fine" : "coarse";
      if (nextDrag !== context.drag) return { context: {
        ...context, drag: nextDrag, dragStartX: context.x, dragStartY: context.y,
        dragStartNormX: event.xNorm, dragStartNormY: event.yNorm,
      }, effects: [] };
      const startXNorm = normalizeAudioValue(context.dragStartX, context.minX, context.maxX, context.lawX);
      const startYNorm = normalizeAudioValue(context.dragStartY, context.minY, context.maxY, context.lawY);
      const xNorm = event.fine ? startXNorm + (event.xNorm - context.dragStartNormX) * 0.1 : event.xNorm;
      const yNorm = event.fine ? startYNorm + (event.yNorm - context.dragStartNormY) * 0.1 : event.yNorm;
      const values = constrained(
        context,
        denormalizeAudioValue(xNorm, context.minX, context.maxX, context.lawX),
        denormalizeAudioValue(yNorm, context.minY, context.maxY, context.lawY),
      );
      return { context: { ...context, ...values }, effects: [{ type: "emitValueChange", ...values }] };
    }
    case "DRAG_END": return context.drag === "none" ? { context, effects: [] } : {
      context: { ...context, drag: "none" },
      effects: [{ type: "emitValueCommit", x: context.x, y: context.y }, { type: "endGesture" }],
    };
    case "NUDGE": {
      if (context.disabled) return { context, effects: [] };
      const fine = event.fine ? 0.1 : 1;
      const multiplier = event.multiplier ?? 1;
      return event.axis === "x"
        ? atomic(context, context.x + event.direction * context.keyboardStepX * multiplier * fine, context.y)
        : atomic(context, context.x, context.y + event.direction * context.keyboardStepY * multiplier * fine);
    }
    case "BOUND": {
      if (context.disabled) return { context, effects: [] };
      if (event.axis === "x") return atomic(context, event.bound === "min" ? context.minX : context.maxX, context.y);
      return atomic(context, context.x, event.bound === "min" ? context.minY : context.maxY);
    }
  }
}

export function xyPadVisualState(context: XYPadContext): XYPadVisualState {
  return {
    xNorm: normalizeAudioValue(context.x, context.minX, context.maxX, context.lawX),
    yNorm: normalizeAudioValue(context.y, context.minY, context.maxY, context.lawY),
    rawX: context.x, rawY: context.y, hover: context.hover, focus: context.focus,
    drag: context.drag, automation: context.automation, enabled: !context.disabled,
  };
}

export function xyPadPointToNorm(point: AudioPoint, rect: AudioRect): { xNorm: number; yNorm: number } {
  return {
    xNorm: Math.min(Math.max((point.x - rect.left) / Math.max(rect.width, 1), 0), 1),
    yNorm: 1 - Math.min(Math.max((point.y - rect.top) / Math.max(rect.height, 1), 0), 1),
  };
}
