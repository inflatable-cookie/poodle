import { describe, expect, test } from "bun:test";
import {
  PEAK_DECAY_DB_PER_SECOND,
  PEAK_HOLD_MS,
  PPM_ATTACK_MS,
  PPM_RELEASE_MS,
  RMS_WINDOW_MS,
  VU_INTEGRATION_MS,
  audioMeterTransition,
  createAudioMeterContext,
  createManualMeterFrameScheduler,
  createMeterBus,
  meterPeakHoldDecayDb,
  meterPpmStepDb,
  meterSamplePeakStepDb,
  meterVuStepDb,
  normalizeMeterDb,
  type AudioMeterContext,
  type AudioMeterMode,
  type MeterBus,
  type MeterBusChannelId,
} from "../src/audio";

// Bus/standalone comparisons use one documented tolerance. Both paths run the
// same shared scalar laws on the same Float64 state, so the tolerance is a
// strict equality guard, not a numerical fudge factor.
const PARITY_TOLERANCE = 1e-12;

const MODES: AudioMeterMode[] = ["vu", "ppm", "sample-peak", "rms"];

interface TraceFrame {
  atMs: number;
  peak: number;
  meanSquare: number;
  durationMs: number;
}

function quantize(frame: TraceFrame): TraceFrame {
  return {
    atMs: frame.atMs,
    peak: Math.fround(frame.peak),
    meanSquare: Math.fround(frame.meanSquare),
    durationMs: frame.durationMs,
  };
}

function busWithChannel(mode: AudioMeterMode, options: { minDb?: number; maxDb?: number } = {}) {
  const scheduler = createManualMeterFrameScheduler();
  const bus = createMeterBus({ scheduler });
  const channel = bus.register(`${mode}-channel`, { mode, ...options });
  return { bus, channel, scheduler };
}

function pushOne(bus: MeterBus, slot: number, frame: TraceFrame): number {
  const data = new Float32Array([slot, frame.peak, frame.meanSquare]);
  return bus.pushFrames(data, frame.atMs, frame.durationMs);
}

function expectSlotMatchesContext(bus: MeterBus, slot: number, context: AudioMeterContext) {
  const view = bus.view;
  expect(Math.abs(view.inputDb[slot]! - context.inputDb)).toBeLessThanOrEqual(PARITY_TOLERANCE);
  expect(Math.abs(view.ballisticDb[slot]! - context.ballisticDb)).toBeLessThanOrEqual(PARITY_TOLERANCE);
  if (context.peakHoldDb === null) expect(Number.isNaN(view.peakHoldDb[slot]!)).toBe(true);
  else expect(Math.abs(view.peakHoldDb[slot]! - context.peakHoldDb)).toBeLessThanOrEqual(PARITY_TOLERANCE);
  expect(view.clip[slot] === 1).toBe(context.clip);
  const busNorm = normalizeMeterDb(view.ballisticDb[slot]!, view.minDb[slot]!, view.maxDb[slot]!);
  const standaloneNorm = normalizeMeterDb(context.ballisticDb, context.minDb, context.maxDb);
  expect(Math.abs(busNorm - standaloneNorm)).toBeLessThanOrEqual(PARITY_TOLERANCE);
}

