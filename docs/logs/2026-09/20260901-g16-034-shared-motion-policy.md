# g16.034 — Shared Motion Policy And Five-Family Pilot

Status: implemented — awaiting review
Date: 2026-09-01
PR: pending
Implementation commit: `c4ede32be`
Card: `docs/roadmaps/g16/034-shared-motion-policy-and-five-family-pilot.md`
Handoff: `docs/handoffs/20260901-130224-g16-034-shared-motion-policy.md`
Governing refs: `docs/architecture/012-semantic-motion-policy.md`,
`docs/architecture/010-native-presentation-construction-context.md`,
`docs/contracts/components/motion-policy-provider.md`,
`docs/contracts/001-working-rules.md`
Branch: `feature/g16-034-shared-motion-policy`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-034-shared-motion-policy`
Base: `origin/main` at `60eea0e25` (planning base `b89c11275` is an ancestor)

## Outcome

One explicit `full | reduced | frozen` host policy now exists in TypeScript and
Rust. Missing preference is full. Nesting is restriction-only. Presentation
scopes preserve motion. Frozen declares no visual clocks.

The five pilot families consume that policy:

- Accordion / Collapsible: clipped block-axis height plus indicator rotation
  after the first committed frame; reduced/frozen snap; closed panels stay
  inert and hidden immediately.
- ToastStack: keyed enter/exit, inert exit remnants, focus fallback
  next → previous → entered-from. Expiry still belongs to ToastHost.
- Tabs `activeEdge="underline"`: one measured paint-only indicator. First
  layout and resize snap; semantic selection can retarget.
- Checkbox / IconButton: semantic state is immediate; reduced keeps opacity
  only; frozen paints the endpoint.
- Skeleton / Spinner: 1.6s opacity pulse and ring/dot/grid loops only in full
  after the first frame. `Skeleton.animated=false` wins.

GPUI still only drives generic opacity and SVG rotation. Translation/scale
declarations record `opacity-stand-in`. Disclosure height has no native
channel and stays a static-endpoint gap. No ledger cell moved.

## Overlap

The operator-driven drag-fix lane already edits Tabs files:

- `packages/svelte/components/src/Tabs.svelte`
- `packages/react/components/src/Tabs.tsx`
- related Tabs tests and `packages/render/src/tabs.rs` (this lane did not
  edit the native Tabs renderer)

This lane still had to add the underline indicator on the web Tabs shells.
The orchestrator owns merge order.

## Falsification

Proofs were committed at `c4ede32be` before planting.

| Row | Plant | Result |
| --- | --- | --- |
| restriction-only (TS) | `restrictMotionPolicy` returns the child request | `nesting is restriction-only` failed: expected reduced, received full |
| restriction-only (Rust) | same nearest-wins plant | `nesting_is_restriction_only` panicked Full vs Reduced |
| frozen clocks (native) | `animation_for_policy(Frozen, spin, false)` already fails if it returns Some; the spinner test `frozen_and_reduced_schedule_no_ring_clock` is the live proof | green after restore |

Restores used `git checkout --` against the committed proofs, not a dirty
index.

## Validation

Focused: `bun test packages/core/test/motion-policy.test.ts`; Svelte/React
provider, Accordion, Collapsible, Tabs, ToastStack, Skeleton, Spinner,
IconButton; `cargo test` motion_policy, context, motion, frozen spinner.

Required boards still to record on the PR: `effigy ci:web`, `effigy ci:rust`,
`effigy ci:native`, `effigy docs:check`, `effigy qa`,
`git diff --check origin/main...HEAD`.
