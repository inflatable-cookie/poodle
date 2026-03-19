# g09.003 Merge Composite Specs

Status: planned
Owner: Pug Core
Depends on: g09.002

## Context

7 modules exist in `pug-gpui-composites` but not `pug-composites`. Shared
types in `types.rs` also differ between the two crates.

## Missing Modules (7)

These exist in `pug-gpui-composites` but not `pug-composites`:

1. `action_discovery_panel`
2. `app_header`
3. `command_palette`
4. `dock_region`
5. `metric_tile`
6. `shell_status_bar`
7. `split_view`

## Actions

- [ ] Copy 7 missing module files into `packages/contracts/composites/src/`
- [ ] Update token imports: `pug_gpui_tokens` → `pug_tokens`
- [ ] Update primitives imports: `pug_gpui_primitives` → `pug_primitives`
- [ ] Register modules in `packages/contracts/composites/src/lib.rs`
- [ ] Merge GPUI `types.rs` additions into contracts `types.rs`
- [ ] `cargo check -p pug-composites`

## Acceptance Criteria

- [ ] `pug-composites` contains all modules from both sources
- [ ] All composite specs reference `pug_tokens` and `pug_primitives`
- [ ] Compiles cleanly