describe("meter bus registration", () => {
  test("registers channels into stable numeric slots", () => {
    const bus = createMeterBus({ scheduler: createManualMeterFrameScheduler() });
    const first = bus.register("track:1", { mode: "vu" });
    const second = bus.register(2, { mode: "rms", minDb: -48, maxDb: 6 });
    expect(first.slot).toBe(0);
    expect(second.slot).toBe(1);
    expect(bus.view.active[0]).toBe(1);
    expect(bus.view.minDb[1]).toBe(-48);
    expect(bus.view.maxDb[1]).toBe(6);
    expect(bus.view.ballisticDb[1]).toBe(-48);
  });

  test("rejects duplicate ids, invalid bounds, and unknown modes", () => {
    const bus = createMeterBus({ scheduler: createManualMeterFrameScheduler() });
    bus.register("a", { mode: "vu" });
    expect(() => bus.register("a", { mode: "vu" })).toThrow(/already registered/);
    expect(() => bus.register("b", { mode: "vu", minDb: 0, maxDb: 0 })).toThrow(RangeError);
    expect(() => bus.register("b", { mode: "vu", minDb: Number.NaN })).toThrow(RangeError);
    expect(() => bus.register("b", { mode: "nope" as AudioMeterMode })).toThrow(RangeError);
  });

  test("a stale handle cannot unregister the replacement that reuses its slot", () => {
    const bus = createMeterBus({ scheduler: createManualMeterFrameScheduler() });
    const first = bus.register("same", { mode: "sample-peak" });
    bus.unregister(first);
    const second = bus.register("same", { mode: "sample-peak" });
    // Slot reuse means `second` has the same id and slot as `first`.
    expect(second.slot).toBe(first.slot);
    expect(() => bus.unregister(first)).toThrow(/not registered/);
    expect(bus.view.active[second.slot]).toBe(1);
    expect(pushOne(bus, second.slot, { atMs: 10, peak: 0.5, meanSquare: 0.25, durationMs: 10 })).toBe(1);
    bus.unregister(second);
    expect(bus.view.active[second.slot]).toBe(0);
  });

  test("a mutated slot cannot corrupt bus internals", () => {
    const bus = createMeterBus({ initialCapacity: 32, scheduler: createManualMeterFrameScheduler() });
    const handle = bus.register("a", { mode: "vu" });
    // The handle is frozen, so the tamper attempt itself fails in strict mode.
    expect(() => { (handle as { slot: number }).slot = 999; }).toThrow(TypeError);
    expect(handle.slot).toBe(0);
    // And unregister acts on the minted slot regardless of the object's fields.
    bus.unregister(handle);
    expect(bus.view.active[0]).toBe(0);
    const next = bus.register("b", { mode: "vu" });
    expect(next.slot).toBe(0);
    expect(bus.view.capacity).toBe(32);
  });

  test("a mutated id cannot strand a registration", () => {
    const bus = createMeterBus({ scheduler: createManualMeterFrameScheduler() });
    const handle = bus.register("a", { mode: "vu" });
    expect(() => { (handle as { id: MeterBusChannelId }).id = "hijacked"; }).toThrow(TypeError);
    expect(handle.id).toBe("a");
    bus.unregister(handle);
    // The original id is released, not left stranded in the id index.
    expect(() => bus.resetClip("a")).toThrow(/not registered/);
    expect(() => bus.register("a", { mode: "vu" })).not.toThrow();
  });

  test("a look-alike handle carrying live values is rejected", () => {
    const bus = createMeterBus({ scheduler: createManualMeterFrameScheduler() });
    const handle = bus.register("a", { mode: "vu" });
    // Same shape and same values, but not the object the bus minted.
    expect(() => bus.unregister({ id: handle.id, slot: handle.slot })).toThrow(/not registered/);
    expect(bus.view.active[handle.slot]).toBe(1);
  });

  test("a handle minted by another bus is rejected", () => {
    const bus = createMeterBus({ scheduler: createManualMeterFrameScheduler() });
    const other = createMeterBus({ scheduler: createManualMeterFrameScheduler() });
    bus.register("a", { mode: "vu" });
    const foreign = other.register("a", { mode: "vu" });
    expect(() => bus.unregister(foreign)).toThrow(/not registered/);
    expect(bus.view.active[0]).toBe(1);
    expect(() => bus.unregister({ id: "a", slot: 0 })).toThrow(/not registered/);
    expect(bus.view.active[0]).toBe(1);
  });

  test("unregister frees the slot and rejects use-after-unregister", () => {
    const bus = createMeterBus({ scheduler: createManualMeterFrameScheduler() });
    const channel = bus.register("a", { mode: "sample-peak" });
    bus.unregister(channel);
    expect(bus.view.active[channel.slot]).toBe(0);
    expect(() => bus.unregister(channel)).toThrow(/not registered/);
    expect(pushOne(bus, channel.slot, { atMs: 10, peak: 1, meanSquare: 1, durationMs: 10 })).toBe(0);
    const reused = bus.register("b", { mode: "sample-peak" });
    expect(reused.slot).toBe(channel.slot);
  });

  test("capacity growth keeps state and bumps the stable view descriptor", () => {
    const bus = createMeterBus({ initialCapacity: 2, scheduler: createManualMeterFrameScheduler() });
    const view = bus.view;
    const first = bus.register("a", { mode: "sample-peak" });
    pushOne(bus, first.slot, { atMs: 10, peak: 0.5, meanSquare: 0.25, durationMs: 10 });
    const before = view.ballisticDb[first.slot]!;
    bus.register("b", { mode: "vu" });
    expect(view.generation).toBe(0);
    bus.register("c", { mode: "vu" });
    expect(bus.view).toBe(view);
    expect(view.generation).toBe(1);
    expect(view.capacity).toBe(4);
    expect(view.ballisticDb[first.slot]).toBe(before);
    expect(Number.isNaN(view.peakHoldDb[3]!)).toBe(true);
  });
});

