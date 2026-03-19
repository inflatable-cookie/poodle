# g09.007 Verify Both Targets

Status: planned
Owner: Pug Core
Depends on: g09.005, g09.006

## Actions

- [ ] `cargo check` for all GPUI crates (adapter, components, preview)
- [ ] `cargo check` for all Jetstream crates (adapter, components, preview)
- [ ] `cargo test` for `pug-primitives`
- [ ] `cargo test` for `pug-composites` (if tests exist)
- [ ] `cargo test` for `pug-gpui` (adapter)
- [ ] `cargo test` for `pug-jetstream` (adapter)
- [ ] Run GPUI preview app — confirm components render
- [ ] Verify Jetstream preview compiles

## Acceptance Criteria

- [ ] Zero compilation errors across all crates
- [ ] All existing tests pass
- [ ] GPUI preview renders correctly
