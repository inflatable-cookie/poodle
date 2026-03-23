# g11.007 Layout Batch

Status: planned
Owner: Poodle Core
Depends on: contract audit

## Components

stack, grid, box, surface, surface_elevation, card, region, separator, spacer,
resize_handle, scroll_shell, toolbar, form_actions

## Structural Issues

- [ ] `box` — contract is `box.md`, Rust spec is `box.rs`, but GPUI uses `bx.rs`
      (Rust keyword conflict). This is expected — just verify the GPUI `Box` struct
      maps to the `BoxSpec` correctly.
- [ ] `spacer` — contract exists (`spacer.md`) but **no Rust spec**. GPUI has
      `spacer.rs` as a zero-field unit struct. May not need a spec if the contract
      defines it as purely structural.
- [ ] `surface_elevation` — contract exists (`surface-elevation.md`) but **no Rust
      spec, no Svelte component, no GPUI component**. Determine if this is a
      standalone component or a sub-pattern of Surface. If standalone, implement
      across all layers. If not, remove the contract.
- [ ] `form_actions` — verify Rust spec `FormActionsSpec` has `FormActionAlign`
      enum matching contract

## Per-Component Compliance

- [ ] stack — audit against `docs/contracts/foundation/stack.md`
- [ ] grid — audit against `docs/contracts/foundation/grid.md`
- [ ] box — audit against `docs/contracts/foundation/box.md`
- [ ] surface — audit against `docs/contracts/foundation/surface.md`
- [ ] surface_elevation — audit against `docs/contracts/foundation/surface-elevation.md`
- [ ] card — audit against `docs/contracts/foundation/card.md`
- [ ] region — audit against `docs/contracts/foundation/region.md`
- [ ] separator — audit against `docs/contracts/foundation/separator.md`
- [ ] spacer — audit against `docs/contracts/foundation/spacer.md`
- [ ] resize_handle — audit against `docs/contracts/foundation/resize-handle.md`
- [ ] scroll_shell — audit against `docs/contracts/foundation/scroll-shell.md`
- [ ] toolbar — audit against `docs/contracts/foundation/toolbar.md`
- [ ] form_actions — audit against `docs/contracts/foundation/form-actions.md`
