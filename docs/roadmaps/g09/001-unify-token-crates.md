# g09.001 Unify Token Crates

Status: planned
Owner: Pug Core
Depends on: —

## Context

`pug-tokens` and `pug-gpui-tokens` both point at the same generated file
(`tokens/artifacts/rust/mod.rs`). The only difference: `pug-tokens` exports
a `typed` module that `pug-gpui-tokens` omits. Two crates for one source is
unnecessary overhead.

## Actions

- [ ] Confirm `pug-tokens` already exports `typed` module
- [ ] Update `pug-gpui-primitives` Cargo.toml: `pug-gpui-tokens` → `pug-tokens`
- [ ] Update `pug-gpui-composites` Cargo.toml: same
- [ ] Update `pug-gpui` (adapter) Cargo.toml: same (if it references gpui-tokens)
- [ ] Find-and-replace `use pug_gpui_tokens::` → `use pug_tokens::` across all
      GPUI crate source files
- [ ] Delete `packages/gpui/tokens/` directory
- [ ] `cargo check` for all GPUI crates

## Acceptance Criteria

- [ ] Zero references to `pug-gpui-tokens` or `pug_gpui_tokens` anywhere
- [ ] `packages/gpui/tokens/` directory does not exist
- [ ] All GPUI crates compile
