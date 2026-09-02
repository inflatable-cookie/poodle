# g16.050 — Icon Geometry Internal Runtime Substrate

Status: implementation complete — PR #160 pending orchestrator review
Date: 2026-09-02
Card: `docs/roadmaps/g16/050-icon-geometry-internal-runtime-substrate.md`
Handoff: `docs/handoffs/20260902-095100-g16-050-icon-geometry-runtime.md`
Architecture: `docs/architecture/013-icon-geometry-substrate.md`
Governing refs: `docs/architecture/012-semantic-motion-policy.md`,
`docs/contracts/components/icon.md`,
`docs/contracts/components/icon-button.md`
Branch: `feature/g16-050-icon-geometry-runtime`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-050-icon-geometry-runtime`
Starting exact head: `9e8e646f25a1dfde818083c798ffba53adea3e95`
Planning base ancestor: `a809792c6cd9873f9716b4954d2b4b803c6b65eb`
Rebased onto live `origin/main`: `595bec72825a9b830edb2b46f82b4ece049f8e1b`
Post-rebase implementation head: `4f71bf3899d0d9678b4cac58ffac8476721a07fb`
Reviewed head (blocked): `63ea7582ec7e596b067b878e804bf717464b2a4b`
Repair code head: `a225058a3a0b200d5508835d41777f0e989ec19e`
PR: https://github.com/inflatable-cookie/poodle/pull/160

## Outcome

Delivered the private icon-geometry runtime over g16.049 candidate fixtures
only. TypeScript and Rust share one dedicated lifecycle (role `icon-geometry`,
channel `glyph`, 180 ms): authored SSR initial, full interpolation, same-pair
A→B→A rebase from the sampled axis, A→B→C replacement, reduced/frozen snap to
the latest semantic endpoint, rejected/missing pair clear, abort, and teardown
with no retained handle.

`NodeKind::Icon` stays a named static asset. A distinct
`NodeKind::ResolvedIconGeometry` leaf carries only compact quantized contours.
Shared construction owns pair lookup. GPUI paints with `PathBuilder` /
`paint_path`. Private Svelte/React shells keep one svg root through
start/mid/end/reverse/frozen, SSR, hydration, focus, and layout. They are not
package exports.

Hot-path sampling mutates a reused contour buffer after plan creation.
Canonical vertices are used at progress 0/1; interiors use 64-sample
correspondence. GPUI `PathBuilder::build` is backend-owned paint, not
composition allocation.

No pair status, eligibility, public IconMorph, Icon/IconProvider/IconButton
behavior, native pixel, AT, Jetstream, or package-export change.

## Exact-head repair

Review of `63ea7582e` blocked four claims. This repair keeps the same branch
and PR.

1. Production scheduler. GPUI `IconGeometryHost` spawns a window task, ticks
   ~16 ms across 180 ms, writes the resolved frame in place, and `drop`s the
   task on teardown. Web shells call `requestAnimationFrame` when
   `decision.liveClock` is set and cancel on unmount. Controlled `progress`
   stays a test harness. Honest limit: detaching the GPUI task did not keep
   it alive on the test platform, so the biting teardown oracle is web rAF
   cancel; GPUI still uses a real spawn+timer and `drop(task)`.
2. Single owner. One runtime holds one owner, one clock, one plan, one frame.
   A second owner retargets; sampling the old key returns null. Concurrent
   instances are separate runtimes/hosts.
3. Zero hot-path allocation and truthful p95. Interior samples allocate no
   `Map` and reuse point rows. Compact and node capacities/pointers stay
   stable across 40 samples. p95 ≤ 1 ms/instance, ≤ 4 ms for four, cold plan
   ≤ 2 ms, from a sorted receipt, not one `Instant`.
4. Sealed Rust consumer path. `NodeKind::ResolvedIconGeometry` stays public.
   `poodle-specs` keeps `icon_geometry` as `pub(crate)`. `poodle-render`
   hides construction by default; `icon-geometry-internal` is the host/test
   route. No crate-root `resolved_icon_geometry`.

## Falsification

Each plant was applied temporarily, the intended check failed, and the exact
source was restored before the green rerun.

First landing:

| Oracle | Plant and observed bite | Restored state |
| --- | --- | --- |
| A→B→A | same-pair reverse branch disabled; TS suite failed `A→B→A before completion rebases from the sampled frame` | reverse rebase from sampled axis |
| Pair replace | `removeClock` skipped on unrelated pair swap; TS suite failed `A→B→C latest-state replacement cancels the old plan` | old plan cancelled before the new target |
| Policy | `setIconGeometryPolicy` snap forced off; TS suite failed `full → reduced → frozen snaps to the latest canonical endpoint` | reduced/frozen still snap and drop the clock |
| Missing lookup | rejected-pair `accepted` forced true; TS suite failed `unrelated rejected pair id cancels and cannot recover meaning` | rejected/missing pair still clears and cannot recover meaning |
| Static Icon | `poodle-render::icon` emitted `ResolvedIconGeometry`; `named_icon_path_stays_a_named_icon_node` panicked | named `NodeKind::Icon` path unchanged |
| Teardown | frame/plan clear skipped; TS suite failed `abort settles the endpoint and teardown drops the handle` | teardown still drops the handle |

Exact-head repair:

| Oracle | Plant and observed bite | Restored state |
| --- | --- | --- |
| Web teardown | `cancelAnimationFrame` stripped from `startIconGeometryFrameLoop`; React teardown test: cancel not called | rAF cancel on unmount |
| Second owner | replacement clock skipped when a clock already existed; TS suite: live clock count 0 after second owner | second owner installs the new clock |
| Allocation | `new Map(...)` restored in `writeFrameAt`; TS suite: Map constructed 34 times | interior samples allocate no Map |
| Seal | `pub use icon_geometry::resolved_icon_geometry` at render crate root; `construction_is_sealed_from_the_crate_root` panicked | no crate-root consumer path |

## Validation

Focused (first landing):

- `bun test packages/core/test/icon-geometry.test.ts packages/core/test/icon-geometry-runtime.test.ts` — 28 pass
- `cargo test --manifest-path packages/contracts/components/Cargo.toml --test icon_geometry` — 4 pass
- `cargo test --manifest-path packages/contracts/node/Cargo.toml --lib` — 12 pass
- `cargo test --manifest-path packages/render/Cargo.toml --lib icon_geometry` — 3 pass
- `cargo test --manifest-path packages/gpui/node-backend/Cargo.toml --lib` — 49 pass
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test icon_geometry_headless` — 2 pass
- `bunx vitest run --project svelte-components --project svelte-components-ssr --project react-components` on the private shells — 10 pass
- `bunx svelte-check --workspace packages/svelte/components --threshold error` — 0 errors

