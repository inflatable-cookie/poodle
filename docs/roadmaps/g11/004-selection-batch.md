# g11.004 Selection Batch

Status: planned
Owner: Poodle Core
Depends on: contract audit

## Components

checkbox, radio_group, switch, tri_state_switch, toggle, toggle_group,
segmented_control, select

## Structural Issues

- [ ] `toggle` — contract exists (`toggle.md`) but **no Rust spec** in
      `packages/contracts/primitives/src/`. GPUI has `toggle.rs` with direct
      fields (no spec struct). Need to create `ToggleSpec` or align with
      whatever the contract defines.
- [ ] `toggle_group` — contract exists (`toggle-group.md`) but **no Rust spec**.
      GPUI has `toggle_group.rs`. Need to create `ToggleGroupSpec`.

## Per-Component Compliance

- [ ] checkbox — audit against `docs/contracts/foundation/checkbox.md`
- [ ] radio_group — audit against `docs/contracts/foundation/radio-group.md`
- [ ] switch — audit against `docs/contracts/foundation/switch.md`
- [ ] tri_state_switch — audit against `docs/contracts/foundation/tri-state-switch.md`
- [ ] toggle — audit against `docs/contracts/foundation/toggle.md`
- [ ] toggle_group — audit against `docs/contracts/foundation/toggle-group.md`
- [ ] segmented_control — audit against `docs/contracts/foundation/segmented-control.md`
- [ ] select — audit against `docs/contracts/foundation/select.md`
