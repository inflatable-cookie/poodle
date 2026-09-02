# g16.050 — Icon Geometry Internal Runtime Substrate

Status: complete — merged in PR #160
Type: implementation — internal capability
Opened: 2026-09-01
Depends on: merged `g16.049` foundation; funded icon-geometry programme recorded in
`../../handoffs/20260901-234025-post-triage-canonical-runway.md`
Governing refs: the architecture produced by `g16.049`,
`../../architecture/012-semantic-motion-policy.md`,
`../../contracts/components/icon.md`,
`../../contracts/components/icon-button.md`
Architecture: `../../architecture/013-icon-geometry-substrate.md`
Execution log: `../../logs/2026-09/20260902-g16-050-icon-geometry-internal-runtime-substrate.md`
PR: https://github.com/inflatable-cookie/poodle/pull/160
Merge: `8377a936ef8bb55e30e44da55fe728e2e5ced429`

## Goal

Deliver IG-03 through IG-05 over candidate geometry fixtures from the validated
registry: pure plan/lifecycle, a distinct resolved geometry node with shared
Rust/GPUI headless proof, and private Svelte/React shells with controlled
browser evidence. Keep the route internal until native visual admission.

## Fixed Boundary

- Architecture 012 remains the only clock/policy authority. Full uses one
  bounded interaction transition; reduced/frozen snap to canonical endpoints.
  Latest state wins, same-pair reversal rebases from current sampled geometry,
  repeated targets are inert, and teardown leaves no live handle.
- Keep NodeKind::Icon static. Add a distinct resolved geometry leaf carrying a
  compact validated frame. It carries no SVG string, provider registry, public
  consumer path, or backend-owned pair lookup.
- Shared composition owns pair lookup, size, color, identity, policy, and frame.
  GPUI owns path construction, paint, invalidation, scheduling, and teardown.
- Web shells keep one stable outer visual root, deterministic SSR endpoint,
  hydration, selection/focus/layout invariants, and the same pure plan.
- g16.049 emits no accepted or runtime-eligible pair. This card may use
  candidate geometry only as an internal test fixture; it must not change pair
  status, add reviewer or acceptance authority, or promote eligibility.
- The route is private/internal. No public IconMorph contract/export, provider
  widening, Icon behavior change, or active-cohort visual admission occurs.
- Initial budgets: zero hot-path allocation after plan creation; p95 geometry
  update at most 1 ms per instance / 4 ms for four; p95 cold plan at most 2 ms;
  one clock per owner/role/channel; zero layout/name/focus change.

## Ordered Work

1. Build pure planning, interpolation, stable-key, policy, interruption,
   cancellation, frozen, SSR, and teardown traces over the `g16.049` registry.
2. Add the resolved geometry node and shared Rust construction. Preserve all
   existing Icon and animation paths.
3. Implement the GPUI production path with PathBuilder/paint_path, focused
   headless invalidation, allocation, scheduling, concurrency, and teardown
   evidence.
4. Implement private Svelte/React shells and controlled browser start,
   midpoint, endpoint, reverse, frozen, SSR/hydration, focus, and layout proof.
5. Record structural and budget evidence in one log. Do not request native
   windowed capture from this card.

## Acceptance

- Shared traces prove exact authored initial/endpoints, A→B→A and A→B→C
  latest-state behavior, policy tightening, pair replacement, cancellation,
  frozen determinism, and zero late work.
- Existing Icon/IconProvider/IconButton paths, names, labels, focus, busy/
  pressed/loading semantics, and named-asset node output stay unchanged.
- GPUI consumes resolved frames through the production backend and meets the
  adopted plan/frame/allocation/scheduler budgets headlessly.
- Svelte and React keep one stable root and equal semantic inputs through SSR,
  hydration, motion policies, interruption, focus, and controlled captures.
- Evidence labels structure/headless/browser results honestly. No native pixel,
  AT, public API, release, consumer, or Jetstream claim appears. Candidate
  geometry remains fixture input only; this card changes no g16.049 pair status
  or runtime-eligibility state.

## Current head proves

Repair of the exact-head review on PR #160. The head now has:

- one GPUI `IconGeometryHost` whose timer re-enters without holding the app
  borrow, writes the resolved frame, calls `window.refresh()`, preserves an
  inert live task, uses the runtime's proportional reverse duration, and
  cancels on policy tightening/teardown
- web shells whose rAF loop resumes the live clock's progress and duration;
  paired tests prove inert continuation, proportional reverse completion, and
  full-to-frozen cancellation
- a single-owner runtime: second owner retargets, old key samples null,
  concurrent instances are separate runtimes/hosts
- zero hot-path `Map`, key clone, or fresh canonical rows on interior samples;
  an allocator-backed probe wraps the actual scheduled GPUI tick and reports
  zero allocations after plan creation; p95 receipts still cover 40 samples
- `NodeKind::ResolvedIconGeometry` public; registry/runtime/construction
  hidden (`pub(crate)` on specs, no crate-root `resolved_icon_geometry` on
  render, `icon-geometry-internal` for hosts/tests)

Controlled-progress props remain a test harness. `PathBuilder::build` remains
backend paint, not a composition allocation.

## Review Oracle

| Invariant | Smallest counterexample | Required proof |
| --- | --- | --- |
| Current sampled state owns retarget | A→B→A before completion | no jump or queued phase; latest A wins |
| Pair replacement is bounded | swap to unrelated pair id | old plan cancels; new target validates or rejects |
| Motion policy is authoritative | full → reduced → frozen | clock stops and canonical latest endpoint paints |
| Backend does not own semantics | remove shared pair lookup | GPUI receives only resolved geometry and cannot recover pair meaning |
| Existing Icon stays unchanged | render provider/static icon | same named node and behavior as before |
| Teardown is exact | unmount during browser/native scheduled frame | no late write/paint or retained handle |

## Writable Scope

Internal geometry plan/lifecycle modules; distinct node payload; shared Rust
composition; GPUI node backend and headless probes; private Svelte/React shells
and bounded browser fixtures; focused tests; architecture implementation
evidence; this card, one log, and new papercuts. Do not edit public Icon/
IconProvider APIs, public package exports, release/workflow surfaces, dedicated
lab code, visual ledger cells, consumers, or Jetstream behavior.

## Validation

Run paired lifecycle vectors, node/render assertions, focused GPUI headless
probes, web SSR/hydration/focus/layout/capture tests, relevant drift/audit
selectors, `effigy ci:web`, `effigy ci:rust`, `effigy ci:native`,
`effigy docs:check`, one final headless `effigy qa`, and `git diff --check
origin/main...HEAD`. Never run local windowed/native-visual selectors.

## Stop Conditions

Stop if the route needs public raw geometry, NodeAnimation path payloads,
backend semantic lookup, per-frame allocation, duplicate clocks, unsupported
paint semantics, web-only public admission, or native pixels to validate the
headless substrate.

## Continuation

Completion of `g16.050` unlocks the gated `g16.051` dedicated-lab visual
review. Candidate geometry remains fixture-only, and public IconMorph remains
uncompiled until that gate passes.
