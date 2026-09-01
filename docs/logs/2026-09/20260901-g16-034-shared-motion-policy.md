# g16.034 — Shared Motion Policy And Five-Family Pilot

Status: implemented — boards still open (repair round 2)
Date: 2026-09-01
PR: https://github.com/inflatable-cookie/poodle/pull/124
Implementation commits: `bb656700f` (initial), `d99b9af83` + `f03f723bc` (review repair)
Card: `docs/roadmaps/g16/034-shared-motion-policy-and-five-family-pilot.md`
Handoff: `docs/handoffs/20260901-130224-g16-034-shared-motion-policy.md`
Governing refs: `docs/architecture/012-semantic-motion-policy.md`,
`docs/architecture/010-native-presentation-construction-context.md`,
`docs/contracts/components/motion-policy-provider.md`,
`docs/contracts/001-working-rules.md`
Branch: `feature/g16-034-shared-motion-policy`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-034-shared-motion-policy`
Base: rebased onto `origin/main` at `b682ebfed` after merged PR #123
(g16.035 closeout); planning base `b89c11275` is an ancestor

## Outcome

One explicit `full | reduced | frozen` host policy exists in TypeScript and
Rust. Missing preference is full. Nesting is restriction-only. Presentation
scopes preserve motion. Frozen declares no visual clocks.

The five pilot families consume that policy:

- Accordion / Collapsible: clipped block-axis height plus indicator rotation
  after the first committed frame; reduced/frozen snap; closed panels stay
  inert and keep content in layout until the clip finishes.
- ToastStack: WAAPI completion drives settle/drop; keyed enter/exit, inert exit
  remnants, focus fallback next → previous → entered-from. Expiry still belongs
  to ToastHost.
- Tabs `activeEdge="underline"`: one measured paint-only indicator. First
  layout and resize snap; semantic selection can retarget; rAF cancelled on
  teardown.
- Checkbox / IconButton: semantic state is immediate; reduced keeps opacity
  only; frozen paints the endpoint.
- Skeleton / Spinner: 1.6s opacity pulse and ring/dot/grid loops only in full
  after the first frame. `Skeleton.animated=false` wins.

GPUI still only drives generic opacity and SVG rotation. Translation/scale
declarations record `opacity-stand-in`. Disclosure height has no native
channel and stays a static-endpoint gap. No existing ledger cell moved. One
new MotionPolicyProvider row was added because the public default-as export
changed the live denominator (176 public / 175 portable). GPUI construction
cells now say 175/175 routes.

## Review repair (PR #124 changes requested)

Addressed on `d99b9af83` + `f03f723bc`:

- Toast lifecycle uses WAAPI `finished` completion, not CSS `animationend`;
  exit rows are `inert` with `tabIndex=-1`; owner-scoped registry keys;
  identity-safe handle deletion; unmount guards block late mutation.
- `setMotionTracePolicy` assigns the requested policy directly (relaxation
  works); reversal duration uses `originalDurationMs`; live runtime retains
  trace state across `playClippedHeight` / `playToastPresence`.
- IconButton reduced CSS is opacity-only; Skeleton skips `useMotionReady` when
  `animated=false`; disclosure keeps content unhidden during close clip;
  Tabs observes selected item and cancels rAF on teardown.
- Native toast/skeleton/spinner skip enter/loop until first frame committed;
  native Tabs owns one paint-only underline indicator.
- Mounted Svelte/React family receipts, GPUI headless regression
  `mounted_motion_policy_construction_does_not_invent_clocks`, and bounded
  Chromium + WebKit probe at `test/motion-policy-probe/`.
- Empty ToastStack default `items` is a stable array; presence sync skips
  no-op Map/visual writes so omitted-items mounts (parity) cannot loop.

## Overlap

The operator-driven drag-fix lane already edits Tabs files:

- `packages/svelte/components/src/Tabs.svelte`
- `packages/react/components/src/Tabs.tsx`
- related Tabs tests and `packages/render/src/tabs.rs` (this lane did not
  edit the native Tabs renderer beyond the underline indicator contract)

The orchestrator owns merge order.

## Falsification

Real proofs committed at `d99b9af83` before planting. Restores used
`git checkout --` against committed sources, not a dirty index.

| Oracle row | Plant | Intended failure | Restore + rerun |
| --- | --- | --- | --- |
| 1 Policy restriction-only | TS `restrictMotionPolicy` returns child; Rust `restrict_motion_policy` returns requested only | TS expected reduced, received full; Rust Full vs Reduced | green |
| 2 Initial state is not invented | skip `intent.initial` so `shouldSchedule` is true | TS `authored initial state` schedule true; Rust `!decision.schedule` | green |
| 3 Latest semantic state owns motion | reversal uses `durationMs` not `originalDurationMs` | second reversal expected 144, received 58 (TS + Rust) | green |
| 4 Reduced and frozen differ | `setMotionTracePolicy` does not drop frozen clocks | liveClockCount expected 0, received 1 | green |
| 5 Cleanup is exact | reject handler `handles.delete(key)` unconditionally | replace-key test expected live 1, received 0 | green |
| 6 Disclosure exception bounded | `gpuiMotionPlan` applies `height` | expected static-endpoint, received none | green |
| 7 Toast semantics | `nextToastVisuals` drops instead of exit remnant | family test `.poodle-toast` null, cannot read `dataset` | green |
| 8 Tabs indicator vs environment | ResizeObserver no-op | probe before=188 after=188 | green |
| 9 Discrete semantics precede paint | Checkbox `emitCheckedChange` reverts native checked | expected true, received false | green |
| 10 Loading loops obey policy | loop schedules without `firstFrameCommitted` | expected schedule false, received true | green |
| 11 Native gaps stay visible | `gpuiMotionPlan` applies `translateY` | expected opacity-stand-in, received none | green |

## Evidence

- Paired TS/Rust trace tests: `packages/core/test/motion-policy.test.ts`,
  `packages/contracts/headless/src/motion_policy.rs` (inline tests).
- Web runtime: `packages/core/test/motion-runtime.test.ts`.
- Mounted family receipts: `packages/svelte/components/test/motion-families.test.ts`,
  `packages/react/components/test/motion-families.test.tsx`.
- GPUI mounted regression: `mounted_motion_policy_construction_does_not_invent_clocks`
  in `packages/gpui/preview/tests/headless_regressions.rs`.
- Browser probe: `effigy test:motion-policy-browser` (Chromium + WebKit).

## Validation

Focused (repair head):

- core motion policy/runtime, Svelte/React provider + family suites — pass
- `cargo test --lib motion` (headless) — pass
- `mounted_motion_policy_construction_does_not_invent_clocks` — pass
- `effigy test:motion-policy-browser` — pass (Chromium + WebKit, Svelte + React)

Boards:

- `effigy docs:check` — pass (re-run after this log edit)
- `effigy ci:rust` — pass
- `git diff --check origin/main...HEAD` — pass
- `effigy ci:web` — 32 GB serial run on `f03f723bc` (before the empty-default
  fix): 369/371 files, 3407/3409 tests. Failures were only ToastStack parity
  (914773ms hang) and React ToastStack smoke (max update depth / invalid array
  length). Specimen census completed on that same worker. Focused rerun after
  the stable-empty + no-op presence sync: family/host/ToastStack unit tests
  plus `ToastStack` smoke/parity — pass (45 tests, ~10s). Full board rerun
  follows.
- `effigy ci:native` — last full run failed `smoke:gpui-window-capture`
  `tests::batch_mode_accepts_no_other_flag` (likely pid+len temp-dir flake).
  Earlier proofs-commit run passed. Retry after `ci:web`.
- `effigy qa` — not run; includes `ci:web` + `ci:native` + `audit:security`.
  `audit:security` remains the known main-red English-word false positive
  (`mask-plus-translated-highlight`).

## Unresolved

- GPUI translation/scale: opacity-stand-in only
- Disclosure height: static-endpoint on native
- No Jetstream admission
- Specimen census needs 32 GB heap even serially (papercut recorded)
