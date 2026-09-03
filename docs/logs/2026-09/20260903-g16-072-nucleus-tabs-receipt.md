# g16.072 — Nucleus Tabs M1 Receipt

Status: complete
Date: 2026-09-03
Card: `docs/roadmaps/g16/072-nucleus-tabs-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-054548-g16-072-nucleus-tabs-receipt.md`
Governing refs: `docs/roadmaps/g16/nucleus-gpui-parity-programme.md`,
`docs/roadmaps/g16/062-nucleus-parity-receipt-foundation.md`,
`docs/roadmaps/g16/nucleus-parity-manifest.json`,
`docs/roadmaps/g16/parity-evidence-ledger.md`,
`docs/contracts/components/tabs.md`
Branch: `feature/g16-072-nucleus-tabs-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-072-nucleus-tabs-receipt`
Planning base: `ec62f0a03` (`origin/main`)
Worker PR: https://github.com/inflatable-cookie/poodle/pull/178
Merged: `99978c1ec581dd5fdd1c5534941b1e6f4a052d47`

## Outcome

`Tabs` now has a validated `M1` execution receipt emitted from the production
GPUI render, node backend, and test-platform path in `effigy regressions:native`.
The strengthened existing regression `tabs_drag_keyboard_and_identity_rebuild_the_host_spec`
verifies production structure (root tablist role, horizontal/vertical orientation,
aria label, layout direction), individual tab roles, visible tab labels at Node
boundary (`intrinsic_text` and text child matching), selected state, roving tab
stops (`a11y.tab_index` 0 / -1), focus patches, controls target association,
disabled opacity and inertness, closable tab close buttons with `x` icon and pointer activation,
tabpanel role with `labelled_by` association and intrinsic panel text, probe channel
capture (`structure.identity.*`, `content.typography.*`, `surface.channels.*`),
mounted layout bounds (positive dimensions, horizontal/vertical containment,
panel placement below tablist), pointer activation with disabled tab inertness,
roving keyboard navigation (ArrowLeft/Right, Home, End, disabled tab skipping,
Up/Down axis inertness for horizontal tabs), manual activation mode (decoupled
focus movement via arrows with explicit Enter/Space activation), vertical orientation
mode (column layout, vertical ArrowUp/Down navigation with disabled skipping and
horizontal key inertness via controlled host rebuild), keyboard reordering (Alt+ArrowLeft/Right with focus
retention), pointer drag-and-drop reordering lifecycle (sub-threshold movement inertness,
drag start threshold, dragged item opacity, drop target indicator shadow ring,
self-drop refusal, and Escape drag cancellation), and two composed instances focus handle
and callback isolation, emitting the M1 receipt at the terminal boundary. The manifest, all existing
receipts (AppHeader, Button, Icon, IconButton, SegmentedControl, SplitView, Surface,
Text), and the new Tabs receipt pin the exact runtime source commit
`f33123492acfecd2ddb37cea932bc1553ee7022a`. The ledger records 9 mounted Nucleus rows.

## What landed

- Contracts:
  - `packages/contracts/components/src/tabs.rs`: added unit tests validating default spec, builder methods, orientation, activation mode, reorderable, bordered, full-width, aria-label, history key, size, size_role, density, current value resolution order (explicit > default > first enabled fallback > empty None), token helpers and visual properties, and TabDefinition builders.
- Headless Regressions:
  - `packages/gpui/preview/tests/headless_regressions.rs`: strengthened `tabs_drag_keyboard_and_identity_rebuild_the_host_spec` across 7 phases:
    1. Structure, token styling, focus patches, visible labels, close button `x` icons and pointer activation, tabpanel association, probe capture, positive layout bounds, horizontal containment, panel placement, disabled tab click inertness, roving keyboard navigation (ArrowLeft/Right, Home, End, disabled skip, vertical key inertness), and Alt+Arrow keyboard reordering with focus retention.
    2. Manual activation mode verifying focus movement without immediate value mutation, and commit on Enter / Space.
    3. Vertical orientation mode verifying column layout, vertical ArrowDown/Up navigation skipping disabled tabs, and horizontal key inertness via controlled host rebuild.
    4. Pointer drag-and-drop reordering lifecycle verifying sub-threshold movement inertness (< 4px), drag start threshold, dragged item opacity, drop target shadow ring, self-drop rejection, and reorder commit.
    5. Pointer drag cancellation via Escape restoring state without reorder.
    6. Two composed instances verifying caller-scoped runtime IDs, isolated focus handles, independent focus state, and separate change sinks.
    7. Terminal M1 receipt emission.
