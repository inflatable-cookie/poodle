import { act, render } from "@testing-library/react";
import { createRef } from "react";
import { describe, expect, it } from "vitest";
import {
  createManualMeterFrameScheduler,
  createMeterBus,
  type MeterDrawPass,
  type MeterSurfacePainter,
} from "@inflatable-cookie/poodle-core";
import { AudioMeter, type AudioMeterHandle } from "../src/AudioMeter";
import { MeterSurface } from "../src/MeterSurface";

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

function setup(options: { rightChannel?: string } = {}) {
  const scheduler = createManualMeterFrameScheduler();
  const bus = createMeterBus({ scheduler });
  const a = bus.register("a", { mode: "sample-peak" });
  const b = bus.register("b", { mode: "sample-peak" });
  const right = options.rightChannel ? bus.register(options.rightChannel, { mode: "sample-peak" }) : null;
  const { painter, log } = createFakePainter();
  return { scheduler, bus, a, b, right, painter, log };
}

describe("AudioMeter standalone tier (react)", () => {
  it("keeps the existing markup, visuals, and accessibility surface", () => {
    const { container } = render(<AudioMeter ariaLabel="Level" />);
    const root = container.querySelector(".poodle-audio-meter")!;
    expect(root.getAttribute("role")).toBe("meter");
    expect(root.hasAttribute("data-surface")).toBe(false);
    expect(root.getAttribute("data-channels")).toBe("mono");
    expect(root.getAttribute("aria-valuetext")).toBe("-60 dB");
    expect(root.querySelectorAll(".poodle-audio-meter-visual").length).toBe(1);
    expect(root.querySelectorAll(".poodle-audio-meter-visual__segment").length).toBe(20);
  });
});

describe("AudioMeter surface tier (react)", () => {
  it("renders layout boxes without visual children inside one canvas surface", () => {
    const { bus, painter } = setup();
    const { container } = render(
      <MeterSurface bus={bus} painter={painter}>
        <AudioMeter surface={bus} channel="a" ariaLabel="Channel A" segments={12} />
        <AudioMeter surface={bus} channel="b" ariaLabel="Channel B" segments={12} />
      </MeterSurface>,
    );
    const roots = container.querySelectorAll(".poodle-audio-meter");
    expect(roots.length).toBe(2);
    for (const root of roots) {
      expect(root.getAttribute("data-surface")).toBe("true");
      expect(root.querySelector(".poodle-audio-meter-visual")).toBeNull();
      expect(root.children.length).toBe(0);
      expect(root.getAttribute("aria-valuetext")).toBe("-60 dB");
    }
    expect(container.querySelectorAll("canvas").length).toBe(1);
    expect(container.querySelector(".poodle-meter-surface__canvas")!.getAttribute("aria-hidden")).toBe("true");
  });

  it("registers placeholders, maps stereo slots, and cleans up on unmount", () => {
    const { bus, painter, log, scheduler, a, right } = setup({ rightChannel: "a-right" });
    const { unmount } = render(
      <MeterSurface bus={bus} painter={painter}>
        <AudioMeter surface={bus} channel="a" rightChannel="a-right" ariaLabel="Channel A" segments={12} />
        <AudioMeter surface={bus} channel="b" ariaLabel="Channel B" segments={12} />
      </MeterSurface>,
    );
    act(() => scheduler.fire(16));
    const pass = log.lastPass!;
    expect(pass.count).toBe(3);
    expect(pass.slot[0]).toBe(a.slot);
    expect(pass.slot[1]).toBe(right!.slot);
    unmount();
    expect(log.destroyed).toBe(1);
    expect(scheduler.pendingCount()).toBe(0);
    expect(bus.view.active[a.slot]).toBe(1);
  });

  it("forwards push and resetClip handles to the registered bus slots", () => {
    const { bus, painter, a } = setup();
    const handle = createRef<AudioMeterHandle>();
    render(
      <MeterSurface bus={bus} painter={painter}>
        <AudioMeter ref={handle} surface={bus} channel="a" ariaLabel="Channel A" />
      </MeterSurface>,
    );
    act(() => handle.current!.push({ atMs: 100, peak: 1.2, meanSquare: 1, durationMs: 66 }));
    expect(bus.view.ballisticDb[a.slot]).toBeGreaterThan(-1);
    expect(bus.view.clip[a.slot]).toBe(1);
    act(() => handle.current!.resetClip());
    expect(bus.view.clip[a.slot]).toBe(0);
  });

  it("refreshes aria through the shared cadence at most 2 Hz", () => {
    const { bus, painter, scheduler, a } = setup();
    const { container } = render(
      <MeterSurface bus={bus} painter={painter}>
        <AudioMeter surface={bus} channel="a" ariaLabel="Channel A" />
      </MeterSurface>,
    );
    const root = container.querySelector(".poodle-audio-meter")!;
    expect(root.getAttribute("aria-valuetext")).toBe("-60 dB");
    act(() => {
      bus.pushFrames(new Float32Array([a.slot, 1, 1]), 100, 66);
      scheduler.fire(116);
    });
    expect(root.getAttribute("aria-valuetext")).toBe("0 dB");
    act(() => {
      bus.pushFrames(new Float32Array([a.slot, 0, 0]), 200, 66);
      scheduler.fire(216);
    });
    expect(root.getAttribute("aria-valuetext")).toBe("0 dB");
    act(() => scheduler.fire(716));
    expect(root.getAttribute("aria-valuetext")).not.toBe("0 dB");
  });

  it("fails clearly on missing surface context and unregistered channels", () => {
    const { bus, painter } = setup();
    const consoleError = console.error;
    console.error = () => {};
    try {
      expect(() => render(<AudioMeter surface={bus} channel="a" />)).toThrow(/enclosing MeterSurface/);
      expect(() => render(<AudioMeter surface={bus} channel={null} />)).toThrow(/registered `channel`/);
      expect(() => render(
        <MeterSurface bus={bus} painter={painter}>
          <AudioMeter surface={bus} channel="missing" />
        </MeterSurface>,
      )).toThrow(/not registered/);
    } finally {
      console.error = consoleError;
    }
  });
});
