import { bipolarCenterForLaw, constrainAudioValue, denormalizeAudioValue, normalizeAudioValue, type AudioValueLaw, linearValueLaw } from "./laws";
import { formatAudioValue, numberValueFormat, parseAudioValue, type AudioValueFormat } from "./format";
import type { AudioAutomationState, AudioControlVisualState, AudioDragState, AudioPoint, AudioRect } from "./types";

export type AudioValueEffect =
  | { type: "emitValueChange"; value: number }
  | { type: "emitValueCommit"; value: number }
  | { type: "beginGesture" }
  | { type: "endGesture" }
  | { type: "requestEntryFocus" };

export interface AudioValueInteraction {
  hover: boolean;
  focus: boolean;
  drag: AudioDragState;
  automation: AudioAutomationState;
  entryOpen: boolean;
  dragStartValue: number;
  dragStartPosition: number;
}

export const idleAudioValueInteraction: AudioValueInteraction = {
  hover: false,
  focus: false,
  drag: "none",
  automation: "none",
  entryOpen: false,
  dragStartValue: 0,
  dragStartPosition: 0,
};

interface AudioValueContextBase extends AudioValueInteraction {
  value: number;
  min: number;
  max: number;
  law: AudioValueLaw;
  defaultValue: number;
  keyboardStep: number;
  format: AudioValueFormat;
  disabled: boolean;
}

export interface KnobContext extends AudioValueContextBase {
  dragMode: "vertical" | "circular";
  dragSensitivity: number;
}

export interface FaderContext extends AudioValueContextBase {
  orientation: "horizontal" | "vertical";
  detents: number[];
  detentSnap: number;
}

export interface DragNumberContext extends AudioValueContextBase {
  dragSensitivity: number;
}

export function createKnobContext(input: Partial<KnobContext> = {}): KnobContext {
  return {
    value: 0, min: 0, max: 1, law: linearValueLaw, defaultValue: 0,
    keyboardStep: 0.01, format: numberValueFormat, disabled: false,
    dragMode: "vertical", dragSensitivity: 160,
    ...idleAudioValueInteraction, ...input,
  };
}

export function createFaderContext(input: Partial<FaderContext> = {}): FaderContext {
  return {
    value: 0, min: 0, max: 1, law: linearValueLaw, defaultValue: 0,
    keyboardStep: 0.01, format: numberValueFormat, disabled: false,
    orientation: "vertical", detents: [], detentSnap: 0.015,
    ...idleAudioValueInteraction, ...input,
  };
}

export function createDragNumberContext(input: Partial<DragNumberContext> = {}): DragNumberContext {
  return {
    value: 0, min: Number.MIN_SAFE_INTEGER, max: Number.MAX_SAFE_INTEGER,
    law: linearValueLaw, defaultValue: 0, keyboardStep: 1,
    format: numberValueFormat, disabled: false, dragSensitivity: 0.1,
    ...idleAudioValueInteraction, ...input,
  };
}

export type AudioValueEvent =
  | { type: "HOVER"; value: boolean }
  | { type: "FOCUS"; value: boolean }
  | { type: "SET_AUTOMATION"; value: AudioAutomationState }
  | { type: "SET_VALUE"; value: number }
  | { type: "DRAG_BEGIN"; position: number; fine: boolean }
  | { type: "DRAG_MOVE"; position: number; fine: boolean }
  | { type: "DRAG_SET_NORM"; valueNorm: number; fine: boolean }
  | { type: "DRAG_END" }
  | { type: "DRAG_CANCEL" }
  | { type: "WHEEL"; direction: -1 | 1; fine: boolean }
  | { type: "RESET" }
  | { type: "KEY_NUDGE"; direction: -1 | 1; multiplier?: number; fine?: boolean }
  | { type: "KEY_BOUND"; bound: "min" | "max" }
  | { type: "ENTRY_OPEN" }
  | { type: "ENTRY_CANCEL" }
  | { type: "ENTRY_COMMIT"; text: string };

export interface AudioValueResult<T> { context: T; effects: AudioValueEffect[] }

function visualState(context: AudioValueContextBase): AudioControlVisualState {
  return {
    valueNorm: normalizeAudioValue(context.value, context.min, context.max, context.law),
    rawValue: context.value,
    bipolarCenter: bipolarCenterForLaw(context.law),
    hover: context.hover,
    focus: context.focus,
    drag: context.drag,
    automation: context.automation,
    enabled: !context.disabled,
  };
}

export const knobVisualState = (context: KnobContext): AudioControlVisualState => visualState(context);
export const faderVisualState = (context: FaderContext): AudioControlVisualState => visualState(context);
export const dragNumberVisualState = (context: DragNumberContext): AudioControlVisualState => visualState(context);
export const valueReadoutVisualState = (value: number, min = 0, max = 1, law: AudioValueLaw = linearValueLaw, enabled = true): AudioControlVisualState => ({
  valueNorm: normalizeAudioValue(value, min, max, law), rawValue: value,
  bipolarCenter: bipolarCenterForLaw(law), hover: false, focus: false,
  drag: "none", automation: "none", enabled,
});

