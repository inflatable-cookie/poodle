# g14.024 — Batched Audio Meter Web Surface

Status: ready — independent web-performance lane
Depends on: approved spec 068 and the AudioMeter contract amendment
Governing refs: `../../architecture/006-headless-core-and-machine-model.md`,
`../../architecture/008-audio-control-family.md`,
`../../specs/068-batched-audio-meter-surface.md`,
`../../contracts/components/audio-meter.md`

## Outcome

Ship the opt-in high-count web meter tier: one `MeterBus`, layout/a11y-only
`AudioMeter` placeholders, and one Canvas2D `MeterSurface` per scroll
container. Preserve standalone AudioMeter exactly as the default. Prove shared
ballistics, no-allocation hot paths, 128-meter performance, and Chromium plus
WebKit behavior in both web runtimes.

This card may run beside the g14 conformance sequence. It does not touch the
conformance corpus, generated interfaces, Rust, GPUI, Jetstream, or the active
Tabs/headless-GPUI branches. Rebase package indexes, preview registries, and
generated web reports before review.

## Goals

- [ ] Update 8, 32, or 128 meter channels through one typed-array bus push.
- [ ] Produce the same per-mode math as standalone AudioMeter for identical
      explicit push/time sequences.
- [ ] Render every visible placeholder through one canvas and one frame loop.
- [ ] Preserve standalone markup, behavior, accessibility, and default API.
- [ ] Prove live theme, resize, scroll, DPR, clip reset, and lifecycle behavior
      in Chromium and WebKit.
- [ ] Record the 128-meter p95 against the `<2 ms/frame` reference target.

## Execution Plan

### Batch A — Shared scalar math and MeterBus

- [ ] Refactor `packages/core/src/audio/meter.ts` only far enough to expose
      pure scalar operations shared by standalone transitions and the bus.
      Preserve every existing standalone golden value.
- [ ] Add `packages/core/src/audio/meter-bus.ts` with the exact channel, feed,
      validation, structure-of-arrays, view, lifecycle, and RMS-ring rules in
      spec 068.
- [ ] Keep opaque public IDs outside the hot buffer. `pushFrames` consumes
      `[slot, peak, meanSquare]` triples plus explicit `atMs` and `durationMs`.
- [ ] Use typed buffers appropriate to precision and flags. Do not force
      timestamps, booleans, or exact shared math into Float32 for cosmetic
      uniformity.
- [ ] Allow allocation on register, unregister, capacity growth, and destroy.
      After warm-up, push, time advance, view reads, draw-pass assembly, and
      paint allocate nothing.
- [ ] Use one injected frame scheduler with a deterministic manual test
      implementation. Do not create one rAF or interval per channel.
- [ ] Add golden tests for VU, PPM attack/release, sample peak, RMS window,
      peak-hold/decay, clip/reset, stale/invalid frames, duplicates, unregister,
      capacity growth, RMS-ring boundary, and destroy.
- [ ] Compare bus and standalone results from the same Float32-quantized input
      sequence using one documented tolerance. Include explicit idle/time
      steps; do not compare only static end fixtures.

### Batch B — Shared DOM controller and Canvas2D painter

- [ ] Add a focused `packages/core/src/dom/meter-surface.ts` controller. It
      owns the canvas, one ResizeObserver, scroll projection, DPR backing-store
      sizing, theme observation, palette probes, culling, frame subscription,
      and cleanup.
- [ ] Define an injectable `MeterSurfacePainter` over one preallocated flat
      draw pass. Keep setup/resize/paint/destroy lifecycle independent of
      Canvas2D, but do not generalize it into an asset or plugin runtime.
- [ ] Implement the default Canvas2D painter. Reproduce bars, segments,
      orientation, mono/stereo layout, peak hold, clip, enabled opacity, size,
      density, and recipe colors.
- [ ] Resolve computed colors only on mount or palette invalidation. Use probe
      elements to normalize CSS/color-mix/recipe output; never parse CSS or call
      `getComputedStyle` in the frame loop.
- [ ] Cache placeholder geometry in content coordinates. Scroll redraws from
      cached geometry and current offsets; it does not remeasure every meter.
