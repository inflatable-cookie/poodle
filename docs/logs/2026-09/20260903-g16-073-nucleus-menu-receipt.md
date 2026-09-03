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
Worker PR: `https://github.com/inflatable-cookie/poodle/pull/179`

## Outcome

`Menu` now has a validated `M1` execution receipt emitted from the production
GPUI render, node backend, and test-platform path in `effigy regressions:native`.
The new regression `menu_items_semantics_activation_and_identity_rebuild_the_host_spec`
verifies production structure (root menu role, surface fill mix, border opacity,
surface corner radius, overlay elevation shadow, overlay posture, min-width, and padding),
individual item roles (`MenuItem`, `MenuItemCheckBox`, `MenuItemRadio`), separator role
(`Splitter`), single-entry tab posture (`tab_index = 0` on first enabled item, `-1` on all
subsequent enabled, disabled, and separator items), focusable states, pointer cursors, hover
and focus highlight patches (`color.accent.base` 16% mix, destructive `color.status.danger` 14% mix),
item layout dimensions (min-height, padding, radius, gap), visible labels and shortcut captions at
the Node boundary, checked indicators rendered via production `check` Icon with accent tint on both
checkbox and radio items, unchecked spacers, mounted layout bounds (positive dimensions, panel containment,
vertical row ordering), pointer activation with disabled and separator row inertness, controlled
rebuild of checkbox and radio states through test-platform dispatch, roving keyboard
navigation with ArrowUp/Down skipping inert rows, boundary navigation via Home/End,
wrap-around at edges, Enter and Space activation on focused items with Escape inertness,
and mounted single-entry tab-stop enforcement, emitting the M1 receipt at the terminal boundary.
The manifest, all 9 existing receipts (AppHeader, Button, Icon, IconButton, SegmentedControl,
SplitView, Surface, Tabs, Text), and the new Menu receipt pin the exact runtime source commit
`3f4f872af363476027e0b56aa06ccc5a0378efdb`. The ledger records 10 mounted Nucleus rows out of 29.

## What landed

- Contracts:
  - `packages/contracts/components/src/menu.rs`: added unit tests validating default spec and builder methods (`open`, `default_open`, `placement`, `dismiss_on_outside_interact`, `aria_label`, `size`, `size_role`, `density`, item counts, and semantic token resolvers) and `MenuEntry` builders/properties (`value`, `label`, `disabled`, `checked`, `shortcut_label`, `kind`, `destructive`).
- Renderer:
  - `packages/render/src/menu.rs`: implemented roving focus keyboard navigation via `poodle_headless::menu::{menu_list_navigate, MenuListMove}`, focus highlight style patches matching hover state per contract §8, non-focusable disabled/separator items with `tab_index = -1` and `cursor: not-allowed`, and single-entry focusable enabled items with `tab_index = 0` for the first enabled item and `tab_index = -1` for all subsequent enabled items. Added renderer unit tests for disabled item focusability/activation suppression, roving key navigation skipping disabled items and separators, and single-entry tab-stop assignment.
- Headless Regressions:
  - `packages/gpui/preview/tests/headless_regressions.rs`: added `menu_items_semantics_activation_and_identity_rebuild_the_host_spec` across 6 phases:
    1. Production Spec & Token Structure Proof: root menu node tokens, layout, single-entry tab posture, and children semantics for action, disabled, separator, destructive, shortcut, checkbox, and radio items.
    2. Mounted Host Setup & Layout Containment: positive dimensions, panel containment, and vertical row ordering.
    3. Pointer Activation, Disabled & Separator Inertness: pointer click dispatch on enabled, disabled, destructive, and shortcut actions.
    4. Controlled Checkbox & Radio State Rebuild: pointer click dispatch updating controlled state and rebuilding checked / unchecked visual representations with production check Icon metadata on both checkbox and radio rows.
    5. Keyboard Navigation & Mounted Single-Entry Tab Posture: roving focus with ArrowDown/ArrowUp skipping disabled and separator rows, Home/End boundary jumps, edge wraparound, Enter/Space activation, Escape inertness, and mounted verification that exactly one enabled row has `tab_index = 0`.
    6. Terminal Receipt Emission.
