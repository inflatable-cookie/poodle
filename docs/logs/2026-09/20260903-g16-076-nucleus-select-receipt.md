# g16.076 — Nucleus Select M1 Receipt

Status: complete
Date: 2026-09-03
Card: `docs/roadmaps/g16/076-nucleus-select-m1.md`
Handoff: `/Users/tom/Dev/projects/poodle/docs/handoffs/20260903-091200-g16-076-nucleus-select-receipt.md`
Governing refs: `docs/roadmaps/g16/nucleus-gpui-parity-programme.md`,
`docs/roadmaps/g16/062-nucleus-parity-receipt-foundation.md`,
`docs/roadmaps/g16/nucleus-parity-manifest.json`,
`docs/roadmaps/g16/parity-evidence-ledger.md`,
`docs/contracts/components/select.md`,
`docs/contracts/components/popover.md`,
`docs/architecture/002-anchored-overlays.md`
Branch: `feature/g16-076-nucleus-select-receipt`
Worktree: `/Users/tom/.paseo/worktrees/1ugbsx1t/g16-076-nucleus-select-receipt`
Planning base: `aae3cdc9f` (`origin/main`)
Worker PR: `https://github.com/inflatable-cookie/poodle/pull/182`

## Outcome

`Select` now has a validated `M1` execution receipt emitted from the
production GPUI render, GPUI Select compat adapter
(`node_compat::Select::from_spec(...).into_element()`), node backend, and
test-platform path in `effigy regressions:native`. The retained regression
`select_two_instances_search_pointer_and_dismiss_through_mounted_rebuilds`
was strengthened — not replaced — into the executed controlled composition.

The fixture proves, through `HeadlessDriver` mounted pointer/keyboard input
and host-owned rebuilds: renderer-owned combobox/listbox relationships,
group labeling, selected/highlighted/disabled option metadata, chevron and
check Icon metadata, and overlay token styling; mounted positive bounds,
containment, and `size.menu.maxHeight` capping without a pixel claim;
caller-scoped duplicate-content isolation (runtime ids, queries, values,
focus, callbacks, layers); disabled whole-control and disabled-option
inertia; OptionsChanged revalidation that refuses a stale cherry highlight;
search editing through the production text-input path (Home/End, query
filtering, caret/selection, option commit, freeform Enter, freeform blur);
clear-to-authored-default; host-opening one instance without dismissing the
other; Escape and outside dismissal on the remaining instance with matching
trigger restoration. The receipt is emitted only after the terminal
assertion.

The manifest, all 12 existing receipts (`AppHeader`, `Button`, `Dialog`,
`Icon`, `IconButton`, `Menu`, `Popover`, `SegmentedControl`, `SplitView`,
`Surface`, `Tabs`, `Text`), and the new Select receipt pin the exact runtime
source commit `939ce87c818e9abd4a759cd3d12af7ed4e41f94d`. The ledger records
13 mounted Nucleus rows out of 29.

## What landed

- Contracts:
  - `packages/contracts/components/src/select.rs`: unit tests for contract
    defaults, the public builder surface, controlled `current_open` /
    `current_value`, elevated overlay fill, `applying_context`, and
    clear-to-authored-default.
  - `packages/contracts/headless/src/select.rs`: machine unit tests for
    disabled inertness, disabled-option commit, highlight skip/last,
    OptionsChanged revalidation, ordered effects, and query highlight.
- Renderer:
  - `packages/render/src/select.rs`: trigger `a11y.controls` names the
    listbox; group headers carry `NodeRole::Group` plus label; disabled
    never presents open (listbox unmounts). Composition test
    `open_composition_owns_exact_structure_tokens_and_option_metadata`.
- GPUI adapter:
  - `packages/gpui/preview/src/node_compat.rs`: crate-internal
    `Select::on_transition` setter for the field the renderer already
    honored. Not a public API.
- Headless regressions:
  - `packages/gpui/preview/tests/headless_regressions.rs`: the retained
    two-instance test now mounts
    `node_compat::Select::from_spec(...).into_element()` through
    `HeadlessDriver::new_element_in_box` across unmounted composition,
    disabled inertia, stale-highlight revalidation, long-menu overflow,
    and the paired-instance receipt path. The retained renderer-mounted
    `a_long_select_menu_clips_overflowing_option_rows` stays.

## Focused repair

No GPUI backend repair was required. Renderer structure, disabled-open
unmount, and the crate-internal transition setter landed with the proof so
the mounted fixture could name those claims. Enter close uses
`dispatch_key_press("enter")` because GPUI synthesizes a trigger click on
Enter key-up after close restores focus.