describe("meter bus feed validation", () => {
  test("invalid data is isolated to its triple", () => {
    const bus = createMeterBus({ scheduler: createManualMeterFrameScheduler() });
    const a = bus.register("a", { mode: "sample-peak" });
    const b = bus.register("b", { mode: "sample-peak" });
    const data = new Float32Array([
      a.slot, Number.NaN, 0,
      b.slot, 0.5, 0.25,
      99, 0.5, 0.25,
    ]);
    expect(bus.pushFrames(data, 10, 10)).toBe(1);
    expect(bus.view.ballisticDb[a.slot]).toBe(-60);
    expect(bus.view.ballisticDb[b.slot]).toBeCloseTo(-6.020599913, 6);
  });

  test("repeated slots in one batch apply only the first triple", () => {
    const bus = createMeterBus({ scheduler: createManualMeterFrameScheduler() });
    const a = bus.register("a", { mode: "sample-peak" });
    const data = new Float32Array([a.slot, 0.5, 0.25, a.slot, 1, 1]);
    expect(bus.pushFrames(data, 10, 10)).toBe(1);
    expect(bus.view.clip[a.slot]).toBe(0);
  });

  test("stale timestamps, negative values, and malformed batches are rejected", () => {
    const bus = createMeterBus({ scheduler: createManualMeterFrameScheduler() });
    const a = bus.register("a", { mode: "sample-peak" });
    expect(pushOne(bus, a.slot, { atMs: 100, peak: 0.5, meanSquare: 0.25, durationMs: 10 })).toBe(1);
    expect(pushOne(bus, a.slot, { atMs: 99, peak: 1, meanSquare: 1, durationMs: 10 })).toBe(0);
    expect(pushOne(bus, a.slot, { atMs: 110, peak: -1, meanSquare: 1, durationMs: 10 })).toBe(0);
    expect(bus.pushFrames(new Float32Array([a.slot, 0.5]), 120, 10)).toBe(0);
    expect(bus.pushFrames(new Float32Array([a.slot, 0.5, 0.25]), Number.NaN, 10)).toBe(0);
    expect(bus.pushFrames(new Float32Array([a.slot, 0.5, 0.25]), 120, 0)).toBe(0);
  });

  test("disabled channels are inert without consuming acceptance", () => {
    const bus = createMeterBus({ scheduler: createManualMeterFrameScheduler() });
    const a = bus.register("a", { mode: "sample-peak", enabled: false });
    expect(pushOne(bus, a.slot, { atMs: 10, peak: 1, meanSquare: 1, durationMs: 10 })).toBe(0);
    expect(bus.view.ballisticDb[a.slot]).toBe(-60);
    bus.setEnabled("a", true);
    expect(pushOne(bus, a.slot, { atMs: 20, peak: 1, meanSquare: 1, durationMs: 10 })).toBe(1);
  });

  test("frames shorter than the declared minimum duration are rejected", () => {
    const bus = createMeterBus({ scheduler: createManualMeterFrameScheduler(), minFrameDurationMs: 10, rmsRingCapacity: 32 });
    const a = bus.register("a", { mode: "rms" });
    expect(pushOne(bus, a.slot, { atMs: 10, peak: 0.5, meanSquare: 0.25, durationMs: 5 })).toBe(0);
    expect(pushOne(bus, a.slot, { atMs: 10, peak: 0.5, meanSquare: 0.25, durationMs: 10 })).toBe(1);
  });

  test("ring limits that cannot cover the RMS window are rejected at construction", () => {
    expect(() => createMeterBus({ rmsRingCapacity: 8, minFrameDurationMs: 5 })).toThrow(RangeError);
    expect(() => createMeterBus({ rmsRingCapacity: 61, minFrameDurationMs: 5 })).not.toThrow();
  });
});

