import { afterEach, describe, expect, it, vi } from "vitest";

import {
  createManualMeterFrameScheduler,
  createMeterBus,
  createMeterSurfaceRegistry,
  type MeterDrawPass,
  type MeterSurfaceElements,
  type MeterSurfacePainter,
  type MeterSurfacePalette,
} from "../../packages/core/src";

interface PainterLog {
  setup: number;
  resize: Array<[number, number, number]>;
  palettes: MeterSurfacePalette[];
  paints: number[];
  lastPass: MeterDrawPass | null;
  destroyed: number;
}

function createFakePainter(): { painter: MeterSurfacePainter; log: PainterLog } {
  const log: PainterLog = { setup: 0, resize: [], palettes: [], paints: [], lastPass: null, destroyed: 0 };
  return {
    log,
    painter: {
      setup: () => { log.setup += 1; },
      resize: (width, height, dpr) => { log.resize.push([width, height, dpr]); },
      setPalette: (palette) => { log.palettes.push(palette); },
      paint: (pass: MeterDrawPass) => {
        log.paints.push(pass.count);
        log.lastPass = pass;
      },
      destroy: () => { log.destroyed += 1; },
    },
  };
}

function mountSurfaceElements(): MeterSurfaceElements {
  const root = document.createElement("div");
  const viewport = document.createElement("div");
  const content = document.createElement("div");
  const canvas = document.createElement("canvas");
  Object.defineProperty(viewport, "clientWidth", { value: 400, configurable: true });
  Object.defineProperty(viewport, "clientHeight", { value: 240, configurable: true });
  viewport.appendChild(content);
  root.appendChild(viewport);
  root.appendChild(canvas);
  document.body.appendChild(root);
  return { root, viewport, content, canvas };
}

function mountMeterElement(content: HTMLElement): HTMLElement {
  const element = document.createElement("div");
  element.className = "poodle-audio-meter";
  content.appendChild(element);
  return element;
}

function setup() {
  const scheduler = createManualMeterFrameScheduler();
  const bus = createMeterBus({ scheduler });
  const registry = createMeterSurfaceRegistry(bus);
  const elements = mountSurfaceElements();
  const { painter, log } = createFakePainter();
  return { scheduler, bus, registry, elements, painter, log };
}

afterEach(() => {
  document.body.replaceChildren();
  vi.unstubAllGlobals();
});

