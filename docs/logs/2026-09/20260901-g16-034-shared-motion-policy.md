# g16.034 — Shared Motion Policy And Five-Family Pilot

Status: implemented — production repair complete; exact-head evidence recorded
Date: 2026-09-01
PR: https://github.com/inflatable-cookie/poodle/pull/124
Implementation commits: `bb656700f` (initial), `d99b9af83` + `f03f723bc` +
`a9fa37d1e` + `9b4006a2d` (review repair), `e93cd3f29` (production execution
repair)
Card: `docs/roadmaps/g16/034-shared-motion-policy-and-five-family-pilot.md`
Handoff: `docs/handoffs/20260901-130224-g16-034-shared-motion-policy.md`
Governing refs: `docs/architecture/012-semantic-motion-policy.md`,
`docs/architecture/010-native-presentation-construction-context.md`,
`docs/contracts/components/motion-policy-provider.md`,
`docs/contracts/001-working-rules.md`
Branch: `feature/g16-034-shared-motion-policy`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-034-shared-motion-policy`
Starting exact review head: `493735e8f15abdff646a9067ae9ca666a787eee5`
Production repair commit: `e93cd3f29a61b9c7568fd066dd3b969f79083ec2`
Base: PR #125 merged into `main` at
`a980cb7748fdf9751dd4ca64b02903111a44d59f`; this branch's merge-base is that
commit. `origin/main` later advanced to
`8f60700ffdde723c2e0ea009296cc20aacfe3e4f` after this exact-head repair was
based and is intentionally not integrated here.
Planning base `b89c11275` is an ancestor.

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
  after the first frame. The GPUI preview commits that frame through the real
  `Window::on_next_frame` path before the mounted loading routes schedule.
  `Skeleton.animated=false` wins.

GPUI still only drives generic opacity and SVG rotation. Translation/scale
declarations record `opacity-stand-in`. Disclosure height has no native
channel and stays a static-endpoint gap. No established ledger cell moved.
The additive MotionPolicyProvider public export/row is reflected in the live
denominator (176 public / 175 portable; `MeterSurface` remains web-only / n/a).
Any generated/static Jetstream catalogue route is registry metadata only;
Jetstream remains deferred and has no mounted-parity admission. GPUI
construction cells now say 175/175 routes.

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

Production execution repair (`e93cd3f29`):

- GPUI `PreviewRoot` now carries first-frame commitment through the real
  `Window::on_next_frame` callback into the production Skeleton/Spinner route;
  a mounted probe proves full-mode loops do not start before that commit.
- Svelte and React Accordion/Collapsible now use the shared clipped-height
  runtime on the actual component path. Mounted rapid controlled reversal
  receipts prove the live clip is the reversal start and duration is
  proportional to remaining distance.
- Natural clipped-height completion settles the style and removes only the
  exact live handle; the core runtime has a finishing regression.
- Svelte ToastStack seeds authored preloaded visuals synchronously, with an SSR
  first-paint receipt proving settled output and no enter animation.
- React controlled prop-driven close retains the visible inert remnant until
  the close clip exists and completes; the mounted reversal receipt proves it.

## Overlap

PR #125 merged into `main` at
`a980cb7748fdf9751dd4ca64b02903111a44d59f` before this production repair. The
branch was already reconciled against that merge. The combined
`Tabs.svelte`, `Tabs.tsx`, `packages/core/src/styles/tabs.css`, and
`packages/render/src/tabs.rs` state preserves accepted drag/drop handlers and
the g16.034 underline/motion observer behavior; this repair does not alter the
accepted drag/drop semantics. Planning/index overlap remains documentation
metadata only: the additive provider denominator is current, while Jetstream's
static catalogue route is deferred registry metadata, not admission or mounted
parity.

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
| 12 Native production first-frame gate | `PreviewRoot::render_component_specimen` passes `false` instead of its committed-frame state | mounted production Skeleton probe fails after the commit callback because the full-mode loop is still absent (`skeleton: ... must enable the full-mode loop`) | restored; 9-specimen probe green |
| 13 Mounted proportional reversal | shared activation uses `existing.axisTo` instead of the semantic target axis | mounted React and Svelte controlled reversal receipts receive `135ms`, not the expected `45ms`, from the live `60px / 80px` clip | restored; both mounted receipts green |
| 14 Exact clipped-height finish cleanup | animation finish deletes the handle without checking exact live-handle identity | replacement runtime receipt expects one live handle and receives zero | restored; runtime receipt green |
| 15 Svelte SSR preloaded ToastStack | initial `visuals` is empty instead of synchronously seeded from authored items | SSR receipt emits an empty list and omits `Saved` | restored; SSR receipt green |
| 16 React controlled close remnant | `Collapsible` keeps content only when `isOpen` | mounted close/reversal receipts see `hidden=true` before the remnant exists | restored; mounted receipt green |

## Evidence

- Paired TS/Rust trace tests: `packages/core/test/motion-policy.test.ts`,
  `packages/contracts/headless/src/motion_policy.rs` (inline tests).
- Web runtime: `packages/core/test/motion-runtime.test.ts`, including
  unsupported-WAAPI cleanup, synchronous replacement identity, and exact
  natural clipped-height handle removal.
- Mounted family receipts: `packages/svelte/components/test/motion-families.test.ts`,
  `packages/svelte/components/test/MotionFamilyHarness.svelte`,
  `packages/svelte/components/src/disclosure-motion.ts`,
  `packages/react/components/test/motion-families.test.tsx`, and
  `packages/react/components/src/disclosure-motion.ts`; the mounted reversal
  cases use a live clipped height and proportional remaining duration.
- Svelte SSR receipt: `packages/svelte/components/test/ssr/ToastStackSsr.test.ts`
  proves authored preloaded items are present and settled on the first paint.
- GPUI mounted regressions: `mounted_motion_policy_construction_does_not_invent_clocks`
  and `production_loading_routes_commit_before_starting_full_mode_loops` in
  `packages/gpui/preview/src/specimen_probe.rs`; the latter mounts the real
  PreviewRoot loading routes and uses the production first-frame commit path.
- Browser probe: `effigy test:motion-policy-browser` (Chromium + WebKit).

## Validation

Focused (post-production-repair):

- `bun test packages/core/test/motion-runtime.test.ts packages/core/test/motion-policy.test.ts packages/core/test/tabs.test.ts` — 37 pass, 0 fail, 132 expect.
- React Tabs/disclosure/family focus — 6 files, 39 pass.
- Svelte Tabs/disclosure/family/ToastStack focus — 6 files, 44 pass; SSR
  ToastStack receipt — 1 file, 1 pass.
- `effigy probe:gpui-specimens` — 9 pass, 0 fail, including the mounted
  production Skeleton/Spinner first-frame gate.
- `effigy test:motion-policy-browser-chromium` — Svelte + React checks pass:
  disclosure, underline resize, preloaded toasts, exit cleanup/focus, and
  reduced IconButton transition.
- `effigy test:motion-policy-browser-webkit` — the same checks pass.
- `effigy docs:check` — pass; 176 evidence rows and zero drift.

Boards:

- `effigy ci:web` — pass: 372 files, 3468 tests; packed consumer 10 files,
  20 tests; 0 Svelte-check errors and 4 existing component warnings.
- `effigy ci:rust` — pass, including 197 headless and 289 component-spec
  tests.
- `effigy ci:native` — pass; all drift checks, render/node-backend, GPUI,
  Jetstream, headless-regression, specimen, dual-dependency, and capture-smoke
  selectors passed. The native board included 167 headless regressions and 9
  GPUI specimen tests.
- Final `effigy qa` — all substantive selectors pass; aggregate exit 1 only at
  `audit:security`. `audit-repository-security.ts` reports an OpenAI-token
  pattern in `PAPERCUTS.md`, the g16.033 handoff/log, and this log because the
  English phrase `mask-plus-translated-highlight` contains a matching
  `sk-` substring with no left boundary. No credential is present. This is the
  known main-baseline false positive recorded in `PAPERCUTS.md` and is outside
  this card's scope.
- Final `git diff --check` and diff-scope/absence checks — pass.

## Remaining explicit boundaries

No remaining in-scope semantic defect was identified. The following are
intentional, documented boundaries:

- Native Tabs has one painted accent indicator in the selected geometry slot;
  inactive slots retain layout-only reserves, and no selected-tab border is
  used.
- GPUI translation/scale remains a named opacity stand-in; native disclosure
  height remains a static endpoint because those channels are not admitted.
- Jetstream has no admission in this card. Any generated/static catalogue
  route is registry metadata only and does not claim Jetstream admission or
  mounted parity; that route remains deferred to a later planning decision.
- The full specimen census needs a 32 GB heap even serially; the papercut is
  recorded and does not block these focused proofs.
