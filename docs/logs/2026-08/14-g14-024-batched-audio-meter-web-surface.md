# 14 — g14.024 Batched Audio Meter Web Surface (batch log)

Branch: `t3code/batched-audio-meter-web-surface`
Date: 2026-08-14
Base SHA: `caaf983a`
Card: `docs/roadmaps/g14/024-batched-audio-meter-web-surface.md`
Contracts: `docs/contracts/components/audio-meter.md`,
`docs/contracts/components/meter-surface.md` (new), spec 068

One opt-in web meter tier: `MeterBus` batches every registered channel through
the same pure scalar laws as standalone `audioMeterTransition`; a shared DOM
controller projects surface-mode `AudioMeter` placeholders onto one Canvas2D
overlay per scroll container. Standalone AudioMeter markup, defaults,
behavior, and accessibility are unchanged — the standalone Svelte/React
component tests and core goldens pass untouched except for new coverage.

## API

- `poodle-core`: `createMeterBus` (`register`/`unregister`/`pushFrames`
  `[slot, peak, meanSquare]` triples + batch `atMs`/`durationMs`,
  `resetClip`, `setEnabled`, `slotOf`, `subscribe`, `destroy`, stable `view`
  descriptor over structure-of-arrays buffers), `createManualMeterFrameScheduler`,
  `createAnimationFrameMeterScheduler`, exported pure scalar laws
  (`meterVuStepDb`, `meterPpmStepDb`, `meterSamplePeakStepDb`,
  `meterPeakHoldDbStep`/`meterPeakHoldDecayDb`, `meterWeightedRmsDb`,
  `isMeterFrameValid`, `meterElapsedMs`, `meterInputDb`, `meterClampDb`,
  `meterClipStep`), `createMeterSurfaceController`,
  `createMeterSurfaceRegistry`, `createCanvas2dMeterSurfacePainter`,
  `MeterSurfacePainter`/`MeterDrawPass`/`MeterSurfacePalette` seam,
  `styles/meter-surface.css`.
- `poodle-svelte` / `poodle-react`: `MeterSurface` wrapper (one scroll
  container + one aria-hidden canvas + registration context); `AudioMeter`
  gains `surface`/`channel`/`rightChannel` exactly as contracted. Surface mode
  renders the existing root layout/a11y box only; `push`/`resetClip` forward
  through one reusable per-instance `Float32Array(3)` scratch.

Decisions worth review attention:

- RMS rings are per-channel preallocated (`rmsRingCapacity`, default 64) with
  an explicit `minFrameDurationMs` (default 5 ms) constructor limit; the
  constructor rejects combinations that cannot cover the 300 ms window plus
  one slice, and `pushFrames` rejects batches below the minimum duration
  instead of silently approximating eviction. 10–15 Hz feeds sit far inside
  the default limits.
- Idle time steps (60 Hz paint between 15 Hz data) hold the last quantized
  input as the ballistic target: VU/PPM keep integrating toward the held
  signal, sample-peak decays no lower than the held input, RMS repaints the
  window result. A push after idle advancement resumes from the advanced
  clock. Push-only sequences reproduce `audioMeterTransition` exactly
  (tolerance 1e-12 in the parity suite).
- Placeholder geometry is measured by injecting a transient standalone-anatomy
  skeleton per channel on cold measurement passes, so the canvas reproduces
  the real stylesheet's track/clip layout instead of a parallel layout model.
- The painter probes recipe/status tokens through hidden computed-style probe
  spans (never in the frame loop) and rebuilds its two unit-space bar
  gradients once per palette change.
- A pre-existing React `AudioMeter` defect surfaced: the per-render
  `context = createAudioMeterContext()` default retriggered the context-sync
  effect on every re-render, spinning the component the first time anything
  else set state. The default is now created once per instance; explicit
  `context` props behave as before.

## Allocation Evidence