describe("meter surface controller", () => {
  it("attaches queued placeholders on connect and assembles one draw pass", () => {
    const { scheduler, bus, registry, elements, painter, log } = setup();
    const mono = bus.register("mono", { mode: "sample-peak" });
    const left = bus.register("left", { mode: "vu" });
    const right = bus.register("right", { mode: "vu" });

    registry.registerMeter(mountMeterElement(elements.content), {
      slot: mono.slot, rightSlot: null, style: "segments", orientation: "vertical", segments: 12,
    });
    registry.registerMeter(mountMeterElement(elements.content), {
      slot: left.slot, rightSlot: right.slot, style: "bar", orientation: "horizontal", segments: 12,
    });
    registry.connect(elements, { painter });

    bus.pushFrames(new Float32Array([mono.slot, 0.5, 0.25]), 100, 66);
    scheduler.fire(116);

    expect(log.setup).toBe(1);
    expect(log.resize).toEqual([[400, 240, 1]]);
    expect(log.palettes.length).toBe(1);
    expect(log.paints).toEqual([3]);
    const pass = log.lastPass!;
    expect(pass.count).toBe(3);
    expect(pass.slot[0]).toBe(mono.slot);
    expect(pass.style[0]).toBe(1);
    expect(pass.segments[0]).toBe(12);
    expect(pass.value[0]).toBeCloseTo(1 - 6.020599913 / 60, 5);
    expect(Number.isNaN(pass.peak[1]!)).toBe(true);
    expect(pass.orientation[1]).toBe(1);
    expect(pass.slot[2]).toBe(right.slot);
  });

  it("culls placeholders outside the scrolled viewport without touching bus state", () => {
    const { scheduler, bus, registry, elements, painter, log } = setup();
    const mono = bus.register("mono", { mode: "sample-peak" });
    registry.registerMeter(mountMeterElement(elements.content), {
      slot: mono.slot, rightSlot: null, style: "segments", orientation: "vertical", segments: 12,
    });
    registry.connect(elements, { painter });
    scheduler.fire(16);
    expect(log.paints.at(-1)).toBe(1);

    Object.defineProperty(elements.viewport, "scrollLeft", { value: 600, configurable: true });
    scheduler.fire(32);
    expect(log.paints.at(-1)).toBe(0);
    expect(bus.view.active[mono.slot]).toBe(1);

    Object.defineProperty(elements.viewport, "scrollLeft", { value: 0, configurable: true });
    scheduler.fire(48);
    expect(log.paints.at(-1)).toBe(1);
  });

  it("samples aria through one shared cadence at most 2 Hz", () => {
    const { scheduler, bus, registry, elements, painter } = setup();
    const mono = bus.register("mono", { mode: "sample-peak" });
    const samples: number[] = [];
    registry.registerMeter(
      mountMeterElement(elements.content),
      { slot: mono.slot, rightSlot: null, style: "segments", orientation: "vertical", segments: 12 },
      (timeMs) => samples.push(timeMs),
    );
    registry.connect(elements, { painter });
    scheduler.fire(0);
    scheduler.fire(100);
    scheduler.fire(400);
    scheduler.fire(510);
    scheduler.fire(600);
    scheduler.fire(1020);
    expect(samples).toEqual([0, 510, 1020]);
  });

  it("re-probes the palette when theme attributes change", async () => {
    const { scheduler, registry, elements, painter, log } = setup();
    registry.connect(elements, { painter });
    scheduler.fire(16);
    expect(log.palettes.length).toBe(1);
    document.documentElement.setAttribute("data-theme", "midnight");
    await new Promise((resolve) => setTimeout(resolve, 0));
    scheduler.fire(32);
    expect(log.palettes.length).toBe(2);
  });

  it("supports update and detach on placeholder handles", () => {
    const { scheduler, bus, registry, elements, painter, log } = setup();
    const a = bus.register("a", { mode: "sample-peak" });
    const b = bus.register("b", { mode: "sample-peak" });
    const handle = registry.registerMeter(mountMeterElement(elements.content), {
      slot: a.slot, rightSlot: null, style: "segments", orientation: "vertical", segments: 12,
    });
    registry.connect(elements, { painter });
    scheduler.fire(16);
    expect(log.paints.at(-1)).toBe(1);

    handle.update({ slot: b.slot, rightSlot: null, style: "bar", orientation: "vertical", segments: 12 });
    scheduler.fire(32);
    expect(log.paints.at(-1)).toBe(1);

    handle.detach();
    scheduler.fire(48);
    expect(log.paints.at(-1)).toBe(0);
  });

  it("skips slots that were unregistered from the bus", () => {
    const { scheduler, bus, registry, elements, painter, log } = setup();
    const mono = bus.register("mono", { mode: "sample-peak" });
    registry.registerMeter(mountMeterElement(elements.content), {
      slot: mono.slot, rightSlot: null, style: "segments", orientation: "vertical", segments: 12,
    });
    registry.connect(elements, { painter });
    scheduler.fire(16);
    expect(log.paints.at(-1)).toBe(1);
    bus.unregister(mono);
    scheduler.fire(32);
    expect(log.paints.at(-1)).toBe(0);
  });

  it("destroy releases the painter and the bus subscription", () => {
    const { scheduler, registry, elements, painter, log } = setup();
    registry.connect(elements, { painter });
    expect(scheduler.pendingCount()).toBe(1);
    registry.destroy();
    expect(log.destroyed).toBe(1);
    expect(scheduler.pendingCount()).toBe(0);
  });
});
