# g16.071 — Nucleus SegmentedControl M1 Receipt

Status: complete
Date: 2026-09-03
Card: `docs/roadmaps/g16/071-nucleus-segmented-control-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-050013-g16-071-nucleus-segmented-control-receipt.md`
Governing refs: `docs/roadmaps/g16/nucleus-gpui-parity-programme.md`,
`docs/roadmaps/g16/062-nucleus-parity-receipt-foundation.md`,
`docs/roadmaps/g16/nucleus-parity-manifest.json`,
`docs/roadmaps/g16/parity-evidence-ledger.md`,
`docs/contracts/components/segmented-control.md`
Branch: `feature/g16-071-nucleus-segmented-control-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-071-nucleus-segmented-control-receipt`
Planning base: `44d23b1ac` (`origin/main`)
Worker PR: https://github.com/inflatable-cookie/poodle/pull/177

## Outcome

`SegmentedControl` now has a validated `M1` execution receipt emitted from the
production GPUI render, node backend, and test-platform path in
`effigy regressions:native`. The strengthened existing regression
`segmented_control_exclusive_focus_identity_and_disabled_paths` verifies
production structure (root radiogroup role, direction, size ladder height,
track background/border/radius, padding/gap), individual segment radiobutton
roles, segment button labels (`NodeKind::Button` and `intrinsic_text`),
selected/toggled state, roving tab stops, typography, truncation, disabled
opacity, focus patches, pointer activation and same-value/disabled inertia,
test-platform directional roving keyboard navigation with independently
legible forward/backward disabled skipping, forward/backward edge wraparound,
arrow keys, Home/End, and Escape inertness, mounted bounds containment and
horizontal ordering, caller-owned option instance identity, and two-instance
focus and callback isolation, emitting the M1 receipt at the terminal
boundary. The manifest, existing receipts (AppHeader, Button, Icon,
IconButton, SplitView, Surface, Text), and new SegmentedControl receipt pin the
exact runtime source commit `cf21e25f1349ed928bd1f3e21800ab0a3ecbcaa1`. The
ledger records 8 mounted Nucleus rows.

## What landed

- Contracts:
  - `packages/contracts/components/src/segmented_control.rs`: added unit tests validating default spec, builder methods, `is_disabled`, `aria_label`, `size_role`, `density`, `selected_fill_token`, controlled value overriding default value, and option icon-only/fallback name and tooltip derivation.
- Headless Regressions:
  - `packages/gpui/preview/tests/headless_regressions.rs`: strengthened `segmented_control_exclusive_focus_identity_and_disabled_paths` with full token and style descriptor verification, segment button labels (`NodeKind::Button` and `intrinsic_text`), disabled segment state (no focus handle, disabled opacity, not-allowed cursor), mounted bounds positive dimensions, horizontal ordering, and track containment, probe channel capture (`structure.identity.*`, `surface.channels.*`, `content.typography.*`), pointer selection and inert same-value/disabled clicks, roving keyboard navigation with distinct forward/backward disabled skipping and edge wrapping steps (ArrowLeft/Right/Up/Down, Home/End, Escape inertness), disabled group inertness, two composed instances focus handle and change sink isolation, and terminal M1 receipt emission.
- Receipts:
  - `docs/roadmaps/g16/nucleus-parity-receipts/segmentedcontrol--nucleus-navigation-segmented-control.json`
  - Refreshed existing receipts for `AppHeader`, `Button`, `Icon`, `IconButton`, `SplitView`, `Surface`, `Text` with source commit `cf21e25f1349ed928bd1f3e21800ab0a3ecbcaa1`.
- Manifest & Ledger:
  - `docs/roadmaps/g16/nucleus-parity-manifest.json`: updated `source_commit` to `cf21e25f1349ed928bd1f3e21800ab0a3ecbcaa1`.
  - `docs/roadmaps/g16/parity-evidence-ledger.md`: regenerated via `bun scripts/parity-evidence-ledger.ts --write`; reports 8 mounted rows (AppHeader, Button, Icon, IconButton, SegmentedControl, SplitView, Surface, Text).

## Review oracle falsification

| Invariant | Smallest counterexample | Required proof / Observed failure |
| --- | --- | --- |
| Production renderer owns structure | substitute raw buttons for the control | exact root/segment metadata fails |
| Exclusive semantics are exact | omit radiogroup/radio selected+toggled state | semantic assertions fail before receipt |
| Identity is caller-scoped | derive option IDs from value alone | the two composed controls alias focus |
| Input is mounted | invoke `on_change` directly | mounted observation or callback trace is absent |
| Controlled rebuild is real | record payload without rebuilding the supplied value | selected/toggled state remains stale |
| Disabled and same-value paths are inert | arm every option identically | callback trace gains a forbidden value |
| Roving focus skips and wraps | focus the disabled option or stop at an edge | exact focus and callback trace fails |
| Visual metadata is exact | change track mix/border/radius or selected segment paint | exact token/style assertion fails |
| Receipt is terminal | fail the final mounted assertion | no SegmentedControl receipt is emitted |
| Evidence identity is exact | retain the g16.070 source SHA | receipt validation fails after source movement |
| Levels stay separate | label the receipt A1 or V1 | schema validation fails |

## Validation

Focused:
- `cargo test --manifest-path packages/contracts/components/Cargo.toml --lib segmented_control::` — pass
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test headless_regressions segmented_control_exclusive_focus_identity_and_disabled_paths` — pass
- `bun test scripts/nucleus-parity-receipts.test.ts` — 8 pass
- `bun test scripts/parity-evidence-ledger.test.ts` — 6 pass
- `bun scripts/parity-evidence-ledger.ts` — validated 176 component evidence rows

Required boards:
- `effigy regressions:native` — 184 pass (all receipts emitted)
- `effigy check:parity-evidence-ledger` — passed
- `effigy ci:rust` — passed
- `effigy ci:native` — passed
- `effigy docs:check` — passed
- `git diff --check origin/main...HEAD` — clean

No windowed or native-visual selectors were run.

## Limits

- `M1` proves mounted production-path render, node backend, track/segment layout containment, and test-platform pointer / roving keyboard navigation dispatch; it does not claim `A1` (accessibility tree verification) or `V1` (pixel-level visual comparison).
- SegmentedControl roving keyboard navigation transfers focus handles and reports selected values; M1 does not infer browser hidden-radio markup or arbitrary custom animation timing.
- Merge remains orchestrator-owned.
