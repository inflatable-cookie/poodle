import type { GainReductionMeterVisualState } from "./types";

export interface GainReductionFrame {
  atMs: number;
  reductionDb: number;
  durationMs: number;
}

export interface GainReductionMeterContext {
  maxReductionDb: number;
  enabled: boolean;
  lastAtMs: number | null;
  reductionDb: number;
  ballisticDb: number;
}

export type GainReductionMeterEvent =
  | { type: "PUSH_FRAME"; frame: GainReductionFrame }
  | { type: "RESET" }
  | { type: "SET_ENABLED"; value: boolean };

export interface GainReductionMeterResult {
  context: GainReductionMeterContext;
  effects: [];
}

export const GAIN_REDUCTION_ATTACK_MS = 10;
export const GAIN_REDUCTION_RELEASE_MS = 300;

const clamp = (value: number, max: number): number => Math.min(Math.max(value, 0), max);
const normalizeMax = (value: number): number => Number.isFinite(value) && value > 0 ? value : 30;

export function createGainReductionMeterContext(input: Partial<GainReductionMeterContext> = {}): GainReductionMeterContext {
  const maxReductionDb = normalizeMax(input.maxReductionDb ?? 30);
  return {
    maxReductionDb,
    enabled: input.enabled ?? true,
    lastAtMs: input.lastAtMs ?? null,
    reductionDb: clamp(Number.isFinite(input.reductionDb) ? input.reductionDb! : 0, maxReductionDb),
    ballisticDb: clamp(Number.isFinite(input.ballisticDb) ? input.ballisticDb! : 0, maxReductionDb),
  };
}

export function gainReductionMeterTransition(context: GainReductionMeterContext, event: GainReductionMeterEvent): GainReductionMeterResult {
  switch (event.type) {
    case "SET_ENABLED": return { context: { ...context, enabled: event.value }, effects: [] };
    case "RESET": return { context: createGainReductionMeterContext({ maxReductionDb: context.maxReductionDb, enabled: context.enabled }), effects: [] };
    case "PUSH_FRAME": {
      if (!context.enabled) return { context, effects: [] };
      const frame = event.frame;
      if (!Number.isFinite(frame.atMs) || !Number.isFinite(frame.reductionDb) || !Number.isFinite(frame.durationMs)
        || frame.reductionDb < 0 || frame.durationMs <= 0
        || (context.lastAtMs !== null && frame.atMs < context.lastAtMs)) return { context, effects: [] };
      const elapsedMs = context.lastAtMs === null ? frame.durationMs : frame.atMs - context.lastAtMs;
      const reductionDb = clamp(frame.reductionDb, context.maxReductionDb);
      const timeMs = reductionDb >= context.ballisticDb ? GAIN_REDUCTION_ATTACK_MS : GAIN_REDUCTION_RELEASE_MS;
      const alpha = 1 - Math.exp(-Math.max(elapsedMs, 0) / timeMs);
      const ballisticDb = clamp(context.ballisticDb + (reductionDb - context.ballisticDb) * alpha, context.maxReductionDb);
      return { context: { ...context, lastAtMs: frame.atMs, reductionDb, ballisticDb }, effects: [] };
    }
  }
}

export function gainReductionMeterVisualState(context: GainReductionMeterContext): GainReductionMeterVisualState {
  const max = Math.max(context.maxReductionDb, Number.EPSILON);
  return {
    valueNorm: clamp(context.reductionDb / max, 1),
    rawValue: context.reductionDb,
    bipolarCenter: null,
    hover: false,
    focus: false,
    drag: "none",
    automation: "none",
    enabled: context.enabled,
    ballisticValue: clamp(context.ballisticDb / max, 1),
    peakHold: null,
    clip: false,
    reductionDb: context.reductionDb,
  };
}
