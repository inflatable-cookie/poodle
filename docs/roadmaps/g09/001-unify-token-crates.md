# g09.001 Unify Token Crates

Status: complete
Owner: Pug Core
Depends on: —

## Context

`pug-tokens` and `pug-gpui-tokens` both point at the same generated file
(`tokens/artifacts/rust/mod.rs`). The only difference: `pug-tokens` exports
a `typed` module that `pug-gpui-tokens` omits. Two crates for one source is
unnecessary overhead.

## Actions

- [x] Confirm `pug-tokens` already exports `typed` module
- [x] Update `pug-gpui-primitives` Cargo.toml: `pug-gpui-tokens` → `pug-tokens`
- [x] Update `pug-gpui-composites` Cargo.toml: same
- [x] Update `pug-gpui-workstation` Cargo.toml: same
- [x] Find-and-replace `use pug_gpui_tokens::` → `use pug_tokens::` across all
      GPUI crate source files (primitives, composites, workstation)
- [x] Delete `packages/gpui/tokens/` directory
- [x] `cargo check` for all GPUI crates (primitives, composites, workstation,
      components, preview — all pass)

## Acceptance Criteria

- [x] Zero references to `pug-gpui-tokens` or `pug_gpui_tokens` in source
- [x] `packages/gpui/tokens/` directory does not exist
- [x] All GPUI crates compile
