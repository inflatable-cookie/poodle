import {
  RMS_WINDOW_MS,
  isMeterFrameValid,
  meterClampDb,
  meterClipStep,
  meterInputDb,
  meterPeakHoldDbStep,
  meterPeakHoldDecayDb,
  meterPeakHoldUntilStep,
  meterPpmStepDb,
  meterSamplePeakStepDb,
  meterVuStepDb,
  meterWeightedRmsDb,
  type AudioMeterMode,
} from "./meter";

export type MeterBusChannelId = string | number;

export interface MeterBusChannel {
  readonly id: MeterBusChannelId;
  readonly slot: number;
}

export interface MeterBusRegistration {
  mode: AudioMeterMode;
  minDb?: number;
  maxDb?: number;
  enabled?: boolean;
}

export interface MeterFrameScheduler {
  request(callback: (timeMs: number) => void): number;
  cancel(handle: number): void;
}

export interface ManualMeterFrameScheduler extends MeterFrameScheduler {
  fire(timeMs: number): void;
  pendingCount(): number;
}

export interface MeterBusView {
  generation: number;
  capacity: number;
  active: Uint8Array;
  mode: Uint8Array;
  enabled: Uint8Array;
  clip: Uint8Array;
  minDb: Float64Array;
  maxDb: Float64Array;
  inputDb: Float64Array;
  ballisticDb: Float64Array;
  /** NaN while a slot has no peak hold yet. */
  peakHoldDb: Float64Array;
}

export interface MeterBusOptions {
  initialCapacity?: number;
  rmsRingCapacity?: number;
  minFrameDurationMs?: number;
  scheduler?: MeterFrameScheduler;
}

export interface MeterBus {
  register(id: MeterBusChannelId, input: MeterBusRegistration): MeterBusChannel;
  unregister(channel: MeterBusChannel): void;
  pushFrames(data: Float32Array, atMs: number, durationMs: number): number;
  resetClip(id: MeterBusChannelId): void;
  setEnabled(id: MeterBusChannelId, value: boolean): void;
  slotOf(id: MeterBusChannelId): number;
  subscribe(listener: (timeMs: number) => void): () => void;
  destroy(): void;
  readonly view: MeterBusView;
}

export const METER_BUS_MODES: readonly AudioMeterMode[] = ["vu", "ppm", "sample-peak", "rms"];

export function meterBusModeCode(mode: AudioMeterMode): number {
  const code = METER_BUS_MODES.indexOf(mode);
  if (code < 0) throw new RangeError(`MeterBus: unknown meter mode "${mode}"`);
  return code;
}

const MODE_VU = 0;
const MODE_PPM = 1;
const MODE_SAMPLE_PEAK = 2;
const MODE_RMS = 3;

export function createAnimationFrameMeterScheduler(): MeterFrameScheduler {
  return {
    request(callback) {
      return typeof requestAnimationFrame === "function" ? requestAnimationFrame(callback) : 0;
    },
    cancel(handle) {
      if (typeof cancelAnimationFrame === "function") cancelAnimationFrame(handle);
    },
  };
}

export function createManualMeterFrameScheduler(): ManualMeterFrameScheduler {
  let nextHandle = 1;
  const pending = new Map<number, (timeMs: number) => void>();
  return {
    request(callback) {
      const handle = nextHandle++;
      pending.set(handle, callback);
      return handle;
    },
    cancel(handle) {
      pending.delete(handle);
    },
    fire(timeMs) {
      const callbacks = [...pending.values()];
      pending.clear();
      for (const callback of callbacks) callback(timeMs);
    },
    pendingCount() {
      return pending.size;
    },
  };
}

class MeterBusImpl implements MeterBus {
  readonly view: MeterBusView;

  #capacity: number;
  #ringCapacity: number;
  #minFrameDurationMs: number;
  #scheduler: MeterFrameScheduler;
  #destroyed = false;

