# g16.034 — Shared Motion Policy And Five-Family Pilot

Status: implemented — ready for exact-head review
Date: 2026-09-01
PR: https://github.com/inflatable-cookie/poodle/pull/124
Implementation commits: `bb656700f` (initial), `d99b9af83` + `f03f723bc` +
`a9fa37d1e` + `9b4006a2d` (review repair)
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
  remnants, focus fallback next → previous → entered-from. Mounted policy
  tightening cancels the old owner and re-drives the current visual state;
  action teardown cancels the latest owner. Expiry still belongs to ToastHost.
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

Addressed on `d99b9af83` + `f03f723bc` + `a9fa37d1e` + `9b4006a2d`:

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
- Mounted Svelte ToastStack policy changes now cancel the prior owner before
  replay; unsupported WAAPI exits abort their trace; registry cancellation
  preserves a synchronous replacement; Tabs measurement rAF teardown is
  identity-safe in both web shells.

## Overlap

The operator-driven drag-fix lane already edits Tabs files:

- `packages/svelte/components/src/Tabs.svelte`
- `packages/react/components/src/Tabs.tsx`
- related Tabs tests and `packages/render/src/tabs.rs` (this lane did not
  edit the native Tabs renderer beyond the underline indicator contract)

The orchestrator owns merge order.

## Falsification

Real proofs were committed at `d99b9af83` before planting. The required
plant → intended failure → restore → green rerun was completed for all 11
rows before this recovery; restores used `git checkout --` against committed
sources while the index was clean. The current recovery did not reset, clean,
or discard workspace changes.

| Oracle row | Plant | Intended failure | Restore + rerun |
| --- | --- | --- | --- |
| 1 Policy restriction-only | TS `restrictMotionPolicy` returns child; Rust `restrict_motion_policy` returns requested only | TS expected reduced, received full; Rust Full vs Reduced | green |
| 2 Initial state is not invented | skip `intent.initial` so `shouldSchedule` is true | TS `authored initial state` schedule true; Rust `!decision.schedule` | green |
| 3 Latest semantic state owns motion | reversal uses `durationMs` not `originalDurationMs` | second reversal expected 144, received 58 (TS + Rust) | green |
| 4 Reduced and frozen differ | `setMotionTracePolicy` does not drop frozen clocks | liveClockCount expected 0, received 1 | green |
| 5 Cleanup is exact | reject handler `handles.delete(key)` unconditionally; synchronous replacement callback is not identity-checked | replace-key test expected live 1, received 0 | green |
| 6 Disclosure exception bounded | `gpuiMotionPlan` applies `height` | expected static-endpoint, received none | green |
| 7 Toast semantics | `nextToastVisuals` drops instead of exit remnant | family test `.poodle-toast` null, cannot read `dataset` | green |
| 8 Tabs indicator vs environment | ResizeObserver no-op | probe before=188 after=188 | green |
| 9 Discrete semantics precede paint | Checkbox `emitCheckedChange` reverts native checked | expected true, received false | green |
| 10 Loading loops obey policy | loop schedules without `firstFrameCommitted` | expected schedule false, received true | green |
| 11 Native gaps stay visible | `gpuiMotionPlan` applies `translateY` | expected opacity-stand-in, received none | green |

## Evidence

- Paired TS/Rust trace tests: `packages/core/test/motion-policy.test.ts`,
  `packages/contracts/headless/src/motion_policy.rs` (inline tests).
- Web runtime: `packages/core/test/motion-runtime.test.ts`, including
  unsupported-WAAPI cleanup and synchronous replacement identity.
- Mounted family receipts: `packages/svelte/components/test/motion-families.test.ts`,
  `packages/svelte/components/test/MotionFamilyHarness.svelte`, and
  `packages/react/components/test/motion-families.test.tsx`.
- GPUI mounted regression: `mounted_motion_policy_construction_does_not_invent_clocks`
  in `packages/gpui/preview/tests/headless_regressions.rs`.
- Browser probe: `effigy test:motion-policy-browser` (Chromium + WebKit).

## Validation

Focused (repair head):

- `bun test packages/core/test/motion-runtime.test.ts packages/core/test/motion-policy.test.ts` — 16 pass, 0 fail, 68 expect.
- Svelte motion-family suite — 9 pass; mounted Svelte/React provider, family,
  and ToastStack suites — 6 files, 39 pass.
- `effigy test:core` — 1125 pass, 0 fail, 3673 expect across 58 files.
- `effigy check:svelte-components` — 0 errors, 4 existing warnings in 2 files.
- Native focused tests — GPUI mounted regression 1 pass, headless motion
  policy 12 pass, render underline 5 pass, GPUI specimens 8 pass.
- `effigy test:motion-policy-browser-chromium` — Svelte + React checks pass:
  disclosure, underline resize, preloaded toasts, exit cleanup/focus, and
  reduced IconButton transition.
- `effigy test:motion-policy-browser-webkit` — the same checks pass.
- Callback/prop/capability/contract drift and packed-consumer proof — pass;
  111 callbacks, 137 props, 36 capability rows, 10 files/20 packed tests.

Boards:

- `effigy ci:web` — pass: 371 files, 3411 tests; packed consumer 10 files,
  20 tests.
- `effigy ci:rust` — pass, including 196 headless and 289 component-spec
  tests.
- `effigy ci:native` — pass: 560 render tests, 134 GPUI tests, 162 Jetstream
  tests, 166 headless regressions, and the 8 GPUI motion specimens.
- `effigy docs:check` — pass; 176 evidence rows and zero drift.
- Final `effigy qa` — substantive selectors pass; aggregate exit 1 only at
  `audit:security`. It flags the known main-baseline English-word false
  positive (`mask-plus-translated-highlight`) in `PAPERCUTS.md`, the g16.033
  handoff/log, and this log. No credential is present; the scanner pattern has
  no left boundary. This is recorded in `PAPERCUTS.md` and is outside this
  card's scope.
- Final `git diff --check` and diff-scope/absence checks — pass.

## Remaining explicit boundaries

No remaining in-scope semantic defect was identified. The following are
intentional, documented boundaries:

- Native Tabs has one painted accent indicator in the selected geometry slot;
  inactive slots retain layout-only reserves, and no selected-tab border is
  used.
- GPUI translation/scale remains a named opacity stand-in; native disclosure
  height remains a static endpoint because those channels are not admitted.
- Jetstream has no admission in this card.
- The full specimen census needs a 32 GB heap even serially; the papercut is
  recorded and does not block these focused proofs.
