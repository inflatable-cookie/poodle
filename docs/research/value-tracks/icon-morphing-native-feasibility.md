# Value Track: Native icon morphing feasibility

Status: spike complete; native admission not established
Created: 2026-09-01
Checked: 2026-09-01
Track: Poodle-owned icon morphing
Origin: [`docs/handoffs/20260901-221757-icon-morph-native-feasibility.md`](../../handoffs/20260901-221757-icon-morph-native-feasibility.md)
Snapshot under test: `d1d819f3e34274e71d01b3f791257e17420b6654`

This is a bounded feasibility record, not a component contract or an
implementation plan. It does not authorize a public `IconMorph`, a change to
`Icon`, a dependency, a new icon provider, or a merge.

## Evidence labels

- **[VF] Verified fact** — observed in the pinned Poodle checkout, generated
  assets, GPUI 0.2.2 source, or the disposable run.
- **[M] Measurement** — output from the disposable headless run. It is not a
  display or GPU benchmark.
- **[WI] Worker inference** — a conclusion drawn from the verified facts and
  current contracts.
- **[LIM] Evidence limit** — a question this spike deliberately cannot settle.

## Decision

The low-level GPUI route exists: a custom element can build a new tessellated
`Path<Pixels>` in each paint pass and submit it with `Window::paint_path`. The
spike rebuilt 61 distinct intermediate geometries between a current Poodle
pair and exercised both stroke and fill path construction. [VF] [M]

