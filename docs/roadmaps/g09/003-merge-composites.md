# g09.003 Merge Composite Specs

Status: complete
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

- [x] Copy 7 missing module files into `packages/contracts/composites/src/`
- [x] Fix imports: `pug_gpui_primitives` → `pug_primitives`,
      `pug_gpui_tokens` → `pug_tokens`
- [x] Register modules in `packages/contracts/composites/src/lib.rs`
- [x] Merge GPUI types.rs additions into contracts types.rs:
  - `DiscoveryState`, `SplitOrientation`, `DockEdge`
  - `CommandActionItem`, `ActionDiscoverySection`, `PanelTabItem`
- [x] Replace stub `page_header.rs` and `toast_stack.rs` with GPUI versions
      (more complete token methods, `PageHeaderAlign`, `Toast`, `ToastTone`,
      `ToastPosition`)
- [x] Replace stub `detail_section.rs` with GPUI version (more complete API)
- [x] `cargo check -p pug-composites` — passes
- [x] `cargo test -p pug-composites` — 9 tests pass

## Acceptance Criteria

- [x] `pug-composites` contains all modules from both sources
- [x] All composite specs reference `pug_tokens` and `pug_primitives`
- [x] Compiles cleanly