`packages/core/test/audio-meter-bus.test.ts` ("warm pushes and idle advances
do not grow the heap"): 128 channels, 200 warm-up batches, then 2,000 batches
of one `pushFrames` (128 triples) plus three idle advances each, `Bun.gc(true)`
before/after — heap growth under 256 KiB for 8,000 hot-path operations over
128 channels (any per-frame/per-channel allocation would cost megabytes). Hot
paths use preallocated SoA buffers, ring indices, and one reused draw pass;
registration/growth/destroy remain the only allocating paths by construction.

## Browser Matrix

`effigy test:meter-surface-browser` — headless Chromium and WebKit over the
Svelte and React previews (`test/visual/meter-surface-probe.ts`), reading
paint truth back from the overlay canvas via `getImageData`:

- canvas/placeholder geometry, DPR backing-store sizing (including
  `deviceScaleFactor: 2` contexts), aria-hidden canvas, zero visual DOM
- 128-meter scene: 132 lightweight roots, no segment DOM, one canvas
- track pixels painted and gap pixels clear across six repeated scroll offsets
- culled meter aria continues sampling from live ballistics under workload
- live theme palette change repaints without canvas remount
- clip latch changes the painted lamp; host reset restores it
- add/remove registration; destroy/remount; window resize backing-store retune

Result: 84/84 checks pass — 21 per browser/framework section plus one DPR=2
check each, across `chromium/svelte`, `chromium/react`, `webkit/svelte`, and
`webkit/react`. WebKit needed no geometry, palette, or alignment concessions
relative to Chromium.

Harness note: the probe takes `--browser=chromium|webkit` and Effigy runs the
two engines as separate invocations (`test:meter-surface-browser` composes
`-chromium` then `-webkit`). A single process driving both engines in sequence
wedged locally on a long-lived driver session; per-engine runs plus a 240 s
section watchdog, a 15 s rAF watchdog, and one fresh-browser retry keep the
matrix bounded. WebKit also never fires `load`/`networkidle` against the vite
dev previews, so navigation waits on `domcontentloaded` plus the canvas
selector.

## Performance

Reference workload per spec 068: 128 meters, 12 segments, 15 Hz data, 60 Hz
paint, culling enabled; the specimen's wrapped frame scheduler measures bus
advance + draw-pass assembly + paint per frame; 60-frame warm-up excluded;
~20 s measurement (`effigy test:meter-surface-perf`).

Environment: Apple M5 Max, 128 GB RAM, macOS 26.5.2, headless Playwright
Chromium and WebKit 26.5, DPR 1, 1280×900 viewport.

| Browser | Preview | Samples | Mean | p50 | p95 | Max |
| --- | --- | --- | --- | --- | --- | --- |
| Chromium | Svelte | 2,343 | 0.166 ms | 0.2 ms | 0.3 ms | 2.1 ms |
| Chromium | React | 2,343 | 0.162 ms | 0.2 ms | 0.3 ms | 0.8 ms |
| WebKit | Svelte | 1,145 | 0.429 ms | 0 ms | 1 ms | 2 ms |
| WebKit | React | 1,145 | 0.379 ms | 0 ms | 1 ms | 2 ms |

All 128 vertical meters plus the page's 4 horizontal examples register 148
channels (stereo on every eighth meter); culling keeps the painted subset to
what the strip viewport shows.

Acceptance uses p95: Chromium 0.3 ms and WebKit 1 ms, both under the 2 ms
reference target. Canvas2D meets the requirement with substantial headroom, so
no WebGL2 follow-up is justified by this evidence.

Read the WebKit percentiles as coarse: WebKit clamps `performance.now()` to
1 ms resolution, so its per-frame deltas quantize to 0/1/2 ms and only the
mean (0.38–0.43 ms, averaged over ~1,100 frames) carries sub-millisecond
information. Its lower sample count comes from the same 20 s window yielding
fewer measured frames. Chromium's finer clock gives the more precise picture;
both agree the frame cost sits well below budget. Max values include the
occasional scheduling outlier, which is why acceptance reads p95.

These are single-machine review evidence, not a CI threshold.

## Validation

- `effigy test:core`, `effigy test:components` (core 749; components 1,338
  across six vitest projects), `effigy test:parity`, `effigy test:a11y`
- `effigy check:svelte` (0 errors), `effigy docs:lint`, `effigy docs:check`
- `effigy docs:callback-drift` green; `effigy docs:contract-drift` reports
  only the recorded main baseline (Button `children`/`leading`/`trailing`) —
  no AudioMeter or MeterSurface drift
- `effigy test:web-pack-install` green with new packed fixtures importing
  `MeterBus` and both `MeterSurface` exports from public entries
- `effigy ci:web`, `git diff --check`
- No source or test imports Loophole; no Loophole domain type in public API

## Files

### Batch A — shared scalar math and MeterBus

| File | What |
| --- | --- |
| `packages/core/src/audio/meter.ts` | extracted exported pure scalar laws; transition routed through them; goldens unchanged |
| `packages/core/src/audio/meter-bus.ts` | new SoA batch engine, feed validation, RMS rings, schedulers, one loop |
| `packages/core/test/audio-meter-bus.test.ts` | new goldens, validation, parity (1e-12), idle-step reference, lifecycle, allocation probe |

### Batch B — DOM controller and Canvas2D painter

| File | What |
| --- | --- |
| `packages/core/src/dom/meter-surface.ts` | new controller + registry: geometry cache, scroll projection, DPR, palette probes, culling, aria cadence |
| `packages/core/src/dom/meter-surface-painter.ts` | new default Canvas2D painter over the flat draw pass |
| `packages/core/src/styles/meter-surface.css` | new surface sheet |
| `packages/core/src/styles/audio-meter.css` | surface-mode root sizing rules only (`[data-surface]`) |
| `test/headless-dom/meter-surface.test.ts` | new controller tests (queueing, culling, cadence, theme re-probe, lifecycle) |

### Batch C — framework surfaces

| File | What |
| --- | --- |
| `packages/svelte/components/src/MeterSurface.svelte`, `meter-surface-context.ts` | new wrapper + context |
| `packages/svelte/components/src/AudioMeter.svelte` | surface props; standalone path unchanged |
| `packages/react/components/src/MeterSurface.tsx` | new wrapper + context |
| `packages/react/components/src/AudioMeter.tsx` | surface props; default-context re-render fix |
| `packages/{svelte,react}/components/src/index.ts` | `MeterSurface` exports |
| `packages/{svelte,react}/components/test/MeterSurface.*` | new component suites + Svelte harness |

### Batch D — specimens, probes, evidence, packaging

| File | What |
| --- | --- |
| `packages/svelte/preview/src/specimens/MeterSurfaceSpecimen.svelte`, `packages/react/preview/src/gallery/specimens/MeterSurfaceSpecimen.tsx` | new 8/32/128 workload pages with perf readout; linked with the standalone AudioMeter page |
| `packages/svelte/preview/src/{component-registry,component-docs,parity}.ts`, specimen registries | catalogue registration |
| `docs/contracts/components/meter-surface.md`, `audio-meter.md`, contract READMEs | contract updates; planned props now public |
| `packages/svelte/preview/scripts/contract-spec-drift.ts` | web-only prop exemptions for the surface tier |
| `test/visual/meter-surface-probe.ts`, `meter-surface-perf.ts`, `tasks/effigy.tasks.toml` | headless browser matrix (per-engine selectors) + perf selector |
| `test/package-install/fixture/MeterSurface*`, `web-preview.ts` | packed-consumer proof |
| `test/visual/config.ts` | visual-gate skip (animating canvas page) |

## Screenshots

- `docs/logs/assets/g14-024-meter-surface-svelte.png`
- `docs/logs/assets/g14-024-meter-surface-react.png`

## Residual Risk

- Recipe overrides that replace `--poodle-recipe-audio-meter-bar-fill` with an
  arbitrary CSS gradient string cannot be reproduced pixel-exactly on canvas;
  the painter rebuilds the documented three-stop gradient from probed status
  colors. Segment/track/peak/clip recipe colors are reproduced exactly.
- The surface tier trusts hosts to keep bus registration ahead of placeholder
  mount; failures throw development errors rather than falling back, per spec.
- Performance numbers are single-machine review evidence; other hardware
  re-runs `effigy test:meter-surface-perf`. WebKit's 1 ms clock quantization
  means a regression between roughly 0.5 ms and 1 ms would not move its p95;
  Chromium's finer clock is the sensitive signal for future comparisons.
- The browser probe drives dev-server previews, so it depends on vite start-up
  and Playwright browser launches; it is deliberately outside `ci:web`, like
  the existing visual gate.
