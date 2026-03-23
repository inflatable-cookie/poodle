# g09.001 Unify Token Crates

Status: complete
Owner: Poodle Core
Depends on: —

## Context

`poodle-tokens` and `poodle-gpui-tokens` both point at the same generated file
(`tokens/artifacts/rust/mod.rs`). The only difference: `poodle-tokens` exports
a `typed` module that `poodle-gpui-tokens` omits. Two crates for one source is
unnecessary overhead.

## Actions

- [x] Confirm `poodle-tokens` already exports `typed` module
- [x] Update `poodle-gpui-primitives` Cargo.toml: `poodle-gpui-tokens` → `poodle-tokens`
- [x] Update `poodle-gpui-composites` Cargo.toml: same
- [x] Update `poodle-gpui-workstation` Cargo.toml: same
- [x] Find-and-replace `use poodle_gpui_tokens::` → `use poodle_tokens::` across all
      GPUI crate source files (primitives, composites, workstation)
- [x] Delete `packages/gpui/tokens/` directory
- [x] `cargo check` for all GPUI crates (primitives, composites, workstation,
      components, preview — all pass)

## Acceptance Criteria

- [x] Zero references to `poodle-gpui-tokens` or `poodle_gpui_tokens` in source
- [x] `packages/gpui/tokens/` directory does not exist
- [x] All GPUI crates compile
