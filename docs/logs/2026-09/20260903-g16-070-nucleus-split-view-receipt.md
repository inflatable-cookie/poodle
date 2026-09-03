# g16.070 — Nucleus SplitView M1 Receipt

Status: complete
Date: 2026-09-03
Card: `docs/roadmaps/g16/070-nucleus-split-view-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-041500-g16-070-nucleus-split-view-receipt.md`
Governing refs: `docs/roadmaps/g16/nucleus-gpui-parity-programme.md`,
`docs/roadmaps/g16/062-nucleus-parity-receipt-foundation.md`,
`docs/roadmaps/g16/nucleus-parity-manifest.json`,
`docs/roadmaps/g16/parity-evidence-ledger.md`,
`docs/contracts/components/split-view.md`,
`docs/contracts/components/resize-handle.md`,
`docs/contracts/components/collapse-toggle.md`,
`docs/contracts/components/icon.md`
Branch: `feature/g16-070-nucleus-split-view-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-070-nucleus-split-view-receipt`
Planning base: `3c17a7c94` (`origin/main`)
Worker PR: https://github.com/inflatable-cookie/poodle/pull/176

## Outcome

`SplitView` now has a validated `M1` execution receipt emitted from the
production GPUI render, node backend, and test-platform path in
`effigy regressions:native`. The strengthened existing regression
`two_composed_split_views_do_not_share_a_divider_focus_handle` verifies
production structure (orientation, fill, min sizes, ratio flex bases, overflow
clipping), caller-owned divider instance identity and focus isolation,
test-platform keyboard resize dispatch (axis arrows, Home/End, cross-axis
filtering), primary collapse toggle composition via production CollapseToggle
and Icon renderers, mounted layout child containment and pane ordering, and
emits the M1 receipt at the terminal boundary. The manifest, existing receipts
(AppHeader, Button, Icon, IconButton, Surface, Text), and new SplitView receipt
pin the exact runtime source commit `a92ed7d1c0ce0af713d25dc536c14f41555dabb4`.
The ledger records 7 mounted Nucleus rows.

## What landed

- Contracts & Render:
  - `packages/contracts/components/src/split_view.rs`: added unit tests validating default spec, builder methods, `divider_instance_id` derivation, `current_ratio` resolution and clamping, `keyboard_resize_supported` logic, and `toggles_hidden_until_hover` policy.
  - `packages/render/src/collapse_toggle.rs`: updated `collapse_toggle` to render chevron through production `crate::icon::icon` using `IconSpec`. Pre-fix counterexample: raw `Node::icon` bypassed the production `Icon` renderer, dropping layout direction/alignment metadata (`LayoutDirection::Row`, `CrossAxisAlignment::Center`, `MainAxisAlignment::Center`) and token-resolved tinting.
  - `packages/render/src/resize_handle.rs`: added `fill_height = true` for horizontal and `fill_width = true` for vertical root handle layout. Pre-fix counterexample: without `fill_height`/`fill_width`, `ResizeHandle`'s only child is an absolute grab overlay, causing GPUI's flex layout with `LayoutSizing::Fit` to resolve cross-axis extent to 0px (`Size { 2px × 0px }` for horizontal dividers), failing the mounted positive dimensions assertion.
  - `packages/render/src/split_view.rs`: retained the contract-documented inline toggle cluster layout `centered(cluster_dir).child(handle).child(cluster)` per `docs/contracts/components/split-view.md` §11a Known Deltas; added unit tests covering horizontal/vertical root and pane layout postures, pane min sizes, fixed pane sizes and collapses, disabled split dimming, and handlers forwarding.
- Headless Regressions:
  - `packages/gpui/preview/tests/headless_regressions.rs`: strengthened `two_composed_split_views_do_not_share_a_divider_focus_handle` with full SplitView variant/token verification, focus handle isolation across distinct instance IDs, dispatched arrow keys (+/-8px), saturating Home/End (+/-9999px), cross-axis key filtering, keyboard activation of primary CollapseToggle, mounted bounds checking for positive dimensions, subject pane containment & divider horizontal containment and ordering, witness containment, and terminal M1 receipt emission.
- Receipts:
  - `docs/roadmaps/g16/nucleus-parity-receipts/splitview--nucleus-shell-split-view.json`
  - Refreshed existing receipts for `AppHeader`, `Button`, `Icon`, `IconButton`, `Surface`, `Text` with source commit `a92ed7d1c0ce0af713d25dc536c14f41555dabb4`.
- Manifest & Ledger:
  - `docs/roadmaps/g16/nucleus-parity-manifest.json`: updated `source_commit` to `a92ed7d1c0ce0af713d25dc536c14f41555dabb4`.
  - `docs/roadmaps/g16/parity-evidence-ledger.md`: regenerated via `bun scripts/parity-evidence-ledger.ts --write`; reports 7 mounted rows (AppHeader, Button, Icon, IconButton, SplitView, Surface, Text).

## Review oracle falsification

| Invariant | Smallest counterexample | Required proof / Observed failure |
| --- | --- | --- |
| Production renderer owns structure | substitute raw row/panes for one SplitView | exact structure, metadata, or containment assertion fails |
| Instance identity is caller-owned | derive both divider IDs from shared orientation or label | focusing one divider aliases or replaces the other |
| Input is mounted | call `on_resize` directly | input observation or callback trace is absent |
| Resize ownership is isolated | route both handlers to one sink | the witness receives the subject's key delta or phase |
| Axis and endpoints are exact | accept a cross-axis key or change Home/End delta | `assertion 'left == right' failed: Dispatched arrow keys must produce +/-8px gestures, Home/End saturating, and cross-axis keys must be ignored` |
| Collapse composition is real | replace CollapseToggle or its Icon with a raw button/node | exact action/icon metadata or mounted activation proof fails |
| Layout metadata is exact | collapse ratio, min-size, or orientation mappings | exact Node metadata or mounted bounds fail |
| Receipt is terminal | fail the final mounted assertion | no SplitView receipt is emitted for the current source |
| Evidence identity is exact | retain the g16.069 source SHA | receipt validation fails after source movement |
| Levels stay separate | label the receipt A1 or V1 | schema validation fails |

## Validation

Focused:
- `cargo test --manifest-path packages/contracts/components/Cargo.toml --lib split_view::` — pass
- `cargo test --manifest-path packages/render/Cargo.toml` — pass (130 tests)
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test headless_regressions two_composed_split_views_do_not_share_a_divider_focus_handle` — pass
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

- `M1` proves mounted production-path render, node backend, pane containment,
  and test-platform keyboard resize / collapse activation dispatch; it does not
  claim `A1` (accessibility tree verification) or `V1` (pixel-level visual comparison).
- SplitView native callback reports `ResizePhase` plus axis pixel delta because
  the host owns rendered extent; M1 does not infer ratio parity or drag-to-collapse.
- Merge remains orchestrator-owned.
