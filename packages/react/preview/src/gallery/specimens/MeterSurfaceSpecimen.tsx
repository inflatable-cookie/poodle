import { useEffect, useMemo, useRef, useState } from "react";
import {
  createMeterBus,
  type AudioMeterMode, type MeterBus, type MeterBusChannel, type MeterFrameScheduler,
} from "@inflatable-cookie/poodle-core";
import { AudioMeter, MeterSurface } from "@inflatable-cookie/poodle-react";
import { AudioSpecimenGroup as Group, AudioSpecimenPage as Page } from "./AudioSpecimen";

const MODES: AudioMeterMode[] = ["vu", "ppm", "sample-peak", "rms"];
const DATA_INTERVAL_MS = 1000 / 15;
const WARMUP_FRAMES = 60;

interface SceneMeter {
  id: string;
  right: string | null;
  style: "bar" | "segments";
  orientation: "vertical" | "horizontal";
  label: string;
}

interface Scene {
  bus: MeterBus;
  meters: SceneMeter[];
  version: number;
  // Keep the handles `register` returns: `unregister` compares handle
  // identity, so a synthesized `{ id, slot }` is (correctly) rejected.
  handles: Map<string, MeterBusChannel>;
}

interface Workload {
  samples: number[];
  running: boolean;
  frameCount: number;
}

function buildMeters(count: number, register: (id: string, index: number) => void): SceneMeter[] {
  const meters: SceneMeter[] = [];
  for (let index = 0; index < count; index += 1) {
    const id = `m${index}`;
    const stereo = index % 8 === 7;
    register(id, index);
    if (stereo) register(`${id}R`, index + 1);
    meters.push({
      id,
      right: stereo ? `${id}R` : null,
      style: index % 5 === 0 ? "bar" : "segments",
      orientation: "vertical",
      label: `Channel ${index + 1}`,
    });
  }
  for (let index = 0; index < 4; index += 1) {
    const id = `h${index}`;
    register(id, index);
    meters.push({
      id,
      right: null,
      style: index % 2 === 0 ? "bar" : "segments",
      orientation: "horizontal",
      label: `Horizontal ${index + 1}`,
    });
  }
  return meters;
}

function channelIds(meters: SceneMeter[]): string[] {
  const ids: string[] = [];
  for (const meter of meters) {
    ids.push(meter.id);
    if (meter.right !== null) ids.push(meter.right);
  }
  return ids;
}

