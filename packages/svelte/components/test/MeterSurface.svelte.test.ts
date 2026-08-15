import { render } from "@testing-library/svelte";
import { tick } from "svelte";
import { describe, expect, it } from "vitest";
import {
  createManualMeterFrameScheduler,
  createMeterBus,
  type MeterDrawPass,
  type MeterSurfacePainter,
} from "@inflatable-cookie/poodle-core";
import AudioMeter from "../src/AudioMeter.svelte";
import MeterSurfaceHarness from "./MeterSurfaceHarness.svelte";

function createFakePainter() {
  const log = { paints: [] as number[], lastPass: null as MeterDrawPass | null, destroyed: 0 };
  const painter: MeterSurfacePainter = {
    setup: () => {},
    resize: () => {},
    setPalette: () => {},
    paint: (pass) => {
      log.paints.push(pass.count);
      log.lastPass = pass;
    },
    destroy: () => { log.destroyed += 1; },
  };
  return { painter, log };
}

function setup(options: { rightChannel?: string | null } = {}) {
  const scheduler = createManualMeterFrameScheduler();
  const bus = createMeterBus({ scheduler });
  const a = bus.register("a", { mode: "sample-peak" });
  const b = bus.register("b", { mode: "sample-peak" });
  const right = options.rightChannel ? bus.register(options.rightChannel, { mode: "sample-peak" }) : null;
  const { painter, log } = createFakePainter();
  return { scheduler, bus, a, b, right, painter, log };
}

describe("AudioMeter standalone tier", () => {
  it("keeps the existing markup, visuals, and accessibility surface", () => {
    const { container } = render(AudioMeter, { ariaLabel: "Level" });
    const root = container.querySelector(".poodle-audio-meter")!;
    expect(root.getAttribute("role")).toBe("meter");
    expect(root.hasAttribute("data-surface")).toBe(false);
    expect(root.getAttribute("data-channels")).toBe("mono");
    expect(root.getAttribute("aria-valuemin")).toBe("-60");
    expect(root.getAttribute("aria-valuemax")).toBe("0");
    expect(root.getAttribute("aria-valuetext")).toBe("-60 dB");
    expect(root.querySelectorAll(".poodle-audio-meter-visual").length).toBe(1);
    expect(root.querySelectorAll(".poodle-audio-meter-visual__segment").length).toBe(20);
  });
});

