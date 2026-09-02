# g16.050 — Icon Geometry Internal Runtime Substrate

Status: implementation complete — PR pending
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
Post-rebase implementation head: `cf8e1ebe8`

## Outcome

Delivered the private icon-geometry runtime over g16.049 candidate fixtures
only. TypeScript and Rust share one dedicated lifecycle (role `icon-geometry`,
channel `glyph`, 180 ms): authored SSR initial, full interpolation, same-pair
A→B→A rebase from the sampled axis, A→B→C replacement, reduced/frozen snap to
the latest semantic endpoint, rejected/missing pair clear, abort, and teardown
with no retained handle.

`NodeKind::Icon` stays a named static asset. A distinct
`NodeKind::ResolvedIconGeometry` leaf carries only compact quantized contours.
Shared composition owns pair lookup. GPUI paints with `PathBuilder` /
`paint_path`. Private Svelte/React shells keep one svg root through
start/mid/end/reverse/frozen, SSR, hydration, focus, and layout. They are not
package exports.

Hot-path sampling mutates a reused contour buffer after plan creation.
Canonical vertices are used at progress 0/1; interiors use 64-sample
correspondence. GPUI `PathBuilder::build` is backend-owned paint, not
composition allocation; the headless sample+rebuild+draw stayed inside the
4 ms × 4 budget.

No pair status, eligibility, public IconMorph, Icon/IconProvider/IconButton
behavior, native pixel, AT, Jetstream, or package-export change.

## Falsification

Each plant was applied temporarily, the intended check failed, and the exact
source was restored before the green rerun.

| Oracle | Plant and observed bite | Restored state |
| --- | --- | --- |
| A→B→A | same-pair reverse branch disabled; TS suite failed `A→B→A before completion rebases from the sampled frame` | reverse rebase from sampled axis |
| Pair replace | `removeClock` skipped on unrelated pair swap; TS suite failed `A→B→C latest-state replacement cancels the old plan` | old plan cancelled before the new target |
| Policy | `setIconGeometryPolicy` snap forced off; TS suite failed `full → reduced → frozen snaps to the latest canonical endpoint` | reduced/frozen still snap and drop the clock |
| Missing lookup | rejected-pair `accepted` forced true; TS suite failed `unrelated rejected pair id cancels and cannot recover meaning` | rejected/missing pair still clears and cannot recover meaning |
| Static Icon | `poodle-render::icon` emitted `ResolvedIconGeometry`; `named_icon_path_stays_a_named_icon_node` panicked | named `NodeKind::Icon` path unchanged |
| Teardown | frame/plan clear skipped; TS suite failed `abort settles the endpoint and teardown drops the handle` | teardown still drops the handle |

## Validation

Focused:

- `bun test packages/core/test/icon-geometry.test.ts packages/core/test/icon-geometry-runtime.test.ts` — 28 pass
- `cargo test --manifest-path packages/contracts/components/Cargo.toml --test icon_geometry` — 4 pass
- `cargo test --manifest-path packages/contracts/node/Cargo.toml --lib` — 12 pass
- `cargo test --manifest-path packages/render/Cargo.toml --lib icon_geometry` — 3 pass
- `cargo test --manifest-path packages/gpui/node-backend/Cargo.toml --lib` — 49 pass
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test icon_geometry_headless` — 2 pass
- `bunx vitest run --project svelte-components --project svelte-components-ssr --project react-components` on the private shells — 10 pass
- `bunx svelte-check --workspace packages/svelte/components --threshold error` — 0 errors

Repository boards:

- `effigy docs:check` — pass
- `effigy ci:web` — pass
- `effigy ci:rust` — pass
- `effigy ci:native` — pass
- `effigy qa` — pass
- `git diff --check origin/main...HEAD` — pass

Never ran local `*-windowed` or native-visual selectors.

## Next task

Orchestrator review and merge. `g16.051` remains gated; candidate geometry
stays fixture-only; public IconMorph stays uncompiled.
