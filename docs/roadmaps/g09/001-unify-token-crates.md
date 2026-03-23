# g09.001 Unify Token Crates

Status: complete
Owner: Flint Core
Depends on: —

## Context

`flint-tokens` and `flint-gpui-tokens` both point at the same generated file
(`tokens/artifacts/rust/mod.rs`). The only difference: `flint-tokens` exports
a `typed` module that `flint-gpui-tokens` omits. Two crates for one source is
unnecessary overhead.

## Actions

- [x] Confirm `flint-tokens` already exports `typed` module
- [x] Update `flint-gpui-primitives` Cargo.toml: `flint-gpui-tokens` → `flint-tokens`
- [x] Update `flint-gpui-composites` Cargo.toml: same
- [x] Update `flint-gpui-workstation` Cargo.toml: same
- [x] Find-and-replace `use flint_gpui_tokens::` → `use flint_tokens::` across all
      GPUI crate source files (primitives, composites, workstation)
- [x] Delete `packages/gpui/tokens/` directory
- [x] `cargo check` for all GPUI crates (primitives, composites, workstation,
      components, preview — all pass)

## Acceptance Criteria

- [x] Zero references to `flint-gpui-tokens` or `flint_gpui_tokens` in source
- [x] `packages/gpui/tokens/` directory does not exist
- [x] All GPUI crates compile