Focused (exact-head repair):

- `bun test packages/core/test/icon-geometry.test.ts packages/core/test/icon-geometry-runtime.test.ts` — 30 pass
- `cargo test --manifest-path packages/contracts/components/Cargo.toml --test icon_geometry` — 5 pass
- `cargo test --manifest-path packages/contracts/node/Cargo.toml --lib` — 12 pass
- `cargo test --manifest-path packages/render/Cargo.toml --lib icon_geometry` — 5 pass (filter also hits one unrelated `empty_state` test)
- `cargo test --manifest-path packages/gpui/node-backend/Cargo.toml --lib` — 49 pass
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test icon_geometry_headless` — 4 pass
- `bunx vitest run --project svelte-components --project svelte-components-ssr --project react-components` on the private shells — 10 pass
- `bunx svelte-check --workspace packages/svelte/components --threshold error` — 0 errors (4 pre-existing warnings)

Repository boards (repair head):

- `effigy docs:check` — pass
- `effigy ci:web` — pass
- `effigy ci:rust` — pass
- `effigy ci:native` — pass
- `effigy qa` — pass
- `git diff --check origin/main...HEAD` — pass

Never ran local `*-windowed` or native-visual selectors.

## Next task

Orchestrator re-review of the same PR. Do not merge from this lane.
`g16.051` remains gated; candidate geometry stays fixture-only; public
IconMorph stays uncompiled.