- [ ] Cull outside the viewport without unregistering or pausing bus state.
- [ ] Make the overlay canvas non-interactive and accessibility-hidden.

### Batch C — Thin Svelte and React surfaces

- [ ] Export matching `MeterSurface` wrappers from both web packages. The
      wrapper owns one scroll container/content region and provides a scoped
      registration context for descendant placeholders.
- [ ] Extend both `AudioMeter` implementations with `surface`, `channel`, and
      `rightChannel` exactly as contracted. Surface mode emits only the existing
      root layout/a11y box and no `AudioMeterVisual` descendants.
- [ ] Enforce one authority: registered bus slots in surface mode; contexts in
      standalone mode. Missing registration, wrong bus, or missing matching
      surface fails clearly in development instead of silently changing tier.
- [ ] Forward existing `push` and `resetClip` handles through reusable scratch
      storage in surface mode. No per-call Float32Array allocation.
- [ ] Update surface aria values immediately and at no more than 2 Hz through
      one shared cadence. Match standalone mono/stereo formatting.
- [ ] Keep component-specific framework code to lifecycle, refs, context, and
      reactive aria exposure. Geometry, palette, painter, and bus policy stay
      shared.
- [ ] Add focused component tests proving standalone DOM is unchanged, surface
      DOM has no visual children, registration cleanup works, stereo maps the
      correct slots, handles forward, and ARIA refreshes without N timers.

### Batch D — Specimens, browser proof, performance, and packaging

- [ ] Add matching Svelte and React `MeterSurface` specimen pages with 8, 32,
      and 128 meters. Include all four modes across the suite, bar and segment
      styles, mono and stereo, vertical and horizontal examples, live theme
      switching, a constrained scrolling strip container, offscreen culling,
      clip latch/reset, registration/unregistration, and destroy/remount.
- [ ] Keep the existing standalone AudioMeter specimen and its outputs intact.
      Link the two pages; do not replace the standalone evidence.
- [ ] Add a deterministic workload control for 15 Hz data plus 60 Hz paint and
      a performance readout that reports warm-up, sample count, mean, p50, p95,
      and max.
- [ ] Add one focused real-browser script and Effigy selector that exercises
      both previews in Chromium and WebKit. Prove scroll alignment after
      repeated scroll, resize, DPR emulation where supported, live theme
      palette change, add/remove, clip reset, and canvas/placeholder geometry.
- [ ] Keep the browser selector headless. Do not use the foreground GPUI or
      native visual runners.
- [ ] Record the reference performance run on the operator Mac for both
      browsers. Acceptance target: 128 meters, 12 segments, 15 Hz data, 60 Hz
      paint, p95 below 2 ms for bus advance + draw-pass assembly + paint.
- [ ] Register exports, component docs, contract drift, parity/a11y evidence,
      package entry points, and packed-install proof. Add one August
      implementation log with API, allocation evidence, browser matrix,
      performance environment/results, screenshots, and residual risk.

## Fixed Decisions

- Canvas2D ships first. Do not add WebGL2, an `auto` backend choice, or a
  half-built GPU abstraction in this card.
- `MeterSurface` is a web rendering coordinator. Do not add it to Rust specs,
  the portable component inventory, GPUI, Jetstream, or g14 conformance.
- AudioMeter remains the semantic component. Surface mode changes paint
  ownership, not meter meaning.
- Host data is aggregate UI-cadence telemetry. No raw PCM, audio callback,
  worker, worklet, SharedArrayBuffer, or synchronization protocol enters
  Poodle.
- Public IDs are opaque. The bus returns numeric slots for the hot feed; it
  never asks hosts to coerce product IDs into floats.
- The feed uses mean square and explicit duration. `rms` amplitude without its
  square is not wire-compatible with existing meter math.
- One bus owns one animation loop and may serve more than one surface. One
  surface owns exactly one scroll container and canvas.
- Hidden or culled meters continue ballistics and accessibility sampling.
- Performance measurement is review evidence, not a hardware-sensitive broad
  CI threshold.
- The painter seam is deliberately small and meter-named. General plugin
  custom-draw/asset architecture waits for a second real consumer.

## Acceptance Criteria