export function MeterSurfaceSpecimen() {
  const workloadRef = useRef<Workload>({ samples: [], running: false, frameCount: 0 });
  const [count, setCount] = useState(8);
  const [mounted, setMounted] = useState(true);
  const [running, setRunning] = useState(false);
  const [themed, setThemed] = useState(false);
  const [disabledDemo, setDisabledDemo] = useState(false);
  const [readout, setReadout] = useState("workload idle");
  const [scene, setScene] = useState<Scene | null>(null);
  const dataFrameRef = useRef<number | null>(null);
  const metersRef = useRef<SceneMeter[]>([]);
  metersRef.current = scene?.meters ?? [];
  const readoutRef = useRef(refreshReadout);
  readoutRef.current = refreshReadout;

  const scheduler = useMemo<MeterFrameScheduler>(() => ({
    // Wraps requestAnimationFrame so each bus tick measures exactly one frame
    // of bus advance + draw-pass assembly + paint.
    request: (callback) => requestAnimationFrame((timeMs) => {
      const startedAt = performance.now();
      callback(timeMs);
      const workload = workloadRef.current;
      if (workload.running) {
        workload.samples.push(performance.now() - startedAt);
        workload.frameCount += 1;
        if (workload.frameCount % 30 === 0) readoutRef.current();
      }
    }),
    cancel: (handle) => cancelAnimationFrame(handle),
  }), []);

  function createScene(nextCount: number, previous: Scene | null): Scene {
    previous?.bus.destroy();
    const bus = createMeterBus({ scheduler, initialCapacity: 160 });
    const handles = new Map<string, MeterBusChannel>();
    const meters = buildMeters(nextCount, (id, index) => {
      handles.set(id, bus.register(id, { mode: MODES[index % MODES.length]! }));
    });
    workloadRef.current.samples = [];
    workloadRef.current.frameCount = 0;
    return { bus, meters, version: (previous?.version ?? 0) + 1, handles };
  }

  useEffect(() => {
    setScene((previous) => createScene(count, previous));
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [count]);
  useEffect(() => () => {
    scene?.bus.destroy();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  function refreshReadout(): void {
    const measured = workloadRef.current.samples.slice(WARMUP_FRAMES);
    if (measured.length === 0 || scene === null) {
      setReadout("collecting warm-up frames");
      return;
    }
    const sorted = [...measured].sort((a, b) => a - b);
    const pick = (q: number) => sorted[Math.min(Math.floor(q * sorted.length), sorted.length - 1)]!;
    setReadout(JSON.stringify({
      meters: count,
      channels: channelIds(scene.meters).length,
      warmupFrames: WARMUP_FRAMES,
      sampleCount: measured.length,
      meanMs: Number((measured.reduce((sum, value) => sum + value, 0) / measured.length).toFixed(4)),
      p50Ms: Number(pick(0.5).toFixed(4)),
      p95Ms: Number(pick(0.95).toFixed(4)),
      maxMs: Number(sorted[sorted.length - 1]!.toFixed(4)),
      dpr: typeof devicePixelRatio === "number" ? devicePixelRatio : 1,
    }));
  }

  function startWorkload(): void {
    if (scene === null || workloadRef.current.running) return;
    workloadRef.current.samples = [];
    workloadRef.current.frameCount = 0;
    workloadRef.current.running = true;
    setRunning(true);
    setReadout("collecting warm-up frames");
    let feed = new Float32Array(channelIds(metersRef.current).length * 3);
    let lastDataAt = 0;
    const dataTick = (timeMs: number) => {
      dataFrameRef.current = requestAnimationFrame(dataTick);
      if (timeMs - lastDataAt < DATA_INTERVAL_MS) return;
      lastDataAt = timeMs;
      const ids = channelIds(metersRef.current);
      if (feed.length !== ids.length * 3) feed = new Float32Array(ids.length * 3);
      const phases = ids.map((_, index) => (index * 37) % 96);
      for (let index = 0; index < ids.length; index += 1) {
        const phase = phases[index]! + timeMs / 900;
        const level = 0.12 + 0.8 * Math.abs(Math.sin(phase)) * (0.6 + 0.4 * Math.abs(Math.sin(phase * 0.31)));
        const peak = Math.min(level, 0.98);
        feed[index * 3] = scene.bus.slotOf(ids[index]!);
        feed[index * 3 + 1] = peak;
        feed[index * 3 + 2] = peak * peak * 0.55;
      }
      scene.bus.pushFrames(feed, timeMs, DATA_INTERVAL_MS);
    };
    dataFrameRef.current = requestAnimationFrame(dataTick);
  }

  function stopWorkload(): void {
    workloadRef.current.running = false;
    setRunning(false);
    if (dataFrameRef.current !== null) cancelAnimationFrame(dataFrameRef.current);
    dataFrameRef.current = null;
    refreshReadout();
  }

  function removeLastMeter(): void {
    if (scene === null) return;
    const verticals = scene.meters.filter((meter) => meter.orientation === "vertical");
    if (verticals.length <= 1) return;
    const meter = verticals[verticals.length - 1]!;
    const leftHandle = scene.handles.get(meter.id);
    if (leftHandle !== undefined) {
      scene.bus.unregister(leftHandle);
      scene.handles.delete(meter.id);
    }
    const rightHandle = meter.right === null ? undefined : scene.handles.get(meter.right);
    if (rightHandle !== undefined && meter.right !== null) {
      scene.bus.unregister(rightHandle);
      scene.handles.delete(meter.right);
    }
    setScene({ ...scene, meters: scene.meters.filter((candidate) => candidate !== meter) });
  }

  function addMeter(): void {
    if (scene === null) return;
    const index = scene.meters.filter((meter) => meter.orientation === "vertical").length;
    const id = `m${index}-added`;
    scene.handles.set(id, scene.bus.register(id, { mode: MODES[index % MODES.length]! }));
    const vertical = scene.meters.filter((meter) => meter.orientation === "vertical");
    const horizontal = scene.meters.filter((meter) => meter.orientation === "horizontal");
    setScene({ ...scene, meters: [...vertical, { id, right: null, style: "segments", orientation: "vertical", label: `Added ${index + 1}` } as SceneMeter, ...horizontal] });
  }

  if (scene === null) return <Page><Group title="Batched meter surface">building scene…</Group></Page>;

  const vertical = scene.meters.filter((meter) => meter.orientation === "vertical");
  const horizontal = scene.meters.filter((meter) => meter.orientation === "horizontal");

  return <Page>
    <Group title="Batched meter surface">
      <p className="meter-surface-note">
        One <code>MeterBus</code>, one canvas, one frame loop. Standalone evidence stays on the{" "}
        <a href="#components/audio-meter">AudioMeter specimen</a>.
      </p>
      <div className="meter-surface-controls">
        {[8, 32, 128].map((option) => (
          <button key={option} type="button" data-count={option} aria-pressed={count === option} onClick={() => { stopWorkload(); setCount(option); }}>{option} meters</button>
        ))}
        <button type="button" data-part="workload-toggle" onClick={() => (running ? stopWorkload() : startWorkload())}>
          {running ? "Stop workload" : "Start 15 Hz workload"}
        </button>
        <button type="button" data-part="clip-trigger" onClick={() => scene.bus.pushFrames(new Float32Array([scene.bus.slotOf("m0"), 1.2, 1]), performance.now(), DATA_INTERVAL_MS)}>Trigger clip</button>
        <button type="button" data-part="clip-reset" onClick={() => scene.bus.resetClip("m0")}>Reset clip</button>
        <button type="button" data-part="enabled-toggle" onClick={() => { scene.bus.setEnabled("m1", disabledDemo); setDisabledDemo(!disabledDemo); }}>{disabledDemo ? "Enable channel 2" : "Disable channel 2"}</button>
        <button type="button" data-part="meter-remove" onClick={removeLastMeter}>Remove meter</button>
        <button type="button" data-part="meter-add" onClick={addMeter}>Add meter</button>
        <button type="button" data-part="remount-toggle" onClick={() => setMounted(!mounted)}>{mounted ? "Destroy surface" : "Remount surface"}</button>
        <button type="button" data-part="theme-toggle" onClick={() => setThemed(!themed)}>{themed ? "Local theme off" : "Local theme on"}</button>
      </div>
      <pre className="meter-surface-readout" data-part="perf-readout">{readout}</pre>
    </Group>
    <Group title="Console strip">
      <div className="meter-surface-strip-wrap" data-theme={themed ? "midnight" : undefined}>
        {mounted ? <MeterSurface key={scene.version} bus={scene.bus}>
          <div className="meter-surface-strip" data-part="meter-strip">
            {vertical.map((meter) => (
              <AudioMeter key={meter.id} surface={scene.bus} channel={meter.id} rightChannel={meter.right} style={meter.style} segments={12} ariaLabel={meter.label} />
            ))}
          </div>
          <div className="meter-surface-strip meter-surface-strip--horizontal">
            {horizontal.map((meter) => (
              <AudioMeter key={meter.id} surface={scene.bus} channel={meter.id} orientation="horizontal" style={meter.style} segments={12} ariaLabel={meter.label} />
            ))}
          </div>
        </MeterSurface> : <p className="meter-surface-note">Surface destroyed — bus channels stay registered.</p>}
      </div>
      <style>{`
        .meter-surface-note { margin: 0; color: var(--poodle-color-text-secondary); font-size: 0.8125rem; }
        .meter-surface-controls { display: flex; flex-wrap: wrap; gap: 0.5rem; }
        .meter-surface-readout { margin: 0; padding: 0.5rem; border-radius: 0.25rem; background: var(--poodle-color-background-surface); font-size: 0.6875rem; white-space: pre-wrap; }
        .meter-surface-strip-wrap { max-width: 40rem; }
        .meter-surface-strip-wrap .poodle-meter-surface { height: 16rem; }
        .meter-surface-strip { display: flex; align-items: flex-end; gap: 0.75rem; padding: 0.5rem; }
        .meter-surface-strip--horizontal { flex-direction: column; align-items: flex-start; }
      `}</style>
    </Group>
  </Page>;
}
