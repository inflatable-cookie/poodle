# g11.006 Navigation Batch

Status: planned
Owner: Flint Core
Depends on: contract audit

## Components

tabs, tab_strip, breadcrumbs, pagination, menu, menubar, navigation_menu,
context_menu

## Structural Issues

- [ ] `tab_strip` — contract exists but **no Svelte `TabStrip.svelte`** (Svelte
      uses `Tabs.svelte` for both tab variants). GPUI has separate `tab_strip.rs`.
      Verify this is the intended split or if tab_strip should be merged into tabs.
- [ ] `breadcrumbs` — lives in `primitives/` in GPUI (moved in g11.001), verify
      Rust spec `BreadcrumbsSpec` is in `flint-primitives` not `flint-composites`
- [ ] `pagination` — verify Rust spec `PaginationSpec` is complete against contract

## Per-Component Compliance

- [ ] tabs — audit against `docs/contracts/foundation/tabs.md`
- [ ] tab_strip — audit against `docs/contracts/foundation/tab-strip.md`
- [ ] breadcrumbs — audit against `docs/contracts/foundation/breadcrumbs.md`
- [ ] pagination — audit against `docs/contracts/foundation/pagination.md`
- [ ] menu — audit against `docs/contracts/foundation/menu.md`
- [ ] menubar — audit against `docs/contracts/foundation/menubar.md`
- [ ] navigation_menu — audit against `docs/contracts/foundation/navigation-menu.md`
- [ ] context_menu — audit against `docs/contracts/foundation/context-menu.md`
