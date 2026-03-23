# g09.003 Merge Composite Specs

Status: complete
Owner: Flint Core
Depends on: g09.002

## Context

7 modules exist in `flint-gpui-composites` but not `flint-composites`. Shared
types in `types.rs` also differ between the two crates.

## Missing Modules (7)

These exist in `flint-gpui-composites` but not `flint-composites`:

1. `action_discovery_panel`
2. `app_header`
3. `command_palette`
4. `dock_region`
5. `metric_tile`
6. `shell_status_bar`
7. `split_view`

## Actions

- [x] Copy 7 missing module files into `packages/contracts/composites/src/`
- [x] Fix imports: `flint_gpui_primitives` → `flint_primitives`,
      `flint_gpui_tokens` → `flint_tokens`
- [x] Register modules in `packages/contracts/composites/src/lib.rs`
- [x] Merge GPUI types.rs additions into contracts types.rs:
  - `DiscoveryState`, `SplitOrientation`, `DockEdge`
  - `CommandActionItem`, `ActionDiscoverySection`, `PanelTabItem`
- [x] Replace stub `page_header.rs` and `toast_stack.rs` with GPUI versions
      (more complete token methods, `PageHeaderAlign`, `Toast`, `ToastTone`,
      `ToastPosition`)
- [x] Replace stub `detail_section.rs` with GPUI version (more complete API)
- [x] `cargo check -p flint-composites` — passes
- [x] `cargo test -p flint-composites` — 9 tests pass

## Acceptance Criteria

- [x] `flint-composites` contains all modules from both sources
- [x] All composite specs reference `flint_tokens` and `flint_primitives`
- [x] Compiles cleanly
