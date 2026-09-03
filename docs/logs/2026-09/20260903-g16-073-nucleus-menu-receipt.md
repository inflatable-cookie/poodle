# g16.073 — Nucleus Menu M1 Receipt

Status: complete
Date: 2026-09-03
Card: `docs/roadmaps/g16/073-nucleus-menu-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-064013-g16-073-nucleus-menu-receipt.md`
Governing refs: `docs/roadmaps/g16/nucleus-gpui-parity-programme.md`,
`docs/roadmaps/g16/062-nucleus-parity-receipt-foundation.md`,
`docs/roadmaps/g16/nucleus-parity-manifest.json`,
`docs/roadmaps/g16/parity-evidence-ledger.md`,
`docs/contracts/components/menu.md`,
`docs/contracts/components/icon.md`
Branch: `feature/g16-073-nucleus-menu-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-073-nucleus-menu-receipt`
Planning base: `d1e57b22c` (`origin/main`)
Worker PR: pending

## Outcome

`Menu` now has a validated `M1` execution receipt emitted from the production
GPUI render, node backend, and test-platform path in `effigy regressions:native`.
The new regression `menu_items_semantics_activation_and_identity_rebuild_the_host_spec`
verifies production structure (root menu role, surface fill mix, border opacity,
surface corner radius, overlay elevation shadow, overlay posture, min-width, and padding),
individual item roles (`MenuItem`, `MenuItemCheckBox`, `MenuItemRadio`), separator role
(`Splitter`), focusable states, pointer cursors, hover and focus highlight patches
(`color.accent.base` 16% mix, destructive `color.status.danger` 14% mix), item layout
dimensions (min-height, padding, radius, gap), visible labels and shortcut captions at
the Node boundary, checked indicators rendered via production `check` Icon with accent tint,
unchecked spacers, mounted layout bounds (positive dimensions, panel containment, vertical
row ordering), pointer activation with disabled and separator row inertness, controlled
rebuild of checkbox and radio states through test-platform dispatch, roving keyboard
navigation with ArrowUp/Down skipping inert rows, boundary navigation via Home/End,
wrap-around at edges, Enter and Space activation on focused items with Escape inertness,
and two composed instances focus handle and callback isolation, emitting the M1 receipt
at the terminal boundary. The manifest, all 9 existing receipts (AppHeader, Button, Icon,
IconButton, SegmentedControl, SplitView, Surface, Tabs, Text), and the new Menu receipt
pin the exact runtime source commit `013e473a6dc4b62d8f46c761075cab70a4d6bef2`. The ledger
records 10 mounted Nucleus rows out of 29.

## What landed

- Contracts:
  - `packages/contracts/components/src/menu.rs`: added unit tests validating default spec and builder methods (`open`, `default_open`, `placement`, `dismiss_on_outside_interact`, `aria_label`, `size`, `size_role`, `density`, item counts, and semantic token resolvers) and `MenuEntry` builders/properties (`value`, `label`, `disabled`, `checked`, `shortcut_label`, `kind`, `destructive`).
- Renderer:
  - `packages/render/src/menu.rs`: implemented roving focus keyboard navigation via `poodle_headless::menu::{menu_list_navigate, MenuListMove}`, focus highlight style patches matching hover state per contract §8, non-focusable disabled/separator items with `tab_index = -1` and `cursor: not-allowed`, and focusable enabled items with `tab_index = 0`. Added renderer unit tests for disabled item focusability/activation suppression and roving key navigation skipping disabled items and separators.
- Headless Regressions:
  - `packages/gpui/preview/tests/headless_regressions.rs`: added `menu_items_semantics_activation_and_identity_rebuild_the_host_spec` across 7 phases:
    1. Production Spec & Token Structure Proof: root menu node tokens, layout, and children semantics for action, disabled, separator, destructive, shortcut, checkbox, and radio items.
    2. Mounted Host Setup & Layout Containment: positive dimensions, panel containment, and vertical row ordering.
    3. Pointer Activation, Disabled & Separator Inertness: pointer click dispatch on enabled, disabled, destructive, and shortcut actions.
    4. Controlled Checkbox & Radio State Rebuild: pointer click dispatch updating controlled state and rebuilding checked / unchecked visual representations.
    5. Keyboard Navigation: roving focus with ArrowDown/ArrowUp skipping disabled and separator rows, Home/End boundary jumps, edge wraparound, Enter/Space activation, and Escape inertness.
    6. Two Composed Instances Focus and Callback Isolation: independent focus handles and action callback sinks.
    7. Terminal Receipt Emission.