export function audioValueText(context: Pick<AudioValueContextBase, "value" | "format">): string {
  return formatAudioValue(context.value, context.format);
}

function atomicValue<T extends AudioValueContextBase>(context: T, value: number): AudioValueResult<T> {
  const next = constrainAudioValue(value, context.min, context.max, context.law);
  return { context: { ...context, value: next }, effects: [
    { type: "emitValueChange", value: next }, { type: "emitValueCommit", value: next },
  ] };
}

function commonTransition<T extends AudioValueContextBase>(context: T, event: AudioValueEvent): AudioValueResult<T> | null {
  switch (event.type) {
    case "HOVER": return { context: { ...context, hover: event.value }, effects: [] };
    case "FOCUS": return { context: { ...context, focus: event.value }, effects: [] };
    case "SET_AUTOMATION": return { context: { ...context, automation: event.value }, effects: [] };
    case "SET_VALUE": return { context: { ...context, value: constrainAudioValue(event.value, context.min, context.max, context.law) }, effects: [] };
    case "RESET": return context.disabled ? { context, effects: [] } : atomicValue(context, context.defaultValue);
    case "WHEEL": {
      if (context.disabled) return { context, effects: [] };
      const scale = event.fine ? 0.1 : 1;
      return atomicValue(context, context.value + event.direction * context.keyboardStep * scale);
    }
    case "KEY_NUDGE": {
      if (context.disabled) return { context, effects: [] };
      const scale = event.fine ? 0.1 : 1;
      return atomicValue(context, context.value + event.direction * context.keyboardStep * (event.multiplier ?? 1) * scale);
    }
    case "KEY_BOUND": return context.disabled ? { context, effects: [] } : atomicValue(context, event.bound === "min" ? context.min : context.max);
    case "ENTRY_OPEN": return context.disabled ? { context, effects: [] } : { context: { ...context, entryOpen: true }, effects: [{ type: "requestEntryFocus" }] };
    case "ENTRY_CANCEL": return { context: { ...context, entryOpen: false }, effects: [] };
    case "ENTRY_COMMIT": {
      const parsed = parseAudioValue(event.text, context.format);
      if (context.disabled || parsed === null) return { context: { ...context, entryOpen: false }, effects: [] };
      const result = atomicValue({ ...context, entryOpen: false }, parsed);
      return result;
    }
    default: return null;
  }
}

/**
 * Terminal for an accepted pointer gesture. Release and cancellation close it
 * the same way — one commit, one `endGesture` — and both are inert once the
 * gesture is closed, so a repeated, stale, lost-capture, or teardown terminal
 * can never duplicate the pair. A control disabled mid-gesture may still
 * close: the begin was accepted while it was enabled, and stranding the
 * gesture would leave the host's automation latched open.
 */
function endDrag<T extends AudioValueContextBase>(context: T): AudioValueResult<T> {
  if (context.drag === "none") return { context, effects: [] };
  return { context: { ...context, drag: "none" }, effects: [
    { type: "emitValueCommit", value: context.value }, { type: "endGesture" },
  ] };
}

/**
 * Accepts one pointer gesture. A second begin while one is open is inert, so
 * `beginGesture` and `endGesture` stay paired exactly once per gesture.
 */
function beginDrag<T extends AudioValueContextBase>(context: T, position: number, fine: boolean): AudioValueResult<T> {
  if (context.disabled || context.drag !== "none") return { context, effects: [] };
  return {
    context: {
      ...context,
      drag: fine ? "fine" : "coarse",
      dragStartValue: context.value,
      dragStartPosition: position,
    },
    effects: [{ type: "beginGesture" }],
  };
}

/**
 * Coarse/fine switching re-anchors at the current value and the current
 * pointer, so holding or releasing the modifier never jumps. The transition
 * that flips the modifier only rebases; travel resumes from the next move.
 */
function rebaseDrag<T extends AudioValueContextBase>(context: T, position: number, fine: boolean): AudioValueResult<T> | null {
  const nextDrag: AudioDragState = fine ? "fine" : "coarse";
  if (context.drag === nextDrag) return null;
  return {
    context: {
      ...context,
      drag: nextDrag,
      dragStartValue: context.value,
      dragStartPosition: position,
    },
    effects: [],
  };
}

/** A move is live only inside an accepted gesture on an enabled control. */
function dragging(context: AudioValueContextBase): boolean {
  return !context.disabled && context.drag !== "none";
}