- Receipts:
  - `docs/roadmaps/g16/nucleus-parity-receipts/tabs--nucleus-navigation-tabs.json`
  - Refreshed existing receipts for `AppHeader`, `Button`, `Icon`, `IconButton`, `SegmentedControl`, `SplitView`, `Surface`, `Text` with source commit `f33123492acfecd2ddb37cea932bc1553ee7022a`.
- Manifest & Ledger:
  - `docs/roadmaps/g16/nucleus-parity-manifest.json`: updated `source_commit` to `f33123492acfecd2ddb37cea932bc1553ee7022a`.
  - `docs/roadmaps/g16/parity-evidence-ledger.md`: regenerated via `bun scripts/parity-evidence-ledger.ts`; reports 9 mounted rows (AppHeader, Button, Icon, IconButton, SegmentedControl, SplitView, Surface, Text, Tabs).

## Review oracle falsification

| Invariant | Smallest counterexample | Required proof / Observed failure |
| --- | --- | --- |
| Production renderer owns structure | substitute `NodeRole::RadioGroup` for `NodeRole::TabList` | `thread 'tabs_drag_keyboard_and_identity_rebuild_the_host_spec' panicked at tests/headless_regressions.rs:4550:18: tablist` |
| Names and relationships are exact | assert close button icon is `"close"` instead of `"x"` | `thread 'tabs_drag_keyboard_and_identity_rebuild_the_host_spec' panicked at tests/headless_regressions.rs:4613:18: close button must contain 'close' icon with size.icon.sm` |
| Identity is caller-scoped | assert beta keyboard navigation fires alpha change handler | `thread 'tabs_drag_keyboard_and_identity_rebuild_the_host_spec' panicked at tests/headless_regressions.rs:5418:9: assertion 'left == right' failed: Keyboard navigation on beta instance must not fire alpha change handler, left: [], right: ["other"]` |
| Input is mounted | invoke handlers directly | mounted observation or callback trace is absent |
| Controlled rebuild is real | omit host rebuild closure in vertical mode | vertical Up arrow fails to select previous enabled tab because context retains initial value |
| Activation modes stay distinct | automatic focus does not select or manual focus selects early | exact focus/change traces fail |
| Axis and disabled behavior are exact | vertical Up arrow lands on disabled tab `skip` instead of `one` | `thread 'tabs_drag_keyboard_and_identity_rebuild_the_host_spec' panicked at tests/headless_regressions.rs:5017:9: assertion 'left == right' failed: Up arrow in vertical tabs selects previous enabled tab, left: "one", right: "skip"` |
| Close behavior is exact | close the wrong tab or retain a removed tab | close trace and rebuilt tree fail |
| Reorder lifecycle is exact (sub-threshold) | 1.0px pointer move arms drag start | `thread 'tabs_drag_keyboard_and_identity_rebuild_the_host_spec' panicked at tests/headless_regressions.rs:5179:9: assertion 'left == right' failed: Sub-threshold movement must not arm drag start, left: [], right: ["one"]` |
| Reorder lifecycle is exact (self-drop) | self-drop target hovering sets drop target to `Some("one")` | `thread 'tabs_drag_keyboard_and_identity_rebuild_the_host_spec' panicked at tests/headless_regressions.rs:5205:9: assertion 'left == right' failed: Self-drop must be rejected, left: None, right: Some("one")` |
| Receipt is terminal | fail any final mounted assertion | no Tabs receipt is emitted |
| Evidence identity is exact | retain stale source commit SHA | `currentSourceMatchesReceipt` fails due to git diff against `SOURCE_PATHS` |
| Levels stay separate | label the receipt A1 or V1 | schema validation fails |

## Validation

Focused:
- `cargo test --manifest-path packages/contracts/components/Cargo.toml --lib tabs::` — 10 passed
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test headless_regressions tabs_drag_keyboard_and_identity_rebuild_the_host_spec` — passed
- `bun test scripts/nucleus-parity-receipts.test.ts` — 8 passed
- `bun test scripts/parity-evidence-ledger.test.ts` — 6 passed
- `bun scripts/parity-evidence-ledger.ts` — validated 176 component evidence rows

Required boards:
- `effigy regressions:native` — 184 passed (all 9 receipts emitted)
- `effigy check:parity-evidence-ledger` — passed
- `effigy ci:rust` — passed
- `effigy ci:native` — passed
- `effigy docs:check` — passed
- `git diff --check origin/main...HEAD` — clean

No windowed or native-visual selectors were run.

## Limits

- `M1` proves mounted production-path render, node backend, tablist/tab/panel layout containment, and test-platform pointer / roving keyboard navigation / drag-and-drop dispatch; it does not claim `A1` (accessibility tree verification) or `V1` (pixel-level visual comparison).
- Merge remains orchestrator-owned.
