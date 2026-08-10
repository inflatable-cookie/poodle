import type { AudioMeterVisualState } from "./types";

export type AudioMeterMode = "vu" | "ppm" | "sample-peak" | "rms";

export interface MeterFeedFrame {
  atMs: number;
  peak: number;
  meanSquare: number;
  durationMs: number;
}

export interface RmsSlice {
  meanSquare: number;
  durationMs: number;
}

export interface AudioMeterContext {
  mode: AudioMeterMode;
  minDb: number;
  maxDb: number;
  enabled: boolean;
  lastAtMs: number | null;
  inputDb: number;
  ballisticDb: number;
  peakHoldDb: number | null;
  peakHoldUntilMs: number | null;
  clip: boolean;
  rmsWindow: RmsSlice[];
}

export type AudioMeterEvent =
  | { type: "PUSH_FRAME"; frame: MeterFeedFrame }
  | { type: "RESET_CLIP" }
  | { type: "RESET" }
  | { type: "SET_ENABLED"; value: boolean };

export interface AudioMeterResult {
  context: AudioMeterContext;
  effects: [];
}

export const VU_INTEGRATION_MS = 300;
export const PPM_ATTACK_MS = 10;
export const PPM_RELEASE_MS = 1500;
export const RMS_WINDOW_MS = 300;
export const PEAK_HOLD_MS = 1500;
export const PEAK_DECAY_DB_PER_SECOND = 20;

export function amplitudeToDb(amplitude: number): number {
  return amplitude > 0 ? 20 * Math.log10(amplitude) : Number.NEGATIVE_INFINITY;
}

export function dbToAmplitude(db: number): number {
  return Number.isFinite(db) ? Math.pow(10, db / 20) : 0;
}

export function normalizeMeterDb(db: number, minDb: number, maxDb: number): number {
  if (maxDb <= minDb || !Number.isFinite(db)) return 0;
  return Math.min(Math.max((db - minDb) / (maxDb - minDb), 0), 1);
}

export function createAudioMeterContext(input: Partial<AudioMeterContext> = {}): AudioMeterContext {
  const minDb = input.minDb ?? -60;
  return {
    mode: "sample-peak",
    minDb,
    maxDb: 0,
    enabled: true,
    lastAtMs: null,
    inputDb: minDb,
    ballisticDb: minDb,
    peakHoldDb: null,
    peakHoldUntilMs: null,
    clip: false,
    rmsWindow: [],
    ...input,
  };
}

function smoothAmplitude(currentDb: number, targetAmplitude: number, elapsedMs: number, timeMs: number): number {
  const current = dbToAmplitude(currentDb);
  const alpha = 1 - Math.exp(-Math.max(elapsedMs, 0) / timeMs);
  return amplitudeToDb(current + (targetAmplitude - current) * alpha);
}

function pushRmsSlice(window: RmsSlice[], frame: MeterFeedFrame): RmsSlice[] {
  const next = [...window, {
    meanSquare: Math.max(frame.meanSquare, 0),
    durationMs: Math.max(frame.durationMs, 0),
  }];
  let excess = next.reduce((sum, slice) => sum + slice.durationMs, 0) - RMS_WINDOW_MS;
  while (excess > 0 && next.length > 0) {
    const first = next[0]!;
    if (first.durationMs <= excess) {
      excess -= first.durationMs;
      next.shift();
    } else {
      next[0] = { ...first, durationMs: first.durationMs - excess };
      excess = 0;
    }
  }
  return next;
}

function rmsDb(window: RmsSlice[], fallbackDb: number): number {
  const duration = window.reduce((sum, slice) => sum + slice.durationMs, 0);
  if (duration <= 0) return fallbackDb;
  const meanSquare = window.reduce((sum, slice) => sum + slice.meanSquare * slice.durationMs, 0) / duration;
  return amplitudeToDb(Math.sqrt(meanSquare));
}