- [ ] Existing standalone core and Svelte/React AudioMeter tests pass without
      snapshot, anatomy, or public-default change.
- [ ] Every mode matches standalone scalar output for the same push/time trace,
      including RMS eviction, peak hold, decay, and clip reset.
- [ ] A planted change to any shared constant or scalar law fails both
      standalone and bus parity evidence.
- [ ] Warm hot paths create no arrays, objects, closures, typed arrays, or
      framework state objects per channel or frame.
- [ ] One 128-meter specimen contains 128 lightweight meter roots, no segment
      DOM, one canvas, one ResizeObserver, and one animation loop.
- [ ] Scroll and resize keep canvas paint aligned with placeholders without
      per-frame layout reads; offscreen meters are not painted.
- [ ] Theme changes replace canvas colors without remount and without
      per-frame computed-style reads.
- [ ] Surface-mode meter ARIA matches standalone at the same sampled bus state;
      canvas contributes no accessible node.
- [ ] Chromium and WebKit pass the same headless browser behavior matrix for
      Svelte and React.
- [ ] The recorded reference run meets `<2 ms/frame` at p95 or the card stops
      with the exact profile and bottleneck instead of claiming completion.
- [ ] Packed consumers import `MeterBus` and both framework `MeterSurface`
      exports from public package entries.
- [ ] No source or test imports Loophole, and no Loophole domain type appears in
      a public Poodle API.

## Stop Conditions

- Matching the bus requires changing existing standalone meter outputs,
  constants, defaults, markup, or accessibility semantics.
- Exact RMS behavior cannot fit a declared fixed ring under the 10–15 Hz feed
  contract without hot-path allocation or silent approximation.
- The implementation needs string IDs, Maps, object snapshots, array spreads,
  `Array.from`, framework renders, layout reads, or computed-style reads in the
  per-frame/per-channel path.
- Svelte and React need different bus, geometry, palette, or painter semantics.
- WebKit cannot keep overlay geometry or theme colors aligned with Chromium.
- Canvas2D misses the p95 budget after bounded profiling. Stop with evidence;
  do not silently shrink the workload or widen into WebGL2.
- A generic plugin renderer, asset runtime, mixer shell, transport adapter,
  gain-reduction bus, native port, conformance case, or release workflow enters
  the diff.

## Writable Scope

- `docs/contracts/components/audio-meter.md` and spec 068 only for discovered
  contradictions; do not redesign the approved API silently
- `packages/core/src/audio/meter.ts`, new focused meter-bus modules, exports,
  and tests
- focused `packages/core/src/dom/` meter-surface controller/painter modules
- shared `packages/core/src/styles/audio-meter.css` and a focused surface sheet
  if separation is clearer
- Svelte/React AudioMeter, new MeterSurface wrappers, indexes, component docs,
  and focused tests
- matching Svelte/React specimens and existing preview/report registries
- focused headless Chromium/WebKit browser probe plus Effigy task wiring
- public package docs/exports and packed-install proof
- one August implementation log and append-only `PAPERCUTS.md`

Do not edit architecture, unrelated component contracts, g14 status,
conformance/codegen, Rust/native sources, Jetstream, Loophole, release
workflows, or broad visual baselines.

## Validation

Use Effigy. Keep every browser and repository check headless:

- focused MeterBus/core tests and allocation probes
- focused Svelte and React AudioMeter/MeterSurface tests
- the new Chromium + WebKit meter-surface browser selector
- `effigy test:core`
- `effigy test:components`
- `effigy test:parity`
- `effigy test:a11y`
- `effigy check:svelte`
- `effigy docs:contract-drift`
- `effigy docs:callback-drift`
- `effigy test:web-pack-install`
- `effigy ci:web`
- `effigy docs:check`
- `git diff --check`

Do not run `test:native-visual`, `ci:conformance-windowed`, or any other
foreground/windowed selector.

## Completion Protocol

Open a PR from the dedicated worktree. Do not mark this roadmap complete or
edit dispatch state. The orchestrator reviews source, browser evidence,
allocation proof, performance numbers, generated reports, and packed install;
records corrections; merges; then decides whether Canvas2D closes the Loophole
need or measured evidence justifies a separate WebGL2 card.