describe("meter bus golden ballistics", () => {
  // Expected values are computed longhand here, independent of the library
  // functions, so a planted change to a shared constant or scalar law fails
  // this suite and the standalone goldens together.
  test("VU reaches one time-constant response after 300ms", () => {
    const { bus, channel } = busWithChannel("vu");
    pushOne(bus, channel.slot, { atMs: 300, peak: 1, meanSquare: 1, durationMs: 300 });
    const start = Math.pow(10, -60 / 20);
    const alpha = 1 - Math.exp(-300 / VU_INTEGRATION_MS);
    const expected = 20 * Math.log10(start + (1 - start) * alpha);
    expect(bus.view.ballisticDb[channel.slot]).toBeCloseTo(expected, 12);
    expect(expected).toBeCloseTo(-3.979, 3);
  });

  test("PPM uses 10ms attack and 1500ms release", () => {
    const { bus, channel } = busWithChannel("ppm");
    pushOne(bus, channel.slot, { atMs: 10, peak: 1, meanSquare: 1, durationMs: 10 });
    const start = Math.pow(10, -60 / 20);
    const attacked = start + (1 - start) * (1 - Math.exp(-10 / PPM_ATTACK_MS));
    expect(bus.view.ballisticDb[channel.slot]).toBeCloseTo(20 * Math.log10(attacked), 12);
    pushOne(bus, channel.slot, { atMs: 1510, peak: 0, meanSquare: 0, durationMs: 1500 });
    const released = attacked + (0 - attacked) * (1 - Math.exp(-1500 / PPM_RELEASE_MS));
    expect(bus.view.ballisticDb[channel.slot]).toBeCloseTo(20 * Math.log10(released), 12);
  });

  test("sample peak attacks immediately and decays at 20 dB/s", () => {
    const { bus, channel } = busWithChannel("sample-peak");
    pushOne(bus, channel.slot, { atMs: 0, peak: 1, meanSquare: 1, durationMs: 16 });
    expect(bus.view.ballisticDb[channel.slot]).toBe(0);
    pushOne(bus, channel.slot, { atMs: 500, peak: 0.001953125, meanSquare: 0, durationMs: 16 });
    expect(bus.view.ballisticDb[channel.slot]).toBeCloseTo(0 - 0.5 * PEAK_DECAY_DB_PER_SECOND, 12);
  });

  test("RMS integrates a duration-weighted 300ms window and evicts partial slices", () => {
    const { bus, channel } = busWithChannel("rms");
    pushOne(bus, channel.slot, { atMs: 100, peak: 1, meanSquare: 1, durationMs: 100 });
    pushOne(bus, channel.slot, { atMs: 300, peak: 0, meanSquare: 0, durationMs: 200 });
    expect(bus.view.ballisticDb[channel.slot]).toBeCloseTo(20 * Math.log10(Math.sqrt(100 / 300)), 12);
    pushOne(bus, channel.slot, { atMs: 500, peak: 0, meanSquare: 0, durationMs: 200 });
    expect(bus.view.ballisticDb[channel.slot]).toBe(-60);
  });

  test("peak hold latches, holds 1500ms, then decays at 20 dB/s", () => {
    const { bus, channel } = busWithChannel("sample-peak");
    pushOne(bus, channel.slot, { atMs: 0, peak: 1.1, meanSquare: 1.21, durationMs: 16 });
    const holdDb = bus.view.peakHoldDb[channel.slot]!;
    expect(holdDb).toBeCloseTo(20 * Math.log10(Math.fround(1.1)), 6);
    expect(bus.view.clip[channel.slot]).toBe(1);
    pushOne(bus, channel.slot, { atMs: PEAK_HOLD_MS, peak: 0.1, meanSquare: 0.01, durationMs: 16 });
    expect(bus.view.peakHoldDb[channel.slot]).toBeCloseTo(holdDb, 6);
    pushOne(bus, channel.slot, { atMs: PEAK_HOLD_MS + 500, peak: 0.1, meanSquare: 0.01, durationMs: 16 });
    expect(bus.view.peakHoldDb[channel.slot]).toBeCloseTo(holdDb - 10, 6);
    bus.resetClip(channel.id);
    expect(bus.view.clip[channel.slot]).toBe(0);
  });
});

