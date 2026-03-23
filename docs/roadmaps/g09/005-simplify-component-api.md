# g09.005 Simplify Component API

Status: complete
Owner: Flint Core
Depends on: g09.004

## Context

GPUI components previously used a `Flint` prefix (`FlintButton`, `FlintCheckbox`)
and required constructing a separate `ButtonSpec` before passing it to the
component. The API should be a single builder chain from one import.

## Completed Actions

### Drop Flint prefix (done in g08)

- [x] Renamed all component structs: `FlintButton` → `Button`, etc.
- [x] Updated `lib.rs` exports
- [x] Updated all specimen files

### Deref containment pattern (97 components)

- [x] Each component owns its spec internally via `Deref<Target = SpecType>`
- [x] `new(theme)` constructor creates default spec (or `new(required_args, theme)`)
- [x] `from_spec(spec, theme)` backward-compat constructor
- [x] 606 forwarded builder methods dropping `with_` prefix
- [x] Naming convention: spec builders have no prefix, GPUI slots use `with_`, callbacks use `on_`

### Re-export common types

- [x] `ButtonVariant`, `ButtonTone`, `ControlSize`, `IconSize`, `IconSpec`,
      `StatusTone`, `ValidationState` re-exported from `flint-gpui-components`

### Subdirectory organization

- [x] Components organized into `src/primitives/` (77) and `src/composites/` (24)
- [x] Matches Svelte directory structure

### Verify

- [x] `cargo check -p flint-gpui-preview` — clean
- [x] `cargo test -p flint-primitives` — 32/32 pass
- [x] `cargo test -p flint-composites` — 9/9 pass
- [x] Jetstream components compile clean

## API Shape

Before:
```rust
use flint_gpui_primitives::{ButtonSpec, ButtonVariant};
use flint_gpui_components::FlintButton;

FlintButton::new(
    ButtonSpec::new().with_variant(ButtonVariant::Primary).with_label("Save"),
    &theme,
).on_click(handler)
```

After:
```rust
use flint_gpui_components::{Button, ButtonVariant};

Button::new(&theme)
    .variant(ButtonVariant::Primary)
    .label("Save")
    .on_click(handler)
```
