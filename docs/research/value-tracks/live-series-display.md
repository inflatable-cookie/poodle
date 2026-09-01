# Live series display

Status: research complete; promotion blocked — no named streaming-series
consumer
Created: 2026-09-01
Checked: 2026-09-01
Track: live windowed series vs sparkline, waveform, meter, and chart
Origin: [`docs/roadmaps/g16/041-live-series-display-research.md`](../../roadmaps/g16/041-live-series-display-research.md)
Intake: DesEngs candidate 5, merged in PR #126
Primary lead: [Liveline](https://github.com/benjitaylor/liveline) at published
`0.0.7`
Pinned revision:
[`684021f8d8d79b06ed971da8d96de9a3c8fc174d`](https://github.com/benjitaylor/liveline/tree/684021f8d8d79b06ed971da8d96de9a3c8fc174d)
(2026-03-12). Later `main` tip
[`069899598a11e00094ea1eb6b838404825f828be`](https://github.com/benjitaylor/liveline/commit/069899598a11e00094ea1eb6b838404825f828be)
only updates `.gitignore`.

This is point-in-time research, not a component contract or an implementation
plan. It does not introduce `LiveSeries`, a charting layer, a canvas or path
exception, scrubbing, source edits, or promotion. Merged PR #124
(architecture 012) supplies the motion and capture boundary; it does not
authorize a new rendering capability.

## Evidence labels

- **[VF] Verified fact** — read from the pinned Liveline revision, a cited
  standard, or an exact Poodle path in this worktree.
- **[SAC] Source-author claim** — a performance or product claim made by
  Liveline or its README about itself. Not an independent benchmark.
- **[LF] Local consumer fact** — observed in a named local consumer checkout
  at the recorded SHA.
- **[WI] Worker inference** — a Poodle conclusion from those facts.

## Executive summary

Liveline is a React Canvas2D trading chart: time-stamped points, a scrolling
window, lerp of the live tip, monotone cubic drawing, candlesticks, multi-series
toggles, scrub, particles, and a continuous `requestAnimationFrame` loop. It
is MIT-licensed and useful as a contrast case. It is not a Poodle primitive
shape. [VF]

Poodle already has four different data contracts that can look similar on a
dashboard:

| Surface | Data | Clock | Interaction |
| --- | --- | --- | --- |
| MetricTile sparkline | index-ordered `number[]`, no time | none | none; SVG `aria-hidden` |
| StateTile sparkline slot | host-owned; tile only reserves space | none | host decides |
| Meter / AudioMeter | current level (AudioMeter adds ballistics) | feed + optional surface rAF | none on the meter |
| WaveformDisplay | reduced peak pyramid, ≤4,096 columns | none | cursor and selection |

None of those is a live windowed series. [VF]

Inspectable workstation and admin-dashboard consumers use MetricTile as a
static count tile. None pass `sparklineData`. Loophole streams **levels** into
AudioMeter / MeterSurface, not a time-series polyline. WaveformDisplay has no
consumer use in the g15 roster. [LF]

**Recommendation: reject.** Do not extend MetricTile into a streaming window.
Do not add a Poodle `LiveSeries` primitive, charting layer, or second canvas
exception. Keep the sparkline a static decorative snapshot. Keep live audio on
AudioMeter. If a product later needs a scrolling series or a full chart, that
product owns data, time, downsampling, pause, and the renderer. Promotion
stays closed until a named semantic consumer exists.

Sibling `g16.040` owns whether MetricTile's **value text** may move. This
dossier does not.

## Method and source inventory

### Method

1. Ran worker preflight on the launcher worktree
   `research/g16-041-live-series-display` and loaded the tracked handoff from
   `HEAD`.
2. Read the card, architecture 012, MetricTile / StateTile / WaveformDisplay /
   Meter / AudioMeter / MeterSurface contracts, architecture 001 and 008, spec
   068, and the shared node vocabulary.
3. Audited Poodle sparkline, waveform, meter, and node source in this worktree.
4. Inspected Liveline from the pinned `0.0.7` tree via GitHub (types, engine,
   interpolate, spline, line draw, component defaults, LICENSE). No upstream
   source was copied into Poodle.
5. Inspected real consumer checkouts under `/Users/tom/Dev/projects` for
   MetricTile, sparkline, WaveformDisplay, and live meter use.
6. Separated facts, author claims, and inferences. No spike or benchmark was
   run; numeric budgets below are proposed gates.

### External source inventory

| ID | Direct source | Use | Checked |
| --- | --- | --- | --- |
| L1 | [Liveline repo](https://github.com/benjitaylor/liveline) at [`684021f8d8d79b06ed971da8d96de9a3c8fc174d`](https://github.com/benjitaylor/liveline/tree/684021f8d8d79b06ed971da8d96de9a3c8fc174d) | Primary artifact: types, engine, draw, math, lifecycle | 2026-09-01 |
| L2 | [LICENSE](https://github.com/benjitaylor/liveline/blob/684021f8d8d79b06ed971da8d96de9a3c8fc174d/LICENSE) | MIT, Copyright 2025–2026 Benji Taylor | 2026-09-01 |
| L3 | GitHub repo metadata and [npm `liveline@0.0.7`](https://www.npmjs.com/package/liveline) | Published version, React peer, MIT, homepage | 2026-09-01 |
| S1 | [WCAG 2.2 Pause, Stop, Hide](https://www.w3.org/WAI/WCAG22/Understanding/pause-stop-hide.html) | Looping moving charts | 2026-09-01 |
| S2 | [WCAG 2.3.3 Animation from Interactions](https://www.w3.org/WAI/WCAG22/Understanding/animation-from-interactions.html) | Non-essential motion | 2026-09-01 |
| S3 | [WCAG 1.1.1 Non-text Content](https://www.w3.org/WAI/WCAG22/Understanding/non-text-content.html) | Chart text alternative | 2026-09-01 |
| S4 | MDN [`requestAnimationFrame`](https://developer.mozilla.org/en-US/docs/Web/API/Window/requestAnimationFrame), [`ResizeObserver`](https://developer.mozilla.org/en-US/docs/Web/API/ResizeObserver), [`Page Visibility`](https://developer.mozilla.org/en-US/docs/Web/API/Page_Visibility_API), [`prefers-reduced-motion`](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-reduced-motion) | Frame loop, resize, hidden tabs, reduced motion | 2026-09-01 |

No Liveline source was copied into this repository.

## Liveline at the pinned revision

### Licensing and product shape

MIT. React peer (`>=18` on npm 0.0.7). Canvas-rendered line, multi-series, and
candlestick chart with window pills, badge, momentum, fill, scrub, pulse, and
optional orderbook / “degen” particles. [VF, L2, L3, `src/types.ts`,
`src/Liveline.tsx`]

Author claim: “60fps, zero CSS imports.” [SAC] That is not a Poodle
measurement.

### Data model

`LivelinePoint` is `{ time: number /* unix seconds */, value: number }`. The
host also passes a separate `value` for the live tip. Multi-series adds
`{ id, data, value, color, label? }`. Candles are OHLC plus `liveCandle`.
There is no point id, no gap/NaN encoding, and no downsample field. [VF,
`src/types.ts`]

Default window is 30 seconds. Default `lerpSpeed` is `0.08`. Default `scrub`
is `true`. [VF, `src/Liveline.tsx`]

### Interpolation and drawing

Three different “smooth” operations:

1. **Live value lerp** — `displayValue` eases toward host `value` with
   adaptive speed; a live tip is appended at `now`. [VF,
   `src/useLivelineEngine.ts`, `src/draw/line.ts`]
2. **Time lookup** — `interpolateAtTime` linearly blends two adjacent points
   by unix time (crosshair / hover). Returns null outside the data range. [VF,
   `src/math/interpolate.ts`]
3. **Stroke** — Fritsch–Carlson monotone cubic spline through screen-space
   points so the curve does not overshoot local min/max. [VF,
   `src/math/spline.ts`]

Visible points are every host point whose time falls in `[leftEdge, rightEdge]`
(with a small left pad). There is no LTTB, bucket, or pixel-cap downsample.
[VF, visible-point loops in `src/useLivelineEngine.ts`]

Gaps are not first-class: a missing interval becomes a long spline segment.
[WI from the point type and spline path]

The live tip at wall-clock `now` and the lerped Y are **decorative motion of
data**, not semantic continuity. They invent in-between values and a moving
endpoint between host samples. Architecture 012 forbids motion from owning
correctness. [WI]

### Lifecycle, pause, reduced motion, cleanup

- One `requestAnimationFrame` loop; `dt` clamped to 50 ms. [VF]
- `document.hidden` stops the loop; `visibilitychange` restarts it. [VF]
- `paused` snapshots data so the window does not erode, then eases
  `pauseProgress` and accumulates time debt for catch-up. [VF]
- Ambient `matchMedia('(prefers-reduced-motion: reduce)')` sets `noMotion`,
  which snaps lerps (`speed = 1`). [VF]
- `ResizeObserver` writes size into a ref; canvas backing store follows DPR
  inside the frame. [VF, `src/canvas/dpr.ts`, engine]
- Unmount cancels rAF, disconnects the observer, and removes pointer
  listeners and the badge DOM node. [VF]

Ambient preference discovery is the pattern architecture 012 rejected for
Poodle. Poodle consumes an explicit host `MotionPolicy`. [VF,
`docs/architecture/012-semantic-motion-policy.md`]

### Accessibility

The chart is a canvas plus optional DOM badge, window pills, and value overlay.
There is no series summary, live region, or accessible point table in the
pinned source. Scrub is pointer/touch hover. A continuously scrolling line is
moving content under WCAG 2.2.2; Liveline’s `paused` prop is the author’s
control, not a Poodle policy. [VF/WI, L1, S1, S3]

## Current Poodle audit

### Motion and renderer budget

Architecture 012: default renderer-neutral properties are opacity, translation,
scale, and rotation. Path drawing, gradients, filters, canvas, and 3D need a
**separate role-specific decision** plus static/reduced fallback. Frozen
schedules no visual clock. Capture hosts select `frozen` explicitly. Semantic
state updates immediately. [VF]

`poodle-node` `NodeKind` is Container, Text, Icon, Image, Progress,
ProgressRing, Button, Input. There is no path, polyline, or canvas kind. [VF,
`packages/contracts/node/src/lib.rs`]

MeterSurface is the one named web canvas exception: it paints existing
AudioMeter semantics, stays `aria-hidden`, and does not exist on native
(GPUI already submits meter nodes). Spec 068’s reference budget is 128 meters,
15 Hz feed, 60 Hz paint, p95 &lt; 2 ms on the operator’s mid-range Apple
Silicon Mac. [VF, architecture 008, spec 068, MeterSurface contract]

### MetricTile — static sparkline

Web builds an SVG path from `sparklineData` (`M`/`L`, viewBox `0 0 64 24`,
min/max normalize, 1-decimal). Requires 2+ points. SVG is `aria-hidden`.
Accessible name is `"{label}: {value}"`. No live region, no time, no window,
no pause, no cap. [VF, contract and
`packages/svelte/components/src/MetricTile.svelte`,
`packages/react/components/src/MetricTile.tsx`]

Native `poodle-render` substitutes a **bar strip** of containers (Tier-3). Same
dimensions and tertiary color; not a polyline. [VF,
`packages/render/src/metric_tile.rs`]

That substitution is already an active-cohort visual gap for a static
sparkline. A live polyline would widen it. [WI]

### StateTile — host slot

`hasSparkline` reserves space. Sparkline data is out of contract. Host must
decide decorative vs summary. Native reserves a 2 rem empty slot. [VF,
StateTile contract, `packages/render/src/state_tile.rs`]

### WaveformDisplay — inspector peaks, not a series

Host supplies a peak pyramid. Core picks a level and caps columns at 4,096.
Raw PCM, timeline tiles, streaming caches, and GPU-scene waveforms are
prohibited. Visual is CSS/node columns, not a time-stamped polyline. Cursor
and selection are sample indices. [VF, WaveformDisplay contract,
`packages/core/src/audio/waveform.ts`,
`packages/svelte/components/src/audio/WaveformVisual.svelte`,
`packages/render/src/audio.rs`]

### Meters — current value, not history

`Meter` is a bounded current measurement. `AudioMeter` adds ballistics and a
host feed. Loophole’s console maps transport levels onto one `MeterBus` and
paints through MeterSurface. Duration stamped on live batches is 100 ms. [VF,
Meter / AudioMeter contracts, Loophole `meter-feed.ts`]

That is a live **scalar**, not a windowed series.

## Consumer audit

g15 roster MetricTile consumers:
`acowtancy`, `compli-me`, `composer`, `contact-patch`, `underlay-reference`.
StateTile: no consumer use. WaveformDisplay: no consumer use. AudioMeter:
`loophole`.

| Consumer | Checkout | What was inspected | Streaming points? |
| --- | --- | --- | --- |
| underlay-reference | `135fab45` `main` | Admin dashboard MetricTiles: users, media, registrations, sessions. Counts + trend text. No `sparklineData`. | No |
| contact-patch | `38fb3ad` `main` | Same Underlay admin MetricTile pattern. | No |
| acowtancy | `1c91040dd` `main` | Dairy overview MetricTiles (pathways, modules, media, 7-day changes) and compact list counts. No sparkline. | No |
| compli-me | `4469968` `main` | Health MetricTile (`value={status}`). No sparkline. | No |
| loophole | `e23e03e` `main` | Console `AudioMeter` on MeterSurface; 100 ms level batches. WaveformDisplay unused. WebGL2 waveform painter is Loophole render-bench, not Poodle. | Levels, not series |
| soundcheck, nucleus, finch, longhorn, songsprout | present locally | No MetricTile / sparkline / WaveformDisplay use found. | No |

`composer` is on the roster but was **not** on disk (`/Users/tom/Dev/projects/composer` missing). Other roster members were searchable. Across every inspectable checkout, **`sparklineData` appears only inside Poodle specimens and tests**. [LF]

Answer to the card’s first question: no inspected Poodle consumer currently
needs streaming points rather than a static sparkline or meter. The honest
evidence is absence of demand, not inability to look. [WI]

## Renderer comparison

| Route | What it is here | CPU / memory | Points | Update rate | Frame | Reduced / frozen / a11y |
| --- | --- | --- | --- | --- | --- | --- |
| SVG path (MetricTile web) | Rebuild `d` when the array changes | Cheap at specimen size (≤10 points). Path string grows with n. | Unbounded in contract | Host re-render | No clock | Sparkline stays hidden; value text is the summary. Frozen = last path. |
| Canvas (Liveline / MeterSurface) | Continuous paint | Liveline: one full-size DPR canvas + rAF while visible. MeterSurface: one overlay, culled, cached geometry. | Liveline: all points in window. MeterSurface: N meters × segments, not points. | Liveline: paint 60 Hz regardless of feed. MeterSurface: 15 Hz feed / 60 Hz paint (spec 068). | Must stop on hidden, unmount, frozen | Canvas `aria-hidden`. Host owns names. Liveline has no series summary. |
| Renderer-neutral polyline | Not in `NodeKind` | Would need a new node, tessellation, and GPUI stroke | Would need an explicit cap | Host or shared core | GPUI has no generic path paint today | Must ship a static fallback (bars, last value, or omitted geometry) |
| Static host slot (StateTile) | Host draws or leaves empty | Whatever the host puts there | Host | Host | None in the tile | Host owns decorative vs summary |

Native MetricTile already uses the static-host-like bar strip because the node
vocabulary cannot stroke a path. [VF]

Architecture 012 therefore makes a live polyline a **renderer-capability
programme**, not a MetricTile mode. [WI]

## Who owns which concern

| Concern | Liveline | MetricTile today | If Poodle owned a live series (rejected) | Honest default |
| --- | --- | --- | --- | --- |
| Point identity | None; `{time,value}` | Array index | Would need stable ids or (t,v) uniqueness | Host |
| Time window | Component (`window`, pills) | None | Must not live in a tile sparkline | Host |
| Downsampling | Host; engine draws all visible | None | Must cap before draw | Host, before any renderer |
| Gaps | Straight spline through neighbors | No time, so no gap | Must be explicit (break vs hold) | Host |
| Current value | Host `value` + decorative lerp | Host formatted `value` string | Semantic value immediate; motion optional | Host; g16.040 for text motion |
| Pause | Prop + tab hidden | N/A | Architecture 012 `frozen` / reduced; WCAG pause for looping scroll | Host policy into Poodle motion |
| Scrub | Default on | Out of MetricTile scope | Out of scope here | Product chrome |
| Accessibility summary | Missing | Decorative sparkline; name is label+value | Latest value + window text; visual hidden or described | Host; visual never the only reading |

## Burst, resize, hidden, unmount, narrow

| Case | Liveline | Poodle implication |
| --- | --- | --- |
| Bursty updates | Lerp absorbs jumps; still draws every visible point | Host must bound rate and count. Do not lerp correctness. |
| Gaps | Continuous curve | Reject silent interpolation as data. |
| Resize | Observer, no per-frame layout read | Same rule as MeterSurface. Recompute projection, do not relayout in rAF. |
| Hidden | Stop rAF | Frozen/hidden must cancel clocks. |
| Unmount | Cancel rAF | Same. |
| Narrow | Chart still paints; badge padding changes | MetricTile sparkline is fixed 4×1.5 rem. A live chart at that size is decoration, not a readable series. |
| Reduced | Snap lerps; loop still runs | Poodle reduced: no translation/loop; snap to last committed points. |
| Frozen / capture | Not modeled; ambient media | No rAF. Last window snapshot. Capture selects frozen. |
| Non-visual | None | Latest value + trend/window sentence. Never announce every point. |

## Options

| Option | Meaning | Verdict |
| --- | --- | --- |
| **Extend** MetricTile / StateTile | Streaming window, time, pause, canvas/path inside the tile | **Reject.** Different data contract; sparkline is decorative; native cannot stroke a path; no consumer uses even the static series. |
| **Add** a Poodle `LiveSeries` primitive | New composite + likely canvas/path exception + node kind | **Reject now.** No named consumer, no node capability, overlaps Liveline/trading chrome and full charts. |
| **Consumer-owned** | Product keeps points, window, downsample, and renderer | **Correct ownership if a product need appears.** Not a Poodle card. |
| **Reject** | No Poodle live-series surface | **Chosen.** Keep static sparkline; keep AudioMeter for live levels; keep WaveformDisplay inspector-scale. |

Full charts (axes, scrub, OHLC, multi-series, order book) stay out of Poodle
under product guardrails. [WI, `docs/architecture/product-guardrails.md`]

## Recommendation

1. **Reject** a Poodle live windowed series. Do not name `LiveSeries`. Do not
   open a canvas/path exception for it. Do not add scrubbing.
2. **Do not extend** MetricTile. `sparklineData` remains an optional static
   snapshot. Hosts that need a chart compose StateTile’s slot or their own
   surface.
3. **Do not reuse** WaveformDisplay or AudioMeter as a series widget.
   Waveform is peaks and cursor. AudioMeter is a live scalar with ballistics.
4. **Interpolation** of a live tip is decorative motion. Semantic continuity
   is last committed point plus immediate current value. g16.040 may still
   animate the number; that is not a series.
5. **Active-cohort posture:** until `NodeKind` can describe a bounded polyline
   (or a documented static substitute is accepted as the native result), a
   live series cannot be a complete Poodle component. MeterSurface does not
   generalize to this.

## Proposed budgets (gates, not measurements)

Use these if a later operator-approved consumer appears. They are not current
Poodle guarantees.

| Axis | Proposed bound | Basis |
| --- | --- | --- |
| Point count after downsample | ≤ 256 for a tile; ≤ 1,024 for a panel | Below WaveformDisplay’s 4,096 inspector cap; Liveline has no cap |
| Host update rate | ≤ 15 Hz | Spec 068 meter feed |
| Paint | 0 Hz when reduced/frozen/hidden/unmounted; otherwise ≤ display refresh | Architecture 012; Liveline hidden-tab stop |
| Tile sparkline | Stay static; if ever live, same 4×1.5 rem and decorative a11y | MetricTile contract |
| Frame CPU | Record p95 on the same class of machine as spec 068; do not CI-gate hardware | Spec 068 method |
| Memory | One backing store per surface; no per-point DOM | MeterSurface lesson |
| Accessibility cadence | Current value ≤ 2 Hz; no per-frame announcements | MeterSurface 500 ms aria interval |

Benchmark plan (only after a named consumer and an architecture exception):

1. Workloads: 32 / 256 / 1,024 points; 1 Hz / 15 Hz / bursty 100 Hz host
   (must downsample to the cap); 1 / 8 / 32 instances.
2. Routes: current SVG sparkline rebuild, a canvas polyline, a hypothetical
   node polyline, and a frozen/static snapshot.
3. Cases: burst, gap (break vs hold), resize, `document.hidden`, unmount,
   narrow 4 rem tile, reduced, frozen, capture.
4. Proof: no rAF after hidden/unmount/frozen; last committed points under
   reduced; accessible name equals latest semantic value; visual not required
   to understand the metric.
5. Native: GPUI must paint the same reduced points or a recorded substitute.
   Headless probes only; no `*-windowed` conformance without operator
   approval.
6. Do not treat Liveline’s “60fps” line as a pass bar.

## Promotion gates

Still closed. Opening an implementation card would require all of:

- a named semantic consumer that needs a **window of points**, not a count,
  meter, or waveform;
- accepted ownership (table above) in architecture, not in this dossier;
- numeric budgets measured on the planned renderer;
- an active-cohort rendering posture (new node capability **or** explicit
  native substitute) and a 012 canvas/path exception if canvas is used;
- reduced / frozen / capture / non-visual summaries.

None of those are true today.

## Explicit non-goals

- No `LiveSeries` API, charting package, or vendored Liveline.
- No MetricTile/StateTile/WaveformDisplay/AudioMeter source or contract edits.
- No canvas/path exception, polyline `NodeKind`, or GPUI stroke spike.
- No scrubbing, OHLC, multi-series chrome, order book, or particles.
- No promotion into architecture, specs, roadmaps, or indexes in this PR.
- No Jetstream admission.
- No decision on MetricTile value-text motion (`g16.040`).

## Risks

| Risk | If ignored | Mitigation |
| --- | --- | --- |
| Sparkline looks “close enough” to Liveline | Tile grows a clock, canvas, and invented values | Keep sparkline static and decorative. |
| Reuse WaveformDisplay | Timeline/streaming scope the contract forbids | Leave the 4,096 inspector ceiling alone. |
| Reuse MeterSurface | Mixes ballistics with history | Canvas exception stays audio-meter-only. |
| Native bar strip vs web path | Live series would silently diverge more | Reject until a polyline capability exists. |
| Ambient `prefers-reduced-motion` | Breaks capture and host policy | Poodle motion stays host-explicit. |
| Looping scroll without pause | WCAG 2.2.2 | Frozen/reduced stop the clock; host pause for product charts. |

## Unresolved operator decisions

Only needed if the reject is later reversed:

1. Which product actually needs a window of points, and is that Poodle-shaped
   or application chrome?
2. If Poodle-owned, is the native result a real polyline or a documented bar /
   last-value substitute?
3. Who downsamples, and what is the hard point cap?

Until a consumer is named, there is nothing to decide beyond **reject**.