That does not make native icon morphing admissible in Poodle today. The current
shared node carries only a named icon asset, and the GPUI adapter resolves that
name to `assets/icons/<name>.svg`. The existing node animation vocabulary has
opacity and transform properties, not path geometry. [VF] from the
[`Icon` node contract](../../../packages/contracts/node/src/lib.rs), the
[GPUI icon branch](../../../packages/gpui/node-backend/src/lib.rs#L431-L440),
and the [current icon research](icon-morphing.md).

Recommendation: keep the native path out of the public surface. Use the
existing static endpoint swap or same-slot cross-fade fallback where the
interaction contract permits it. A future native `IconMorph` decision needs a
separate renderer-neutral geometry capability, a curated pair registry, and a
real-window visual proof; none of those are introduced here. [WI]

## Scope and pinned method

The disposable crate lived only under the ignored
`.effigy/research/icon-morph-native-feasibility/` directory. It depended on
`gpui = "=0.2.2"` with `test-support`, parsed only the selected generated Poodle
assets, and was removed after the run. No source, generated asset, sample,
dependency, or product file was changed by the spike.

The run used the existing GPUI 0.2.2 registry package recorded in
[`packages/gpui/preview/Cargo.lock`](../../../packages/gpui/preview/Cargo.lock):

```text
gpui 0.2.2
checksum 979b45cfa6ec723b6f42330915a1b3769b930d02b2d505f9697f8ca602bee707
rustc 1.97.1 (8bab26f4f 2026-07-14)
cargo 1.97.1 (c980f4866 2026-06-30)
macOS 26.5.2 (25F84)
```

Commands:

```text
cargo test --manifest-path .effigy/research/icon-morph-native-feasibility/Cargo.toml --lib -- --nocapture
cargo run --manifest-path .effigy/research/icon-morph-native-feasibility/Cargo.toml --release
```

The test macro was not used. The repository’s GPUI preview harness documents
that `gpui-macros 0.2.2`’s test macro can crash on this toolchain; the spike
used the same plain-test context and parked-queue shutdown shape as
[`headless_regressions.rs`](../../../packages/gpui/preview/tests/headless_regressions.rs).

## What GPUI 0.2.2 can do

The pinned upstream surface is sufficient for a custom geometry element:

- [`canvas`](https://docs.rs/gpui/0.2.2/src/gpui/elements/canvas.rs.html)
  exposes one-shot prepaint and paint callbacks.
- [`PathBuilder`](https://docs.rs/gpui/0.2.2/gpui/struct.PathBuilder.html)
  builds tessellated paths and exposes stroke, fill, line, curve, arc, polygon,
  and close operations.
- [`Window::paint_path`](https://docs.rs/gpui/0.2.2/gpui/struct.Window.html)
  inserts a `Path<Pixels>` into the next scene and accepts a color.
- [`Window::request_animation_frame`](https://docs.rs/gpui/0.2.2/gpui/struct.Window.html#method.request_animation_frame)
  schedules notification of the current view on the next platform frame.

The static SVG route is a different capability. GPUI’s
[`Svg::path`](https://docs.rs/gpui/0.2.2/src/gpui/elements/svg.rs.html) accepts
an SVG asset path, while `paint_svg` renders through the SVG renderer and its
atlas key includes the supplied path and raster size. [VF] from the pinned
[SVG element source](https://docs.rs/gpui/0.2.2/src/gpui/elements/svg.rs.html)
and [window source](https://docs.rs/gpui/0.2.2/gpui/struct.Window.html).
Feeding a newly serialized SVG string as the per-frame geometry channel would
be a raster/cache path with unbounded key churn, not the direct vector path
route proven here. [WI]

GPUI’s [`Transformation`](https://docs.rs/gpui/0.2.2/gpui/struct.Transformation.html)
only describes scale, translation, and rotation for an SVG element. The
pinned [`AnimationExt` source](https://docs.rs/gpui/0.2.2/src/gpui/elements/animation.rs.html)
uses an `Instant` and requests another frame, but its element animator still
changes the element or its transform; it does not expose mutable SVG path
geometry. [VF]

## Pair and normalization probe

The current Poodle default manifest is `lucide-static 1.31.0`, with 92
canonical names and 16 aliases. The sample used the generated 24×24 SVG assets
already present in the repository. [VF] from
[`default-icons.json`](../../../packages/core/src/icons/default-icons.json#L1-L4)
and the [icon contract](../../contracts/components/icon.md#L106-L127).

The disposable normalizer accepted only `M/m`, `L/l`, `H/h`, `V/v`, and `Z/z`
path commands, plus explicit `line` and `rect` elements; rounded rects were
recognized but rejected. It converted each contour to eight equally spaced
points and required the two endpoints to have the same contour count, closure
flag, and original point count. This is a deliberately strict admission gate,
not a production SVG parser. [VF]

| Pair | Observed endpoint shape | Strict result | Reason |
| --- | --- | --- | --- |
| `chevron-left` ↔ `chevron-right` | 1 open contour, 3 points each | accepted | same command/contour signature |
| `plus` ↔ `x` | 2 open contours, 2 points each | accepted | same command/contour signature |
| `menu` ↔ `x` | 3 contours versus 2 | rejected | unequal contour count |
| `lock` ↔ `lock-open` | rounded rect plus arc path | rejected | curves/rounded corners need a curve normalizer |
| `volume-2` ↔ `volume-x` | curved speaker geometry versus line marks | rejected | unsupported curve command and different topology |
| `play` ↔ `pause` | one closed curved path versus 2 rounded rects | rejected | unsupported curves and contour topology |

For the accepted `plus` ↔ `x` pair, the spike sampled both endpoints to two
open eight-point contours, linearly interpolated every point, and rebuilt a
stroke `Path<Pixels>` at each sampled state. This proves a deterministic
geometry channel for compatible curated pairs. It does not prove that a
general icon catalogue can be matched without semantic quality review. [VF]

The probe’s pair table held only pair IDs and endpoint names. Source version,
license, and per-asset digests were checked separately; no consumer-facing raw
path data was placed in the pair registry. That is the provenance shape a
future generated registry could preserve without making arbitrary provider
paths part of a public morph contract. [WI]

Selected generated asset SHA-256 values at the tested snapshot:

| Asset | SHA-256 |
| --- | --- |
| `plus.svg` | `ffaf84b7a03995465249165b6a7a738e2dd4f7b086dfd37f1091f4f9afb0983c` |
| `x.svg` | `91a32e01a9ec4d8c3850a54a087f5411456f708e01e492ae1a9bf86c9d139b8b` |
| `chevron-left.svg` | `bf2e8a6eafcdd28301934adc3bc28829939cb211de1c2f793f295eb54160dec9` |
| `chevron-right.svg` | `a00aab3751226d6530849cf2938dd248cf50666c3d77f157a41f1158933736ca` |
| `menu.svg` | `af5b1e5176362457fa6a5ffc8e233011016b1f638efd15244e61fe9a70f6fbf3` |
| `lock.svg` | `eeb7a555b1cd485cefff381aec44bfb6846b256186b96ced975db6e1575480b3` |
| `lock-open.svg` | `7630b6b08f26b2d47a75c3694ee6a0daa425cb315d6aa31dfdcdb8fa8543911a` |
| `volume-2.svg` | `21940346aca05c9c9e26f17144cdfc50eba62f7cfff1b6cdb559880b4cbba58f` |
| `volume-x.svg` | `7467b4ea2a8372c6392dd1e56a026c0b0b19e2b070da5731e0f710aa23c14343` |
| `play.svg` | `e742be65b6e0fdcc6ccf9392fd97f4779871470a460b712711209cf4ca0ea377` |
| `pause.svg` | `af83aa5ee327b60b4a53571f9b08f488af14437a4909f4104a1802dfee467a13` |

## Frame pacing and retained invalidation

The GPUI test view read its current frame from shared state, built the
interpolated geometry in the paint callback, and called `paint_path` with a
fixed color. The release report was:

```text
gpui.frames=123
gpui.initial_render=2
gpui.initial_paint=2
gpui.no_notify_render=2
gpui.explicit_render=3
gpui.animation_requests=123
gpui.distinct_paths=61
gpui.paints_after_teardown=123
timing.draw_ns.min=3917 median=4083 p95=7459 max=10917
timing.build_ns.min=1583 median=1666 p95=3000 max=8084
```

`gpui.frames` is the total observed paint-callback count across the initial
draw, an explicit refresh, and 60 notified frame updates; it is not a count of
displayed frames. `distinct_paths=61` is the geometry fingerprint count. The
timings are nanoseconds from this one macOS headless process: `draw_ns` covers
the explicit `Window::draw` call and parked executor, while `build_ns` covers
the custom path build. They are directional feasibility measurements, not a
frame-budget claim. [M]

The state mutation without `Context::notify` produced no new render after the
executor was parked. A notify followed by `Window::refresh` and an explicit
draw produced the next paint. The view also called the real
`request_animation_frame` API in full mode. The GPUI test platform’s
`on_request_frame` implementation is a no-op, so no platform callback advanced
the probe by itself; every measured update was manually driven. [VF] [M]

This is enough to show where a future retained node could invalidate and paint
dynamic geometry. It is not enough to claim real display frame pacing or prove
that a production backend can carry the state without a new geometry channel.
[LIM]

## Paint style and lifecycle behavior

- `PathBuilder::stroke(px(2.0))` built the interpolated open pair and
  `Window::paint_path` submitted it with a non-default color.
- `PathBuilder::fill()` built a closed triangle successfully. The current
  Lucide default is stroke-oriented; fill support is a GPUI primitive fact, not
  a recommendation to admit fill morphs.
- Full policy owns a clock in the pure state machine. Reduced and frozen both
  resolve to an endpoint with no clock; frozen also cancels the live machine.
- Retargeting starts from the current sampled geometry, so reversal does not
  jump back to the original source. A second retarget replaces the pending
  target, and freeze makes the current sample the stable endpoint.
- The headless view ran its full shutdown sequence after the borrowed visual
  context ended. No paint occurred after teardown (`paints_after_teardown` was
  unchanged). [VF] [M]

These lifecycle checks match the shared
[`full | reduced | frozen` motion policy](../../architecture/012-semantic-motion-policy.md)
and the icon-button fallback contract. They do not cover a real compositor,
late platform callbacks, or cross-window destruction races. [LIM]

## Integration boundary and accessibility

The current path is static at the product boundary:

1. shared composition creates `NodeKind::Icon { name, size }`;
2. the GPUI backend maps the name to a generated SVG asset;
3. generic native animation can apply the admitted opacity/rotation paths, but
   there is no path geometry field to update per frame.

Adding morph data to `NodeKind::Icon`, changing the GPUI icon branch to accept
arbitrary path data, or making provider-supplied `IconNodes` implicitly
morphable would all be public or product-surface changes outside this spike.
The existing [icon provider contract](../../contracts/components/icon-provider.md)
and [icon contract](../../contracts/components/icon.md) remain unchanged.

GPUI 0.2.2 has no content accessibility tree or role/label API. Poodle’s GPUI
adapter carries the icon label metadata but cannot map it to GPUI accessibility
attributes under the documented
[native accessibility rule](../../contracts/003-native-accessibility.md).
The spike therefore provides no assistive-technology evidence. A future native
morph must preserve the static icon’s semantic root and be evaluated against
that known runtime boundary. [VF] [LIM]

## Evidence limits

- The test platform’s `TestWindow::draw` does not expose a presented image. No
  pixel equivalence, stroke-cap/join fidelity, color blending, GPU cost, or
  compositor pacing was measured. [LIM]
- The six-pair sample is representative of topology failures, not catalogue
  coverage. The disposable parser intentionally rejects curves, rounded
  corners, and richer SVG syntax; a rejected pair is a strict-probe rejection,
  not a mathematical impossibility. [LIM]
- The run did not import Morphicons, copy its runtime or data, add a dependency,
  or evaluate arbitrary raw SVG. [VF]
- No provider registry, public API, shared Rust node, GPUI adapter, contract,
  architecture document, existing dossier, front door, or `PAPERCUTS.md` file
  was changed. [VF]

## Recommendation and next decision

Do not promote native icon morphing on the strength of this spike. The result
is **low-level GPUI feasibility: yes; current Poodle production admission: no**.

If the operator wants to reopen the native path, the next decision is whether
to fund a separate geometry capability with these admission gates:

- a curated, generated pair registry keyed by semantic pair identity and
  source provenance;
- canonical 24×24 geometry with explicit primitive, contour, closure, point,
  and correspondence rules;
- a renderer-neutral geometry node or equivalent shared composition channel;
- full/reduced/frozen lifecycle behavior with latest-state-wins retargeting and
  teardown cancellation;
- paired web/native structural tests plus a real GPUI window capture proving
  endpoint fidelity and display behavior.

Until that decision is made, the safe path is the existing static endpoint or
same-slot cross-fade fallback. Jetstream remains deferred, custom provider
paths remain static, and no public API is implied by this dossier.
