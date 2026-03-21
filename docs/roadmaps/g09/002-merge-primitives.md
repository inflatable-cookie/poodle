# g09.002 Merge GPUI-Only Specs into Contracts Primitives

Status: complete
Owner: Pug Core
Depends on: g09.001

## Context

`pug-gpui-primitives` has 77 modules; `pug-primitives` has 65. The 12 extra
modules in GPUI need to move into the shared crate. Additionally, some specs
that exist in both crates have diverged — GPUI versions added hardcoded pixel
helpers that belong in the renderer, not the spec.

## Missing Modules (12)

These exist in `pug-gpui-primitives` but not `pug-primitives`:

1. `alert_dialog`
2. `breadcrumbs`
3. `bulk_action_bar`
4. `card`
5. `combobox`
6. `detail_row`
7. `icon`
8. `list_card`
9. `nav_card`
10. `order_by`
11. `pagination`
12. `table`

## Diverged Specs

- **ButtonSpec**: Added `chevron: bool` field and `with_chevron()` builder to
  shared spec. Fixed `icon_size_token()` to always return `SIZE_ICON_SM` per
  contract. The `*_px()` methods remain in GPUI only (rendering concerns).
- **ListCardSpec**: Removed `leading_size_px()` and `leading_radius_px()` from
  shared spec (hardcoded pixel values → renderer concern).
- **PaginationSpec**: Removed `page_gap_px()` and `nav_gap_px()` from shared
  spec (hardcoded pixel values → renderer concern).
- **types.rs**: Added `TabVariant` enum to contracts (was only in GPUI).

## Actions

- [x] Copy 12 missing module files into `packages/contracts/primitives/src/`
- [x] Token imports already use `pug_tokens` (files were from post-g09.001 GPUI)
- [x] Register modules in `packages/contracts/primitives/src/lib.rs`
- [x] For each diverged spec:
  - Added `chevron` field to ButtonSpec
  - Fixed `icon_size_token()` to always return `SIZE_ICON_SM`
  - Removed hardcoded pixel helpers from list_card and pagination
- [x] Added `TabVariant` enum to contracts `types.rs`
- [x] `cargo check -p pug-primitives` — passes
- [x] `cargo test -p pug-primitives` — 32 tests pass
- [x] `cargo check -p pug-jetstream-components` — passes (no regression)

## Acceptance Criteria

- [x] `pug-primitives` has all 77 modules
- [x] No hardcoded pixel values in shared spec methods
- [x] Token methods return token path strings, not pixel values
- [x] Jetstream crates still compile
