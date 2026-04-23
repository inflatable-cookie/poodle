# g09.003 Merge Composite Specs

Status: complete
Owner: Poodle Core
Depends on: g09.002

> Historical note: this milestone records the pre-`g10.004` split-crate state.
> The live merged crate is now `poodle-specs`.

## Context

7 modules exist in `poodle-gpui-composites` but not `poodle-composites`. Shared
types in `types.rs` also differ between the two crates.

## Missing Modules (7)

These exist in `poodle-gpui-composites` but not `poodle-composites`:

1. `action_discovery_panel`
2. `app_header`
3. `command_palette`
4. `dock_region`
5. `metric_tile`
6. `shell_status_bar`
7. `split_view`

## Actions

- [x] Copy 7 missing module files into `packages/contracts/components/src/`
- [x] Fix imports: `poodle_gpui_primitives` → `poodle_primitives`,
      `poodle_gpui_tokens` → `poodle_tokens`
- [x] Register modules in `packages/contracts/components/src/lib.rs`
- [x] Merge GPUI types.rs additions into contracts types.rs:
  - `DiscoveryState`, `SplitOrientation`, `DockEdge`
  - `CommandActionItem`, `ActionDiscoverySection`, `PanelTabItem`
- [x] Replace stub `page_header.rs` and `toast_stack.rs` with GPUI versions
      (more complete token methods, `PageHeaderAlign`, `Toast`, `ToastTone`,
      `ToastPosition`)
- [x] Replace stub `detail_section.rs` with GPUI version (more complete API)
- [x] `cargo check -p poodle-composites` — passes
- [x] `cargo test -p poodle-composites` — 9 tests pass

## Acceptance Criteria

- [x] `poodle-composites` contains all modules from both sources
- [x] All composite specs reference `poodle_tokens` and `poodle_primitives`
- [x] Compiles cleanly
