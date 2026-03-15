# g06.002 — Contract Crate Restructuring and Rename

Status: Completed
Updated: 2026-03-14

## Objective

Rename and restructure the shared Rust crates from GPUI-specific naming to
renderer-neutral naming, and relocate them from `packages/gpui/` to
`packages/contracts/`.

## Changes

### Crate Rename

| Old Crate | New Crate | Old Path | New Path |
|-----------|-----------|----------|----------|
| `pug-gpui-tokens` | `pug-tokens` | `packages/gpui/tokens/` | `packages/contracts/tokens/` |
| `pug-gpui-primitives` | `pug-primitives` | `packages/gpui/primitives/` | `packages/contracts/primitives/` |
| `pug-gpui-composites` | `pug-composites` | `packages/gpui/composites/` | `packages/contracts/composites/` |
| `pug-gpui-workstation` | `pug-workstation` | `packages/gpui/workstation/` | `packages/contracts/workstation/` |

### Import Updates

All `use pug_gpui_tokens::` → `use pug_tokens::`
All `use pug_gpui_primitives::` → `use pug_primitives::`
All `use pug_gpui_composites::` → `use pug_composites::`

Updated across 77 source files spanning all four crates.

### Cargo.toml Updates

- Package names updated to drop `-gpui-` infix
- Descriptions updated to reference "multi-renderer" instead of "GPUI"
- Dependency paths updated to relative `../` within `packages/contracts/`

### Generation Constants

`CURRENT_GENERATION` updated to `"g06.002"` in all three spec crates.

### Token Path

The token crate's `#[path]` directive to `../../../tokens/artifacts/rust/mod.rs`
remains valid — the depth from `packages/contracts/tokens/src/` to
`packages/tokens/` is identical to the old `packages/gpui/tokens/src/` path.

## Verification

- [x] `pug-tokens` — `cargo check` passes
- [x] `pug-primitives` — `cargo check` passes, 29 tests pass
- [x] `pug-composites` — `cargo check` passes, 10 tests pass
- [x] `pug-workstation` — `cargo check` passes, 6 tests pass
- [x] Zero references to `pug_gpui_*` remain in contracts source
- [x] Old `packages/gpui/` retained for validation artifacts and baselines