- Receipts:
  - `docs/roadmaps/g16/nucleus-parity-receipts/menu--nucleus-navigation-menu.json`
  - Refreshed existing receipts for `AppHeader`, `Button`, `Icon`, `IconButton`, `SegmentedControl`, `SplitView`, `Surface`, `Tabs`, `Text` with source commit `013e473a6dc4b62d8f46c761075cab70a4d6bef2`.
- Manifest & Ledger:
  - `docs/roadmaps/g16/nucleus-parity-manifest.json`: updated Menu `expected_test` to `menu_items_semantics_activation_and_identity_rebuild_the_host_spec` and `source_commit` to `013e473a6dc4b62d8f46c761075cab70a4d6bef2`.
  - `docs/roadmaps/g16/parity-evidence-ledger.md`: regenerated via `bun scripts/parity-evidence-ledger.ts --write`; reports 10 mounted rows.

## Review oracle falsification

| Invariant | Smallest counterexample | Required proof / Observed failure |
| --- | --- | --- |
| Production renderer owns structure | substitute raw container without focus patch | `thread 'menu_items_semantics_activation_and_identity_rebuild_the_host_spec' panicked at tests/headless_regressions.rs:23108:9: assertion 'left == right' failed, left: None, right: Some(StylePatch { background: Some(ColorValue(...)) })` |
| Flat Nucleus shape is honest | omit destructive item danger styling | `thread 'menu_items_semantics_activation_and_identity_rebuild_the_host_spec' panicked at tests/headless_regressions.rs:23235:9: text_color must match danger_color` |
| Selectable semantics are exact | collapse checkbox/radio into action rows | `thread 'menu_items_semantics_activation_and_identity_rebuild_the_host_spec' panicked at tests/headless_regressions.rs:23277:9: role must be MenuItemCheckBox` |
| Icon dependency is real | replace check icon with text glyph | `assert_eq!(check.intrinsic_icon(), Some("check"))` fails |
| Input is mounted | invoke handlers directly | mounted observation or callback trace is absent |
| Controlled rebuild is real | omit host rebuild closure on checkbox toggle | checked toggle state remains stale |
| Disabled and separator paths are inert | arm disabled/separator row with click | callback trace receives unexpected value |
| Keyboard navigation is exact | arrow down does not skip separator/disabled | `thread 'menu_items_semantics_activation_and_identity_rebuild_the_host_spec' panicked at tests/headless_regressions.rs:23533:9: ArrowDown must skip separator and disabled item to land on delete` |
| Host ownership stays bounded | infer trigger/open/placement behavior from the panel | receipt validation or scope check fails |
| Receipt is terminal | fail any final mounted assertion | no Menu receipt is emitted |
| Evidence identity is exact | retain stale source commit SHA | `currentSourceMatchesReceipt` fails due to git diff against `SOURCE_PATHS` |
| Levels stay separate | label the receipt A1 or V1 | schema validation fails |

## Validation

Focused:
- `cargo test --manifest-path packages/contracts/components/Cargo.toml --lib menu::` — 8 passed
- `cargo test --manifest-path packages/render/Cargo.toml menu` — 15 passed
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test headless_regressions menu_items_semantics_activation_and_identity_rebuild_the_host_spec` — passed
- `bun test scripts/nucleus-parity-receipts.test.ts` — 8 passed
- `bun test scripts/parity-evidence-ledger.test.ts` — 6 passed
- `bun scripts/parity-evidence-ledger.ts` — validated 176 component evidence rows

Required boards:
- `effigy regressions:native` — 185 passed (all 10 receipts emitted)
- `effigy check:parity-evidence-ledger` — passed
- `effigy ci:rust` — passed
- `effigy ci:native` — passed
- `effigy docs:check` — passed
- `git diff --check origin/main...HEAD` — clean

No windowed or native-visual selectors were run.

## Limits

- `M1` proves mounted production-path render, node backend, panel/item layout containment, and test-platform pointer / roving keyboard navigation dispatch; it does not claim `A1` (accessibility tree verification), `V1` (pixel-level visual comparison), trigger ownership, or nested flyout submenu pointer policy.
- Merge remains orchestrator-owned.
