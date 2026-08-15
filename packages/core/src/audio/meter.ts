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

// Pure per-channel scalar laws. Standalone `audioMeterTransition` and the
// batched `MeterBus` both route through these; a constant or law change here
// moves standalone goldens and bus parity evidence together.

export function isMeterFrameValid(atMs: number, peak: number, meanSquare: number, durationMs: number, lastAtMs: number | null): boolean {
  return Number.isFinite(atMs) && Number.isFinite(peak)
    && Number.isFinite(meanSquare) && Number.isFinite(durationMs)
    && peak >= 0 && meanSquare >= 0 && durationMs > 0
    && !(lastAtMs !== null && atMs < lastAtMs);
}

export function meterElapsedMs(lastAtMs: number | null, atMs: number, durationMs: number): number {
  return lastAtMs === null ? Math.max(durationMs, 0) : Math.max(atMs - lastAtMs, 0);
}

export function meterInputDb(peak: number, minDb: number): number {
  return Math.max(amplitudeToDb(Math.max(peak, 0)), minDb);
}

export function meterSmoothedDb(currentDb: number, targetAmplitude: number, elapsedMs: number, timeMs: number): number {
  const current = dbToAmplitude(currentDb);
  const alpha = 1 - Math.exp(-Math.max(elapsedMs, 0) / timeMs);
  return amplitudeToDb(current + (targetAmplitude - current) * alpha);
}

export function meterVuStepDb(currentDb: number, meanSquare: number, elapsedMs: number): number {
  return meterSmoothedDb(currentDb, Math.sqrt(Math.max(meanSquare, 0)), elapsedMs, VU_INTEGRATION_MS);
}

export function meterPpmStepDb(currentDb: number, peak: number, inputDb: number, elapsedMs: number): number {
  return meterSmoothedDb(currentDb, peak, elapsedMs, inputDb >= currentDb ? PPM_ATTACK_MS : PPM_RELEASE_MS);
}

export function meterSamplePeakStepDb(currentDb: number, inputDb: number, elapsedMs: number, minDb: number): number {
  return inputDb >= currentDb
    ? inputDb
    : Math.max(currentDb - (elapsedMs / 1000) * PEAK_DECAY_DB_PER_SECOND, inputDb, minDb);
}

export function meterWeightedRmsDb(weightedMeanSquareSum: number, durationSum: number, fallbackDb: number): number {
  if (durationSum <= 0) return fallbackDb;
  return amplitudeToDb(Math.sqrt(weightedMeanSquareSum / durationSum));
}

export function meterClampDb(db: number, minDb: number, maxDb: number): number {
  return Math.max(Math.min(db, maxDb), minDb);
}

export function meterClipStep(clip: boolean, peak: number): boolean {
  return clip || peak >= 1;
}

export function meterPeakHoldDecayDb(holdDb: number, holdUntilMs: number, sinceMs: number, atMs: number, minDb: number): number {
  if (atMs <= holdUntilMs) return holdDb;
  const decayStart = Math.max(sinceMs, holdUntilMs);
  return Math.max(holdDb - ((atMs - decayStart) / 1000) * PEAK_DECAY_DB_PER_SECOND, minDb);
}

export function meterPeakHoldDbStep(holdDb: number | null, holdUntilMs: number | null, lastAtMs: number | null, inputDb: number, atMs: number, minDb: number): number {
  if (holdDb === null || inputDb >= holdDb) return inputDb;
  const holdUntil = holdUntilMs ?? atMs;
  return meterPeakHoldDecayDb(holdDb, holdUntil, lastAtMs ?? holdUntil, atMs, minDb);
}

export function meterPeakHoldUntilStep(holdDb: number | null, holdUntilMs: number | null, inputDb: number, atMs: number): number {
  if (holdDb === null || inputDb >= holdDb) return atMs + PEAK_HOLD_MS;
  return holdUntilMs ?? atMs;
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
  const weighted = window.reduce((sum, slice) => sum + slice.meanSquare * slice.durationMs, 0);
  return meterWeightedRmsDb(weighted, duration, fallbackDb);
}

export function audioMeterTransition(context: AudioMeterContext, event: AudioMeterEvent): AudioMeterResult {
  switch (event.type) {
    case "SET_ENABLED": return { context: { ...context, enabled: event.value }, effects: [] };
    case "RESET_CLIP": return { context: { ...context, clip: false }, effects: [] };
    case "RESET": return { context: createAudioMeterContext({ mode: context.mode, minDb: context.minDb, maxDb: context.maxDb, enabled: context.enabled }), effects: [] };
    case "PUSH_FRAME": {
      if (!context.enabled) return { context, effects: [] };
      const frame = event.frame;
      if (!isMeterFrameValid(frame.atMs, frame.peak, frame.meanSquare, frame.durationMs, context.lastAtMs)) {
        return { context, effects: [] };
      }
      const elapsedMs = meterElapsedMs(context.lastAtMs, frame.atMs, frame.durationMs);
      const peak = Math.max(frame.peak, 0);
      const inputDb = meterInputDb(frame.peak, context.minDb);
      const rmsWindow = pushRmsSlice(context.rmsWindow, frame);
      let ballisticDb: number;

      switch (context.mode) {
        case "vu":
          ballisticDb = meterVuStepDb(context.ballisticDb, frame.meanSquare, elapsedMs);
          break;
        case "ppm":
          ballisticDb = meterPpmStepDb(context.ballisticDb, peak, inputDb, elapsedMs);
          break;
        case "sample-peak":
          ballisticDb = meterSamplePeakStepDb(context.ballisticDb, inputDb, elapsedMs, context.minDb);
          break;
        case "rms":
          ballisticDb = Math.max(rmsDb(rmsWindow, context.minDb), context.minDb);
          break;
      }

      return { context: {
        ...context,
        lastAtMs: frame.atMs,
        inputDb,
        ballisticDb: meterClampDb(ballisticDb, context.minDb, context.maxDb),
        peakHoldDb: meterPeakHoldDbStep(context.peakHoldDb, context.peakHoldUntilMs, context.lastAtMs, inputDb, frame.atMs, context.minDb),
        peakHoldUntilMs: meterPeakHoldUntilStep(context.peakHoldDb, context.peakHoldUntilMs, inputDb, frame.atMs),
        clip: meterClipStep(context.clip, peak),
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
