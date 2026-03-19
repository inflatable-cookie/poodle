# g09.004 Delete Duplicate GPUI Crates and Update Imports

Status: planned
Owner: Pug Core
Depends on: g09.003

## Context

With specs unified into `pug-primitives` and `pug-composites`, the
GPUI-specific spec crates are redundant and should be deleted. All import
paths need updating.

## Actions

### Update Cargo.toml files

- [ ] `packages/gpui/adapter/Cargo.toml` — replace `pug-gpui-primitives`,
      `pug-gpui-composites` with `pug-primitives`, `pug-composites`
- [ ] `packages/gpui/components/Cargo.toml` — same
- [ ] `packages/gpui/preview/Cargo.toml` — same

### Update source imports

- [ ] Find-and-replace across all `.rs` files in `packages/gpui/`:
  - `pug_gpui_primitives` → `pug_primitives`
  - `pug_gpui_composites` → `pug_composites`
- [ ] Handle any moved types (e.g. if GPUI `types.rs` had types that are
      now in `pug_primitives::types`)
- [ ] Fix any method signature changes from 002 (specs that lost `*_px()`
      helpers — move that logic into the component renderer)

### Delete crates

- [ ] Delete `packages/gpui/primitives/`
- [ ] Delete `packages/gpui/composites/`

### Verify

- [ ] `cargo check -p pug-gpui-components`
- [ ] `cargo check -p pug-gpui-preview`
- [ ] `cargo check -p pug-jetstream-components` (no regression)

## Acceptance Criteria

- [ ] Zero references to `pug_gpui_primitives` or `pug_gpui_composites`
- [ ] Deleted directories do not exist
- [ ] All GPUI and Jetstream crates compile
