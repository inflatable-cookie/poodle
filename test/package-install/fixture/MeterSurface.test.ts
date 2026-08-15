import { render } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";

import "@inflatable-cookie/poodle-core/styles/audio-meter.css";
import "@inflatable-cookie/poodle-core/styles/meter-surface.css";
import {
  createManualMeterFrameScheduler,
  createMeterBus,
  type MeterSurfacePainter,
} from "@inflatable-cookie/poodle-core";

import MeterSurfaceHarness from "./MeterSurfaceHarness.svelte";

const painter: MeterSurfacePainter = {
  setup: () => {},
  resize: () => {},
  setPalette: () => {},
  paint: () => {},
  destroy: () => {},
};

describe("packed MeterBus + Svelte MeterSurface", () => {
  it("mounts a surface-mode meter from public package entries", () => {
    const scheduler = createManualMeterFrameScheduler();
    const bus = createMeterBus({ scheduler });
    const channel = bus.register("packed-channel", { mode: "sample-peak" });
    const { container } = render(MeterSurfaceHarness, { bus, painter });

    const root = container.querySelector(".poodle-audio-meter[data-surface]");
    expect(root).not.toBeNull();
    expect(root!.querySelector(".poodle-audio-meter-visual")).toBeNull();
    expect(container.querySelector(".poodle-meter-surface__canvas")).not.toBeNull();

    expect(bus.pushFrames(new Float32Array([channel.slot, 0.5, 0.25]), 16, 66)).toBe(1);
    expect(bus.view.ballisticDb[channel.slot]!).toBeCloseTo(-6.0206, 3);
    bus.destroy();
  });
});
