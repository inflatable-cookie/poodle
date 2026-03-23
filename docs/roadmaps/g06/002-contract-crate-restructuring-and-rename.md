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
| `flint-gpui-tokens` | `flint-tokens` | `packages/gpui/tokens/` | `packages/contracts/tokens/` |
| `flint-gpui-primitives` | `flint-primitives` | `packages/gpui/primitives/` | `packages/contracts/primitives/` |
| `flint-gpui-composites` | `flint-composites` | `packages/gpui/composites/` | `packages/contracts/composites/` |
| `flint-gpui-workstation` | `flint-workstation` | `packages/gpui/workstation/` | `packages/contracts/workstation/` |

### Import Updates

All `use flint_gpui_tokens::` → `use flint_tokens::`
All `use flint_gpui_primitives::` → `use flint_primitives::`
All `use flint_gpui_composites::` → `use flint_composites::`

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

- [x] `flint-tokens` — `cargo check` passes
- [x] `flint-primitives` — `cargo check` passes, 29 tests pass
- [x] `flint-composites` — `cargo check` passes, 10 tests pass
- [x] `flint-workstation` — `cargo check` passes, 6 tests pass
- [x] Zero references to `flint_gpui_*` remain in contracts source
- [x] Old `packages/gpui/` retained for validation artifacts and baselines