describe("meter bus and standalone parity", () => {
  const trace: TraceFrame[] = [
    { atMs: 66, peak: 0.9, meanSquare: 0.5, durationMs: 66 },
    { atMs: 133, peak: 1.2, meanSquare: 0.9, durationMs: 66 },
    { atMs: 200, peak: 0.4, meanSquare: 0.1, durationMs: 66 },
    { atMs: 266, peak: 0.4, meanSquare: 0.1, durationMs: 66 },
    { atMs: 333, peak: 0.05, meanSquare: 0.002, durationMs: 66 },
    { atMs: 400, peak: 0, meanSquare: 0, durationMs: 66 },
    { atMs: 1900, peak: 0, meanSquare: 0, durationMs: 66 },
    { atMs: 2400, peak: 0.7, meanSquare: 0.45, durationMs: 66 },
  ];

  for (const mode of MODES) {
    test(`${mode} matches audioMeterTransition for an identical Float32 push trace`, () => {
      const { bus, channel } = busWithChannel(mode);
      let context = createAudioMeterContext({ mode });
      for (const raw of trace) {
        const frame = quantize(raw);
        expect(pushOne(bus, channel.slot, frame)).toBe(1);
        context = audioMeterTransition(context, { type: "PUSH_FRAME", frame }).context;
        expectSlotMatchesContext(bus, channel.slot, context);
      }
    });
  }

  test("stale and invalid frames stay inert on both paths", () => {
    const { bus, channel } = busWithChannel("sample-peak");
    let context = createAudioMeterContext({ mode: "sample-peak" });
    const good = quantize({ atMs: 100, peak: 0.5, meanSquare: 0.25, durationMs: 66 });
    pushOne(bus, channel.slot, good);
    context = audioMeterTransition(context, { type: "PUSH_FRAME", frame: good }).context;
    for (const bad of [
      quantize({ atMs: 50, peak: 0.9, meanSquare: 0.8, durationMs: 66 }),
      quantize({ atMs: 150, peak: Number.NaN, meanSquare: 0, durationMs: 66 }),
      quantize({ atMs: 150, peak: 0.5, meanSquare: -1, durationMs: 66 }),
    ]) {
      expect(pushOne(bus, channel.slot, bad)).toBe(0);
      const next = audioMeterTransition(context, { type: "PUSH_FRAME", frame: bad }).context;
      expect(next).toEqual(context);
      expectSlotMatchesContext(bus, channel.slot, context);
    }
  });

  test("explicit idle time steps follow the shared scalar laws", () => {
    const scheduler = createManualMeterFrameScheduler();
    const bus = createMeterBus({ scheduler });
    const vu = bus.register("vu", { mode: "vu" });
    const ppm = bus.register("ppm", { mode: "ppm" });
    const peakChannel = bus.register("peak", { mode: "sample-peak" });
    const unsubscribe = bus.subscribe(() => {});

    const frame = quantize({ atMs: 100, peak: 1, meanSquare: 0.81, durationMs: 66 });
    bus.pushFrames(new Float32Array([
      vu.slot, frame.peak, frame.meanSquare,
      ppm.slot, frame.peak, frame.meanSquare,
      peakChannel.slot, frame.peak, frame.meanSquare,
    ]), frame.atMs, frame.durationMs);

    // Reference stepper: the same pure scalar laws applied at explicit
    // timestamps. The bus must agree exactly.
    let vuDb = bus.view.ballisticDb[vu.slot]!;
    let ppmDb = bus.view.ballisticDb[ppm.slot]!;
    let peakDb = bus.view.ballisticDb[peakChannel.slot]!;
    let holdDb = bus.view.peakHoldDb[peakChannel.slot]!;
    let last = 100;
    for (const timeMs of [116, 133, 150, 1700, 1750]) {
      scheduler.fire(timeMs);
      const elapsed = timeMs - last;
      vuDb = Math.min(Math.max(meterVuStepDb(vuDb, frame.meanSquare, elapsed), -60), 0);
      ppmDb = Math.min(Math.max(meterPpmStepDb(ppmDb, frame.peak, bus.view.inputDb[ppm.slot]!, elapsed), -60), 0);
      peakDb = Math.min(Math.max(meterSamplePeakStepDb(peakDb, bus.view.inputDb[peakChannel.slot]!, elapsed, -60), -60), 0);
      holdDb = meterPeakHoldDecayDb(holdDb, 100 + PEAK_HOLD_MS, last, timeMs, -60);
      last = timeMs;
      expect(bus.view.ballisticDb[vu.slot]).toBe(vuDb);
      expect(bus.view.ballisticDb[ppm.slot]).toBe(ppmDb);
      expect(bus.view.ballisticDb[peakChannel.slot]).toBe(peakDb);
      expect(bus.view.peakHoldDb[peakChannel.slot]).toBe(holdDb);
    }
    expect(bus.view.peakHoldDb[peakChannel.slot]!).toBeLessThan(0);
    unsubscribe();
    expect(scheduler.pendingCount()).toBe(0);
  });

  test("a batch stamped before the advance clock cannot rewind bus state", () => {
    // A delayed telemetry batch must not be applied after the frame loop has
    // already advanced past its timestamp: clamping the negative elapsed to
    // zero would make the batched trace diverge from the same push/time
    // sequence applied in order.
    const build = (delayed: boolean) => {
      const scheduler = createManualMeterFrameScheduler();
      const bus = createMeterBus({ scheduler });
      const channel = bus.register("vu", { mode: "vu" });
      const unsubscribe = bus.subscribe(() => {});
      pushOne(bus, channel.slot, quantize({ atMs: 100, peak: 0.9, meanSquare: 0.5, durationMs: 66 }));
      scheduler.fire(600);
      const staleAccepted = delayed
        ? pushOne(bus, channel.slot, quantize({ atMs: 500, peak: 0.2, meanSquare: 0.04, durationMs: 66 }))
        : 0;
      pushOne(bus, channel.slot, quantize({ atMs: 700, peak: 0.3, meanSquare: 0.09, durationMs: 66 }));
      const ballisticDb = bus.view.ballisticDb[channel.slot]!;
      unsubscribe();
      return { ballisticDb, staleAccepted };
    };

    const inOrder = build(false);
    const withDelayed = build(true);
    expect(withDelayed.staleAccepted).toBe(0);
    expect(withDelayed.ballisticDb).toBe(inOrder.ballisticDb);
  });

  test("a push after idle advancement resumes from the advanced state", () => {
    const scheduler = createManualMeterFrameScheduler();
    const bus = createMeterBus({ scheduler });
    const channel = bus.register("peak", { mode: "sample-peak" });
    const unsubscribe = bus.subscribe(() => {});
    pushOne(bus, channel.slot, quantize({ atMs: 100, peak: 1, meanSquare: 1, durationMs: 66 }));
    // Sample-peak holds at the persisting input level between data frames.
    scheduler.fire(600);
    expect(bus.view.ballisticDb[channel.slot]).toBe(0);
    // The next silent frame decays from the advanced clock, not the push clock.
    pushOne(bus, channel.slot, quantize({ atMs: 700, peak: 0, meanSquare: 0, durationMs: 66 }));
    expect(bus.view.ballisticDb[channel.slot]).toBeCloseTo(-2, 9);
    unsubscribe();
  });
});

