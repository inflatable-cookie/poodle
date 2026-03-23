# g09.007 Verify Both Targets

Status: complete
Owner: Poodle Core
Depends on: g09.005, g09.006

## Completed Actions

- [x] `cargo check` for all GPUI crates (adapter, components, preview) — clean
- [x] `cargo check` for all Jetstream crates (adapter, components) — clean
- [x] `cargo test -p poodle-primitives` — 32/32 pass
- [x] `cargo test -p poodle-composites` — 9/9 pass
- [x] Jetstream preview has pre-existing Color/Vec4 type errors unrelated to g09

## Known Issues

- Jetstream preview (`poodle-jetstream-preview`) has 4 pre-existing type mismatch
  errors (`Option<Color>` vs `Option<Vec4>`) in `src/shell.rs`. These predate
  g09 and are caused by a Jetstream runtime API change. Not a g09 regression.
