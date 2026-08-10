import { describe, expect, test } from "bun:test";
import {
  denormalizeAudioValue,
  normalizeAudioValue,
  snapAudioValue,
} from "../src/audio/laws";

describe("audio value laws", () => {
  test("linear law round trips golden values", () => {
    expect(normalizeAudioValue(-30, -60, 0, { type: "linear" })).toBe(0.5);
    expect(denormalizeAudioValue(0.25, -60, 0, { type: "linear" })).toBe(-45);
  });

  test("logarithmic frequency mapping uses geometric midpoint", () => {
    expect(normalizeAudioValue(1000, 10, 100_000, { type: "logarithmic" })).toBeCloseTo(0.5, 12);
    expect(denormalizeAudioValue(0.5, 20, 20_000, { type: "logarithmic" })).toBeCloseTo(632.455532, 6);
  });

  test("exponential law is invertible", () => {
    const law = { type: "exponential", exponent: 2 } as const;
    expect(denormalizeAudioValue(0.5, 0, 100, law)).toBe(25);
    expect(normalizeAudioValue(25, 0, 100, law)).toBe(0.5);
  });

  test("stepped law snaps in plain-value space", () => {
    const law = { type: "stepped", step: 0.1 } as const;
    expect(snapAudioValue(0.30000000000000004, 0, 0.1)).toBe(0.3);
    expect(denormalizeAudioValue(0.34, 0, 1, law)).toBe(0.3);
    expect(normalizeAudioValue(0.34, 0, 1, law)).toBe(0.3);
  });

  test("stepped law preserves scientific-notation precision", () => {
    expect(snapAudioValue(0.00000034, 0, 1e-7)).toBe(0.0000003);
    expect(snapAudioValue(1.23456, 0, 1.25e-3)).toBe(1.235);
  });

  test("bipolar center occupies normalized midpoint", () => {
    const law = { type: "bipolar-center", center: 0 } as const;
    expect(normalizeAudioValue(0, -24, 12, law)).toBe(0.5);
    expect(denormalizeAudioValue(0.25, -24, 12, law)).toBe(-12);
    expect(denormalizeAudioValue(0.75, -24, 12, law)).toBe(6);
  });

  test("invalid laws fail explicitly", () => {
    expect(() => normalizeAudioValue(0, 0, 10, { type: "logarithmic" })).toThrow(RangeError);
    expect(() => normalizeAudioValue(1, 0, 10, { type: "exponential", exponent: 0 })).toThrow(RangeError);
    expect(() => normalizeAudioValue(1, 0, 10, { type: "bipolar-center", center: 10 })).toThrow(RangeError);
  });
});
