import type { AudioSwitchVisualState } from "./types";

export type AudioSwitchMode = "latch" | "momentary" | "multi";

export interface AudioSwitchContext {
  mode: AudioSwitchMode;
  state: number;
  stateCount: number;
  lampOn: boolean | null;
  pressed: boolean;
  hover: boolean;
  focus: boolean;
  disabled: boolean;
}

export type AudioSwitchEffect =
  | { type: "emitStateChange"; state: number }
  | { type: "emitStateCommit"; state: number };

export type AudioSwitchEvent =
  | { type: "SET_STATE"; state: number }
  | { type: "SET_LAMP"; value: boolean | null }
  | { type: "HOVER"; value: boolean }
  | { type: "FOCUS"; value: boolean }
  | { type: "PRESS" }
  | { type: "RELEASE" }
  | { type: "CANCEL" };

export interface AudioSwitchResult {
  context: AudioSwitchContext;
  effects: AudioSwitchEffect[];
}

function count(value: number): number {
  return Math.max(Math.floor(Number.isFinite(value) ? value : 2), 2);
}

function state(value: number, stateCount: number): number {
  return Math.min(Math.max(Math.round(Number.isFinite(value) ? value : 0), 0), count(stateCount) - 1);
}

export function createAudioSwitchContext(input: Partial<AudioSwitchContext> = {}): AudioSwitchContext {
  const stateCount = count(input.stateCount ?? 2);
  return {
    mode: input.mode ?? "latch",
    state: state(input.state ?? 0, stateCount),
    stateCount,
    lampOn: input.lampOn ?? null,
    pressed: input.pressed ?? false,
    hover: input.hover ?? false,
    focus: input.focus ?? false,
    disabled: input.disabled ?? false,
  };
}

function changed(context: AudioSwitchContext, nextState: number, commit: boolean): AudioSwitchResult {
  const next = state(nextState, context.stateCount);
  return { context: { ...context, state: next }, effects: [
    { type: "emitStateChange", state: next },
    ...(commit ? [{ type: "emitStateCommit", state: next } as const] : []),
  ] };
}

export function audioSwitchTransition(context: AudioSwitchContext, event: AudioSwitchEvent): AudioSwitchResult {
  switch (event.type) {
    case "SET_STATE": return { context: { ...context, state: state(event.state, context.stateCount) }, effects: [] };
    case "SET_LAMP": return { context: { ...context, lampOn: event.value }, effects: [] };
    case "HOVER": return { context: { ...context, hover: event.value }, effects: [] };
    case "FOCUS": return { context: { ...context, focus: event.value }, effects: [] };
    case "PRESS": {
      if (context.disabled || context.pressed) return { context, effects: [] };
      const pressed = { ...context, pressed: true };
      return context.mode === "momentary" ? changed(pressed, 1, false) : { context: pressed, effects: [] };
    }
    case "RELEASE": {
      if (!context.pressed) return { context, effects: [] };
      const released = { ...context, pressed: false };
      if (context.mode === "momentary") return changed(released, 0, true);
      if (context.disabled) return { context: released, effects: [] };
      if (context.mode === "multi") return changed(released, context.state + 1 >= context.stateCount ? 0 : context.state + 1, true);
      return changed(released, context.state === 0 ? 1 : 0, true);
    }
    case "CANCEL": {
      if (!context.pressed) return { context, effects: [] };
      const released = { ...context, pressed: false };
      return context.mode === "momentary" ? changed(released, 0, true) : { context: released, effects: [] };
    }
  }
}

export function audioSwitchVisualState(context: AudioSwitchContext): AudioSwitchVisualState {
  return {
    state: context.state,
    stateCount: context.stateCount,
    pressed: context.pressed,
    lampOn: context.lampOn ?? context.state > 0,
    hover: context.hover,
    focus: context.focus,
    enabled: !context.disabled,
  };
}