describe("meter bus lifecycle and scheduling", () => {
  test("one loop serves every channel and stops with the last subscriber", () => {
    const scheduler = createManualMeterFrameScheduler();
    const bus = createMeterBus({ scheduler });
    bus.register("a", { mode: "vu" });
    bus.register("b", { mode: "ppm" });
    expect(scheduler.pendingCount()).toBe(0);
    const first = bus.subscribe(() => {});
    const second = bus.subscribe(() => {});
    expect(scheduler.pendingCount()).toBe(1);
    scheduler.fire(16);
    expect(scheduler.pendingCount()).toBe(1);
    first();
    second();
    expect(scheduler.pendingCount()).toBe(0);
  });

  test("destroy cancels the loop and rejects further use", () => {
    const scheduler = createManualMeterFrameScheduler();
    const bus = createMeterBus({ scheduler });
    const channel = bus.register("a", { mode: "vu" });
    bus.subscribe(() => {});
    bus.destroy();
    expect(scheduler.pendingCount()).toBe(0);
    expect(bus.view.active[channel.slot]).toBe(0);
    expect(() => pushOne(bus, channel.slot, { atMs: 10, peak: 1, meanSquare: 1, durationMs: 10 })).toThrow(/destroyed/);
    expect(() => bus.register("b", { mode: "vu" })).toThrow(/destroyed/);
    bus.destroy();
  });
});

