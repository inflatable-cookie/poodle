# g09.004 Delete Duplicate GPUI Crates and Update Imports

Status: complete
Owner: Poodle Core
Depends on: g09.003

## Context

With specs unified into `poodle-primitives` and `poodle-composites`, the
GPUI-specific spec crates are redundant and should be deleted. All import
paths need updating.

## Completed Actions

### Update Cargo.toml files

- [x] `packages/gpui/adapter/Cargo.toml` — replaced `poodle-gpui-primitives`,
      `poodle-gpui-composites` with `poodle-primitives`, `poodle-composites`
- [x] `packages/gpui/components/Cargo.toml` — same
- [x] `packages/gpui/preview/Cargo.toml` — same

### Update source imports

- [x] Find-and-replace across all `.rs` files in `packages/gpui/`:
  - `poodle_gpui_primitives` → `poodle_primitives`
  - `poodle_gpui_composites` → `poodle_composites`
- [x] Handled moved types (GPUI `types.rs` merged into contracts)
- [x] Fixed method signature changes from 002 (specs that lost `*_px()`
      helpers — logic moved into component renderers)

### Delete crates

- [x] Deleted `packages/gpui/primitives/`
- [x] Deleted `packages/gpui/composites/`
- [x] Deleted `packages/gpui/tokens/`

### Verify

- [x] `cargo check -p poodle-gpui-components` — clean
- [x] `cargo check -p poodle-gpui-preview` — clean
- [x] `cargo check -p poodle-jetstream-components` — clean
