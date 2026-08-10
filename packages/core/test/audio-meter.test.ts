import { describe, expect, test } from "bun:test";
import {
  amplitudeToDb,
  audioMeterTransition,
  audioMeterVisualState,
  createAudioMeterContext,
  dbToAmplitude,
  normalizeMeterDb,
} from "../src/audio/meter";

const frame = (atMs: number, peak: number, durationMs: number, meanSquare = peak * peak) => ({
  atMs, peak, durationMs, meanSquare,
});

describe("audio meter math", () => {
  test("dB mappings have golden values", () => {
    expect(amplitudeToDb(1)).toBe(0);
    expect(amplitudeToDb(0.5)).toBeCloseTo(-6.020599913, 9);
    expect(dbToAmplitude(-6.020599913)).toBeCloseTo(0.5, 9);
    expect(normalizeMeterDb(-30, -60, 0)).toBe(0.5);
    expect(normalizeMeterDb(Number.NEGATIVE_INFINITY, -60, 0)).toBe(0);
  });

  test("VU reaches one time-constant response after 300ms", () => {
    const context = createAudioMeterContext({ mode: "vu", ballisticDb: Number.NEGATIVE_INFINITY });
    const result = audioMeterTransition(context, { type: "PUSH_FRAME", frame: frame(300, 1, 300) });
    expect(result.context.ballisticDb).toBeCloseTo(-3.984, 3);
  });

  test("PPM uses 10ms attack and 1500ms release", () => {
    let context = createAudioMeterContext({ mode: "ppm", ballisticDb: Number.NEGATIVE_INFINITY });
    context = audioMeterTransition(context, { type: "PUSH_FRAME", frame: frame(10, 1, 10) }).context;
    expect(context.ballisticDb).toBeCloseTo(-3.984, 3);
    const released = audioMeterTransition(context, { type: "PUSH_FRAME", frame: frame(1510, 0, 1500, 0) }).context;
    expect(released.ballisticDb).toBeLessThan(context.ballisticDb);
    expect(released.ballisticDb).toBeGreaterThan(-20);
  });

  test("sample peak holds, decays, and latches clip", () => {
    let context = createAudioMeterContext({ mode: "sample-peak" });
    context = audioMeterTransition(context, { type: "PUSH_FRAME", frame: frame(0, 1.1, 16) }).context;
    expect(context.clip).toBe(true);
    expect(context.peakHoldDb).toBeGreaterThan(0);
    context = audioMeterTransition(context, { type: "PUSH_FRAME", frame: frame(1500, 0.1, 16) }).context;
    const held = context.peakHoldDb;
    context = audioMeterTransition(context, { type: "PUSH_FRAME", frame: frame(2000, 0.1, 16) }).context;
    expect(context.peakHoldDb).toBeCloseTo(held! - 10, 8);
    expect(audioMeterTransition(context, { type: "RESET_CLIP" }).context.clip).toBe(false);
  });

  test("RMS uses a duration-weighted 300ms window", () => {
    let context = createAudioMeterContext({ mode: "rms" });
    context = audioMeterTransition(context, { type: "PUSH_FRAME", frame: frame(100, 1, 100, 1) }).context;
    context = audioMeterTransition(context, { type: "PUSH_FRAME", frame: frame(300, 0, 200, 0) }).context;
    expect(context.ballisticDb).toBeCloseTo(amplitudeToDb(Math.sqrt(1 / 3)), 10);
    context = audioMeterTransition(context, { type: "PUSH_FRAME", frame: frame(500, 0, 200, 0) }).context;
    expect(context.ballisticDb).toBe(-60);
  });

  test("VisualState is serializable and renderer complete", () => {
    const context = audioMeterTransition(createAudioMeterContext(), { type: "PUSH_FRAME", frame: frame(16, 0.5, 16) }).context;
    const visual = audioMeterVisualState(context);
    expect(JSON.parse(JSON.stringify(visual))).toEqual(visual);
    expect(visual.rawValue).toBeCloseTo(-6.020599913, 9);
    expect(visual.peakHold).not.toBeNull();
  });

  test("invalid and stale feed frames are inert", () => {
    const context = audioMeterTransition(createAudioMeterContext(), { type: "PUSH_FRAME", frame: frame(100, 0.5, 16) }).context;
    expect(audioMeterTransition(context, { type: "PUSH_FRAME", frame: frame(99, 1, 16) }).context).toEqual(context);
    expect(audioMeterTransition(context, { type: "PUSH_FRAME", frame: frame(120, Number.NaN, 16) }).context).toEqual(context);
    expect(audioMeterTransition(context, { type: "PUSH_FRAME", frame: frame(120, 1, 0) }).context).toEqual(context);
  });
});