## Review oracle falsification

| Invariant | Smallest counterexample | Required proof / Observed failure |
| --- | --- | --- |
| Production adapter owns execution | mount `poodle_render::select` directly | the fixture mounts `node_compat::Select::from_spec(...).into_element()` through `HeadlessDriver::new_element_in_box` |
| Instance identity is scoped | compose both children with scope `left` | `focus handle for select:right:trigger never appeared` |
| Input is mounted | call transition handlers directly | every claim is driven through `pointer_activate_id`, `pointer_press`/`release`, or `dispatch_key*` |
| Controlled ownership is real | skip `applying_context` and only record callbacks | `open panel records containment bounds` — listbox never paints |
| Disabled paths are inert | bind trigger activation while enabled | `left: 1 right: 0` on the disabled-phase callback count |
| Highlight revalidates | disable cherry without `OptionsChanged` | `left: Some("cherry") right: Some("cherry")` on `assert_ne!` |
| Search editing is real | replace text-input dispatch with direct query mutation | caret/selection/query assertions in phase 4 would be absent |
| Dismissal is isolated | host-open a sibling without scopes, or Escape both open | host-open leaves the left layer; Escape while both are open still closes the unfocused instance via trigger blur, so the fixture host-closes the sibling then proves Escape/outside on the remaining instance |
| Focus restoration is scoped | restore the other instance trigger | searchable close and Escape/outside assert the matching trigger handle |
| Structure and tokens are exact | drop `a11y.controls` | `left: None right: Some("select:proof:listbox")` |
| Geometry is bounded | drop `max_height` on the panel | `left: None right: Some(240.0)` |
| Receipt is terminal | skip freeform blur so a layer remains | `left: 1 right: 0` `terminal assertion: searchable freeform close left no live layer` |
| Evidence identity is exact | retain the g16.075 source SHA | receipts and manifest pin `939ce87c8`; `currentSourceMatchesReceipt` diffs `SOURCE_PATHS` |
| Levels stay separate | label the receipt A1 or V1 | `scripts/nucleus-parity-receipts.test.ts` rejects `proof_level: "A1"` |

## Validation

Focused:
- `cargo test --manifest-path packages/contracts/components/Cargo.toml --lib select::` — 16 passed
- `cargo test --manifest-path packages/contracts/headless/Cargo.toml --lib select::` — 7 passed
- `cargo test --manifest-path packages/render/Cargo.toml select::` — 29 passed
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test headless_regressions select_two_instances_search_pointer_and_dismiss_through_mounted_rebuilds` — passed
- `cargo test --manifest-path packages/gpui/preview/Cargo.toml --test headless_regressions a_long_select_menu_clips_overflowing_option_rows` — passed
- `bun test scripts/nucleus-parity-receipts.test.ts` — 8 passed
- `bun test scripts/parity-evidence-ledger.test.ts` — 6 passed
- `bun scripts/parity-evidence-ledger.ts --write` then `bun scripts/parity-evidence-ledger.ts` — 176 component evidence rows validated

Required boards:
- `effigy regressions:native` — 187 passed (all 13 receipts emitted at runtime commit `939ce87c8`)
- `effigy check:parity-evidence-ledger` — passed (176 component evidence rows)
- `effigy ci:rust` — passed
- `effigy ci:native` — passed
- `effigy docs:check` — passed
- `git diff --check origin/main...HEAD` — clean

No windowed or native-visual selectors were run.

## Limits

- `M1` proves the mounted production render/adapter/backend path: renderer
  metadata and tokens, controlled rebuilds, search/caret/freeform editing,
  disablement, stale-highlight revalidation, long-menu overflow, sibling
  isolation of host-open, and Escape/outside dismissal of one remaining
  instance. It does not claim `A1` (accessibility tree), `V1` (pixel
  comparison), browser native/custom mode or portal collision parity, or
  Nucleus adoption.
- Two open Selects cannot take a focused sibling Escape the way nested
  Popovers can: focusing the other search blurs an open trigger and
  `emit_blur` closes it. Isolation of two live layers is proved by host-open
  / host-close, not by Escape-while-both-open.
- Non-searchable trigger End/ArrowDown through GPUI does not move highlight;
  skip-disabled Home/End is proved by machine tests and the searchable
  instance.
- Overlay `min_width` of `12rem` is asserted on the Node; GPUI layout did
  not honor a 192px floor. No pixel claim.
- Merge remains orchestrator-owned.