function updatePeakHold(context: AudioMeterContext, inputDb: number, atMs: number): Pick<AudioMeterContext, "peakHoldDb" | "peakHoldUntilMs"> {
  if (context.peakHoldDb === null || inputDb >= context.peakHoldDb) {
    return { peakHoldDb: inputDb, peakHoldUntilMs: atMs + PEAK_HOLD_MS };
  }
  const holdUntil = context.peakHoldUntilMs ?? atMs;
  if (atMs <= holdUntil) return { peakHoldDb: context.peakHoldDb, peakHoldUntilMs: holdUntil };
  const decayStart = Math.max(context.lastAtMs ?? holdUntil, holdUntil);
  const decayed = context.peakHoldDb - ((atMs - decayStart) / 1000) * PEAK_DECAY_DB_PER_SECOND;
  return { peakHoldDb: Math.max(decayed, context.minDb), peakHoldUntilMs: holdUntil };
}

export function audioMeterTransition(context: AudioMeterContext, event: AudioMeterEvent): AudioMeterResult {
  switch (event.type) {
    case "SET_ENABLED": return { context: { ...context, enabled: event.value }, effects: [] };
    case "RESET_CLIP": return { context: { ...context, clip: false }, effects: [] };
    case "RESET": return { context: createAudioMeterContext({ mode: context.mode, minDb: context.minDb, maxDb: context.maxDb, enabled: context.enabled }), effects: [] };
    case "PUSH_FRAME": {
      if (!context.enabled) return { context, effects: [] };
      const frame = event.frame;
      if (
        !Number.isFinite(frame.atMs) || !Number.isFinite(frame.peak)
        || !Number.isFinite(frame.meanSquare) || !Number.isFinite(frame.durationMs)
        || frame.peak < 0 || frame.meanSquare < 0 || frame.durationMs <= 0
        || (context.lastAtMs !== null && frame.atMs < context.lastAtMs)
      ) return { context, effects: [] };
      const elapsedMs = context.lastAtMs === null
        ? Math.max(frame.durationMs, 0)
        : Math.max(frame.atMs - context.lastAtMs, 0);
      const peak = Math.max(frame.peak, 0);
      const inputDb = Math.max(amplitudeToDb(peak), context.minDb);
      const rmsWindow = pushRmsSlice(context.rmsWindow, frame);
      let ballisticDb: number;

      switch (context.mode) {
        case "vu":
          ballisticDb = smoothAmplitude(context.ballisticDb, Math.sqrt(Math.max(frame.meanSquare, 0)), elapsedMs, VU_INTEGRATION_MS);
          break;
        case "ppm": {
          const time = inputDb >= context.ballisticDb ? PPM_ATTACK_MS : PPM_RELEASE_MS;
          ballisticDb = smoothAmplitude(context.ballisticDb, peak, elapsedMs, time);
          break;
        }
        case "sample-peak":
          ballisticDb = inputDb >= context.ballisticDb
            ? inputDb
            : Math.max(context.ballisticDb - (elapsedMs / 1000) * PEAK_DECAY_DB_PER_SECOND, inputDb, context.minDb);
          break;
        case "rms":
          ballisticDb = Math.max(rmsDb(rmsWindow, context.minDb), context.minDb);
          break;
      }

      const hold = updatePeakHold(context, inputDb, frame.atMs);
      return { context: {
        ...context,
        lastAtMs: frame.atMs,
        inputDb,
        ballisticDb: Math.max(Math.min(ballisticDb, context.maxDb), context.minDb),
        ...hold,
        clip: context.clip || peak >= 1,
        rmsWindow,
      }, effects: [] };
    }
  }
}

export function audioMeterVisualState(context: AudioMeterContext): AudioMeterVisualState {
  return {
    valueNorm: normalizeMeterDb(context.inputDb, context.minDb, context.maxDb),
    rawValue: context.inputDb,
    bipolarCenter: null,
    hover: false,
    focus: false,
    drag: "none",
    automation: "none",
    enabled: context.enabled,
    ballisticValue: normalizeMeterDb(context.ballisticDb, context.minDb, context.maxDb),
    peakHold: context.peakHoldDb === null ? null : normalizeMeterDb(context.peakHoldDb, context.minDb, context.maxDb),
    clip: context.clip,
  };
}
