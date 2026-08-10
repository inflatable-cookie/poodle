import { describe, expect, test } from "bun:test";
import {
  createGainReductionMeterContext,
  gainReductionMeterTransition,
  gainReductionMeterVisualState,
} from "../src/audio/gain-reduction-meter";

const frame = (atMs: number, reductionDb: number, durationMs: number) => ({ atMs, reductionDb, durationMs });

describe("gain reduction meter", () => {
  test("uses 10ms attack and 300ms release golden values", () => {
    let context = createGainReductionMeterContext();
    context = gainReductionMeterTransition(context, { type: "PUSH_FRAME", frame: frame(10, 10, 10) }).context;
    expect(context.ballisticDb).toBeCloseTo(6.321205588, 9);
    context = gainReductionMeterTransition(context, { type: "PUSH_FRAME", frame: frame(310, 0, 300) }).context;
    expect(context.ballisticDb).toBeCloseTo(2.32544158, 8);
  });

  test("clamps the positive magnitude and emits an inverted-scale VisualState", () => {
    const context = gainReductionMeterTransition(createGainReductionMeterContext({ maxReductionDb: 20 }), {
      type: "PUSH_FRAME", frame: frame(100, 40, 100),
    }).context;
    expect(context.reductionDb).toBe(20);
    const visual = gainReductionMeterVisualState(context);
    expect(visual).toMatchObject({ valueNorm: 1, reductionDb: 20, peakHold: null, clip: false });
    expect(JSON.parse(JSON.stringify(visual))).toEqual(visual);
  });

  test("invalid, stale, disabled, and reset paths are deterministic", () => {
    let context = gainReductionMeterTransition(createGainReductionMeterContext(), { type: "PUSH_FRAME", frame: frame(100, 5, 16) }).context;
    expect(gainReductionMeterTransition(context, { type: "PUSH_FRAME", frame: frame(99, 8, 16) }).context).toEqual(context);
    expect(gainReductionMeterTransition(context, { type: "PUSH_FRAME", frame: frame(120, -1, 16) }).context).toEqual(context);
    context = gainReductionMeterTransition(context, { type: "SET_ENABLED", value: false }).context;
    expect(gainReductionMeterTransition(context, { type: "PUSH_FRAME", frame: frame(120, 8, 16) }).context).toEqual(context);
    expect(gainReductionMeterTransition(context, { type: "RESET" }).context).toMatchObject({ maxReductionDb: 30, enabled: false, reductionDb: 0 });
  });
});
