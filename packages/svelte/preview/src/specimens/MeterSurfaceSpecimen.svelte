<script lang="ts">
  import { onDestroy } from "svelte";
  import {
    createMeterBus,
    type AudioMeterMode, type MeterBus, type MeterBusChannel, type MeterFrameScheduler,
  } from "@inflatable-cookie/poodle-core";
  import { AudioMeter, MeterSurface } from "@inflatable-cookie/poodle-svelte";
  import SpecimenGroup from "../components/SpecimenGroup.svelte";
  import SpecimenLayout from "../components/SpecimenLayout.svelte";

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

  let count = $state(8);
  let mounted = $state(true);
  let running = $state(false);
  let themed = $state(false);
  let disabledDemo = $state(false);
  let readout = $state("workload idle");
  let sceneVersion = $state(0);

  let samples: number[] = [];
  let frameCount = 0;

  // Wraps requestAnimationFrame so each bus tick measures exactly one frame of
  // bus advance + draw-pass assembly + paint for the performance readout.
  const scheduler: MeterFrameScheduler = {
    request: (callback) => requestAnimationFrame((timeMs) => {
      const startedAt = performance.now();
      callback(timeMs);
      if (running) {
        samples.push(performance.now() - startedAt);
        frameCount += 1;
        if (frameCount % 30 === 0) refreshReadout();
      }
    }),
    cancel: (handle) => cancelAnimationFrame(handle),
  };

  let bus: MeterBus = $state(createMeterBus({ scheduler, initialCapacity: 160 }));
  let meters: SceneMeter[] = $state([]);
  let feed = new Float32Array(0);
  let phases: number[] = [];

  // Keep the handle `register` returns: `unregister` compares handle identity,
  // so a synthesized `{ id, slot }` is (correctly) rejected.
  let channelHandles = new Map<string, MeterBusChannel>();

  function registerSceneChannel(id: string, index: number): void {
    channelHandles.set(id, bus.register(id, { mode: MODES[index % MODES.length]! }));
  }

  function buildScene(nextCount: number): void {
    bus.destroy();
    bus = createMeterBus({ scheduler, initialCapacity: 160 });
    channelHandles = new Map();
    const next: SceneMeter[] = [];
    for (let index = 0; index < nextCount; index += 1) {
      const id = `m${index}`;
      const stereo = index % 8 === 7;
      registerSceneChannel(id, index);
      if (stereo) registerSceneChannel(`${id}R`, index + 1);
      next.push({
        id,
        right: stereo ? `${id}R` : null,
        style: index % 5 === 0 ? "bar" : "segments",
        orientation: "vertical",
        label: `Channel ${index + 1}`,
      });
    }
    for (let index = 0; index < 4; index += 1) {
      const id = `h${index}`;
      registerSceneChannel(id, index);
      next.push({
        id,
        right: null,
        style: index % 2 === 0 ? "bar" : "segments",
        orientation: "horizontal",
        label: `Horizontal ${index + 1}`,
      });
    }
    meters = next;
    rebuildFeed();
    samples = [];
    frameCount = 0;
    sceneVersion += 1;
  }

  function rebuildFeed(): void {
    const ids = activeChannelIds();
    feed = new Float32Array(ids.length * 3);
    phases = ids.map((_, index) => (index * 37) % 96);
  }

  function activeChannelIds(): string[] {
    const ids: string[] = [];
    for (const meter of meters) {
      ids.push(meter.id);
      if (meter.right !== null) ids.push(meter.right);
    }
    return ids;
  }

  let dataFrame: number | null = null;
  let lastDataAt = 0;

  function dataTick(timeMs: number): void {
    dataFrame = requestAnimationFrame(dataTick);
    if (timeMs - lastDataAt < DATA_INTERVAL_MS) return;
    lastDataAt = timeMs;
    const ids = activeChannelIds();
    for (let index = 0; index < ids.length; index += 1) {
      const phase = phases[index]! + timeMs / 900;
      const level = 0.12 + 0.8 * Math.abs(Math.sin(phase)) * (0.6 + 0.4 * Math.abs(Math.sin(phase * 0.31)));
      const peak = Math.min(level, 0.98);
      feed[index * 3] = bus.slotOf(ids[index]!);
      feed[index * 3 + 1] = peak;
      feed[index * 3 + 2] = peak * peak * 0.55;
    }
    bus.pushFrames(feed, timeMs, DATA_INTERVAL_MS);
  }

  function refreshReadout(): void {
    const measured = samples.slice(WARMUP_FRAMES);
    if (measured.length === 0) {
      readout = "collecting warm-up frames";
      return;
    }
    const sorted = [...measured].sort((a, b) => a - b);
    const pick = (q: number) => sorted[Math.min(Math.floor(q * sorted.length), sorted.length - 1)]!;
    readout = JSON.stringify({
      meters: count,
      channels: activeChannelIds().length,
      warmupFrames: WARMUP_FRAMES,
      sampleCount: measured.length,
      meanMs: Number((measured.reduce((sum, value) => sum + value, 0) / measured.length).toFixed(4)),
      p50Ms: Number(pick(0.5).toFixed(4)),
      p95Ms: Number(pick(0.95).toFixed(4)),
      maxMs: Number(sorted[sorted.length - 1]!.toFixed(4)),
      dpr: typeof devicePixelRatio === "number" ? devicePixelRatio : 1,
    });
  }

  function setCount(nextCount: number): void {
    stopWorkload();
    count = nextCount;
    buildScene(nextCount);
  }

  function startWorkload(): void {
    if (running) return;
    samples = [];
    frameCount = 0;
    running = true;
    lastDataAt = 0;
    dataFrame = requestAnimationFrame(dataTick);
    readout = "collecting warm-up frames";
  }

  function stopWorkload(): void {
    running = false;
    if (dataFrame !== null) cancelAnimationFrame(dataFrame);
    dataFrame = null;
    refreshReadout();
  }

  function triggerClip(): void {
    const scratch = new Float32Array([bus.slotOf("m0"), 1.2, 1]);
    bus.pushFrames(scratch, performance.now(), DATA_INTERVAL_MS);
  }

  function resetClip(): void {
    bus.resetClip("m0");
  }

  function toggleEnabled(): void {
    disabledDemo = !disabledDemo;
    bus.setEnabled("m1", !disabledDemo);
  }

  function removeLastMeter(): void {
    const meter = [...meters].reverse().find((candidate) => candidate.orientation === "vertical");
    if (meter === undefined || meters.filter((candidate) => candidate.orientation === "vertical").length <= 1) return;
    const leftHandle = channelHandles.get(meter.id);
    const rightHandle = meter.right === null ? undefined : channelHandles.get(meter.right);
    if (leftHandle !== undefined) {
      bus.unregister(leftHandle);
      channelHandles.delete(meter.id);
    }
    if (rightHandle !== undefined && meter.right !== null) {
      bus.unregister(rightHandle);
      channelHandles.delete(meter.right);
    }
    meters = meters.filter((candidate) => candidate !== meter);
    rebuildFeed();
  }

  function addMeter(): void {
    const index = meters.filter((candidate) => candidate.orientation === "vertical").length;
    const id = `m${index}-added`;
    registerSceneChannel(id, index);
    meters = [...meters.filter((candidate) => candidate.orientation === "vertical"), {
      id, right: null, style: "segments", orientation: "vertical", label: `Added ${index + 1}`,
    }, ...meters.filter((candidate) => candidate.orientation === "horizontal")];
    rebuildFeed();
  }

  buildScene(count);

  onDestroy(() => {
    stopWorkload();
    bus.destroy();
  });