  /** Handles minted by this bus and not yet unregistered. */
  #liveChannels = new WeakSet<MeterBusChannel>();
  #ids: Array<MeterBusChannelId | null>;
  #slotById = new Map<MeterBusChannelId, number>();
  #freeSlots: number[] = [];
  #slotCount = 0;

  #lastFrameAtMs: Float64Array;
  #lastAdvanceMs: Float64Array;
  #lastPeak: Float64Array;
  #lastMeanSquare: Float64Array;
  #peakHoldUntilMs: Float64Array;
  #batchStamp: Int32Array;
  #batchCounter = 0;

  #rmsMeanSquare: Float64Array;
  #rmsDurationMs: Float64Array;
  #rmsHead: Int32Array;
  #rmsCount: Int32Array;

  #listeners: Array<(timeMs: number) => void> = [];
  #frameHandle: number | null = null;
  #tick: (timeMs: number) => void;

  constructor(options: MeterBusOptions = {}) {
    const initialCapacity = options.initialCapacity ?? 32;
    const ringCapacity = options.rmsRingCapacity ?? 64;
    const minFrameDurationMs = options.minFrameDurationMs ?? 5;
    if (!Number.isInteger(initialCapacity) || initialCapacity <= 0) {
      throw new RangeError("MeterBus: initialCapacity must be a positive integer");
    }
    if (!Number.isInteger(ringCapacity) || ringCapacity <= 0) {
      throw new RangeError("MeterBus: rmsRingCapacity must be a positive integer");
    }
    if (!Number.isFinite(minFrameDurationMs) || minFrameDurationMs <= 0) {
      throw new RangeError("MeterBus: minFrameDurationMs must be a positive number");
    }
    // The RMS ring must always be able to evict the whole window before it
    // fills: capacity * minDuration >= window + one more slice.
    if (ringCapacity * minFrameDurationMs < RMS_WINDOW_MS + minFrameDurationMs) {
      throw new RangeError(
        "MeterBus: rmsRingCapacity * minFrameDurationMs must cover the "
        + `${RMS_WINDOW_MS} ms RMS window plus one slice`,
      );
    }
    this.#capacity = initialCapacity;
    this.#ringCapacity = ringCapacity;
    this.#minFrameDurationMs = minFrameDurationMs;
    this.#scheduler = options.scheduler ?? createAnimationFrameMeterScheduler();

    this.#ids = new Array(initialCapacity).fill(null);
    this.#lastFrameAtMs = new Float64Array(initialCapacity).fill(Number.NaN);
    this.#lastAdvanceMs = new Float64Array(initialCapacity).fill(Number.NaN);
    this.#lastPeak = new Float64Array(initialCapacity);
    this.#lastMeanSquare = new Float64Array(initialCapacity);
    this.#peakHoldUntilMs = new Float64Array(initialCapacity);
    this.#batchStamp = new Int32Array(initialCapacity);
    this.#rmsMeanSquare = new Float64Array(initialCapacity * ringCapacity);
    this.#rmsDurationMs = new Float64Array(initialCapacity * ringCapacity);
    this.#rmsHead = new Int32Array(initialCapacity);
    this.#rmsCount = new Int32Array(initialCapacity);

    this.view = {
      generation: 0,
      capacity: initialCapacity,
      active: new Uint8Array(initialCapacity),
      mode: new Uint8Array(initialCapacity),
      enabled: new Uint8Array(initialCapacity),
      clip: new Uint8Array(initialCapacity),
      minDb: new Float64Array(initialCapacity),
      maxDb: new Float64Array(initialCapacity),
      inputDb: new Float64Array(initialCapacity),
      ballisticDb: new Float64Array(initialCapacity),
      peakHoldDb: new Float64Array(initialCapacity).fill(Number.NaN),
    };

    this.#tick = (timeMs: number) => {
      this.#frameHandle = null;
      this.#advance(timeMs);
      const listeners = this.#listeners;
      for (let index = 0; index < listeners.length; index += 1) listeners[index]!(timeMs);
      if (!this.#destroyed && this.#listeners.length > 0) {
        this.#frameHandle = this.#scheduler.request(this.#tick);
      }
    };
  }

  register(id: MeterBusChannelId, input: MeterBusRegistration): MeterBusChannel {
    this.#assertAlive();
    if (typeof id !== "string" && typeof id !== "number") {
      throw new TypeError("MeterBus: channel id must be a string or number");
    }
    if (this.#slotById.has(id)) throw new Error(`MeterBus: channel "${String(id)}" is already registered`);
    const mode = meterBusModeCode(input.mode);
    const minDb = input.minDb ?? -60;
    const maxDb = input.maxDb ?? 0;
    if (!Number.isFinite(minDb) || !Number.isFinite(maxDb) || maxDb <= minDb) {
      throw new RangeError("MeterBus: registration requires finite bounds with maxDb > minDb");
    }
    let slot: number;
    if (this.#freeSlots.length > 0) slot = this.#freeSlots.pop()!;
    else {
      if (this.#slotCount === this.#capacity) this.#grow();
      slot = this.#slotCount;
      this.#slotCount += 1;
    }
    const view = this.view;
    this.#ids[slot] = id;
    this.#slotById.set(id, slot);
    view.active[slot] = 1;
    view.mode[slot] = mode;
    view.enabled[slot] = input.enabled === false ? 0 : 1;
    view.clip[slot] = 0;
    view.minDb[slot] = minDb;
    view.maxDb[slot] = maxDb;
    view.inputDb[slot] = minDb;
    view.ballisticDb[slot] = minDb;
    view.peakHoldDb[slot] = Number.NaN;
    this.#lastFrameAtMs[slot] = Number.NaN;
    this.#lastAdvanceMs[slot] = Number.NaN;
    this.#lastPeak[slot] = 0;
    this.#lastMeanSquare[slot] = 0;
    this.#peakHoldUntilMs[slot] = 0;
    this.#batchStamp[slot] = 0;
    this.#rmsHead[slot] = 0;
    this.#rmsCount[slot] = 0;
    const channel: MeterBusChannel = { id, slot };
    this.#liveChannels.add(channel);
    return channel;
  }

  unregister(channel: MeterBusChannel): void {
    this.#assertAlive();
    // Identity, not (id, slot): slots are reused, so re-registering the same
    // id after an unregister would otherwise let the first — now stale —
    // handle deactivate its replacement. Membership in this set also rejects
    // handles minted by another bus.
    if (channel === null || typeof channel !== "object" || !this.#liveChannels.has(channel)) {
      throw new Error(`MeterBus: channel "${String(channel?.id)}" is not registered on this bus`);
    }
    const slot = channel.slot;
    this.#liveChannels.delete(channel);
    this.view.active[slot] = 0;
    this.#slotById.delete(channel.id);
    this.#ids[slot] = null;
    this.#freeSlots.push(slot);
  }

  pushFrames(data: Float32Array, atMs: number, durationMs: number): number {
    this.#assertAlive();
    if (
      !Number.isFinite(atMs) || !Number.isFinite(durationMs)
      || durationMs < this.#minFrameDurationMs
      || data.length % 3 !== 0
    ) return 0;
    const view = this.view;
    const stamp = this.#nextBatchStamp();
    let accepted = 0;
    for (let index = 0; index < data.length; index += 3) {
      const slotValue = data[index]!;
      const peak = data[index + 1]!;
      const meanSquare = data[index + 2]!;
      const slot = slotValue | 0;
      if (slot !== slotValue || slot < 0 || slot >= this.#capacity || view.active[slot] !== 1) continue;
      if (this.#batchStamp[slot] === stamp) continue;
      this.#batchStamp[slot] = stamp;
      if (view.enabled[slot] !== 1) continue;
      const lastFrameAt = this.#lastFrameAtMs[slot]!;
      const hasFrame = !Number.isNaN(lastFrameAt);
      // Staleness is judged against the slot's advance clock, not just its
      // last telemetry stamp. An idle time step moves state forward to
      // `#lastAdvanceMs`; accepting a batch stamped before that would rewind
      // the clock, clamp a negative elapsed to zero, and diverge from the
      // same push/time trace applied in order.
      const lastAdvance = this.#lastAdvanceMs[slot]!;
      const staleFloor = hasFrame ? (Number.isNaN(lastAdvance) ? lastFrameAt : Math.max(lastFrameAt, lastAdvance)) : null;
      if (!isMeterFrameValid(atMs, peak, meanSquare, durationMs, staleFloor)) continue;
      this.#applyFrame(slot, hasFrame, atMs, peak, meanSquare, durationMs);
      accepted += 1;
    }
    return accepted;
  }

  resetClip(id: MeterBusChannelId): void {
    this.#assertAlive();
    const slot = this.#slotById.get(id);
    if (slot === undefined) throw new Error(`MeterBus: channel "${String(id)}" is not registered on this bus`);
    this.view.clip[slot] = 0;
  }

  setEnabled(id: MeterBusChannelId, value: boolean): void {
    this.#assertAlive();
    const slot = this.#slotById.get(id);
    if (slot === undefined) throw new Error(`MeterBus: channel "${String(id)}" is not registered on this bus`);
    this.view.enabled[slot] = value ? 1 : 0;
  }

  slotOf(id: MeterBusChannelId): number {
    this.#assertAlive();
    const slot = this.#slotById.get(id);
    if (slot === undefined) throw new Error(`MeterBus: channel "${String(id)}" is not registered on this bus`);
    return slot;
  }

  subscribe(listener: (timeMs: number) => void): () => void {
    this.#assertAlive();
    this.#listeners.push(listener);
    if (this.#listeners.length === 1 && this.#frameHandle === null) {
      this.#frameHandle = this.#scheduler.request(this.#tick);
    }
    let active = true;
    return () => {
      if (!active) return;
      active = false;
      const index = this.#listeners.indexOf(listener);
      if (index >= 0) this.#listeners.splice(index, 1);
      if (this.#listeners.length === 0 && this.#frameHandle !== null) {
        this.#scheduler.cancel(this.#frameHandle);
        this.#frameHandle = null;
      }
    };
  }

  destroy(): void {
    if (this.#destroyed) return;
    this.#destroyed = true;
    if (this.#frameHandle !== null) {
      this.#scheduler.cancel(this.#frameHandle);
      this.#frameHandle = null;
    }
    this.#listeners.length = 0;
    this.#slotById.clear();
    this.view.active.fill(0);
  }

  #assertAlive(): void {
    if (this.#destroyed) throw new Error("MeterBus: bus has been destroyed");
  }

  #nextBatchStamp(): number {
    this.#batchCounter += 1;
    if (this.#batchCounter === 0x7fffffff) {
      this.#batchCounter = 1;
      this.#batchStamp.fill(0);
    }
    return this.#batchCounter;
  }

  #applyFrame(slot: number, hasFrame: boolean, atMs: number, rawPeak: number, rawMeanSquare: number, durationMs: number): void {
    const view = this.view;
    const minDb = view.minDb[slot]!;
    const maxDb = view.maxDb[slot]!;
    const lastAdvance = this.#lastAdvanceMs[slot]!;
    const elapsedMs = hasFrame ? Math.max(atMs - lastAdvance, 0) : Math.max(durationMs, 0);
    const peak = Math.max(rawPeak, 0);
    const inputDb = meterInputDb(rawPeak, minDb);
    const mode = view.mode[slot]!;

    let ballisticDb: number;
    if (mode === MODE_VU) {
      ballisticDb = meterVuStepDb(view.ballisticDb[slot]!, rawMeanSquare, elapsedMs);
    } else if (mode === MODE_PPM) {
      ballisticDb = meterPpmStepDb(view.ballisticDb[slot]!, peak, inputDb, elapsedMs);
    } else if (mode === MODE_SAMPLE_PEAK) {
      ballisticDb = meterSamplePeakStepDb(view.ballisticDb[slot]!, inputDb, elapsedMs, minDb);
    } else {
      this.#pushRmsSlice(slot, Math.max(rawMeanSquare, 0), Math.max(durationMs, 0));
      ballisticDb = Math.max(this.#rmsDb(slot, minDb), minDb);
    }

    const holdRaw = view.peakHoldDb[slot]!;
    const holdDb = Number.isNaN(holdRaw) ? null : holdRaw;
    const holdUntilMs = Number.isNaN(holdRaw) ? null : this.#peakHoldUntilMs[slot]!;
    const sinceMs = hasFrame ? lastAdvance : null;
    view.peakHoldDb[slot] = meterPeakHoldDbStep(holdDb, holdUntilMs, sinceMs, inputDb, atMs, minDb);
    this.#peakHoldUntilMs[slot] = meterPeakHoldUntilStep(holdDb, holdUntilMs, inputDb, atMs);
    view.inputDb[slot] = inputDb;
    view.ballisticDb[slot] = meterClampDb(ballisticDb, minDb, maxDb);
    view.clip[slot] = meterClipStep(view.clip[slot] === 1, peak) ? 1 : 0;
    this.#lastFrameAtMs[slot] = atMs;
    this.#lastAdvanceMs[slot] = atMs;
    this.#lastPeak[slot] = peak;
    this.#lastMeanSquare[slot] = rawMeanSquare;
  }

  #pushRmsSlice(slot: number, meanSquare: number, durationMs: number): void {
    const ringCapacity = this.#ringCapacity;
    const base = slot * ringCapacity;
    let head = this.#rmsHead[slot]!;
    let count = this.#rmsCount[slot]!;
    let total = 0;
    for (let offset = 0; offset < count; offset += 1) {
      total += this.#rmsDurationMs[base + ((head + offset) % ringCapacity)]!;
    }
    let excess = total + durationMs - RMS_WINDOW_MS;
    let appendDurationMs = durationMs;
    while (excess > 0 && count > 0) {
      const headDuration = this.#rmsDurationMs[base + head]!;
      if (headDuration <= excess) {
        excess -= headDuration;
        head = (head + 1) % ringCapacity;
        count -= 1;
      } else {
        this.#rmsDurationMs[base + head] = headDuration - excess;
        excess = 0;
      }
    }
    if (excess > 0) appendDurationMs = durationMs - excess;
    const tail = (head + count) % ringCapacity;
    this.#rmsMeanSquare[base + tail] = meanSquare;
    this.#rmsDurationMs[base + tail] = appendDurationMs;
    this.#rmsHead[slot] = head;
    this.#rmsCount[slot] = count + 1;
  }

  #rmsDb(slot: number, fallbackDb: number): number {
    const ringCapacity = this.#ringCapacity;
    const base = slot * ringCapacity;
    const head = this.#rmsHead[slot]!;
    const count = this.#rmsCount[slot]!;
    let durationSum = 0;
    for (let offset = 0; offset < count; offset += 1) {
      durationSum += this.#rmsDurationMs[base + ((head + offset) % ringCapacity)]!;
    }
    let weightedSum = 0;
    for (let offset = 0; offset < count; offset += 1) {
      const index = base + ((head + offset) % ringCapacity);
      weightedSum += this.#rmsMeanSquare[index]! * this.#rmsDurationMs[index]!;
    }
    return meterWeightedRmsDb(weightedSum, durationSum, fallbackDb);
  }

  #advance(timeMs: number): void {
    const view = this.view;
    for (let slot = 0; slot < this.#slotCount; slot += 1) {
      if (view.active[slot] !== 1 || view.enabled[slot] !== 1) continue;
      const lastAdvance = this.#lastAdvanceMs[slot]!;
      if (Number.isNaN(lastAdvance)) continue;
      const elapsedMs = timeMs - lastAdvance;
      if (elapsedMs <= 0) continue;
      const minDb = view.minDb[slot]!;
      const mode = view.mode[slot]!;
      if (mode === MODE_VU) {
        view.ballisticDb[slot] = meterClampDb(
          meterVuStepDb(view.ballisticDb[slot]!, this.#lastMeanSquare[slot]!, elapsedMs),
          minDb, view.maxDb[slot]!,
        );
      } else if (mode === MODE_PPM) {
        view.ballisticDb[slot] = meterClampDb(
          meterPpmStepDb(view.ballisticDb[slot]!, this.#lastPeak[slot]!, view.inputDb[slot]!, elapsedMs),
          minDb, view.maxDb[slot]!,
        );
      } else if (mode === MODE_SAMPLE_PEAK) {
        view.ballisticDb[slot] = meterClampDb(
          meterSamplePeakStepDb(view.ballisticDb[slot]!, view.inputDb[slot]!, elapsedMs, minDb),
          minDb, view.maxDb[slot]!,
        );
      }
      const holdDb = view.peakHoldDb[slot]!;
      if (!Number.isNaN(holdDb)) {
        view.peakHoldDb[slot] = meterPeakHoldDecayDb(holdDb, this.#peakHoldUntilMs[slot]!, lastAdvance, timeMs, minDb);
      }
      this.#lastAdvanceMs[slot] = timeMs;
    }
  }

  #grow(): void {
    const previousCapacity = this.#capacity;
    const capacity = previousCapacity * 2;
    const ringCapacity = this.#ringCapacity;
    const view = this.view;

    const growUint8 = (source: Uint8Array) => {
      const next = new Uint8Array(capacity);
      next.set(source);
      return next;
    };
    const growFloat64 = (source: Float64Array, fill: number) => {
      const next = new Float64Array(capacity).fill(fill);
      next.set(source);
      return next;
    };

    view.active = growUint8(view.active);
    view.mode = growUint8(view.mode);
    view.enabled = growUint8(view.enabled);
    view.clip = growUint8(view.clip);
    view.minDb = growFloat64(view.minDb, 0);
    view.maxDb = growFloat64(view.maxDb, 0);
    view.inputDb = growFloat64(view.inputDb, 0);
    view.ballisticDb = growFloat64(view.ballisticDb, 0);
    view.peakHoldDb = growFloat64(view.peakHoldDb, Number.NaN);
    view.capacity = capacity;
    view.generation += 1;

    this.#lastFrameAtMs = growFloat64(this.#lastFrameAtMs, Number.NaN);
    this.#lastAdvanceMs = growFloat64(this.#lastAdvanceMs, Number.NaN);
    this.#lastPeak = growFloat64(this.#lastPeak, 0);
    this.#lastMeanSquare = growFloat64(this.#lastMeanSquare, 0);
    this.#peakHoldUntilMs = growFloat64(this.#peakHoldUntilMs, 0);
    const batchStamp = new Int32Array(capacity);
    batchStamp.set(this.#batchStamp);
    this.#batchStamp = batchStamp;
    const rmsMeanSquare = new Float64Array(capacity * ringCapacity);
    rmsMeanSquare.set(this.#rmsMeanSquare);
    this.#rmsMeanSquare = rmsMeanSquare;
    const rmsDurationMs = new Float64Array(capacity * ringCapacity);
    rmsDurationMs.set(this.#rmsDurationMs);
    this.#rmsDurationMs = rmsDurationMs;
    const rmsHead = new Int32Array(capacity);
    rmsHead.set(this.#rmsHead);
    this.#rmsHead = rmsHead;
    const rmsCount = new Int32Array(capacity);
    rmsCount.set(this.#rmsCount);
    this.#rmsCount = rmsCount;

    const ids = new Array<MeterBusChannelId | null>(capacity).fill(null);
    for (let slot = 0; slot < previousCapacity; slot += 1) ids[slot] = this.#ids[slot]!;
    this.#ids = ids;
    this.#capacity = capacity;
  }
}

export function createMeterBus(options: MeterBusOptions = {}): MeterBus {
  return new MeterBusImpl(options);
}