- Receipts:
  - `docs/roadmaps/g16/nucleus-parity-receipts/menu--nucleus-navigation-menu.json`
  - Refreshed existing receipts for `AppHeader`, `Button`, `Icon`, `IconButton`, `SegmentedControl`, `SplitView`, `Surface`, `Tabs`, `Text` with source commit `3f4f872af363476027e0b56aa06ccc5a0378efdb`.
- Manifest & Ledger:
  - `docs/roadmaps/g16/nucleus-parity-manifest.json`: updated Menu `expected_test` to `menu_items_semantics_activation_and_identity_rebuild_the_host_spec` and `source_commit` to `3f4f872af363476027e0b56aa06ccc5a0378efdb`.
  - `docs/roadmaps/g16/parity-evidence-ledger.md`: regenerated via `bun scripts/parity-evidence-ledger.ts`; reports 10 mounted rows.

## Review oracle falsification

| Invariant | Smallest counterexample | Required proof / Observed failure |
| --- | --- | --- |
| Production renderer owns structure | substitute raw container without focus patch | `assertion 'left == right' failed, left: None, right: Some(StylePatch { background: Some(ColorValue(...)) })` |
| Flat Nucleus shape is honest | omit destructive item danger styling | `text_color must match danger_color` |
| Single-entry tab posture is exact | assign tab_index=0 to every enabled item | `Menu must have exactly one single tab_index=0 entry stop` fails |
| Selectable semantics are exact | collapse checkbox/radio into action rows | `role must be MenuItemCheckBox` fails |
| Icon dependency is real | replace check icon with text glyph or omit radio check icon | `matches!(&radio_check.kind, NodeKind::Icon { name, size } if name == "check" ...)` fails |
| Input is mounted | invoke handlers directly | mounted observation or callback trace is absent |
| Controlled rebuild is real | omit host rebuild closure on checkbox/radio toggle | checked toggle state remains stale |
| Disabled and separator paths are inert | arm disabled/separator row with click | callback trace receives unexpected value |
| Keyboard navigation is exact | arrow down does not skip separator/disabled | `ArrowDown must skip separator and disabled item to land on delete` fails |
| Host ownership stays bounded | infer trigger/open/placement behavior from the panel | receipt validation or scope check fails |
| Receipt is terminal | fail any final mounted assertion | no Menu receipt is emitted |
| Evidence identity is exact | retain stale source commit SHA | `currentSourceMatchesReceipt` fails due to git diff against `SOURCE_PATHS` |
| Levels stay separate | label the receipt A1 or V1 | schema validation fails |

## Validation

Focused:
- `cargo test --manifest-path packages/contracts/components/Cargo.toml --lib menu::` — 8 passed
- `cargo test --manifest-path packages/render/Cargo.toml menu` — 16 passed
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

- `M1` proves mounted production-path render, node backend, panel/item layout containment, single-entry tab posture, and test-platform pointer / roving keyboard navigation dispatch; it does not claim `A1` (accessibility tree verification), `V1` (pixel-level visual comparison), trigger ownership, or nested flyout submenu pointer policy.
- Multi-instance concurrent mounting: `MenuSpec` in `poodle-specs` and `poodle_render::menu` does not have an `instance_scope` / `instance_id` property (unlike `TabsSpec`, `RadioGroupSpec`, `SegmentedControlSpec`), so production `menu()` emits static `menu-item:{value}` runtime IDs. Per the card stop condition, synthetic post-render ID mutations have been removed and no public API widening was introduced. Concurrent multi-instance isolation across duplicate-valued menu panels remains an open planning consideration for when composite overlay hosts (`MenuButton` / `Popover`) are introduced.
- Merge remains orchestrator-owned.