</script>

<SpecimenLayout variantDirection="row">
  <div class="page">
    <SpecimenGroup label="Batched meter surface">
      <p class="note">
        One <code>MeterBus</code>, one canvas, one frame loop. Standalone evidence stays on the
        <a href="#components/audio-meter">AudioMeter specimen</a>.
      </p>
      <div class="controls">
        {#each [8, 32, 128] as option}
          <button type="button" data-count={option} aria-pressed={count === option} onclick={() => setCount(option)}>{option} meters</button>
        {/each}
        <button type="button" data-part="workload-toggle" onclick={() => (running ? stopWorkload() : startWorkload())}>
          {running ? "Stop workload" : "Start 15 Hz workload"}
        </button>
        <button type="button" data-part="clip-trigger" onclick={triggerClip}>Trigger clip</button>
        <button type="button" data-part="clip-reset" onclick={resetClip}>Reset clip</button>
        <button type="button" data-part="enabled-toggle" onclick={toggleEnabled}>{disabledDemo ? "Enable channel 2" : "Disable channel 2"}</button>
        <button type="button" data-part="meter-remove" onclick={removeLastMeter}>Remove meter</button>
        <button type="button" data-part="meter-add" onclick={addMeter}>Add meter</button>
        <button type="button" data-part="remount-toggle" onclick={() => (mounted = !mounted)}>{mounted ? "Destroy surface" : "Remount surface"}</button>
        <button type="button" data-part="theme-toggle" onclick={() => (themed = !themed)}>{themed ? "Local theme off" : "Local theme on"}</button>
      </div>
      <pre class="readout" data-part="perf-readout">{readout}</pre>
    </SpecimenGroup>

    <SpecimenGroup label="Live meter strip">
      <div class="strip-wrap" data-theme={themed ? "midnight" : undefined}>
        {#if mounted}
          {#key sceneVersion}
            <MeterSurface {bus}>
              <div class="strip" data-part="meter-strip">
                {#each meters.filter((meter) => meter.orientation === "vertical") as meter (meter.id)}
                  <AudioMeter surface={bus} channel={meter.id} rightChannel={meter.right} style={meter.style} segments={12} ariaLabel={meter.label} />
                {/each}
              </div>
              <div class="strip strip--horizontal">
                {#each meters.filter((meter) => meter.orientation === "horizontal") as meter (meter.id)}
                  <AudioMeter surface={bus} channel={meter.id} orientation="horizontal" style={meter.style} segments={12} ariaLabel={meter.label} />
                {/each}
              </div>
            </MeterSurface>
          {/key}
        {:else}
          <p class="note">Surface destroyed — bus channels stay registered.</p>
        {/if}
      </div>
    </SpecimenGroup>
  </div>
</SpecimenLayout>

<style>
  .page { display: grid; gap: 1.5rem; }
.note { margin: 0; color: var(--poodle-color-text-secondary); font-size: 0.8125rem; }
  .controls { display: flex; flex-wrap: wrap; gap: 0.5rem; }
  .readout { margin: 0; padding: 0.5rem; border-radius: 0.25rem; background: var(--poodle-color-background-surface); font-size: 0.6875rem; white-space: pre-wrap; }
  .strip-wrap { max-width: 40rem; }
  .strip-wrap :global(.poodle-meter-surface) { height: 16rem; }
  .strip { display: flex; align-items: flex-end; gap: 0.75rem; padding: 0.5rem; }
  .strip--horizontal { flex-direction: column; align-items: flex-start; }
</style>