export function knobTransition(context: KnobContext, event: AudioValueEvent): AudioValueResult<KnobContext> {
  const common = commonTransition(context, event);
  if (common) return common;
  switch (event.type) {
    case "DRAG_BEGIN": return beginDrag(context, event.position, event.fine);
    case "DRAG_MOVE": {
      // Vertical mapping: anchored pointer delta over `dragSensitivity`.
      if (context.dragMode !== "vertical" || !dragging(context)) return { context, effects: [] };
      const rebased = rebaseDrag(context, event.position, event.fine);
      if (rebased) return rebased;
      const scale = event.fine ? 0.1 : 1;
      const startNorm = normalizeAudioValue(context.dragStartValue, context.min, context.max, context.law);
      const norm = startNorm + ((context.dragStartPosition - event.position) / Math.max(context.dragSensitivity, 1)) * scale;
      const value = denormalizeAudioValue(norm, context.min, context.max, context.law);
      return { context: { ...context, value }, effects: [{ type: "emitValueChange", value }] };
    }
    case "DRAG_SET_NORM": {
      // Circular mapping: the adapter resolves the 270 degree sweep position.
      if (context.dragMode !== "circular" || !dragging(context)) return { context, effects: [] };
      const rebased = rebaseDrag(context, event.valueNorm, event.fine);
      if (rebased) return rebased;
      const startNorm = normalizeAudioValue(context.dragStartValue, context.min, context.max, context.law);
      const target = event.fine ? startNorm + (event.valueNorm - context.dragStartPosition) * 0.1 : event.valueNorm;
      const value = denormalizeAudioValue(target, context.min, context.max, context.law);
      return { context: { ...context, value }, effects: [{ type: "emitValueChange", value }] };
    }
    case "DRAG_END":
    case "DRAG_CANCEL": return endDrag(context);
    default: return { context, effects: [] };
  }
}

/**
 * Nearest declared detent inside the normalized snap radius. The radius is
 * inclusive and the first declared detent wins a tie, so two detents that are
 * equidistant from the pointer always resolve the same way.
 */
function snapFaderDetent(context: FaderContext, norm: number): number {
  let best = norm;
  let distance = Number.POSITIVE_INFINITY;
  for (const detent of context.detents) {
    const detentNorm = normalizeAudioValue(detent, context.min, context.max, context.law);
    const candidate = Math.abs(norm - detentNorm);
    if (candidate <= context.detentSnap && candidate < distance) { best = detentNorm; distance = candidate; }
  }
  return best;
}

export function faderTransition(context: FaderContext, event: AudioValueEvent): AudioValueResult<FaderContext> {
  const common = commonTransition(context, event);
  if (common) return common;
  switch (event.type) {
    case "DRAG_BEGIN": return beginDrag(context, event.position, event.fine);
    case "DRAG_SET_NORM": {
      // The adapter resolves the axis position through `faderPointToNorm`.
      if (!dragging(context)) return { context, effects: [] };
      const rebased = rebaseDrag(context, event.valueNorm, event.fine);
      if (rebased) return rebased;
      const startNorm = normalizeAudioValue(context.dragStartValue, context.min, context.max, context.law);
      const target = event.fine ? startNorm + (event.valueNorm - context.dragStartPosition) * 0.1 : event.valueNorm;
      const value = denormalizeAudioValue(snapFaderDetent(context, target), context.min, context.max, context.law);
      return { context: { ...context, value }, effects: [{ type: "emitValueChange", value }] };
    }
    case "DRAG_MOVE": return { context, effects: [] };
    case "DRAG_END":
    case "DRAG_CANCEL": return endDrag(context);
    default: return { context, effects: [] };
  }
}

export function dragNumberTransition(context: DragNumberContext, event: AudioValueEvent): AudioValueResult<DragNumberContext> {
  const common = commonTransition(context, event);
  if (common) return common;
  switch (event.type) {
    case "DRAG_BEGIN": return beginDrag(context, event.position, event.fine);
    case "DRAG_MOVE": {
      if (!dragging(context)) return { context, effects: [] };
      const rebased = rebaseDrag(context, event.position, event.fine);
      if (rebased) return rebased;
      const scale = event.fine ? 0.1 : 1;
      const value = constrainAudioValue(context.dragStartValue + (event.position - context.dragStartPosition) * context.dragSensitivity * scale, context.min, context.max, context.law);
      return { context: { ...context, value }, effects: [{ type: "emitValueChange", value }] };
    }
    case "DRAG_SET_NORM": return { context, effects: [] };
    case "DRAG_END":
    case "DRAG_CANCEL": return endDrag(context);
    default: return { context, effects: [] };
  }
}

/** Standard 270° knob sweep: -135° is zero and +135° is one. */
export function knobPointToNorm(point: AudioPoint, rect: AudioRect): number {
  const x = point.x - (rect.left + rect.width / 2);
  const y = point.y - (rect.top + rect.height / 2);
  let degrees = Math.atan2(y, x) * 180 / Math.PI + 90;
  if (degrees < -180) degrees += 360;
  if (degrees > 180) degrees -= 360;
  return Math.min(Math.max((degrees + 135) / 270, 0), 1);
}

export function faderPointToNorm(point: AudioPoint, rect: AudioRect, orientation: "horizontal" | "vertical"): number {
  if (orientation === "horizontal") return Math.min(Math.max((point.x - rect.left) / Math.max(rect.width, 1), 0), 1);
  return 1 - Math.min(Math.max((point.y - rect.top) / Math.max(rect.height, 1), 0), 1);
}
