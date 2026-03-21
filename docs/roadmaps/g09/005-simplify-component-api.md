# g09.005 Simplify Component API

Status: complete
Owner: Pug Core
Depends on: g09.004

## Context

GPUI components currently use a `Pug` prefix (`PugButton`, `PugCheckbox`)
which is redundant since the crate name already namespaces them. Developers
also have to construct a separate `ButtonSpec` before passing it to
`PugButton::new()`. The API should be as simple as importing one crate.

## Actions

### Drop Pug prefix

- [ ] Rename all component structs: `PugButton` → `Button`,
      `PugCheckbox` → `Checkbox`, etc. (~100 files)
- [ ] Update `lib.rs` exports
- [ ] Update all specimen files in `packages/gpui/preview/src/specimens/`
- [ ] Update any other internal references (demo_view, etc.)

### Re-export common types

- [ ] From `pug-gpui-components` lib.rs, re-export:
  - `ButtonVariant`, `ButtonTone`, `ControlSize` (from `pug_primitives`)
  - `IconSize`, `IconSpec` (from `pug_primitives`)
  - `StatusTone`, `ValidationState` (from `pug_primitives`)
  - Other commonly used enums
- [ ] This allows: `use pug_gpui_components::{Button, ButtonVariant}`

### Verify

- [ ] `cargo check -p pug-gpui-preview`
- [ ] Grep for any remaining `Pug` prefixed component names

## Acceptance Criteria

- [ ] Zero `PugButton`, `PugCheckbox`, etc. references remain
- [ ] Common types importable from `pug_gpui_components`
- [ ] All preview specimens compile with new names