describe("meter bus hot-path allocation", () => {
  test("warm pushes and idle advances do not grow the heap", () => {
    const scheduler = createManualMeterFrameScheduler();
    const bus = createMeterBus({ initialCapacity: 128, scheduler });
    const channels = [];
    for (let index = 0; index < 128; index += 1) {
      channels.push(bus.register(index, { mode: MODES[index % MODES.length]! }));
    }
    const unsubscribe = bus.subscribe(() => {});
    const data = new Float32Array(128 * 3);
    const fill = (atMs: number) => {
      for (let index = 0; index < 128; index += 1) {
        data[index * 3] = index;
        data[index * 3 + 1] = Math.fround(0.1 + (index % 10) / 10);
        data[index * 3 + 2] = Math.fround(0.05 + (index % 7) / 14);
      }
      return atMs;
    };
    let atMs = 0;
    const run = (iterations: number) => {
      for (let iteration = 0; iteration < iterations; iteration += 1) {
        atMs += 66;
        bus.pushFrames(data, fill(atMs), 66);
        scheduler.fire(atMs + 16);
        scheduler.fire(atMs + 33);
        scheduler.fire(atMs + 50);
      }
    };
    run(200); // warm-up
    Bun.gc(true);
    const before = process.memoryUsage().heapUsed;
    run(2000);
    Bun.gc(true);
    const after = process.memoryUsage().heapUsed;
    unsubscribe();
    // 2000 batches x (1 push + 3 advances) over 128 channels. Any per-frame or
    // per-channel allocation would grow the heap by megabytes; allow slack for
    // runtime noise only.
    expect(after - before).toBeLessThan(256 * 1024);
  });
});
