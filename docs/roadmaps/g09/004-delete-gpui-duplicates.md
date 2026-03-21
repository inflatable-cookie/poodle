# g09.004 Delete Duplicate GPUI Crates and Update Imports

Status: complete
Owner: Pug Core
Depends on: g09.003

## Context

With specs unified into `pug-primitives` and `pug-composites`, the
GPUI-specific spec crates are redundant and should be deleted. All import
paths need updating.

## Completed Actions

### Update Cargo.toml files

- [x] `packages/gpui/adapter/Cargo.toml` — replaced `pug-gpui-primitives`,
      `pug-gpui-composites` with `pug-primitives`, `pug-composites`
- [x] `packages/gpui/components/Cargo.toml` — same
- [x] `packages/gpui/preview/Cargo.toml` — same

### Update source imports

- [x] Find-and-replace across all `.rs` files in `packages/gpui/`:
  - `pug_gpui_primitives` → `pug_primitives`
  - `pug_gpui_composites` → `pug_composites`
- [x] Handled moved types (GPUI `types.rs` merged into contracts)
- [x] Fixed method signature changes from 002 (specs that lost `*_px()`
      helpers — logic moved into component renderers)

### Delete crates

- [x] Deleted `packages/gpui/primitives/`
- [x] Deleted `packages/gpui/composites/`
- [x] Deleted `packages/gpui/tokens/`

### Verify

- [x] `cargo check -p pug-gpui-components` — clean
- [x] `cargo check -p pug-gpui-preview` — clean
- [x] `cargo check -p pug-jetstream-components` — clean