describe("AudioMeter surface tier", () => {
  it("renders the layout box with meter semantics and no visual children", async () => {
    const { bus, painter } = setup();
    const { container } = render(MeterSurfaceHarness, { bus, painter });
    await tick();
    const roots = container.querySelectorAll(".poodle-audio-meter");
    expect(roots.length).toBe(2);
    for (const root of roots) {
      expect(root.getAttribute("role")).toBe("meter");
      expect(root.getAttribute("data-surface")).toBe("true");
      expect(root.querySelector(".poodle-audio-meter-visual")).toBeNull();
      expect(root.children.length).toBe(0);
      expect(root.getAttribute("aria-valuetext")).toBe("-60 dB");
    }
    const canvas = container.querySelector(".poodle-meter-surface__canvas")!;
    expect(canvas.getAttribute("aria-hidden")).toBe("true");
    expect(container.querySelectorAll("canvas").length).toBe(1);
  });

  it("registers placeholders on the shared surface and cleans up on unmount", async () => {
    const { bus, painter, log, scheduler } = setup();
    const { unmount } = render(MeterSurfaceHarness, { bus, painter });
    await tick();
    scheduler.fire(16);
    expect(log.paints.at(-1)).toBe(2);
    unmount();
    expect(log.destroyed).toBe(1);
    expect(scheduler.pendingCount()).toBe(0);
    expect(bus.view.active[0]).toBe(1);
  });

  it("maps stereo placeholders onto the correct bus slots", async () => {
    const { bus, painter, log, scheduler, a, right } = setup({ rightChannel: "a-right" });
    render(MeterSurfaceHarness, { bus, painter, rightChannel: "a-right" });
    await tick();
    scheduler.fire(16);
    const pass = log.lastPass!;
    expect(pass.count).toBe(3);
    expect(pass.slot[0]).toBe(a.slot);
    expect(pass.slot[1]).toBe(right!.slot);
    const stereoRoot = document.querySelector('.poodle-audio-meter[data-channels="stereo"]');
    expect(stereoRoot).not.toBeNull();
  });

  it("forwards push and resetClip handles to the registered bus slots", async () => {
    const { bus, painter, a } = setup();
    const { component } = render(MeterSurfaceHarness, { bus, painter });
    await tick();
    const meter = component.meter()!;
    meter.push({ atMs: 100, peak: 1.2, meanSquare: 1, durationMs: 66 });
    expect(bus.view.ballisticDb[a.slot]).toBeGreaterThan(-1);
    expect(bus.view.clip[a.slot]).toBe(1);
    meter.resetClip();
    expect(bus.view.clip[a.slot]).toBe(0);
  });

  it("refreshes aria through the shared cadence at most 2 Hz", async () => {
    const { bus, painter, scheduler, a } = setup();
    const { container } = render(MeterSurfaceHarness, { bus, painter });
    await tick();
    const root = container.querySelector(".poodle-audio-meter")!;
    expect(root.getAttribute("aria-valuetext")).toBe("-60 dB");
    bus.pushFrames(new Float32Array([a.slot, 1, 1]), 100, 66);
    scheduler.fire(116);
    await tick();
    expect(root.getAttribute("aria-valuetext")).toBe("0 dB");
    bus.pushFrames(new Float32Array([a.slot, 0, 0]), 200, 66);
    scheduler.fire(216);
    await tick();
    expect(root.getAttribute("aria-valuetext")).toBe("0 dB");
    scheduler.fire(716);
    await tick();
    expect(root.getAttribute("aria-valuetext")).not.toBe("0 dB");
  });

  it("detaches the canvas registration when a meter leaves surface mode", async () => {
    const { bus, painter, log, scheduler } = setup();
    const { rerender, container } = render(MeterSurfaceHarness, { bus, painter, firstSurface: true });
    await tick();
    scheduler.fire(16);
    expect(log.paints.at(-1)).toBe(2);

    await rerender({ bus, painter, firstSurface: false });
    await tick();
    scheduler.fire(32);
    // Only the still-surface meter is painted; the standalone one is not.
    expect(log.paints.at(-1)).toBe(1);
    const standalone = container.querySelector('.poodle-audio-meter:not([data-surface])')!;
    expect(standalone.querySelectorAll(".poodle-audio-meter-visual").length).toBe(1);
  });

  it("registers when a meter enters surface mode later", async () => {
    const { bus, painter, log, scheduler } = setup();
    const { rerender } = render(MeterSurfaceHarness, { bus, painter, firstSurface: false });
    await tick();
    scheduler.fire(16);
    expect(log.paints.at(-1)).toBe(1);

    await rerender({ bus, painter, firstSurface: true });
    await tick();
    scheduler.fire(32);
    expect(log.paints.at(-1)).toBe(2);
  });

  it("re-registers onto the new slot when the channel is replaced", async () => {
    const { bus, painter, log, scheduler, a, b } = setup();
    const { rerender } = render(MeterSurfaceHarness, { bus, painter, showSecond: false, firstChannel: "a" });
    await tick();
    scheduler.fire(16);
    expect(log.lastPass!.count).toBe(1);
    expect(log.lastPass!.slot[0]).toBe(a.slot);

    await rerender({ bus, painter, showSecond: false, firstChannel: "b" });
    await tick();
    scheduler.fire(32);
    // Exactly one record, pointing at the replacement slot — not two.
    expect(log.lastPass!.count).toBe(1);
    expect(log.lastPass!.slot[0]).toBe(b.slot);
  });

  it("validates a later transition into surface mode", async () => {
    const scheduler = createManualMeterFrameScheduler();
    const bus = createMeterBus({ scheduler });
    bus.register("a", { mode: "sample-peak" });
    bus.register("b", { mode: "sample-peak" });
    const { painter } = createFakePainter();
    const { rerender } = render(MeterSurfaceHarness, { bus, painter, showSecond: false, firstSurface: false });
    await tick();
    await expect(
      rerender({ bus, painter, showSecond: false, firstSurface: true, firstChannel: "missing" }),
    ).rejects.toThrow(/not registered/);
  });

  it("fails clearly on missing surface context, wrong bus, and unregistered channels", () => {
    const scheduler = createManualMeterFrameScheduler();
    const bus = createMeterBus({ scheduler });
    bus.register("a", { mode: "sample-peak" });
    expect(() => render(AudioMeter, { surface: bus, channel: "a" })).toThrow(/enclosing MeterSurface/);
    expect(() => render(AudioMeter, { surface: bus, channel: null })).toThrow(/registered `channel`/);
    const other = createMeterBus({ scheduler: createManualMeterFrameScheduler() });
    other.register("a", { mode: "sample-peak" });
    other.register("b", { mode: "sample-peak" });
    const { painter } = createFakePainter();
    expect(() => render(MeterSurfaceHarness, { bus, meterBus: other, painter })).toThrow(/must be the bus/);
    expect(() => render(MeterSurfaceHarness, { bus, painter, rightChannel: "missing" })).toThrow(/not registered/);
  });
});
